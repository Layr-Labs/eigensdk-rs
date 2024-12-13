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

interface StakeRegistryHarness {
    type StakeType is uint8;

    event LookAheadPeriodChanged(uint32 oldLookAheadDays, uint32 newLookAheadDays);
    event MinimumStakeForQuorumUpdated(uint8 indexed quorumNumber, uint96 minimumStake);
    event OperatorStakeUpdate(bytes32 indexed operatorId, uint8 quorumNumber, uint96 stake);
    event QuorumCreated(uint8 indexed quorumNumber);
    event StakeTypeSet(StakeType newStakeType);
    event StrategyAddedToQuorum(uint8 indexed quorumNumber, address strategy);
    event StrategyMultiplierUpdated(uint8 indexed quorumNumber, address strategy, uint256 multiplier);
    event StrategyRemovedFromQuorum(uint8 indexed quorumNumber, address strategy);

    constructor(address _registryCoordinator, address _delegationManager, address _avsDirectory, address _serviceManager);

    function MAX_WEIGHING_FUNCTION_LENGTH() external view returns (uint8);
    function WEIGHTING_DIVISOR() external view returns (uint256);
    function addStrategies(uint8 quorumNumber, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
    function applyDelta(uint96 value, int256 delta) external pure returns (uint96);
    function avsDirectory() external view returns (address);
    function calculateDelta(uint96 prev, uint96 cur) external pure returns (int256);
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
    function initializeDelegatedStakeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
    function initializeSlashableStakeQuorum(uint8 quorumNumber, uint96 minimumStake, uint32 lookAheadPeriod, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
    function minimumStakeForQuorum(uint8) external view returns (uint96);
    function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
    function recordOperatorStakeUpdate(bytes32 operatorId, uint8 quorumNumber, uint96 newStake) external returns (int256);
    function recordTotalStakeUpdate(uint8 quorumNumber, int256 stakeDelta) external;
    function registerOperator(address operator, bytes32 operatorId, bytes memory quorumNumbers) external returns (uint96[] memory, uint96[] memory);
    function registryCoordinator() external view returns (address);
    function removeStrategies(uint8 quorumNumber, uint256[] memory indicesToRemove) external;
    function serviceManager() external view returns (address);
    function setMinimumStakeForQuorum(uint8 quorumNumber, uint96 minimumStake) external;
    function setSlashableStakeLookahead(uint8 quorumNumber, uint32 _lookAheadPeriod) external;
    function setStakeType(uint8 quorumNumber, StakeType _stakeType) external;
    function slashableStakeLookAheadPerQuorum(uint8) external view returns (uint32);
    function stakeTypePerQuorum(uint8) external view returns (StakeType);
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
      },
      {
        "name": "_avsDirectory",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      },
      {
        "name": "_serviceManager",
        "type": "address",
        "internalType": "contract IServiceManager"
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
    "name": "applyDelta",
    "inputs": [
      {
        "name": "value",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "delta",
        "type": "int256",
        "internalType": "int256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "stateMutability": "pure"
  },
  {
    "type": "function",
    "name": "avsDirectory",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IAVSDirectory"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "calculateDelta",
    "inputs": [
      {
        "name": "prev",
        "type": "uint96",
        "internalType": "uint96"
      },
      {
        "name": "cur",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "int256",
        "internalType": "int256"
      }
    ],
    "stateMutability": "pure"
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
    "name": "initializeDelegatedStakeQuorum",
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
    "name": "initializeSlashableStakeQuorum",
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
        "name": "lookAheadPeriod",
        "type": "uint32",
        "internalType": "uint32"
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
    "name": "recordOperatorStakeUpdate",
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
        "name": "newStake",
        "type": "uint96",
        "internalType": "uint96"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "int256",
        "internalType": "int256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "recordTotalStakeUpdate",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "stakeDelta",
        "type": "int256",
        "internalType": "int256"
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
    "name": "serviceManager",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IServiceManager"
      }
    ],
    "stateMutability": "view"
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
    "name": "setSlashableStakeLookahead",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "_lookAheadPeriod",
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setStakeType",
    "inputs": [
      {
        "name": "quorumNumber",
        "type": "uint8",
        "internalType": "uint8"
      },
      {
        "name": "_stakeType",
        "type": "uint8",
        "internalType": "enum StakeType"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "slashableStakeLookAheadPerQuorum",
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
        "type": "uint32",
        "internalType": "uint32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "stakeTypePerQuorum",
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
        "type": "uint8",
        "internalType": "enum StakeType"
      }
    ],
    "stateMutability": "view"
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
    "name": "LookAheadPeriodChanged",
    "inputs": [
      {
        "name": "oldLookAheadDays",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      },
      {
        "name": "newLookAheadDays",
        "type": "uint32",
        "indexed": false,
        "internalType": "uint32"
      }
    ],
    "anonymous": false
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
    "name": "StakeTypeSet",
    "inputs": [
      {
        "name": "newStakeType",
        "type": "uint8",
        "indexed": false,
        "internalType": "enum StakeType"
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
pub mod StakeRegistryHarness {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100604052348015610010575f5ffd5b506040516163b23803806163b28339818101604052810190610032919061023b565b83838383838383838373ffffffffffffffffffffffffffffffffffffffff1660e08173ffffffffffffffffffffffffffffffffffffffff16815250508273ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1660a08173ffffffffffffffffffffffffffffffffffffffff16815250508073ffffffffffffffffffffffffffffffffffffffff1660c08173ffffffffffffffffffffffffffffffffffffffff168152505050505050505050505050505061029f565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6101488261011f565b9050919050565b5f6101598261013e565b9050919050565b6101698161014f565b8114610173575f5ffd5b50565b5f8151905061018481610160565b92915050565b5f6101948261013e565b9050919050565b6101a48161018a565b81146101ae575f5ffd5b50565b5f815190506101bf8161019b565b92915050565b5f6101cf8261013e565b9050919050565b6101df816101c5565b81146101e9575f5ffd5b50565b5f815190506101fa816101d6565b92915050565b5f61020a8261013e565b9050919050565b61021a81610200565b8114610224575f5ffd5b50565b5f8151905061023581610211565b92915050565b5f5f5f5f608085870312156102535761025261011b565b5b5f61026087828801610176565b9450506020610271878288016101b1565b9350506040610282878288016101ec565b925050606061029387828801610227565b91505092959194509250565b60805160a05160c05160e0516160bd6102f55f395f81816114de015281816129400152612a3c01525f8181610ee6015281816137d6015261388901525f6114ba01525f8181611f8201526126c301526160bd5ff3fe608060405234801561000f575f5ffd5b506004361061025c575f3560e01c806381c0750211610144578063c601527d116100c1578063df5cf72311610085578063df5cf72314610814578063e086adb314610832578063f2be94ae1461084e578063f509551a1461087e578063f851e198146108ae578063fa28c627146108de5761025c565b8063c601527d1461074c578063c8294c5614610768578063cc5a7c2014610798578063d5eccc05146107b4578063dd9846b9146107e45761025c565b8063adc804da11610108578063adc804da14610684578063b6904b78146106b4578063bc9a40c3146106e4578063bd29b8cd14610700578063c46778a51461071c5761025c565b806381c07502146105a857806386c06856146105d85780639ab4d6ff146105f45780639f3ccf6514610624578063ac6bfb03146106545761025c565b80635401ed27116101dd5780636b3aa72e116101a15780636b3aa72e146104d25780636d14a987146104f057806374454c6d1461050e57806375d4173a1461053e5780637c1723471461055a5780637f429822146105785761025c565b80635401ed27146104085780635e5a6775146104385780635f1f2d771461045657806366acfefe14610472578063697fbd93146104a25761025c565b8063255047771161022457806325504777146103295780632cd959401461035a5780633998fdd31461038a5780633ca5a5f5146103a85780634bd26e09146103d85761025c565b80630390a4d5146102605780630491b41c1461027c57806308732461146102ac5780631f9b74e0146102dd57806320b662981461030d575b5f5ffd5b61027a60048036038101906102759190613a60565b61090e565b005b61029660048036038101906102919190613a9e565b61091d565b6040516102a39190613ae1565b60405180910390f35b6102c660048036038101906102c19190613b24565b610940565b6040516102d4929190613c02565b60405180910390f35b6102f760048036038101906102f29190613c64565b6109ac565b6040516103049190613ca2565b60405180910390f35b61032760048036038101906103229190613d71565b6109d0565b005b610343600480360381019061033e9190613e8a565b610bf4565b604051610351929190613fb2565b60405180910390f35b610374600480360381019061036f9190613fe7565b610ddf565b604051610381919061412b565b60405180910390f35b610392610ee4565b60405161039f919061416b565b60405180910390f35b6103c260048036038101906103bd9190613a9e565b610f08565b6040516103cf9190613ae1565b60405180910390f35b6103f260048036038101906103ed9190613fe7565b610f2b565b6040516103ff9190613ae1565b60405180910390f35b610422600480360381019061041d9190613fe7565b610f5e565b60405161042f9190613ca2565b60405180910390f35b610440610f7a565b60405161044d9190613ae1565b60405180910390f35b610470600480360381019061046b91906142cc565b610f86565b005b61048c60048036038101906104879190613e8a565b6113d4565b6040516104999190614358565b60405180910390f35b6104bc60048036038101906104b79190613a9e565b61149b565b6040516104c991906143e4565b60405180910390f35b6104da6114b8565b6040516104e7919061441d565b60405180910390f35b6104f86114dc565b6040516105059190614445565b60405180910390f35b61052860048036038101906105239190614488565b611500565b60405161053591906144e7565b60405180910390f35b6105586004803603810190610553919061464c565b611515565b005b610562611679565b60405161056f91906146c7565b60405180910390f35b610592600480360381019061058d91906146e0565b61167e565b60405161059f9190613ca2565b60405180910390f35b6105c260048036038101906105bd9190614748565b611691565b6040516105cf919061484d565b60405180910390f35b6105f260048036038101906105ed9190614890565b6118b9565b005b61060e60048036038101906106099190613a9e565b6118cf565b60405161061b91906148dd565b60405180910390f35b61063e60048036038101906106399190613b24565b6118ef565b60405161064b91906148f6565b60405180910390f35b61066e6004803603810190610669919061490f565b611937565b60405161067b919061499f565b60405180910390f35b61069e60048036038101906106999190613b24565b611a19565b6040516106ab91906149f4565b60405180910390f35b6106ce60048036038101906106c99190613b24565b611af6565b6040516106db919061499f565b60405180910390f35b6106fe60048036038101906106f99190614a0d565b611bc8565b005b61071a60048036038101906107159190614a4b565b611be9565b005b61073660048036038101906107319190613a9e565b611c5b565b6040516107439190613ca2565b60405180910390f35b61076660048036038101906107619190614aa8565b611c82565b005b610782600480360381019061077d9190614b02565b611ca3565b60405161078f9190613ca2565b60405180910390f35b6107b260048036038101906107ad9190614b52565b611d82565b005b6107ce60048036038101906107c99190613a9e565b611ef2565b6040516107db9190613ca2565b60405180910390f35b6107fe60048036038101906107f99190614bd2565b611f6b565b60405161080b91906148dd565b60405180910390f35b61081c611f80565b6040516108299190614c42565b60405180910390f35b61084c60048036038101906108479190614c5b565b611fa4565b005b61086860048036038101906108639190614c99565b611fba565b6040516108759190613ca2565b60405180910390f35b61089860048036038101906108939190614cfd565b6120a9565b6040516108a591906144e7565b60405180910390f35b6108c860048036038101906108c39190613fe7565b6120bc565b6040516108d5919061499f565b60405180910390f35b6108f860048036038101906108f39190614bd2565b6121f5565b6040516109059190613ca2565b60405180910390f35b610918828261226a565b505050565b5f60015f8360ff1660ff1681526020019081526020015f20805490509050919050565b6003602052815f5260405f208181548110610959575f80fd5b905f5260205f20015f9150915050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690805f0160149054906101000a90046bffffffffffffffffffffffff16905082565b5f826109b781612499565b5f6109c285856124e4565b509050809250505092915050565b6109d861293e565b846109e281612499565b5f8585905090505f8111610a2b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a2290614dbb565b60405180910390fd5b808484905014610a70576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a6790614e49565b60405180910390fd5b5f60035f8960ff1660ff1681526020019081526020015f2090505f5f90505b82811015610be957858582818110610aaa57610aa9614e67565b5b9050602002016020810190610abf9190614e94565b82898984818110610ad357610ad2614e67565b5b9050602002013581548110610aeb57610aea614e67565b5b905f5260205f20015f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a85818110610b5c57610b5b614e67565b5b9050602002013581548110610b7457610b73614e67565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16888885818110610bb157610bb0614e67565b5b9050602002016020810190610bc69190614e94565b604051610bd4929190614eef565b60405180910390a28080600101915050610a8f565b505050505050505050565b606080610bff612a3a565b5f8484905067ffffffffffffffff811115610c1d57610c1c614194565b5b604051908082528060200260200182016040528015610c4b5781602001602082028036833780820191505090505b5090505f8585905067ffffffffffffffff811115610c6c57610c6b614194565b5b604051908082528060200260200182016040528015610c9a5781602001602082028036833780820191505090505b5090505f5f90505b86869050811015610dcd575f878783818110610cc157610cc0614e67565b5b9050013560f81c60f81b60f81c9050610cd981612499565b5f5f610ce5838d6124e4565b9150915080610d29576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d2090614fac565b60405180910390fd5b5f610d358c8585612aca565b905082878681518110610d4b57610d4a614e67565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff1681525050610d7f848261226a565b868681518110610d9257610d91614e67565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff1681525050505050508080600101915050610ca2565b50818193509350505094509492505050565b606060025f8481526020019081526020015f205f8360ff1660ff1681526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015610ed8578382905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505081526020019060010190610e26565b50505050905092915050565b7f000000000000000000000000000000000000000000000000000000000000000081565b5f60035f8360ff1660ff1681526020019081526020015f20805490509050919050565b5f60025f8481526020019081526020015f205f8360ff1660ff1681526020019081526020015f2080549050905092915050565b5f5f610f6a84846120bc565b9050806040015191505092915050565b670de0b6b3a764000081565b610f8e61293e565b81610f9881612499565b5f825190505f8111610fdf576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610fd69061503a565b60405180910390fd5b5f60035f8660ff1660ff1681526020019081526020015f2090505f60045f8760ff1660ff1681526020019081526020015f2090505f5f90505b838110156113cb578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f78488848151811061105957611058614e67565b5b60200260200101518154811061107257611071614e67565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040516110a991906148f6565b60405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75848884815181106110ea576110e9614e67565b5b60200260200101518154811061110357611102614e67565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f60405161113c929190615091565b60405180910390a2826001848054905061115691906150e5565b8154811061116757611166614e67565b5b905f5260205f20018387838151811061118357611182614e67565b5b60200260200101518154811061119c5761119b614e67565b5b905f5260205f20015f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f820160149054906101000a90046bffffffffffffffffffffffff16815f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055509050508280548061126857611267615118565b5b600190038181905f5260205f20015f5f82015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff02191690555f820160146101000a8154906bffffffffffffffffffffffff02191690555050905581600183805490506112d091906150e5565b815481106112e1576112e0614e67565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168287838151811061131d5761131c614e67565b5b60200260200101518154811061133657611335614e67565b5b905f5260205f20015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508180548061138c5761138b615118565b5b600190038181905f5260205f20015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905590558080600101915050611018565b50505050505050565b5f6113dd612a3a565b5f5f5f5f90505b8585905081101561148d575f86868381811061140357611402614e67565b5b9050013560f81c60f81b60f81c905061141b81612499565b5f5f611427838c6124e4565b9150915080611463575f9150611460838777ffffffffffffffffffffffffffffffffffffffffffffffff16612e6f90919063ffffffff16565b95505b5f61146f8b8585612aca565b905061147b848261226a565b505050505080806001019150506113e4565b508192505050949350505050565b6005602052805f5260405f205f915054906101000a900460ff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b5f61150c848484612aca565b90509392505050565b61151d612a3a565b61152683612e82565b15611566576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161155d906151b5565b60405180910390fd5b6115708382612ea8565b61157a8383613324565b611584835f6133a7565b60015f8460ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505050505050565b602081565b5f6116898383613420565b905092915050565b60605f8383905067ffffffffffffffff8111156116b1576116b0614194565b5b6040519080825280602002602001820160405280156116df5781602001602082028036833780820191505090505b5090505f5f90505b848490508110156118ad575f85858381811061170657611705614e67565b5b9050013560f81c60f81b60f81c905061171e81612499565b8663ffffffff1660015f8360ff1660ff1681526020019081526020015f205f8154811061174e5761174d614e67565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff1611156117af576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016117a690615269565b60405180910390fd5b5f60015f8360ff1660ff1681526020019081526020015f208054905090505f5f90505b8181101561189d578863ffffffff1660015f8560ff1660ff1681526020019081526020015f206001838561180691906150e5565b61181091906150e5565b8154811061182157611820614e67565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff1611611890576001818361185491906150e5565b61185e91906150e5565b85858151811061187157611870614e67565b5b602002602001019063ffffffff16908163ffffffff168152505061189d565b80806001019150506117d2565b50505080806001019150506116e7565b50809150509392505050565b6118c161293e565b6118cb82826133a7565b5050565b6006602052805f5260405f205f915054906101000a900463ffffffff1681565b6004602052815f5260405f208181548110611908575f80fd5b905f5260205f20015f915091509054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b61193f613972565b60025f8481526020019081526020015f205f8560ff1660ff1681526020019081526020015f20828154811061197757611976614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505090509392505050565b611a216139aa565b60035f8460ff1660ff1681526020019081526020015f208281548110611a4a57611a49614e67565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905092915050565b611afe613972565b60015f8460ff1660ff1681526020019081526020015f208281548110611b2757611b26614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905092915050565b611bd061293e565b81611bda81612499565b611be48383613324565b505050565b611bf1612a3a565b5f5f90505b82829050811015611c55575f838383818110611c1557611c14614e67565b5b9050013560f81c60f81b60f81c9050611c2d81612499565b5f611c3986835f612aca565b9050611c45828261226a565b5050508080600101915050611bf6565b50505050565b5f602052805f5260405f205f915054906101000a90046bffffffffffffffffffffffff1681565b611c8a61293e565b81611c9481612499565b611c9e8383612ea8565b505050565b5f5f60015f8660ff1660ff1681526020019081526020015f208381548110611cce57611ccd614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff16815250509050611d73818561345a565b80604001519150509392505050565b611d8a612a3a565b611d9384612e82565b15611dd3576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611dca906151b5565b60405180910390fd5b611ddd8482612ea8565b611de78484613324565b611df28460016133a7565b611dfc8483613516565b60015f8560ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550505050505050565b5f60015f8360ff1660ff1681526020019081526020015f206001805f8560ff1660ff1681526020019081526020015f2080549050611f3091906150e5565b81548110611f4157611f40614e67565b5b905f5260205f20015f0160089054906101000a90046bffffffffffffffffffffffff169050919050565b5f611f778484846135b3565b90509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b611fac61293e565b611fb68282613516565b5050565b5f5f60025f8581526020019081526020015f205f8760ff1660ff1681526020019081526020015f208381548110611ff457611ff3614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff16815250509050612099818661345a565b8060400151915050949350505050565b5f6120b483836136ca565b905092915050565b6120c4613972565b5f60025f8581526020019081526020015f205f8460ff1660ff1681526020019081526020015f208054905090506120f9613972565b5f820361210a5780925050506121ef565b60025f8681526020019081526020015f205f8560ff1660ff1681526020019081526020015f2060018361213d91906150e5565b8154811061214e5761214d614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905080925050505b92915050565b5f60025f8581526020019081526020015f205f8460ff1660ff1681526020019081526020015f206122278585856135b3565b63ffffffff168154811061223e5761223d614e67565b5b905f5260205f20015f0160089054906101000a90046bffffffffffffffffffffffff1690509392505050565b5f5f60015f8560ff1660ff1681526020019081526020015f208054905090505f60015f8660ff1660ff1681526020019081526020015f206001836122ae91906150e5565b815481106122bf576122be614e67565b5b905f5260205f200190505f84036122f557805f0160089054906101000a90046bffffffffffffffffffffffff1692505050612493565b5f61231b825f0160089054906101000a90046bffffffffffffffffffffffff1686613420565b90504363ffffffff16825f015f9054906101000a900463ffffffff1663ffffffff16036123795780825f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555061248c565b43825f0160046101000a81548163ffffffff021916908363ffffffff16021790555060015f8760ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001836bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555050505b8093505050505b92915050565b6124a281612e82565b6124e1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016124d8906152f7565b60405180910390fd5b50565b5f5f5f5f6124f186610f08565b90506124fb6139aa565b60606001808111156125105761250f614371565b5b60055f8a60ff1660ff1681526020019081526020015f205f9054906101000a900460ff16600181111561254657612545614371565b5b036126c15761255588886136fb565b90505f5f90505b838110156126bb5760035f8a60ff1660ff1681526020019081526020015f20818154811061258d5761258c614e67565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505092505f82828151811061264757612646614e67565b5b602002602001015111156126ae57670de0b6b3a764000083602001516bffffffffffffffffffffffff1683838151811061268457612683614e67565b5b60200260200101516126969190615315565b6126a09190615383565b856126ab91906153b3565b94505b808060010191505061255c565b506128dc565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663900413478860045f8c60ff1660ff1681526020019081526020015f206040518363ffffffff1660e01b81526004016127329291906154f9565b5f60405180830381865afa15801561274c573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061277491906155d0565b90505f5f90505b838110156128da5760035f8a60ff1660ff1681526020019081526020015f2081815481106127ac576127ab614e67565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505092505f82828151811061286657612865614e67565b5b602002602001015111156128cd57670de0b6b3a764000083602001516bffffffffffffffffffffffff168383815181106128a3576128a2614e67565b5b60200260200101516128b59190615315565b6128bf9190615383565b856128ca91906153b3565b94505b808060010191505061277b565b505b5f5f5f8a60ff1660ff1681526020019081526020015f205f9054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff16856bffffffffffffffffffffffff161015905084819650965050505050509250929050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156129a7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129cb919061562b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612a38576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612a2f906156ec565b60405180910390fd5b565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612ac8576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612abf906157a0565b60405180910390fd5b565b5f5f5f60025f8781526020019081526020015f205f8660ff1660ff1681526020019081526020015f208054905090505f8103612c045760025f8781526020019081526020015f205f8660ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001866bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505050612e1f565b5f60025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f20600183612c3891906150e5565b81548110612c4957612c48614e67565b5b905f5260205f20019050805f0160089054906101000a90046bffffffffffffffffffffffff169250846bffffffffffffffffffffffff16836bffffffffffffffffffffffff1603612c9f575f9350505050612e68565b4363ffffffff16815f015f9054906101000a900463ffffffff1663ffffffff1603612cfb5784815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550612e1d565b43815f0160046101000a81548163ffffffff021916908363ffffffff16021790555060025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001876bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555050505b505b857f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d8686604051612e519291906157be565b60405180910390a2612e6382856136ca565b925050505b9392505050565b5f8160ff166001901b8317905092915050565b5f5f60015f8460ff1660ff1681526020019081526020015f208054905014159050919050565b5f815111612eeb576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612ee290615855565b60405180910390fd5b5f815190505f60035f8560ff1660ff1681526020019081526020015f20805490509050602060ff168282612f1f9190615873565b1115612f60576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612f579061593c565b60405180910390fd5b5f5f90505b8281101561331d575f5f90505b8183612f7e9190615873565b81101561306f57848281518110612f9857612f97614e67565b5b60200260200101515f015173ffffffffffffffffffffffffffffffffffffffff1660035f8860ff1660ff1681526020019081526020015f208281548110612fe257612fe1614e67565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603613062576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401613059906159ca565b60405180910390fd5b8080600101915050612f72565b505f84828151811061308457613083614e67565b5b6020026020010151602001516bffffffffffffffffffffffff16116130de576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016130d590615a7e565b60405180910390fd5b60035f8660ff1660ff1681526020019081526020015f2084828151811061310857613107614e67565b5b6020026020010151908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151815f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550505060045f8660ff1660ff1681526020019081526020015f208482815181106131d8576131d7614e67565b5b60200260200101515f0151908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508460ff167f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540485838151811061327857613277614e67565b5b60200260200101515f015160405161329091906148f6565b60405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a758583815181106132d0576132cf614e67565b5b60200260200101515f01518684815181106132ee576132ed614e67565b5b602002602001015160200151604051613308929190614eef565b60405180910390a28080600101915050612f65565b5050505050565b805f5f8460ff1660ff1681526020019081526020015f205f6101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055508160ff167f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf8260405161339b9190613ca2565b60405180910390a25050565b8060055f8460ff1660ff1681526020019081526020015f205f6101000a81548160ff021916908360018111156133e0576133df614371565b5b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d8160405161341491906143e4565b60405180910390a15050565b5f5f821215613445578161343390615a9c565b8361343e9190615ae2565b9050613454565b818361345191906153b3565b90505b92915050565b815f015163ffffffff168163ffffffff1610156134ac576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016134a390615bb7565b60405180910390fd5b5f826020015163ffffffff1614806134d35750816020015163ffffffff168163ffffffff16105b613512576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161350990615c91565b60405180910390fd5b5050565b5f60065f8460ff1660ff1681526020019081526020015f205f9054906101000a900463ffffffff1690508160065f8560ff1660ff1681526020019081526020015f205f6101000a81548163ffffffff021916908363ffffffff1602179055507f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c781836040516135a6929190615caf565b60405180910390a1505050565b5f5f60025f8681526020019081526020015f205f8560ff1660ff1681526020019081526020015f208054905090505f8190505b5f811115613687578363ffffffff1660025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f2060018361362891906150e5565b8154811061363957613638614e67565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff16116136745760018161366b91906150e5565b925050506136c3565b808061367f90615cd6565b9150506135e6565b506040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016136ba90615ddf565b60405180910390fd5b9392505050565b5f826bffffffffffffffffffffffff16826bffffffffffffffffffffffff166136f39190615dfd565b905092915050565b60605f600167ffffffffffffffff81111561371957613718614194565b5b6040519080825280602002602001820160405280156137475781602001602082028036833780820191505090505b50905082815f8151811061375e5761375d614e67565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250505f60065f8660ff1660ff1681526020019081526020015f205f9054906101000a900463ffffffff1663ffffffff16426137d19190615873565b90505f7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663ca8aa7c76040518163ffffffff1660e01b8152600401602060405180830381865afa15801561383d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613861919061562b565b73ffffffffffffffffffffffffffffffffffffffff16632bab2c4a60405180604001604052807f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1681526020018960ff1663ffffffff168152508560045f8b60ff1660ff1681526020019081526020015f20866040518563ffffffff1660e01b81526004016139089493929190615f11565b5f60405180830381865afa158015613922573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061394a9190616040565b9050805f8151811061395f5761395e614e67565b5b6020026020010151935050505092915050565b60405180606001604052805f63ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff1681525090565b60405180604001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f6bffffffffffffffffffffffff1681525090565b5f604051905090565b5f5ffd5b5f5ffd5b5f60ff82169050919050565b613a0c816139f7565b8114613a16575f5ffd5b50565b5f81359050613a2781613a03565b92915050565b5f819050919050565b613a3f81613a2d565b8114613a49575f5ffd5b50565b5f81359050613a5a81613a36565b92915050565b5f5f60408385031215613a7657613a756139ef565b5b5f613a8385828601613a19565b9250506020613a9485828601613a4c565b9150509250929050565b5f60208284031215613ab357613ab26139ef565b5b5f613ac084828501613a19565b91505092915050565b5f819050919050565b613adb81613ac9565b82525050565b5f602082019050613af45f830184613ad2565b92915050565b613b0381613ac9565b8114613b0d575f5ffd5b50565b5f81359050613b1e81613afa565b92915050565b5f5f60408385031215613b3a57613b396139ef565b5b5f613b4785828601613a19565b9250506020613b5885828601613b10565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f819050919050565b5f613ba4613b9f613b9a84613b62565b613b81565b613b62565b9050919050565b5f613bb582613b8a565b9050919050565b5f613bc682613bab565b9050919050565b613bd681613bbc565b82525050565b5f6bffffffffffffffffffffffff82169050919050565b613bfc81613bdc565b82525050565b5f604082019050613c155f830185613bcd565b613c226020830184613bf3565b9392505050565b5f613c3382613b62565b9050919050565b613c4381613c29565b8114613c4d575f5ffd5b50565b5f81359050613c5e81613c3a565b92915050565b5f5f60408385031215613c7a57613c796139ef565b5b5f613c8785828601613a19565b9250506020613c9885828601613c50565b9150509250929050565b5f602082019050613cb55f830184613bf3565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112613cdc57613cdb613cbb565b5b8235905067ffffffffffffffff811115613cf957613cf8613cbf565b5b602083019150836020820283011115613d1557613d14613cc3565b5b9250929050565b5f5f83601f840112613d3157613d30613cbb565b5b8235905067ffffffffffffffff811115613d4e57613d4d613cbf565b5b602083019150836020820283011115613d6a57613d69613cc3565b5b9250929050565b5f5f5f5f5f60608688031215613d8a57613d896139ef565b5b5f613d9788828901613a19565b955050602086013567ffffffffffffffff811115613db857613db76139f3565b5b613dc488828901613cc7565b9450945050604086013567ffffffffffffffff811115613de757613de66139f3565b5b613df388828901613d1c565b92509250509295509295909350565b5f819050919050565b613e1481613e02565b8114613e1e575f5ffd5b50565b5f81359050613e2f81613e0b565b92915050565b5f5f83601f840112613e4a57613e49613cbb565b5b8235905067ffffffffffffffff811115613e6757613e66613cbf565b5b602083019150836001820283011115613e8357613e82613cc3565b5b9250929050565b5f5f5f5f60608587031215613ea257613ea16139ef565b5b5f613eaf87828801613c50565b9450506020613ec087828801613e21565b935050604085013567ffffffffffffffff811115613ee157613ee06139f3565b5b613eed87828801613e35565b925092505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613f2d81613bdc565b82525050565b5f613f3e8383613f24565b60208301905092915050565b5f602082019050919050565b5f613f6082613efb565b613f6a8185613f05565b9350613f7583613f15565b805f5b83811015613fa5578151613f8c8882613f33565b9750613f9783613f4a565b925050600181019050613f78565b5085935050505092915050565b5f6040820190508181035f830152613fca8185613f56565b90508181036020830152613fde8184613f56565b90509392505050565b5f5f60408385031215613ffd57613ffc6139ef565b5b5f61400a85828601613e21565b925050602061401b85828601613a19565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f63ffffffff82169050919050565b6140668161404e565b82525050565b606082015f8201516140805f85018261405d565b506020820151614093602085018261405d565b5060408201516140a66040850182613f24565b50505050565b5f6140b7838361406c565b60608301905092915050565b5f602082019050919050565b5f6140d982614025565b6140e3818561402f565b93506140ee8361403f565b805f5b8381101561411e57815161410588826140ac565b9750614110836140c3565b9250506001810190506140f1565b5085935050505092915050565b5f6020820190508181035f83015261414381846140cf565b905092915050565b5f61415582613bab565b9050919050565b6141658161414b565b82525050565b5f60208201905061417e5f83018461415c565b92915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6141ca82614184565b810181811067ffffffffffffffff821117156141e9576141e8614194565b5b80604052505050565b5f6141fb6139e6565b905061420782826141c1565b919050565b5f67ffffffffffffffff82111561422657614225614194565b5b602082029050602081019050919050565b5f6142496142448461420c565b6141f2565b9050808382526020820190506020840283018581111561426c5761426b613cc3565b5b835b8181101561429557806142818882613b10565b84526020840193505060208101905061426e565b5050509392505050565b5f82601f8301126142b3576142b2613cbb565b5b81356142c3848260208601614237565b91505092915050565b5f5f604083850312156142e2576142e16139ef565b5b5f6142ef85828601613a19565b925050602083013567ffffffffffffffff8111156143105761430f6139f3565b5b61431c8582860161429f565b9150509250929050565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff82169050919050565b61435281614326565b82525050565b5f60208201905061436b5f830184614349565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b600281106143af576143ae614371565b5b50565b5f8190506143bf8261439e565b919050565b5f6143ce826143b2565b9050919050565b6143de816143c4565b82525050565b5f6020820190506143f75f8301846143d5565b92915050565b5f61440782613bab565b9050919050565b614417816143fd565b82525050565b5f6020820190506144305f83018461440e565b92915050565b61443f81613c29565b82525050565b5f6020820190506144585f830184614436565b92915050565b61446781613bdc565b8114614471575f5ffd5b50565b5f813590506144828161445e565b92915050565b5f5f5f6060848603121561449f5761449e6139ef565b5b5f6144ac86828701613e21565b93505060206144bd86828701613a19565b92505060406144ce86828701614474565b9150509250925092565b6144e181613a2d565b82525050565b5f6020820190506144fa5f8301846144d8565b92915050565b5f67ffffffffffffffff82111561451a57614519614194565b5b602082029050602081019050919050565b5f5ffd5b5f61453982613c29565b9050919050565b6145498161452f565b8114614553575f5ffd5b50565b5f8135905061456481614540565b92915050565b5f6040828403121561457f5761457e61452b565b5b61458960406141f2565b90505f61459884828501614556565b5f8301525060206145ab84828501614474565b60208301525092915050565b5f6145c96145c484614500565b6141f2565b905080838252602082019050604084028301858111156145ec576145eb613cc3565b5b835b818110156146155780614601888261456a565b8452602084019350506040810190506145ee565b5050509392505050565b5f82601f83011261463357614632613cbb565b5b81356146438482602086016145b7565b91505092915050565b5f5f5f60608486031215614663576146626139ef565b5b5f61467086828701613a19565b935050602061468186828701614474565b925050604084013567ffffffffffffffff8111156146a2576146a16139f3565b5b6146ae8682870161461f565b9150509250925092565b6146c1816139f7565b82525050565b5f6020820190506146da5f8301846146b8565b92915050565b5f5f604083850312156146f6576146f56139ef565b5b5f61470385828601614474565b925050602061471485828601613a4c565b9150509250929050565b6147278161404e565b8114614731575f5ffd5b50565b5f813590506147428161471e565b92915050565b5f5f5f6040848603121561475f5761475e6139ef565b5b5f61476c86828701614734565b935050602084013567ffffffffffffffff81111561478d5761478c6139f3565b5b61479986828701613e35565b92509250509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6147d9838361405d565b60208301905092915050565b5f602082019050919050565b5f6147fb826147a5565b61480581856147af565b9350614810836147bf565b805f5b8381101561484057815161482788826147ce565b9750614832836147e5565b925050600181019050614813565b5085935050505092915050565b5f6020820190508181035f83015261486581846147f1565b905092915050565b60028110614879575f5ffd5b50565b5f8135905061488a8161486d565b92915050565b5f5f604083850312156148a6576148a56139ef565b5b5f6148b385828601613a19565b92505060206148c48582860161487c565b9150509250929050565b6148d78161404e565b82525050565b5f6020820190506148f05f8301846148ce565b92915050565b5f6020820190506149095f830184613bcd565b92915050565b5f5f5f60608486031215614926576149256139ef565b5b5f61493386828701613a19565b935050602061494486828701613e21565b925050604061495586828701613b10565b9150509250925092565b606082015f8201516149735f85018261405d565b506020820151614986602085018261405d565b5060408201516149996040850182613f24565b50505050565b5f6060820190506149b25f83018461495f565b92915050565b6149c181613bbc565b82525050565b604082015f8201516149db5f8501826149b8565b5060208201516149ee6020850182613f24565b50505050565b5f604082019050614a075f8301846149c7565b92915050565b5f5f60408385031215614a2357614a226139ef565b5b5f614a3085828601613a19565b9250506020614a4185828601614474565b9150509250929050565b5f5f5f60408486031215614a6257614a616139ef565b5b5f614a6f86828701613e21565b935050602084013567ffffffffffffffff811115614a9057614a8f6139f3565b5b614a9c86828701613e35565b92509250509250925092565b5f5f60408385031215614abe57614abd6139ef565b5b5f614acb85828601613a19565b925050602083013567ffffffffffffffff811115614aec57614aeb6139f3565b5b614af88582860161461f565b9150509250929050565b5f5f5f60608486031215614b1957614b186139ef565b5b5f614b2686828701613a19565b9350506020614b3786828701614734565b9250506040614b4886828701613b10565b9150509250925092565b5f5f5f5f60808587031215614b6a57614b696139ef565b5b5f614b7787828801613a19565b9450506020614b8887828801614474565b9350506040614b9987828801614734565b925050606085013567ffffffffffffffff811115614bba57614bb96139f3565b5b614bc68782880161461f565b91505092959194509250565b5f5f5f60608486031215614be957614be86139ef565b5b5f614bf686828701613e21565b9350506020614c0786828701613a19565b9250506040614c1886828701614734565b9150509250925092565b5f614c2c82613bab565b9050919050565b614c3c81614c22565b82525050565b5f602082019050614c555f830184614c33565b92915050565b5f5f60408385031215614c7157614c706139ef565b5b5f614c7e85828601613a19565b9250506020614c8f85828601614734565b9150509250929050565b5f5f5f5f60808587031215614cb157614cb06139ef565b5b5f614cbe87828801613a19565b9450506020614ccf87828801614734565b9350506040614ce087828801613e21565b9250506060614cf187828801613b10565b91505092959194509250565b5f5f60408385031215614d1357614d126139ef565b5b5f614d2085828601614474565b9250506020614d3185828601614474565b9150509250929050565b5f82825260208201905092915050565b7f5374616b6552656769737472792e6d6f646966795374726174656779506172615f8201527f6d733a206e6f20737472617465677920696e64696365732070726f7669646564602082015250565b5f614da5604083614d3b565b9150614db082614d4b565b604082019050919050565b5f6020820190508181035f830152614dd281614d99565b9050919050565b7f5374616b6552656769737472792e6d6f646966795374726174656779506172615f8201527f6d733a20696e707574206c656e677468206d69736d6174636800000000000000602082015250565b5f614e33603983614d3b565b9150614e3e82614dd9565b604082019050919050565b5f6020820190508181035f830152614e6081614e27565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215614ea957614ea86139ef565b5b5f614eb684828501614474565b91505092915050565b5f614ed9614ed4614ecf84613bdc565b613b81565b613ac9565b9050919050565b614ee981614ebf565b82525050565b5f604082019050614f025f830185613bcd565b614f0f6020830184614ee0565b9392505050565b7f5374616b6552656769737472792e72656769737465724f70657261746f723a205f8201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360208201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000604082015250565b5f614f96605b83614d3b565b9150614fa182614f16565b606082019050919050565b5f6020820190508181035f830152614fc381614f8a565b9050919050565b7f5374616b6552656769737472792e72656d6f7665537472617465676965733a205f8201527f6e6f20696e646963657320746f2072656d6f76652070726f7669646564000000602082015250565b5f615024603d83614d3b565b915061502f82614fca565b604082019050919050565b5f6020820190508181035f83015261505181615018565b9050919050565b5f819050919050565b5f61507b61507661507184615058565b613b81565b613ac9565b9050919050565b61508b81615061565b82525050565b5f6040820190506150a45f830185613bcd565b6150b16020830184615082565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6150ef82613ac9565b91506150fa83613ac9565b9250828203905081811115615112576151116150b8565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b7f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a205f8201527f71756f72756d20616c7265616479206578697374730000000000000000000000602082015250565b5f61519f603583614d3b565b91506151aa82615145565b604082019050919050565b5f6020820190508181035f8301526151cc81615193565b9050919050565b7f5374616b6552656769737472792e676574546f74616c5374616b65496e6469635f8201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360208201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000604082015250565b5f615253605b83614d3b565b915061525e826151d3565b606082019050919050565b5f6020820190508181035f83015261528081615247565b9050919050565b7f5374616b6552656769737472792e71756f72756d4578697374733a2071756f725f8201527f756d20646f6573206e6f74206578697374000000000000000000000000000000602082015250565b5f6152e1603183614d3b565b91506152ec82615287565b604082019050919050565b5f6020820190508181035f83015261530e816152d5565b9050919050565b5f61531f82613ac9565b915061532a83613ac9565b925082820261533881613ac9565b9150828204841483151761534f5761534e6150b8565b5b5092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61538d82613ac9565b915061539883613ac9565b9250826153a8576153a7615356565b5b828204905092915050565b5f6153bd82613bdc565b91506153c883613bdc565b925082820190506bffffffffffffffffffffffff8111156153ec576153eb6150b8565b5b92915050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f61542983836149b8565b60208301905092915050565b5f815f1c9050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61547161546c83615435565b615440565b9050919050565b5f615483825461545f565b9050919050565b5f600182019050919050565b5f6154a0826153f2565b6154aa81856153fc565b93506154b58361540c565b805f5b838110156154ec576154c982615478565b6154d3888261541e565b97506154de8361548a565b9250506001810190506154b8565b5085935050505092915050565b5f60408201905061550c5f830185614436565b818103602083015261551e8184615496565b90509392505050565b5f8151905061553581613afa565b92915050565b5f61554d6155488461420c565b6141f2565b905080838252602082019050602084028301858111156155705761556f613cc3565b5b835b8181101561559957806155858882615527565b845260208401935050602081019050615572565b5050509392505050565b5f82601f8301126155b7576155b6613cbb565b5b81516155c784826020860161553b565b91505092915050565b5f602082840312156155e5576155e46139ef565b5b5f82015167ffffffffffffffff811115615602576156016139f3565b5b61560e848285016155a3565b91505092915050565b5f8151905061562581613c3a565b92915050565b5f602082840312156156405761563f6139ef565b5b5f61564d84828501615617565b91505092915050565b7f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e5f8201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f66207460208201527f6865207265676973747279436f6f7264696e61746f7200000000000000000000604082015250565b5f6156d6605683614d3b565b91506156e182615656565b606082019050919050565b5f6020820190508181035f830152615703816156ca565b9050919050565b7f5374616b6552656769737472792e6f6e6c795265676973747279436f6f7264695f8201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260208201527f79436f6f7264696e61746f720000000000000000000000000000000000000000604082015250565b5f61578a604c83614d3b565b91506157958261570a565b606082019050919050565b5f6020820190508181035f8301526157b78161577e565b9050919050565b5f6040820190506157d15f8301856146b8565b6157de6020830184613bf3565b9392505050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a206e6f20737472617465676965732070726f76696465640000000000000000602082015250565b5f61583f603883614d3b565b915061584a826157e5565b604082019050919050565b5f6020820190508181035f83015261586c81615833565b9050919050565b5f61587d82613ac9565b915061588883613ac9565b92508282019050808211156158a05761589f6150b8565b5b92915050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60208201527f454e475448000000000000000000000000000000000000000000000000000000604082015250565b5f615926604583614d3b565b9150615931826158a6565b606082019050919050565b5f6020820190508181035f8301526159538161591a565b9050919050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a2063616e6e6f74206164642073616d65207374726174656779203278000000602082015250565b5f6159b4603d83614d3b565b91506159bf8261595a565b604082019050919050565b5f6020820190508181035f8301526159e1816159a8565b9050919050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f2060208201527f7765696768740000000000000000000000000000000000000000000000000000604082015250565b5f615a68604683614d3b565b9150615a73826159e8565b606082019050919050565b5f6020820190508181035f830152615a9581615a5c565b9050919050565b5f615aa682613a2d565b91507f80000000000000000000000000000000000000000000000000000000000000008203615ad857615ad76150b8565b5b815f039050919050565b5f615aec82613bdc565b9150615af783613bdc565b925082820390506bffffffffffffffffffffffff811115615b1b57615b1a6150b8565b5b92915050565b7f5374616b6552656769737472792e5f76616c69646174655374616b65557064615f8201527f74654174426c6f636b4e756d6265723a207374616b655570646174652069732060208201527f66726f6d20616674657220626c6f636b4e756d62657200000000000000000000604082015250565b5f615ba1605683614d3b565b9150615bac82615b21565b606082019050919050565b5f6020820190508181035f830152615bce81615b95565b9050919050565b7f5374616b6552656769737472792e5f76616c69646174655374616b65557064615f8201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560208201527f72207374616b6555706461746520617661696c61626c65206265666f7265206260408201527f6c6f636b4e756d62657200000000000000000000000000000000000000000000606082015250565b5f615c7b606a83614d3b565b9150615c8682615bd5565b608082019050919050565b5f6020820190508181035f830152615ca881615c6f565b9050919050565b5f604082019050615cc25f8301856148ce565b615ccf60208301846148ce565b9392505050565b5f615ce082613ac9565b91505f8203615cf257615cf16150b8565b5b600182039050919050565b7f5374616b6552656769737472792e5f6765745374616b65557064617465496e645f8201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360208201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460408201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560608201527f7200000000000000000000000000000000000000000000000000000000000000608082015250565b5f615dc9608183614d3b565b9150615dd482615cfd565b60a082019050919050565b5f6020820190508181035f830152615df681615dbd565b9050919050565b5f615e0782613a2d565b9150615e1283613a2d565b925082820390508181125f8412168282135f851215161715615e3757615e366150b8565b5b92915050565b615e4681613c29565b82525050565b604082015f820151615e605f850182615e3d565b506020820151615e73602085018261405d565b50505050565b5f81519050919050565b5f819050602082019050919050565b5f615e9d8383615e3d565b60208301905092915050565b5f602082019050919050565b5f615ebf82615e79565b615ec981856153fc565b9350615ed483615e83565b805f5b83811015615f04578151615eeb8882615e92565b9750615ef683615ea9565b925050600181019050615ed7565b5085935050505092915050565b5f60a082019050615f245f830187615e4c565b8181036040830152615f368186615eb5565b90508181036060830152615f4a8185615496565b9050615f5960808301846148ce565b95945050505050565b5f67ffffffffffffffff821115615f7c57615f7b614194565b5b602082029050602081019050919050565b5f615f9f615f9a84615f62565b6141f2565b90508083825260208201905060208402830185811115615fc257615fc1613cc3565b5b835b8181101561600957805167ffffffffffffffff811115615fe757615fe6613cbb565b5b808601615ff489826155a3565b85526020850194505050602081019050615fc4565b5050509392505050565b5f82601f83011261602757616026613cbb565b5b8151616037848260208601615f8d565b91505092915050565b5f60208284031215616055576160546139ef565b5b5f82015167ffffffffffffffff811115616072576160716139f3565b5b61607e84828501616013565b9150509291505056fea26469706673582212203f7b3c1ef2a299fecb4293eba34b760ee66ba6a887256eff2ffb3b5ec10b61ed64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qac\xB28\x03\x80ac\xB2\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x02;V[\x83\x83\x83\x83\x83\x83\x83\x83\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPPPPPPPPPPa\x02\x9FV[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x01H\x82a\x01\x1FV[\x90P\x91\x90PV[_a\x01Y\x82a\x01>V[\x90P\x91\x90PV[a\x01i\x81a\x01OV[\x81\x14a\x01sW__\xFD[PV[_\x81Q\x90Pa\x01\x84\x81a\x01`V[\x92\x91PPV[_a\x01\x94\x82a\x01>V[\x90P\x91\x90PV[a\x01\xA4\x81a\x01\x8AV[\x81\x14a\x01\xAEW__\xFD[PV[_\x81Q\x90Pa\x01\xBF\x81a\x01\x9BV[\x92\x91PPV[_a\x01\xCF\x82a\x01>V[\x90P\x91\x90PV[a\x01\xDF\x81a\x01\xC5V[\x81\x14a\x01\xE9W__\xFD[PV[_\x81Q\x90Pa\x01\xFA\x81a\x01\xD6V[\x92\x91PPV[_a\x02\n\x82a\x01>V[\x90P\x91\x90PV[a\x02\x1A\x81a\x02\0V[\x81\x14a\x02$W__\xFD[PV[_\x81Q\x90Pa\x025\x81a\x02\x11V[\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x02SWa\x02Ra\x01\x1BV[[_a\x02`\x87\x82\x88\x01a\x01vV[\x94PP` a\x02q\x87\x82\x88\x01a\x01\xB1V[\x93PP`@a\x02\x82\x87\x82\x88\x01a\x01\xECV[\x92PP``a\x02\x93\x87\x82\x88\x01a\x02'V[\x91PP\x92\x95\x91\x94P\x92PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa`\xBDa\x02\xF5_9_\x81\x81a\x14\xDE\x01R\x81\x81a)@\x01Ra*<\x01R_\x81\x81a\x0E\xE6\x01R\x81\x81a7\xD6\x01Ra8\x89\x01R_a\x14\xBA\x01R_\x81\x81a\x1F\x82\x01Ra&\xC3\x01Ra`\xBD_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\\W_5`\xE0\x1C\x80c\x81\xC0u\x02\x11a\x01DW\x80c\xC6\x01R}\x11a\0\xC1W\x80c\xDF\\\xF7#\x11a\0\x85W\x80c\xDF\\\xF7#\x14a\x08\x14W\x80c\xE0\x86\xAD\xB3\x14a\x082W\x80c\xF2\xBE\x94\xAE\x14a\x08NW\x80c\xF5\tU\x1A\x14a\x08~W\x80c\xF8Q\xE1\x98\x14a\x08\xAEW\x80c\xFA(\xC6'\x14a\x08\xDEWa\x02\\V[\x80c\xC6\x01R}\x14a\x07LW\x80c\xC8)LV\x14a\x07hW\x80c\xCCZ| \x14a\x07\x98W\x80c\xD5\xEC\xCC\x05\x14a\x07\xB4W\x80c\xDD\x98F\xB9\x14a\x07\xE4Wa\x02\\V[\x80c\xAD\xC8\x04\xDA\x11a\x01\x08W\x80c\xAD\xC8\x04\xDA\x14a\x06\x84W\x80c\xB6\x90Kx\x14a\x06\xB4W\x80c\xBC\x9A@\xC3\x14a\x06\xE4W\x80c\xBD)\xB8\xCD\x14a\x07\0W\x80c\xC4gx\xA5\x14a\x07\x1CWa\x02\\V[\x80c\x81\xC0u\x02\x14a\x05\xA8W\x80c\x86\xC0hV\x14a\x05\xD8W\x80c\x9A\xB4\xD6\xFF\x14a\x05\xF4W\x80c\x9F<\xCFe\x14a\x06$W\x80c\xACk\xFB\x03\x14a\x06TWa\x02\\V[\x80cT\x01\xED'\x11a\x01\xDDW\x80ck:\xA7.\x11a\x01\xA1W\x80ck:\xA7.\x14a\x04\xD2W\x80cm\x14\xA9\x87\x14a\x04\xF0W\x80ctELm\x14a\x05\x0EW\x80cu\xD4\x17:\x14a\x05>W\x80c|\x17#G\x14a\x05ZW\x80c\x7FB\x98\"\x14a\x05xWa\x02\\V[\x80cT\x01\xED'\x14a\x04\x08W\x80c^Zgu\x14a\x048W\x80c_\x1F-w\x14a\x04VW\x80cf\xAC\xFE\xFE\x14a\x04rW\x80ci\x7F\xBD\x93\x14a\x04\xA2Wa\x02\\V[\x80c%PGw\x11a\x02$W\x80c%PGw\x14a\x03)W\x80c,\xD9Y@\x14a\x03ZW\x80c9\x98\xFD\xD3\x14a\x03\x8AW\x80c<\xA5\xA5\xF5\x14a\x03\xA8W\x80cK\xD2n\t\x14a\x03\xD8Wa\x02\\V[\x80c\x03\x90\xA4\xD5\x14a\x02`W\x80c\x04\x91\xB4\x1C\x14a\x02|W\x80c\x08s$a\x14a\x02\xACW\x80c\x1F\x9Bt\xE0\x14a\x02\xDDW\x80c \xB6b\x98\x14a\x03\rW[__\xFD[a\x02z`\x04\x806\x03\x81\x01\x90a\x02u\x91\x90a:`V[a\t\x0EV[\0[a\x02\x96`\x04\x806\x03\x81\x01\x90a\x02\x91\x91\x90a:\x9EV[a\t\x1DV[`@Qa\x02\xA3\x91\x90a:\xE1V[`@Q\x80\x91\x03\x90\xF3[a\x02\xC6`\x04\x806\x03\x81\x01\x90a\x02\xC1\x91\x90a;$V[a\t@V[`@Qa\x02\xD4\x92\x91\x90a<\x02V[`@Q\x80\x91\x03\x90\xF3[a\x02\xF7`\x04\x806\x03\x81\x01\x90a\x02\xF2\x91\x90a<dV[a\t\xACV[`@Qa\x03\x04\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x03'`\x04\x806\x03\x81\x01\x90a\x03\"\x91\x90a=qV[a\t\xD0V[\0[a\x03C`\x04\x806\x03\x81\x01\x90a\x03>\x91\x90a>\x8AV[a\x0B\xF4V[`@Qa\x03Q\x92\x91\x90a?\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x03t`\x04\x806\x03\x81\x01\x90a\x03o\x91\x90a?\xE7V[a\r\xDFV[`@Qa\x03\x81\x91\x90aA+V[`@Q\x80\x91\x03\x90\xF3[a\x03\x92a\x0E\xE4V[`@Qa\x03\x9F\x91\x90aAkV[`@Q\x80\x91\x03\x90\xF3[a\x03\xC2`\x04\x806\x03\x81\x01\x90a\x03\xBD\x91\x90a:\x9EV[a\x0F\x08V[`@Qa\x03\xCF\x91\x90a:\xE1V[`@Q\x80\x91\x03\x90\xF3[a\x03\xF2`\x04\x806\x03\x81\x01\x90a\x03\xED\x91\x90a?\xE7V[a\x0F+V[`@Qa\x03\xFF\x91\x90a:\xE1V[`@Q\x80\x91\x03\x90\xF3[a\x04\"`\x04\x806\x03\x81\x01\x90a\x04\x1D\x91\x90a?\xE7V[a\x0F^V[`@Qa\x04/\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x04@a\x0FzV[`@Qa\x04M\x91\x90a:\xE1V[`@Q\x80\x91\x03\x90\xF3[a\x04p`\x04\x806\x03\x81\x01\x90a\x04k\x91\x90aB\xCCV[a\x0F\x86V[\0[a\x04\x8C`\x04\x806\x03\x81\x01\x90a\x04\x87\x91\x90a>\x8AV[a\x13\xD4V[`@Qa\x04\x99\x91\x90aCXV[`@Q\x80\x91\x03\x90\xF3[a\x04\xBC`\x04\x806\x03\x81\x01\x90a\x04\xB7\x91\x90a:\x9EV[a\x14\x9BV[`@Qa\x04\xC9\x91\x90aC\xE4V[`@Q\x80\x91\x03\x90\xF3[a\x04\xDAa\x14\xB8V[`@Qa\x04\xE7\x91\x90aD\x1DV[`@Q\x80\x91\x03\x90\xF3[a\x04\xF8a\x14\xDCV[`@Qa\x05\x05\x91\x90aDEV[`@Q\x80\x91\x03\x90\xF3[a\x05(`\x04\x806\x03\x81\x01\x90a\x05#\x91\x90aD\x88V[a\x15\0V[`@Qa\x055\x91\x90aD\xE7V[`@Q\x80\x91\x03\x90\xF3[a\x05X`\x04\x806\x03\x81\x01\x90a\x05S\x91\x90aFLV[a\x15\x15V[\0[a\x05ba\x16yV[`@Qa\x05o\x91\x90aF\xC7V[`@Q\x80\x91\x03\x90\xF3[a\x05\x92`\x04\x806\x03\x81\x01\x90a\x05\x8D\x91\x90aF\xE0V[a\x16~V[`@Qa\x05\x9F\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x05\xC2`\x04\x806\x03\x81\x01\x90a\x05\xBD\x91\x90aGHV[a\x16\x91V[`@Qa\x05\xCF\x91\x90aHMV[`@Q\x80\x91\x03\x90\xF3[a\x05\xF2`\x04\x806\x03\x81\x01\x90a\x05\xED\x91\x90aH\x90V[a\x18\xB9V[\0[a\x06\x0E`\x04\x806\x03\x81\x01\x90a\x06\t\x91\x90a:\x9EV[a\x18\xCFV[`@Qa\x06\x1B\x91\x90aH\xDDV[`@Q\x80\x91\x03\x90\xF3[a\x06>`\x04\x806\x03\x81\x01\x90a\x069\x91\x90a;$V[a\x18\xEFV[`@Qa\x06K\x91\x90aH\xF6V[`@Q\x80\x91\x03\x90\xF3[a\x06n`\x04\x806\x03\x81\x01\x90a\x06i\x91\x90aI\x0FV[a\x197V[`@Qa\x06{\x91\x90aI\x9FV[`@Q\x80\x91\x03\x90\xF3[a\x06\x9E`\x04\x806\x03\x81\x01\x90a\x06\x99\x91\x90a;$V[a\x1A\x19V[`@Qa\x06\xAB\x91\x90aI\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x06\xCE`\x04\x806\x03\x81\x01\x90a\x06\xC9\x91\x90a;$V[a\x1A\xF6V[`@Qa\x06\xDB\x91\x90aI\x9FV[`@Q\x80\x91\x03\x90\xF3[a\x06\xFE`\x04\x806\x03\x81\x01\x90a\x06\xF9\x91\x90aJ\rV[a\x1B\xC8V[\0[a\x07\x1A`\x04\x806\x03\x81\x01\x90a\x07\x15\x91\x90aJKV[a\x1B\xE9V[\0[a\x076`\x04\x806\x03\x81\x01\x90a\x071\x91\x90a:\x9EV[a\x1C[V[`@Qa\x07C\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x07f`\x04\x806\x03\x81\x01\x90a\x07a\x91\x90aJ\xA8V[a\x1C\x82V[\0[a\x07\x82`\x04\x806\x03\x81\x01\x90a\x07}\x91\x90aK\x02V[a\x1C\xA3V[`@Qa\x07\x8F\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x07\xB2`\x04\x806\x03\x81\x01\x90a\x07\xAD\x91\x90aKRV[a\x1D\x82V[\0[a\x07\xCE`\x04\x806\x03\x81\x01\x90a\x07\xC9\x91\x90a:\x9EV[a\x1E\xF2V[`@Qa\x07\xDB\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x07\xFE`\x04\x806\x03\x81\x01\x90a\x07\xF9\x91\x90aK\xD2V[a\x1FkV[`@Qa\x08\x0B\x91\x90aH\xDDV[`@Q\x80\x91\x03\x90\xF3[a\x08\x1Ca\x1F\x80V[`@Qa\x08)\x91\x90aLBV[`@Q\x80\x91\x03\x90\xF3[a\x08L`\x04\x806\x03\x81\x01\x90a\x08G\x91\x90aL[V[a\x1F\xA4V[\0[a\x08h`\x04\x806\x03\x81\x01\x90a\x08c\x91\x90aL\x99V[a\x1F\xBAV[`@Qa\x08u\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x08\x98`\x04\x806\x03\x81\x01\x90a\x08\x93\x91\x90aL\xFDV[a \xA9V[`@Qa\x08\xA5\x91\x90aD\xE7V[`@Q\x80\x91\x03\x90\xF3[a\x08\xC8`\x04\x806\x03\x81\x01\x90a\x08\xC3\x91\x90a?\xE7V[a \xBCV[`@Qa\x08\xD5\x91\x90aI\x9FV[`@Q\x80\x91\x03\x90\xF3[a\x08\xF8`\x04\x806\x03\x81\x01\x90a\x08\xF3\x91\x90aK\xD2V[a!\xF5V[`@Qa\t\x05\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\t\x18\x82\x82a\"jV[PPPV[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\tYW_\x80\xFD[\x90_R` _ \x01_\x91P\x91PP\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82V[_\x82a\t\xB7\x81a$\x99V[_a\t\xC2\x85\x85a$\xE4V[P\x90P\x80\x92PPP\x92\x91PPV[a\t\xD8a)>V[\x84a\t\xE2\x81a$\x99V[_\x85\x85\x90P\x90P_\x81\x11a\n+W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\"\x90aM\xBBV[`@Q\x80\x91\x03\x90\xFD[\x80\x84\x84\x90P\x14a\npW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\ng\x90aNIV[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x89`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P__\x90P[\x82\x81\x10\x15a\x0B\xE9W\x85\x85\x82\x81\x81\x10a\n\xAAWa\n\xA9aNgV[[\x90P` \x02\x01` \x81\x01\x90a\n\xBF\x91\x90aN\x94V[\x82\x89\x89\x84\x81\x81\x10a\n\xD3Wa\n\xD2aNgV[[\x90P` \x02\x015\x81T\x81\x10a\n\xEBWa\n\xEAaNgV[[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x0B\\Wa\x0B[aNgV[[\x90P` \x02\x015\x81T\x81\x10a\x0BtWa\x0BsaNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x88\x85\x81\x81\x10a\x0B\xB1Wa\x0B\xB0aNgV[[\x90P` \x02\x01` \x81\x01\x90a\x0B\xC6\x91\x90aN\x94V[`@Qa\x0B\xD4\x92\x91\x90aN\xEFV[`@Q\x80\x91\x03\x90\xA2\x80\x80`\x01\x01\x91PPa\n\x8FV[PPPPPPPPPV[``\x80a\x0B\xFFa*:V[_\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x1DWa\x0C\x1CaA\x94V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CKW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x85\x85\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0ClWa\x0CkaA\x94V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x9AW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x86\x86\x90P\x81\x10\x15a\r\xCDW_\x87\x87\x83\x81\x81\x10a\x0C\xC1Wa\x0C\xC0aNgV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x0C\xD9\x81a$\x99V[__a\x0C\xE5\x83\x8Da$\xE4V[\x91P\x91P\x80a\r)W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r \x90aO\xACV[`@Q\x80\x91\x03\x90\xFD[_a\r5\x8C\x85\x85a*\xCAV[\x90P\x82\x87\x86\x81Q\x81\x10a\rKWa\rJaNgV[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\r\x7F\x84\x82a\"jV[\x86\x86\x81Q\x81\x10a\r\x92Wa\r\x91aNgV[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPP\x80\x80`\x01\x01\x91PPa\x0C\xA2V[P\x81\x81\x93P\x93PPP\x94P\x94\x92PPPV[```\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0E\xD8W\x83\x82\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0E&V[PPPP\x90P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x92\x91PPV[__a\x0Fj\x84\x84a \xBCV[\x90P\x80`@\x01Q\x91PP\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x0F\x8Ea)>V[\x81a\x0F\x98\x81a$\x99V[_\x82Q\x90P_\x81\x11a\x0F\xDFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xD6\x90aP:V[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P_`\x04_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P__\x90P[\x83\x81\x10\x15a\x13\xCBW\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x10YWa\x10XaNgV[[` \x02` \x01\x01Q\x81T\x81\x10a\x10rWa\x10qaNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa\x10\xA9\x91\x90aH\xF6V[`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x10\xEAWa\x10\xE9aNgV[[` \x02` \x01\x01Q\x81T\x81\x10a\x11\x03Wa\x11\x02aNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_`@Qa\x11<\x92\x91\x90aP\x91V[`@Q\x80\x91\x03\x90\xA2\x82`\x01\x84\x80T\x90Pa\x11V\x91\x90aP\xE5V[\x81T\x81\x10a\x11gWa\x11faNgV[[\x90_R` _ \x01\x83\x87\x83\x81Q\x81\x10a\x11\x83Wa\x11\x82aNgV[[` \x02` \x01\x01Q\x81T\x81\x10a\x11\x9CWa\x11\x9BaNgV[[\x90_R` _ \x01_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x90PP\x82\x80T\x80a\x12hWa\x12gaQ\x18V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01__\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U_\x82\x01`\x14a\x01\0\n\x81T\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UPP\x90U\x81`\x01\x83\x80T\x90Pa\x12\xD0\x91\x90aP\xE5V[\x81T\x81\x10a\x12\xE1Wa\x12\xE0aNgV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x87\x83\x81Q\x81\x10a\x13\x1DWa\x13\x1CaNgV[[` \x02` \x01\x01Q\x81T\x81\x10a\x136Wa\x135aNgV[[\x90_R` _ \x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x80T\x80a\x13\x8CWa\x13\x8BaQ\x18V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x90U\x80\x80`\x01\x01\x91PPa\x10\x18V[PPPPPPPV[_a\x13\xDDa*:V[____\x90P[\x85\x85\x90P\x81\x10\x15a\x14\x8DW_\x86\x86\x83\x81\x81\x10a\x14\x03Wa\x14\x02aNgV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x14\x1B\x81a$\x99V[__a\x14'\x83\x8Ca$\xE4V[\x91P\x91P\x80a\x14cW_\x91Pa\x14`\x83\x87w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a.o\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P[_a\x14o\x8B\x85\x85a*\xCAV[\x90Pa\x14{\x84\x82a\"jV[PPPPP\x80\x80`\x01\x01\x91PPa\x13\xE4V[P\x81\x92PPP\x94\x93PPPPV[`\x05` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_a\x15\x0C\x84\x84\x84a*\xCAV[\x90P\x93\x92PPPV[a\x15\x1Da*:V[a\x15&\x83a.\x82V[\x15a\x15fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15]\x90aQ\xB5V[`@Q\x80\x91\x03\x90\xFD[a\x15p\x83\x82a.\xA8V[a\x15z\x83\x83a3$V[a\x15\x84\x83_a3\xA7V[`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[` \x81V[_a\x16\x89\x83\x83a4 V[\x90P\x92\x91PPV[``_\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xB1Wa\x16\xB0aA\x94V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xDFW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x84\x84\x90P\x81\x10\x15a\x18\xADW_\x85\x85\x83\x81\x81\x10a\x17\x06Wa\x17\x05aNgV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x17\x1E\x81a$\x99V[\x86c\xFF\xFF\xFF\xFF\x16`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x81\x10a\x17NWa\x17MaNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xAFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\xA6\x90aRiV[`@Q\x80\x91\x03\x90\xFD[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P__\x90P[\x81\x81\x10\x15a\x18\x9DW\x88c\xFF\xFF\xFF\xFF\x16`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83\x85a\x18\x06\x91\x90aP\xE5V[a\x18\x10\x91\x90aP\xE5V[\x81T\x81\x10a\x18!Wa\x18 aNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a\x18\x90W`\x01\x81\x83a\x18T\x91\x90aP\xE5V[a\x18^\x91\x90aP\xE5V[\x85\x85\x81Q\x81\x10a\x18qWa\x18paNgV[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x18\x9DV[\x80\x80`\x01\x01\x91PPa\x17\xD2V[PPP\x80\x80`\x01\x01\x91PPa\x16\xE7V[P\x80\x91PP\x93\x92PPPV[a\x18\xC1a)>V[a\x18\xCB\x82\x82a3\xA7V[PPV[`\x06` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x19\x08W_\x80\xFD[\x90_R` _ \x01_\x91P\x91P\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x19?a9rV[`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x19wWa\x19vaNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x93\x92PPPV[a\x1A!a9\xAAV[`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x1AJWa\x1AIaNgV[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x1A\xFEa9rV[`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x1B'Wa\x1B&aNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x1B\xD0a)>V[\x81a\x1B\xDA\x81a$\x99V[a\x1B\xE4\x83\x83a3$V[PPPV[a\x1B\xF1a*:V[__\x90P[\x82\x82\x90P\x81\x10\x15a\x1CUW_\x83\x83\x83\x81\x81\x10a\x1C\x15Wa\x1C\x14aNgV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x1C-\x81a$\x99V[_a\x1C9\x86\x83_a*\xCAV[\x90Pa\x1CE\x82\x82a\"jV[PPP\x80\x80`\x01\x01\x91PPa\x1B\xF6V[PPPPV[_` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x1C\x8Aa)>V[\x81a\x1C\x94\x81a$\x99V[a\x1C\x9E\x83\x83a.\xA8V[PPPV[__`\x01_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x1C\xCEWa\x1C\xCDaNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90Pa\x1Ds\x81\x85a4ZV[\x80`@\x01Q\x91PP\x93\x92PPPV[a\x1D\x8Aa*:V[a\x1D\x93\x84a.\x82V[\x15a\x1D\xD3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xCA\x90aQ\xB5V[`@Q\x80\x91\x03\x90\xFD[a\x1D\xDD\x84\x82a.\xA8V[a\x1D\xE7\x84\x84a3$V[a\x1D\xF2\x84`\x01a3\xA7V[a\x1D\xFC\x84\x83a5\x16V[`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPV[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x80_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa\x1F0\x91\x90aP\xE5V[\x81T\x81\x10a\x1FAWa\x1F@aNgV[[\x90_R` _ \x01_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[_a\x1Fw\x84\x84\x84a5\xB3V[\x90P\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x1F\xACa)>V[a\x1F\xB6\x82\x82a5\x16V[PPV[__`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x1F\xF4Wa\x1F\xF3aNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90Pa \x99\x81\x86a4ZV[\x80`@\x01Q\x91PP\x94\x93PPPPV[_a \xB4\x83\x83a6\xCAV[\x90P\x92\x91PPV[a \xC4a9rV[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90Pa \xF9a9rV[_\x82\x03a!\nW\x80\x92PPPa!\xEFV[`\x02_\x86\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a!=\x91\x90aP\xE5V[\x81T\x81\x10a!NWa!MaNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x80\x92PPP[\x92\x91PPV[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ a\"'\x85\x85\x85a5\xB3V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\">Wa\"=aNgV[[\x90_R` _ \x01_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x93\x92PPPV[__`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_`\x01_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\"\xAE\x91\x90aP\xE5V[\x81T\x81\x10a\"\xBFWa\"\xBEaNgV[[\x90_R` _ \x01\x90P_\x84\x03a\"\xF5W\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92PPPa$\x93V[_a#\x1B\x82_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86a4 V[\x90PCc\xFF\xFF\xFF\xFF\x16\x82_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a#yW\x80\x82_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa$\x8CV[C\x82_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[\x80\x93PPPP[\x92\x91PPV[a$\xA2\x81a.\x82V[a$\xE1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xD8\x90aR\xF7V[`@Q\x80\x91\x03\x90\xFD[PV[____a$\xF1\x86a\x0F\x08V[\x90Pa$\xFBa9\xAAV[```\x01\x80\x81\x11\x15a%\x10Wa%\x0FaCqV[[`\x05_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a%FWa%EaCqV[[\x03a&\xC1Wa%U\x88\x88a6\xFBV[\x90P__\x90P[\x83\x81\x10\x15a&\xBBW`\x03_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a%\x8DWa%\x8CaNgV[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x92P_\x82\x82\x81Q\x81\x10a&GWa&FaNgV[[` \x02` \x01\x01Q\x11\x15a&\xAEWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10a&\x84Wa&\x83aNgV[[` \x02` \x01\x01Qa&\x96\x91\x90aS\x15V[a&\xA0\x91\x90aS\x83V[\x85a&\xAB\x91\x90aS\xB3V[\x94P[\x80\x80`\x01\x01\x91PPa%\\V[Pa(\xDCV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\x04\x13G\x88`\x04_\x8C`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a'2\x92\x91\x90aT\xF9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'LW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a't\x91\x90aU\xD0V[\x90P__\x90P[\x83\x81\x10\x15a(\xDAW`\x03_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a'\xACWa'\xABaNgV[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x92P_\x82\x82\x81Q\x81\x10a(fWa(eaNgV[[` \x02` \x01\x01Q\x11\x15a(\xCDWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10a(\xA3Wa(\xA2aNgV[[` \x02` \x01\x01Qa(\xB5\x91\x90aS\x15V[a(\xBF\x91\x90aS\x83V[\x85a(\xCA\x91\x90aS\xB3V[\x94P[\x80\x80`\x01\x01\x91PPa'{V[P[___\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15\x90P\x84\x81\x96P\x96PPPPPP\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xCB\x91\x90aV+V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a*8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*/\x90aV\xECV[`@Q\x80\x91\x03\x90\xFD[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a*\xC8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\xBF\x90aW\xA0V[`@Q\x80\x91\x03\x90\xFD[V[___`\x02_\x87\x81R` \x01\x90\x81R` \x01_ _\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a,\x04W`\x02_\x87\x81R` \x01\x90\x81R` \x01_ _\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPa.\x1FV[_`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a,8\x91\x90aP\xE5V[\x81T\x81\x10a,IWa,HaNgV[[\x90_R` _ \x01\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92P\x84k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a,\x9FW_\x93PPPPa.hV[Cc\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a,\xFBW\x84\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa.\x1DV[C\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[P[\x85\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x86\x86`@Qa.Q\x92\x91\x90aW\xBEV[`@Q\x80\x91\x03\x90\xA2a.c\x82\x85a6\xCAV[\x92PPP[\x93\x92PPPV[_\x81`\xFF\x16`\x01\x90\x1B\x83\x17\x90P\x92\x91PPV[__`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15\x90P\x91\x90PV[_\x81Q\x11a.\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xE2\x90aXUV[`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P_`\x03_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P` `\xFF\x16\x82\x82a/\x1F\x91\x90aXsV[\x11\x15a/`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/W\x90aY<V[`@Q\x80\x91\x03\x90\xFD[__\x90P[\x82\x81\x10\x15a3\x1DW__\x90P[\x81\x83a/~\x91\x90aXsV[\x81\x10\x15a0oW\x84\x82\x81Q\x81\x10a/\x98Wa/\x97aNgV[[` \x02` \x01\x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a/\xE2Wa/\xE1aNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a0bW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0Y\x90aY\xCAV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa/rV[P_\x84\x82\x81Q\x81\x10a0\x84Wa0\x83aNgV[[` \x02` \x01\x01Q` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a0\xDEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0\xD5\x90aZ~V[`@Q\x80\x91\x03\x90\xFD[`\x03_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x84\x82\x81Q\x81\x10a1\x08Wa1\x07aNgV[[` \x02` \x01\x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP`\x04_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x84\x82\x81Q\x81\x10a1\xD8Wa1\xD7aNgV[[` \x02` \x01\x01Q_\x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\xFF\x16\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x85\x83\x81Q\x81\x10a2xWa2waNgV[[` \x02` \x01\x01Q_\x01Q`@Qa2\x90\x91\x90aH\xF6V[`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a2\xD0Wa2\xCFaNgV[[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a2\xEEWa2\xEDaNgV[[` \x02` \x01\x01Q` \x01Q`@Qa3\x08\x92\x91\x90aN\xEFV[`@Q\x80\x91\x03\x90\xA2\x80\x80`\x01\x01\x91PPa/eV[PPPPPV[\x80__\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\xFF\x16\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x82`@Qa3\x9B\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xA2PPV[\x80`\x05_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a3\xE0Wa3\xDFaCqV[[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa4\x14\x91\x90aC\xE4V[`@Q\x80\x91\x03\x90\xA1PPV[__\x82\x12\x15a4EW\x81a43\x90aZ\x9CV[\x83a4>\x91\x90aZ\xE2V[\x90Pa4TV[\x81\x83a4Q\x91\x90aS\xB3V[\x90P[\x92\x91PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a4\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\xA3\x90a[\xB7V[`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x14\x80a4\xD3WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a5\x12W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5\t\x90a\\\x91V[`@Q\x80\x91\x03\x90\xFD[PPV[_`\x06_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x81`\x06_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x81\x83`@Qa5\xA6\x92\x91\x90a\\\xAFV[`@Q\x80\x91\x03\x90\xA1PPPV[__`\x02_\x86\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x90P[_\x81\x11\x15a6\x87W\x83c\xFF\xFF\xFF\xFF\x16`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a6(\x91\x90aP\xE5V[\x81T\x81\x10a69Wa68aNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a6tW`\x01\x81a6k\x91\x90aP\xE5V[\x92PPPa6\xC3V[\x80\x80a6\x7F\x90a\\\xD6V[\x91PPa5\xE6V[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a6\xBA\x90a]\xDFV[`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[_\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\xF3\x91\x90a]\xFDV[\x90P\x92\x91PPV[``_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\x19Wa7\x18aA\x94V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7GW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x82\x81_\x81Q\x81\x10a7^Wa7]aNgV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_`\x06_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16Ba7\xD1\x91\x90aXsV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x8A\xA7\xC7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8a\x91\x90aV+V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c+\xAB,J`@Q\x80`@\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RP\x85`\x04_\x8B`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9\x08\x94\x93\x92\x91\x90a_\x11V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\"W=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9J\x91\x90a`@V[\x90P\x80_\x81Q\x81\x10a9_Wa9^aNgV[[` \x02` \x01\x01Q\x93PPPP\x92\x91PPV[`@Q\x80``\x01`@R\x80_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80`@\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a:\x0C\x81a9\xF7V[\x81\x14a:\x16W__\xFD[PV[_\x815\x90Pa:'\x81a:\x03V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a:?\x81a:-V[\x81\x14a:IW__\xFD[PV[_\x815\x90Pa:Z\x81a:6V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a:vWa:ua9\xEFV[[_a:\x83\x85\x82\x86\x01a:\x19V[\x92PP` a:\x94\x85\x82\x86\x01a:LV[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a:\xB3Wa:\xB2a9\xEFV[[_a:\xC0\x84\x82\x85\x01a:\x19V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a:\xDB\x81a:\xC9V[\x82RPPV[_` \x82\x01\x90Pa:\xF4_\x83\x01\x84a:\xD2V[\x92\x91PPV[a;\x03\x81a:\xC9V[\x81\x14a;\rW__\xFD[PV[_\x815\x90Pa;\x1E\x81a:\xFAV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a;:Wa;9a9\xEFV[[_a;G\x85\x82\x86\x01a:\x19V[\x92PP` a;X\x85\x82\x86\x01a;\x10V[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a;\xA4a;\x9Fa;\x9A\x84a;bV[a;\x81V[a;bV[\x90P\x91\x90PV[_a;\xB5\x82a;\x8AV[\x90P\x91\x90PV[_a;\xC6\x82a;\xABV[\x90P\x91\x90PV[a;\xD6\x81a;\xBCV[\x82RPPV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a;\xFC\x81a;\xDCV[\x82RPPV[_`@\x82\x01\x90Pa<\x15_\x83\x01\x85a;\xCDV[a<\"` \x83\x01\x84a;\xF3V[\x93\x92PPPV[_a<3\x82a;bV[\x90P\x91\x90PV[a<C\x81a<)V[\x81\x14a<MW__\xFD[PV[_\x815\x90Pa<^\x81a<:V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a<zWa<ya9\xEFV[[_a<\x87\x85\x82\x86\x01a:\x19V[\x92PP` a<\x98\x85\x82\x86\x01a<PV[\x91PP\x92P\x92\x90PV[_` \x82\x01\x90Pa<\xB5_\x83\x01\x84a;\xF3V[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a<\xDCWa<\xDBa<\xBBV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xF9Wa<\xF8a<\xBFV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a=\x15Wa=\x14a<\xC3V[[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a=1Wa=0a<\xBBV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=NWa=Ma<\xBFV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a=jWa=ia<\xC3V[[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15a=\x8AWa=\x89a9\xEFV[[_a=\x97\x88\x82\x89\x01a:\x19V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xB8Wa=\xB7a9\xF3V[[a=\xC4\x88\x82\x89\x01a<\xC7V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xE7Wa=\xE6a9\xF3V[[a=\xF3\x88\x82\x89\x01a=\x1CV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x81\x90P\x91\x90PV[a>\x14\x81a>\x02V[\x81\x14a>\x1EW__\xFD[PV[_\x815\x90Pa>/\x81a>\x0BV[\x92\x91PPV[__\x83`\x1F\x84\x01\x12a>JWa>Ia<\xBBV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>gWa>fa<\xBFV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a>\x83Wa>\x82a<\xC3V[[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a>\xA2Wa>\xA1a9\xEFV[[_a>\xAF\x87\x82\x88\x01a<PV[\x94PP` a>\xC0\x87\x82\x88\x01a>!V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xE1Wa>\xE0a9\xF3V[[a>\xED\x87\x82\x88\x01a>5V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a?-\x81a;\xDCV[\x82RPPV[_a?>\x83\x83a?$V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a?`\x82a>\xFBV[a?j\x81\x85a?\x05V[\x93Pa?u\x83a?\x15V[\x80_[\x83\x81\x10\x15a?\xA5W\x81Qa?\x8C\x88\x82a?3V[\x97Pa?\x97\x83a?JV[\x92PP`\x01\x81\x01\x90Pa?xV[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra?\xCA\x81\x85a?VV[\x90P\x81\x81\x03` \x83\x01Ra?\xDE\x81\x84a?VV[\x90P\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a?\xFDWa?\xFCa9\xEFV[[_a@\n\x85\x82\x86\x01a>!V[\x92PP` a@\x1B\x85\x82\x86\x01a:\x19V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a@f\x81a@NV[\x82RPPV[``\x82\x01_\x82\x01Qa@\x80_\x85\x01\x82a@]V[P` \x82\x01Qa@\x93` \x85\x01\x82a@]V[P`@\x82\x01Qa@\xA6`@\x85\x01\x82a?$V[PPPPV[_a@\xB7\x83\x83a@lV[``\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a@\xD9\x82a@%V[a@\xE3\x81\x85a@/V[\x93Pa@\xEE\x83a@?V[\x80_[\x83\x81\x10\x15aA\x1EW\x81QaA\x05\x88\x82a@\xACV[\x97PaA\x10\x83a@\xC3V[\x92PP`\x01\x81\x01\x90Pa@\xF1V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaAC\x81\x84a@\xCFV[\x90P\x92\x91PPV[_aAU\x82a;\xABV[\x90P\x91\x90PV[aAe\x81aAKV[\x82RPPV[_` \x82\x01\x90PaA~_\x83\x01\x84aA\\V[\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aA\xCA\x82aA\x84V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aA\xE9WaA\xE8aA\x94V[[\x80`@RPPPV[_aA\xFBa9\xE6V[\x90PaB\x07\x82\x82aA\xC1V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB&WaB%aA\x94V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_aBIaBD\x84aB\x0CV[aA\xF2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aBlWaBka<\xC3V[[\x83[\x81\x81\x10\x15aB\x95W\x80aB\x81\x88\x82a;\x10V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaBnV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aB\xB3WaB\xB2a<\xBBV[[\x815aB\xC3\x84\x82` \x86\x01aB7V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15aB\xE2WaB\xE1a9\xEFV[[_aB\xEF\x85\x82\x86\x01a:\x19V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\x10WaC\x0Fa9\xF3V[[aC\x1C\x85\x82\x86\x01aB\x9FV[\x91PP\x92P\x92\x90PV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aCR\x81aC&V[\x82RPPV[_` \x82\x01\x90PaCk_\x83\x01\x84aCIV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aC\xAFWaC\xAEaCqV[[PV[_\x81\x90PaC\xBF\x82aC\x9EV[\x91\x90PV[_aC\xCE\x82aC\xB2V[\x90P\x91\x90PV[aC\xDE\x81aC\xC4V[\x82RPPV[_` \x82\x01\x90PaC\xF7_\x83\x01\x84aC\xD5V[\x92\x91PPV[_aD\x07\x82a;\xABV[\x90P\x91\x90PV[aD\x17\x81aC\xFDV[\x82RPPV[_` \x82\x01\x90PaD0_\x83\x01\x84aD\x0EV[\x92\x91PPV[aD?\x81a<)V[\x82RPPV[_` \x82\x01\x90PaDX_\x83\x01\x84aD6V[\x92\x91PPV[aDg\x81a;\xDCV[\x81\x14aDqW__\xFD[PV[_\x815\x90PaD\x82\x81aD^V[\x92\x91PPV[___``\x84\x86\x03\x12\x15aD\x9FWaD\x9Ea9\xEFV[[_aD\xAC\x86\x82\x87\x01a>!V[\x93PP` aD\xBD\x86\x82\x87\x01a:\x19V[\x92PP`@aD\xCE\x86\x82\x87\x01aDtV[\x91PP\x92P\x92P\x92V[aD\xE1\x81a:-V[\x82RPPV[_` \x82\x01\x90PaD\xFA_\x83\x01\x84aD\xD8V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aE\x1AWaE\x19aA\x94V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_aE9\x82a<)V[\x90P\x91\x90PV[aEI\x81aE/V[\x81\x14aESW__\xFD[PV[_\x815\x90PaEd\x81aE@V[\x92\x91PPV[_`@\x82\x84\x03\x12\x15aE\x7FWaE~aE+V[[aE\x89`@aA\xF2V[\x90P_aE\x98\x84\x82\x85\x01aEVV[_\x83\x01RP` aE\xAB\x84\x82\x85\x01aDtV[` \x83\x01RP\x92\x91PPV[_aE\xC9aE\xC4\x84aE\0V[aA\xF2V[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15aE\xECWaE\xEBa<\xC3V[[\x83[\x81\x81\x10\x15aF\x15W\x80aF\x01\x88\x82aEjV[\x84R` \x84\x01\x93PP`@\x81\x01\x90PaE\xEEV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aF3WaF2a<\xBBV[[\x815aFC\x84\x82` \x86\x01aE\xB7V[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15aFcWaFba9\xEFV[[_aFp\x86\x82\x87\x01a:\x19V[\x93PP` aF\x81\x86\x82\x87\x01aDtV[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\xA2WaF\xA1a9\xF3V[[aF\xAE\x86\x82\x87\x01aF\x1FV[\x91PP\x92P\x92P\x92V[aF\xC1\x81a9\xF7V[\x82RPPV[_` \x82\x01\x90PaF\xDA_\x83\x01\x84aF\xB8V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aF\xF6WaF\xF5a9\xEFV[[_aG\x03\x85\x82\x86\x01aDtV[\x92PP` aG\x14\x85\x82\x86\x01a:LV[\x91PP\x92P\x92\x90PV[aG'\x81a@NV[\x81\x14aG1W__\xFD[PV[_\x815\x90PaGB\x81aG\x1EV[\x92\x91PPV[___`@\x84\x86\x03\x12\x15aG_WaG^a9\xEFV[[_aGl\x86\x82\x87\x01aG4V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\x8DWaG\x8Ca9\xF3V[[aG\x99\x86\x82\x87\x01a>5V[\x92P\x92PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_aG\xD9\x83\x83a@]V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aG\xFB\x82aG\xA5V[aH\x05\x81\x85aG\xAFV[\x93PaH\x10\x83aG\xBFV[\x80_[\x83\x81\x10\x15aH@W\x81QaH'\x88\x82aG\xCEV[\x97PaH2\x83aG\xE5V[\x92PP`\x01\x81\x01\x90PaH\x13V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaHe\x81\x84aG\xF1V[\x90P\x92\x91PPV[`\x02\x81\x10aHyW__\xFD[PV[_\x815\x90PaH\x8A\x81aHmV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aH\xA6WaH\xA5a9\xEFV[[_aH\xB3\x85\x82\x86\x01a:\x19V[\x92PP` aH\xC4\x85\x82\x86\x01aH|V[\x91PP\x92P\x92\x90PV[aH\xD7\x81a@NV[\x82RPPV[_` \x82\x01\x90PaH\xF0_\x83\x01\x84aH\xCEV[\x92\x91PPV[_` \x82\x01\x90PaI\t_\x83\x01\x84a;\xCDV[\x92\x91PPV[___``\x84\x86\x03\x12\x15aI&WaI%a9\xEFV[[_aI3\x86\x82\x87\x01a:\x19V[\x93PP` aID\x86\x82\x87\x01a>!V[\x92PP`@aIU\x86\x82\x87\x01a;\x10V[\x91PP\x92P\x92P\x92V[``\x82\x01_\x82\x01QaIs_\x85\x01\x82a@]V[P` \x82\x01QaI\x86` \x85\x01\x82a@]V[P`@\x82\x01QaI\x99`@\x85\x01\x82a?$V[PPPPV[_``\x82\x01\x90PaI\xB2_\x83\x01\x84aI_V[\x92\x91PPV[aI\xC1\x81a;\xBCV[\x82RPPV[`@\x82\x01_\x82\x01QaI\xDB_\x85\x01\x82aI\xB8V[P` \x82\x01QaI\xEE` \x85\x01\x82a?$V[PPPPV[_`@\x82\x01\x90PaJ\x07_\x83\x01\x84aI\xC7V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aJ#WaJ\"a9\xEFV[[_aJ0\x85\x82\x86\x01a:\x19V[\x92PP` aJA\x85\x82\x86\x01aDtV[\x91PP\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aJbWaJaa9\xEFV[[_aJo\x86\x82\x87\x01a>!V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\x90WaJ\x8Fa9\xF3V[[aJ\x9C\x86\x82\x87\x01a>5V[\x92P\x92PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aJ\xBEWaJ\xBDa9\xEFV[[_aJ\xCB\x85\x82\x86\x01a:\x19V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\xECWaJ\xEBa9\xF3V[[aJ\xF8\x85\x82\x86\x01aF\x1FV[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15aK\x19WaK\x18a9\xEFV[[_aK&\x86\x82\x87\x01a:\x19V[\x93PP` aK7\x86\x82\x87\x01aG4V[\x92PP`@aKH\x86\x82\x87\x01a;\x10V[\x91PP\x92P\x92P\x92V[____`\x80\x85\x87\x03\x12\x15aKjWaKia9\xEFV[[_aKw\x87\x82\x88\x01a:\x19V[\x94PP` aK\x88\x87\x82\x88\x01aDtV[\x93PP`@aK\x99\x87\x82\x88\x01aG4V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xBAWaK\xB9a9\xF3V[[aK\xC6\x87\x82\x88\x01aF\x1FV[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15aK\xE9WaK\xE8a9\xEFV[[_aK\xF6\x86\x82\x87\x01a>!V[\x93PP` aL\x07\x86\x82\x87\x01a:\x19V[\x92PP`@aL\x18\x86\x82\x87\x01aG4V[\x91PP\x92P\x92P\x92V[_aL,\x82a;\xABV[\x90P\x91\x90PV[aL<\x81aL\"V[\x82RPPV[_` \x82\x01\x90PaLU_\x83\x01\x84aL3V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aLqWaLpa9\xEFV[[_aL~\x85\x82\x86\x01a:\x19V[\x92PP` aL\x8F\x85\x82\x86\x01aG4V[\x91PP\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15aL\xB1WaL\xB0a9\xEFV[[_aL\xBE\x87\x82\x88\x01a:\x19V[\x94PP` aL\xCF\x87\x82\x88\x01aG4V[\x93PP`@aL\xE0\x87\x82\x88\x01a>!V[\x92PP``aL\xF1\x87\x82\x88\x01a;\x10V[\x91PP\x92\x95\x91\x94P\x92PV[__`@\x83\x85\x03\x12\x15aM\x13WaM\x12a9\xEFV[[_aM \x85\x82\x86\x01aDtV[\x92PP` aM1\x85\x82\x86\x01aDtV[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FStakeRegistry.modifyStrategyPara_\x82\x01R\x7Fms: no strategy indices provided` \x82\x01RPV[_aM\xA5`@\x83aM;V[\x91PaM\xB0\x82aMKV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaM\xD2\x81aM\x99V[\x90P\x91\x90PV[\x7FStakeRegistry.modifyStrategyPara_\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0` \x82\x01RPV[_aN3`9\x83aM;V[\x91PaN>\x82aM\xD9V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaN`\x81aN'V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aN\xA9WaN\xA8a9\xEFV[[_aN\xB6\x84\x82\x85\x01aDtV[\x91PP\x92\x91PPV[_aN\xD9aN\xD4aN\xCF\x84a;\xDCV[a;\x81V[a:\xC9V[\x90P\x91\x90PV[aN\xE9\x81aN\xBFV[\x82RPPV[_`@\x82\x01\x90PaO\x02_\x83\x01\x85a;\xCDV[aO\x0F` \x83\x01\x84aN\xE0V[\x93\x92PPPV[\x7FStakeRegistry.registerOperator: _\x82\x01R\x7FOperator does not meet minimum s` \x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`@\x82\x01RPV[_aO\x96`[\x83aM;V[\x91PaO\xA1\x82aO\x16V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaO\xC3\x81aO\x8AV[\x90P\x91\x90PV[\x7FStakeRegistry.removeStrategies: _\x82\x01R\x7Fno indices to remove provided\0\0\0` \x82\x01RPV[_aP$`=\x83aM;V[\x91PaP/\x82aO\xCAV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaPQ\x81aP\x18V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_aP{aPvaPq\x84aPXV[a;\x81V[a:\xC9V[\x90P\x91\x90PV[aP\x8B\x81aPaV[\x82RPPV[_`@\x82\x01\x90PaP\xA4_\x83\x01\x85a;\xCDV[aP\xB1` \x83\x01\x84aP\x82V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aP\xEF\x82a:\xC9V[\x91PaP\xFA\x83a:\xC9V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aQ\x12WaQ\x11aP\xB8V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[\x7FStakeRegistry.initializeQuorum: _\x82\x01R\x7Fquorum already exists\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aQ\x9F`5\x83aM;V[\x91PaQ\xAA\x82aQEV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaQ\xCC\x81aQ\x93V[\x90P\x91\x90PV[\x7FStakeRegistry.getTotalStakeIndic_\x82\x01R\x7FesAtBlockNumber: quorum has no s` \x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`@\x82\x01RPV[_aRS`[\x83aM;V[\x91PaR^\x82aQ\xD3V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaR\x80\x81aRGV[\x90P\x91\x90PV[\x7FStakeRegistry.quorumExists: quor_\x82\x01R\x7Fum does not exist\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aR\xE1`1\x83aM;V[\x91PaR\xEC\x82aR\x87V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaS\x0E\x81aR\xD5V[\x90P\x91\x90PV[_aS\x1F\x82a:\xC9V[\x91PaS*\x83a:\xC9V[\x92P\x82\x82\x02aS8\x81a:\xC9V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aSOWaSNaP\xB8V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_aS\x8D\x82a:\xC9V[\x91PaS\x98\x83a:\xC9V[\x92P\x82aS\xA8WaS\xA7aSVV[[\x82\x82\x04\x90P\x92\x91PPV[_aS\xBD\x82a;\xDCV[\x91PaS\xC8\x83a;\xDCV[\x92P\x82\x82\x01\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\xECWaS\xEBaP\xB8V[[\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_aT)\x83\x83aI\xB8V[` \x83\x01\x90P\x92\x91PPV[_\x81_\x1C\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aTqaTl\x83aT5V[aT@V[\x90P\x91\x90PV[_aT\x83\x82TaT_V[\x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_aT\xA0\x82aS\xF2V[aT\xAA\x81\x85aS\xFCV[\x93PaT\xB5\x83aT\x0CV[\x80_[\x83\x81\x10\x15aT\xECWaT\xC9\x82aTxV[aT\xD3\x88\x82aT\x1EV[\x97PaT\xDE\x83aT\x8AV[\x92PP`\x01\x81\x01\x90PaT\xB8V[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90PaU\x0C_\x83\x01\x85aD6V[\x81\x81\x03` \x83\x01RaU\x1E\x81\x84aT\x96V[\x90P\x93\x92PPPV[_\x81Q\x90PaU5\x81a:\xFAV[\x92\x91PPV[_aUMaUH\x84aB\x0CV[aA\xF2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aUpWaUoa<\xC3V[[\x83[\x81\x81\x10\x15aU\x99W\x80aU\x85\x88\x82aU'V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaUrV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aU\xB7WaU\xB6a<\xBBV[[\x81QaU\xC7\x84\x82` \x86\x01aU;V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aU\xE5WaU\xE4a9\xEFV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\x02WaV\x01a9\xF3V[[aV\x0E\x84\x82\x85\x01aU\xA3V[\x91PP\x92\x91PPV[_\x81Q\x90PaV%\x81a<:V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aV@WaV?a9\xEFV[[_aVM\x84\x82\x85\x01aV\x17V[\x91PP\x92\x91PPV[\x7FStakeRegistry.onlyCoordinatorOwn_\x82\x01R\x7Fer: caller is not the owner of t` \x82\x01R\x7Fhe registryCoordinator\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aV\xD6`V\x83aM;V[\x91PaV\xE1\x82aVVV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\x03\x81aV\xCAV[\x90P\x91\x90PV[\x7FStakeRegistry.onlyRegistryCoordi_\x82\x01R\x7Fnator: caller is not the Registr` \x82\x01R\x7FyCoordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aW\x8A`L\x83aM;V[\x91PaW\x95\x82aW\nV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\xB7\x81aW~V[\x90P\x91\x90PV[_`@\x82\x01\x90PaW\xD1_\x83\x01\x85aF\xB8V[aW\xDE` \x83\x01\x84a;\xF3V[\x93\x92PPPV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0` \x82\x01RPV[_aX?`8\x83aM;V[\x91PaXJ\x82aW\xE5V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaXl\x81aX3V[\x90P\x91\x90PV[_aX}\x82a:\xC9V[\x91PaX\x88\x83a:\xC9V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aX\xA0WaX\x9FaP\xB8V[[\x92\x91PPV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L` \x82\x01R\x7FENGTH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aY&`E\x83aM;V[\x91PaY1\x82aX\xA6V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaYS\x81aY\x1AV[\x90P\x91\x90PV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: cannot add same strategy 2x\0\0\0` \x82\x01RPV[_aY\xB4`=\x83aM;V[\x91PaY\xBF\x82aYZV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaY\xE1\x81aY\xA8V[\x90P\x91\x90PV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: cannot add strategy with zero ` \x82\x01R\x7Fweight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aZh`F\x83aM;V[\x91PaZs\x82aY\xE8V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaZ\x95\x81aZ\\V[\x90P\x91\x90PV[_aZ\xA6\x82a:-V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03aZ\xD8WaZ\xD7aP\xB8V[[\x81_\x03\x90P\x91\x90PV[_aZ\xEC\x82a;\xDCV[\x91PaZ\xF7\x83a;\xDCV[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x1BWa[\x1AaP\xB8V[[\x92\x91PPV[\x7FStakeRegistry._validateStakeUpda_\x82\x01R\x7FteAtBlockNumber: stakeUpdate is ` \x82\x01R\x7Ffrom after blockNumber\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a[\xA1`V\x83aM;V[\x91Pa[\xAC\x82a[!V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra[\xCE\x81a[\x95V[\x90P\x91\x90PV[\x7FStakeRegistry._validateStakeUpda_\x82\x01R\x7FteAtBlockNumber: there is a newe` \x82\x01R\x7Fr stakeUpdate available before b`@\x82\x01R\x7FlockNumber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a\\{`j\x83aM;V[\x91Pa\\\x86\x82a[\xD5V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\\\xA8\x81a\\oV[\x90P\x91\x90PV[_`@\x82\x01\x90Pa\\\xC2_\x83\x01\x85aH\xCEV[a\\\xCF` \x83\x01\x84aH\xCEV[\x93\x92PPPV[_a\\\xE0\x82a:\xC9V[\x91P_\x82\x03a\\\xF2Wa\\\xF1aP\xB8V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStakeRegistry._getStakeUpdateInd_\x82\x01R\x7FexForOperatorAtBlockNumber: no s` \x82\x01R\x7Ftake update found for operatorId`@\x82\x01R\x7F and quorumNumber at block numbe``\x82\x01R\x7Fr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01RPV[_a]\xC9`\x81\x83aM;V[\x91Pa]\xD4\x82a\\\xFDV[`\xA0\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra]\xF6\x81a]\xBDV[\x90P\x91\x90PV[_a^\x07\x82a:-V[\x91Pa^\x12\x83a:-V[\x92P\x82\x82\x03\x90P\x81\x81\x12_\x84\x12\x16\x82\x82\x13_\x85\x12\x15\x16\x17\x15a^7Wa^6aP\xB8V[[\x92\x91PPV[a^F\x81a<)V[\x82RPPV[`@\x82\x01_\x82\x01Qa^`_\x85\x01\x82a^=V[P` \x82\x01Qa^s` \x85\x01\x82a@]V[PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a^\x9D\x83\x83a^=V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a^\xBF\x82a^yV[a^\xC9\x81\x85aS\xFCV[\x93Pa^\xD4\x83a^\x83V[\x80_[\x83\x81\x10\x15a_\x04W\x81Qa^\xEB\x88\x82a^\x92V[\x97Pa^\xF6\x83a^\xA9V[\x92PP`\x01\x81\x01\x90Pa^\xD7V[P\x85\x93PPPP\x92\x91PPV[_`\xA0\x82\x01\x90Pa_$_\x83\x01\x87a^LV[\x81\x81\x03`@\x83\x01Ra_6\x81\x86a^\xB5V[\x90P\x81\x81\x03``\x83\x01Ra_J\x81\x85aT\x96V[\x90Pa_Y`\x80\x83\x01\x84aH\xCEV[\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a_|Wa_{aA\x94V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a_\x9Fa_\x9A\x84a_bV[aA\xF2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a_\xC2Wa_\xC1a<\xC3V[[\x83[\x81\x81\x10\x15a`\tW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_\xE7Wa_\xE6a<\xBBV[[\x80\x86\x01a_\xF4\x89\x82aU\xA3V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa_\xC4V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a`'Wa`&a<\xBBV[[\x81Qa`7\x84\x82` \x86\x01a_\x8DV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a`UWa`Ta9\xEFV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`rWa`qa9\xF3V[[a`~\x84\x82\x85\x01a`\x13V[\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 ?{<\x1E\xF2\xA2\x99\xFE\xCBB\x93\xEB\xA3Kv\x0E\xE6k\xA6\xA8\x87%n\xFF/\xFB;^\xC1\x0Ba\xEDdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b506004361061025c575f3560e01c806381c0750211610144578063c601527d116100c1578063df5cf72311610085578063df5cf72314610814578063e086adb314610832578063f2be94ae1461084e578063f509551a1461087e578063f851e198146108ae578063fa28c627146108de5761025c565b8063c601527d1461074c578063c8294c5614610768578063cc5a7c2014610798578063d5eccc05146107b4578063dd9846b9146107e45761025c565b8063adc804da11610108578063adc804da14610684578063b6904b78146106b4578063bc9a40c3146106e4578063bd29b8cd14610700578063c46778a51461071c5761025c565b806381c07502146105a857806386c06856146105d85780639ab4d6ff146105f45780639f3ccf6514610624578063ac6bfb03146106545761025c565b80635401ed27116101dd5780636b3aa72e116101a15780636b3aa72e146104d25780636d14a987146104f057806374454c6d1461050e57806375d4173a1461053e5780637c1723471461055a5780637f429822146105785761025c565b80635401ed27146104085780635e5a6775146104385780635f1f2d771461045657806366acfefe14610472578063697fbd93146104a25761025c565b8063255047771161022457806325504777146103295780632cd959401461035a5780633998fdd31461038a5780633ca5a5f5146103a85780634bd26e09146103d85761025c565b80630390a4d5146102605780630491b41c1461027c57806308732461146102ac5780631f9b74e0146102dd57806320b662981461030d575b5f5ffd5b61027a60048036038101906102759190613a60565b61090e565b005b61029660048036038101906102919190613a9e565b61091d565b6040516102a39190613ae1565b60405180910390f35b6102c660048036038101906102c19190613b24565b610940565b6040516102d4929190613c02565b60405180910390f35b6102f760048036038101906102f29190613c64565b6109ac565b6040516103049190613ca2565b60405180910390f35b61032760048036038101906103229190613d71565b6109d0565b005b610343600480360381019061033e9190613e8a565b610bf4565b604051610351929190613fb2565b60405180910390f35b610374600480360381019061036f9190613fe7565b610ddf565b604051610381919061412b565b60405180910390f35b610392610ee4565b60405161039f919061416b565b60405180910390f35b6103c260048036038101906103bd9190613a9e565b610f08565b6040516103cf9190613ae1565b60405180910390f35b6103f260048036038101906103ed9190613fe7565b610f2b565b6040516103ff9190613ae1565b60405180910390f35b610422600480360381019061041d9190613fe7565b610f5e565b60405161042f9190613ca2565b60405180910390f35b610440610f7a565b60405161044d9190613ae1565b60405180910390f35b610470600480360381019061046b91906142cc565b610f86565b005b61048c60048036038101906104879190613e8a565b6113d4565b6040516104999190614358565b60405180910390f35b6104bc60048036038101906104b79190613a9e565b61149b565b6040516104c991906143e4565b60405180910390f35b6104da6114b8565b6040516104e7919061441d565b60405180910390f35b6104f86114dc565b6040516105059190614445565b60405180910390f35b61052860048036038101906105239190614488565b611500565b60405161053591906144e7565b60405180910390f35b6105586004803603810190610553919061464c565b611515565b005b610562611679565b60405161056f91906146c7565b60405180910390f35b610592600480360381019061058d91906146e0565b61167e565b60405161059f9190613ca2565b60405180910390f35b6105c260048036038101906105bd9190614748565b611691565b6040516105cf919061484d565b60405180910390f35b6105f260048036038101906105ed9190614890565b6118b9565b005b61060e60048036038101906106099190613a9e565b6118cf565b60405161061b91906148dd565b60405180910390f35b61063e60048036038101906106399190613b24565b6118ef565b60405161064b91906148f6565b60405180910390f35b61066e6004803603810190610669919061490f565b611937565b60405161067b919061499f565b60405180910390f35b61069e60048036038101906106999190613b24565b611a19565b6040516106ab91906149f4565b60405180910390f35b6106ce60048036038101906106c99190613b24565b611af6565b6040516106db919061499f565b60405180910390f35b6106fe60048036038101906106f99190614a0d565b611bc8565b005b61071a60048036038101906107159190614a4b565b611be9565b005b61073660048036038101906107319190613a9e565b611c5b565b6040516107439190613ca2565b60405180910390f35b61076660048036038101906107619190614aa8565b611c82565b005b610782600480360381019061077d9190614b02565b611ca3565b60405161078f9190613ca2565b60405180910390f35b6107b260048036038101906107ad9190614b52565b611d82565b005b6107ce60048036038101906107c99190613a9e565b611ef2565b6040516107db9190613ca2565b60405180910390f35b6107fe60048036038101906107f99190614bd2565b611f6b565b60405161080b91906148dd565b60405180910390f35b61081c611f80565b6040516108299190614c42565b60405180910390f35b61084c60048036038101906108479190614c5b565b611fa4565b005b61086860048036038101906108639190614c99565b611fba565b6040516108759190613ca2565b60405180910390f35b61089860048036038101906108939190614cfd565b6120a9565b6040516108a591906144e7565b60405180910390f35b6108c860048036038101906108c39190613fe7565b6120bc565b6040516108d5919061499f565b60405180910390f35b6108f860048036038101906108f39190614bd2565b6121f5565b6040516109059190613ca2565b60405180910390f35b610918828261226a565b505050565b5f60015f8360ff1660ff1681526020019081526020015f20805490509050919050565b6003602052815f5260405f208181548110610959575f80fd5b905f5260205f20015f9150915050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690805f0160149054906101000a90046bffffffffffffffffffffffff16905082565b5f826109b781612499565b5f6109c285856124e4565b509050809250505092915050565b6109d861293e565b846109e281612499565b5f8585905090505f8111610a2b576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a2290614dbb565b60405180910390fd5b808484905014610a70576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610a6790614e49565b60405180910390fd5b5f60035f8960ff1660ff1681526020019081526020015f2090505f5f90505b82811015610be957858582818110610aaa57610aa9614e67565b5b9050602002016020810190610abf9190614e94565b82898984818110610ad357610ad2614e67565b5b9050602002013581548110610aeb57610aea614e67565b5b905f5260205f20015f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a85818110610b5c57610b5b614e67565b5b9050602002013581548110610b7457610b73614e67565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16888885818110610bb157610bb0614e67565b5b9050602002016020810190610bc69190614e94565b604051610bd4929190614eef565b60405180910390a28080600101915050610a8f565b505050505050505050565b606080610bff612a3a565b5f8484905067ffffffffffffffff811115610c1d57610c1c614194565b5b604051908082528060200260200182016040528015610c4b5781602001602082028036833780820191505090505b5090505f8585905067ffffffffffffffff811115610c6c57610c6b614194565b5b604051908082528060200260200182016040528015610c9a5781602001602082028036833780820191505090505b5090505f5f90505b86869050811015610dcd575f878783818110610cc157610cc0614e67565b5b9050013560f81c60f81b60f81c9050610cd981612499565b5f5f610ce5838d6124e4565b9150915080610d29576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610d2090614fac565b60405180910390fd5b5f610d358c8585612aca565b905082878681518110610d4b57610d4a614e67565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff1681525050610d7f848261226a565b868681518110610d9257610d91614e67565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff1681525050505050508080600101915050610ca2565b50818193509350505094509492505050565b606060025f8481526020019081526020015f205f8360ff1660ff1681526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015610ed8578382905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505081526020019060010190610e26565b50505050905092915050565b7f000000000000000000000000000000000000000000000000000000000000000081565b5f60035f8360ff1660ff1681526020019081526020015f20805490509050919050565b5f60025f8481526020019081526020015f205f8360ff1660ff1681526020019081526020015f2080549050905092915050565b5f5f610f6a84846120bc565b9050806040015191505092915050565b670de0b6b3a764000081565b610f8e61293e565b81610f9881612499565b5f825190505f8111610fdf576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610fd69061503a565b60405180910390fd5b5f60035f8660ff1660ff1681526020019081526020015f2090505f60045f8760ff1660ff1681526020019081526020015f2090505f5f90505b838110156113cb578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f78488848151811061105957611058614e67565b5b60200260200101518154811061107257611071614e67565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff166040516110a991906148f6565b60405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75848884815181106110ea576110e9614e67565b5b60200260200101518154811061110357611102614e67565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f60405161113c929190615091565b60405180910390a2826001848054905061115691906150e5565b8154811061116757611166614e67565b5b905f5260205f20018387838151811061118357611182614e67565b5b60200260200101518154811061119c5761119b614e67565b5b905f5260205f20015f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f820160149054906101000a90046bffffffffffffffffffffffff16815f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055509050508280548061126857611267615118565b5b600190038181905f5260205f20015f5f82015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff02191690555f820160146101000a8154906bffffffffffffffffffffffff02191690555050905581600183805490506112d091906150e5565b815481106112e1576112e0614e67565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168287838151811061131d5761131c614e67565b5b60200260200101518154811061133657611335614e67565b5b905f5260205f20015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508180548061138c5761138b615118565b5b600190038181905f5260205f20015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905590558080600101915050611018565b50505050505050565b5f6113dd612a3a565b5f5f5f5f90505b8585905081101561148d575f86868381811061140357611402614e67565b5b9050013560f81c60f81b60f81c905061141b81612499565b5f5f611427838c6124e4565b9150915080611463575f9150611460838777ffffffffffffffffffffffffffffffffffffffffffffffff16612e6f90919063ffffffff16565b95505b5f61146f8b8585612aca565b905061147b848261226a565b505050505080806001019150506113e4565b508192505050949350505050565b6005602052805f5260405f205f915054906101000a900460ff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b5f61150c848484612aca565b90509392505050565b61151d612a3a565b61152683612e82565b15611566576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161155d906151b5565b60405180910390fd5b6115708382612ea8565b61157a8383613324565b611584835f6133a7565b60015f8460ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505050505050565b602081565b5f6116898383613420565b905092915050565b60605f8383905067ffffffffffffffff8111156116b1576116b0614194565b5b6040519080825280602002602001820160405280156116df5781602001602082028036833780820191505090505b5090505f5f90505b848490508110156118ad575f85858381811061170657611705614e67565b5b9050013560f81c60f81b60f81c905061171e81612499565b8663ffffffff1660015f8360ff1660ff1681526020019081526020015f205f8154811061174e5761174d614e67565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff1611156117af576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016117a690615269565b60405180910390fd5b5f60015f8360ff1660ff1681526020019081526020015f208054905090505f5f90505b8181101561189d578863ffffffff1660015f8560ff1660ff1681526020019081526020015f206001838561180691906150e5565b61181091906150e5565b8154811061182157611820614e67565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff1611611890576001818361185491906150e5565b61185e91906150e5565b85858151811061187157611870614e67565b5b602002602001019063ffffffff16908163ffffffff168152505061189d565b80806001019150506117d2565b50505080806001019150506116e7565b50809150509392505050565b6118c161293e565b6118cb82826133a7565b5050565b6006602052805f5260405f205f915054906101000a900463ffffffff1681565b6004602052815f5260405f208181548110611908575f80fd5b905f5260205f20015f915091509054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b61193f613972565b60025f8481526020019081526020015f205f8560ff1660ff1681526020019081526020015f20828154811061197757611976614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505090509392505050565b611a216139aa565b60035f8460ff1660ff1681526020019081526020015f208281548110611a4a57611a49614e67565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905092915050565b611afe613972565b60015f8460ff1660ff1681526020019081526020015f208281548110611b2757611b26614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905092915050565b611bd061293e565b81611bda81612499565b611be48383613324565b505050565b611bf1612a3a565b5f5f90505b82829050811015611c55575f838383818110611c1557611c14614e67565b5b9050013560f81c60f81b60f81c9050611c2d81612499565b5f611c3986835f612aca565b9050611c45828261226a565b5050508080600101915050611bf6565b50505050565b5f602052805f5260405f205f915054906101000a90046bffffffffffffffffffffffff1681565b611c8a61293e565b81611c9481612499565b611c9e8383612ea8565b505050565b5f5f60015f8660ff1660ff1681526020019081526020015f208381548110611cce57611ccd614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff16815250509050611d73818561345a565b80604001519150509392505050565b611d8a612a3a565b611d9384612e82565b15611dd3576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611dca906151b5565b60405180910390fd5b611ddd8482612ea8565b611de78484613324565b611df28460016133a7565b611dfc8483613516565b60015f8560ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550505050505050565b5f60015f8360ff1660ff1681526020019081526020015f206001805f8560ff1660ff1681526020019081526020015f2080549050611f3091906150e5565b81548110611f4157611f40614e67565b5b905f5260205f20015f0160089054906101000a90046bffffffffffffffffffffffff169050919050565b5f611f778484846135b3565b90509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b611fac61293e565b611fb68282613516565b5050565b5f5f60025f8581526020019081526020015f205f8760ff1660ff1681526020019081526020015f208381548110611ff457611ff3614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff16815250509050612099818661345a565b8060400151915050949350505050565b5f6120b483836136ca565b905092915050565b6120c4613972565b5f60025f8581526020019081526020015f205f8460ff1660ff1681526020019081526020015f208054905090506120f9613972565b5f820361210a5780925050506121ef565b60025f8681526020019081526020015f205f8560ff1660ff1681526020019081526020015f2060018361213d91906150e5565b8154811061214e5761214d614e67565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905080925050505b92915050565b5f60025f8581526020019081526020015f205f8460ff1660ff1681526020019081526020015f206122278585856135b3565b63ffffffff168154811061223e5761223d614e67565b5b905f5260205f20015f0160089054906101000a90046bffffffffffffffffffffffff1690509392505050565b5f5f60015f8560ff1660ff1681526020019081526020015f208054905090505f60015f8660ff1660ff1681526020019081526020015f206001836122ae91906150e5565b815481106122bf576122be614e67565b5b905f5260205f200190505f84036122f557805f0160089054906101000a90046bffffffffffffffffffffffff1692505050612493565b5f61231b825f0160089054906101000a90046bffffffffffffffffffffffff1686613420565b90504363ffffffff16825f015f9054906101000a900463ffffffff1663ffffffff16036123795780825f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555061248c565b43825f0160046101000a81548163ffffffff021916908363ffffffff16021790555060015f8760ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001836bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555050505b8093505050505b92915050565b6124a281612e82565b6124e1576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016124d8906152f7565b60405180910390fd5b50565b5f5f5f5f6124f186610f08565b90506124fb6139aa565b60606001808111156125105761250f614371565b5b60055f8a60ff1660ff1681526020019081526020015f205f9054906101000a900460ff16600181111561254657612545614371565b5b036126c15761255588886136fb565b90505f5f90505b838110156126bb5760035f8a60ff1660ff1681526020019081526020015f20818154811061258d5761258c614e67565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505092505f82828151811061264757612646614e67565b5b602002602001015111156126ae57670de0b6b3a764000083602001516bffffffffffffffffffffffff1683838151811061268457612683614e67565b5b60200260200101516126969190615315565b6126a09190615383565b856126ab91906153b3565b94505b808060010191505061255c565b506128dc565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663900413478860045f8c60ff1660ff1681526020019081526020015f206040518363ffffffff1660e01b81526004016127329291906154f9565b5f60405180830381865afa15801561274c573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061277491906155d0565b90505f5f90505b838110156128da5760035f8a60ff1660ff1681526020019081526020015f2081815481106127ac576127ab614e67565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505092505f82828151811061286657612865614e67565b5b602002602001015111156128cd57670de0b6b3a764000083602001516bffffffffffffffffffffffff168383815181106128a3576128a2614e67565b5b60200260200101516128b59190615315565b6128bf9190615383565b856128ca91906153b3565b94505b808060010191505061277b565b505b5f5f5f8a60ff1660ff1681526020019081526020015f205f9054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff16856bffffffffffffffffffffffff161015905084819650965050505050509250929050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa1580156129a7573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906129cb919061562b565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612a38576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612a2f906156ec565b60405180910390fd5b565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612ac8576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612abf906157a0565b60405180910390fd5b565b5f5f5f60025f8781526020019081526020015f205f8660ff1660ff1681526020019081526020015f208054905090505f8103612c045760025f8781526020019081526020015f205f8660ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001866bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505050612e1f565b5f60025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f20600183612c3891906150e5565b81548110612c4957612c48614e67565b5b905f5260205f20019050805f0160089054906101000a90046bffffffffffffffffffffffff169250846bffffffffffffffffffffffff16836bffffffffffffffffffffffff1603612c9f575f9350505050612e68565b4363ffffffff16815f015f9054906101000a900463ffffffff1663ffffffff1603612cfb5784815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550612e1d565b43815f0160046101000a81548163ffffffff021916908363ffffffff16021790555060025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001876bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555050505b505b857f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d8686604051612e519291906157be565b60405180910390a2612e6382856136ca565b925050505b9392505050565b5f8160ff166001901b8317905092915050565b5f5f60015f8460ff1660ff1681526020019081526020015f208054905014159050919050565b5f815111612eeb576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612ee290615855565b60405180910390fd5b5f815190505f60035f8560ff1660ff1681526020019081526020015f20805490509050602060ff168282612f1f9190615873565b1115612f60576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612f579061593c565b60405180910390fd5b5f5f90505b8281101561331d575f5f90505b8183612f7e9190615873565b81101561306f57848281518110612f9857612f97614e67565b5b60200260200101515f015173ffffffffffffffffffffffffffffffffffffffff1660035f8860ff1660ff1681526020019081526020015f208281548110612fe257612fe1614e67565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603613062576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401613059906159ca565b60405180910390fd5b8080600101915050612f72565b505f84828151811061308457613083614e67565b5b6020026020010151602001516bffffffffffffffffffffffff16116130de576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016130d590615a7e565b60405180910390fd5b60035f8660ff1660ff1681526020019081526020015f2084828151811061310857613107614e67565b5b6020026020010151908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151815f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550505060045f8660ff1660ff1681526020019081526020015f208482815181106131d8576131d7614e67565b5b60200260200101515f0151908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508460ff167f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540485838151811061327857613277614e67565b5b60200260200101515f015160405161329091906148f6565b60405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a758583815181106132d0576132cf614e67565b5b60200260200101515f01518684815181106132ee576132ed614e67565b5b602002602001015160200151604051613308929190614eef565b60405180910390a28080600101915050612f65565b5050505050565b805f5f8460ff1660ff1681526020019081526020015f205f6101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055508160ff167f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf8260405161339b9190613ca2565b60405180910390a25050565b8060055f8460ff1660ff1681526020019081526020015f205f6101000a81548160ff021916908360018111156133e0576133df614371565b5b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d8160405161341491906143e4565b60405180910390a15050565b5f5f821215613445578161343390615a9c565b8361343e9190615ae2565b9050613454565b818361345191906153b3565b90505b92915050565b815f015163ffffffff168163ffffffff1610156134ac576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016134a390615bb7565b60405180910390fd5b5f826020015163ffffffff1614806134d35750816020015163ffffffff168163ffffffff16105b613512576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161350990615c91565b60405180910390fd5b5050565b5f60065f8460ff1660ff1681526020019081526020015f205f9054906101000a900463ffffffff1690508160065f8560ff1660ff1681526020019081526020015f205f6101000a81548163ffffffff021916908363ffffffff1602179055507f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c781836040516135a6929190615caf565b60405180910390a1505050565b5f5f60025f8681526020019081526020015f205f8560ff1660ff1681526020019081526020015f208054905090505f8190505b5f811115613687578363ffffffff1660025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f2060018361362891906150e5565b8154811061363957613638614e67565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff16116136745760018161366b91906150e5565b925050506136c3565b808061367f90615cd6565b9150506135e6565b506040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016136ba90615ddf565b60405180910390fd5b9392505050565b5f826bffffffffffffffffffffffff16826bffffffffffffffffffffffff166136f39190615dfd565b905092915050565b60605f600167ffffffffffffffff81111561371957613718614194565b5b6040519080825280602002602001820160405280156137475781602001602082028036833780820191505090505b50905082815f8151811061375e5761375d614e67565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250505f60065f8660ff1660ff1681526020019081526020015f205f9054906101000a900463ffffffff1663ffffffff16426137d19190615873565b90505f7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663ca8aa7c76040518163ffffffff1660e01b8152600401602060405180830381865afa15801561383d573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190613861919061562b565b73ffffffffffffffffffffffffffffffffffffffff16632bab2c4a60405180604001604052807f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1681526020018960ff1663ffffffff168152508560045f8b60ff1660ff1681526020019081526020015f20866040518563ffffffff1660e01b81526004016139089493929190615f11565b5f60405180830381865afa158015613922573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f8201168201806040525081019061394a9190616040565b9050805f8151811061395f5761395e614e67565b5b6020026020010151935050505092915050565b60405180606001604052805f63ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff1681525090565b60405180604001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f6bffffffffffffffffffffffff1681525090565b5f604051905090565b5f5ffd5b5f5ffd5b5f60ff82169050919050565b613a0c816139f7565b8114613a16575f5ffd5b50565b5f81359050613a2781613a03565b92915050565b5f819050919050565b613a3f81613a2d565b8114613a49575f5ffd5b50565b5f81359050613a5a81613a36565b92915050565b5f5f60408385031215613a7657613a756139ef565b5b5f613a8385828601613a19565b9250506020613a9485828601613a4c565b9150509250929050565b5f60208284031215613ab357613ab26139ef565b5b5f613ac084828501613a19565b91505092915050565b5f819050919050565b613adb81613ac9565b82525050565b5f602082019050613af45f830184613ad2565b92915050565b613b0381613ac9565b8114613b0d575f5ffd5b50565b5f81359050613b1e81613afa565b92915050565b5f5f60408385031215613b3a57613b396139ef565b5b5f613b4785828601613a19565b9250506020613b5885828601613b10565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f819050919050565b5f613ba4613b9f613b9a84613b62565b613b81565b613b62565b9050919050565b5f613bb582613b8a565b9050919050565b5f613bc682613bab565b9050919050565b613bd681613bbc565b82525050565b5f6bffffffffffffffffffffffff82169050919050565b613bfc81613bdc565b82525050565b5f604082019050613c155f830185613bcd565b613c226020830184613bf3565b9392505050565b5f613c3382613b62565b9050919050565b613c4381613c29565b8114613c4d575f5ffd5b50565b5f81359050613c5e81613c3a565b92915050565b5f5f60408385031215613c7a57613c796139ef565b5b5f613c8785828601613a19565b9250506020613c9885828601613c50565b9150509250929050565b5f602082019050613cb55f830184613bf3565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112613cdc57613cdb613cbb565b5b8235905067ffffffffffffffff811115613cf957613cf8613cbf565b5b602083019150836020820283011115613d1557613d14613cc3565b5b9250929050565b5f5f83601f840112613d3157613d30613cbb565b5b8235905067ffffffffffffffff811115613d4e57613d4d613cbf565b5b602083019150836020820283011115613d6a57613d69613cc3565b5b9250929050565b5f5f5f5f5f60608688031215613d8a57613d896139ef565b5b5f613d9788828901613a19565b955050602086013567ffffffffffffffff811115613db857613db76139f3565b5b613dc488828901613cc7565b9450945050604086013567ffffffffffffffff811115613de757613de66139f3565b5b613df388828901613d1c565b92509250509295509295909350565b5f819050919050565b613e1481613e02565b8114613e1e575f5ffd5b50565b5f81359050613e2f81613e0b565b92915050565b5f5f83601f840112613e4a57613e49613cbb565b5b8235905067ffffffffffffffff811115613e6757613e66613cbf565b5b602083019150836001820283011115613e8357613e82613cc3565b5b9250929050565b5f5f5f5f60608587031215613ea257613ea16139ef565b5b5f613eaf87828801613c50565b9450506020613ec087828801613e21565b935050604085013567ffffffffffffffff811115613ee157613ee06139f3565b5b613eed87828801613e35565b925092505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613f2d81613bdc565b82525050565b5f613f3e8383613f24565b60208301905092915050565b5f602082019050919050565b5f613f6082613efb565b613f6a8185613f05565b9350613f7583613f15565b805f5b83811015613fa5578151613f8c8882613f33565b9750613f9783613f4a565b925050600181019050613f78565b5085935050505092915050565b5f6040820190508181035f830152613fca8185613f56565b90508181036020830152613fde8184613f56565b90509392505050565b5f5f60408385031215613ffd57613ffc6139ef565b5b5f61400a85828601613e21565b925050602061401b85828601613a19565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f63ffffffff82169050919050565b6140668161404e565b82525050565b606082015f8201516140805f85018261405d565b506020820151614093602085018261405d565b5060408201516140a66040850182613f24565b50505050565b5f6140b7838361406c565b60608301905092915050565b5f602082019050919050565b5f6140d982614025565b6140e3818561402f565b93506140ee8361403f565b805f5b8381101561411e57815161410588826140ac565b9750614110836140c3565b9250506001810190506140f1565b5085935050505092915050565b5f6020820190508181035f83015261414381846140cf565b905092915050565b5f61415582613bab565b9050919050565b6141658161414b565b82525050565b5f60208201905061417e5f83018461415c565b92915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6141ca82614184565b810181811067ffffffffffffffff821117156141e9576141e8614194565b5b80604052505050565b5f6141fb6139e6565b905061420782826141c1565b919050565b5f67ffffffffffffffff82111561422657614225614194565b5b602082029050602081019050919050565b5f6142496142448461420c565b6141f2565b9050808382526020820190506020840283018581111561426c5761426b613cc3565b5b835b8181101561429557806142818882613b10565b84526020840193505060208101905061426e565b5050509392505050565b5f82601f8301126142b3576142b2613cbb565b5b81356142c3848260208601614237565b91505092915050565b5f5f604083850312156142e2576142e16139ef565b5b5f6142ef85828601613a19565b925050602083013567ffffffffffffffff8111156143105761430f6139f3565b5b61431c8582860161429f565b9150509250929050565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff82169050919050565b61435281614326565b82525050565b5f60208201905061436b5f830184614349565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b600281106143af576143ae614371565b5b50565b5f8190506143bf8261439e565b919050565b5f6143ce826143b2565b9050919050565b6143de816143c4565b82525050565b5f6020820190506143f75f8301846143d5565b92915050565b5f61440782613bab565b9050919050565b614417816143fd565b82525050565b5f6020820190506144305f83018461440e565b92915050565b61443f81613c29565b82525050565b5f6020820190506144585f830184614436565b92915050565b61446781613bdc565b8114614471575f5ffd5b50565b5f813590506144828161445e565b92915050565b5f5f5f6060848603121561449f5761449e6139ef565b5b5f6144ac86828701613e21565b93505060206144bd86828701613a19565b92505060406144ce86828701614474565b9150509250925092565b6144e181613a2d565b82525050565b5f6020820190506144fa5f8301846144d8565b92915050565b5f67ffffffffffffffff82111561451a57614519614194565b5b602082029050602081019050919050565b5f5ffd5b5f61453982613c29565b9050919050565b6145498161452f565b8114614553575f5ffd5b50565b5f8135905061456481614540565b92915050565b5f6040828403121561457f5761457e61452b565b5b61458960406141f2565b90505f61459884828501614556565b5f8301525060206145ab84828501614474565b60208301525092915050565b5f6145c96145c484614500565b6141f2565b905080838252602082019050604084028301858111156145ec576145eb613cc3565b5b835b818110156146155780614601888261456a565b8452602084019350506040810190506145ee565b5050509392505050565b5f82601f83011261463357614632613cbb565b5b81356146438482602086016145b7565b91505092915050565b5f5f5f60608486031215614663576146626139ef565b5b5f61467086828701613a19565b935050602061468186828701614474565b925050604084013567ffffffffffffffff8111156146a2576146a16139f3565b5b6146ae8682870161461f565b9150509250925092565b6146c1816139f7565b82525050565b5f6020820190506146da5f8301846146b8565b92915050565b5f5f604083850312156146f6576146f56139ef565b5b5f61470385828601614474565b925050602061471485828601613a4c565b9150509250929050565b6147278161404e565b8114614731575f5ffd5b50565b5f813590506147428161471e565b92915050565b5f5f5f6040848603121561475f5761475e6139ef565b5b5f61476c86828701614734565b935050602084013567ffffffffffffffff81111561478d5761478c6139f3565b5b61479986828701613e35565b92509250509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6147d9838361405d565b60208301905092915050565b5f602082019050919050565b5f6147fb826147a5565b61480581856147af565b9350614810836147bf565b805f5b8381101561484057815161482788826147ce565b9750614832836147e5565b925050600181019050614813565b5085935050505092915050565b5f6020820190508181035f83015261486581846147f1565b905092915050565b60028110614879575f5ffd5b50565b5f8135905061488a8161486d565b92915050565b5f5f604083850312156148a6576148a56139ef565b5b5f6148b385828601613a19565b92505060206148c48582860161487c565b9150509250929050565b6148d78161404e565b82525050565b5f6020820190506148f05f8301846148ce565b92915050565b5f6020820190506149095f830184613bcd565b92915050565b5f5f5f60608486031215614926576149256139ef565b5b5f61493386828701613a19565b935050602061494486828701613e21565b925050604061495586828701613b10565b9150509250925092565b606082015f8201516149735f85018261405d565b506020820151614986602085018261405d565b5060408201516149996040850182613f24565b50505050565b5f6060820190506149b25f83018461495f565b92915050565b6149c181613bbc565b82525050565b604082015f8201516149db5f8501826149b8565b5060208201516149ee6020850182613f24565b50505050565b5f604082019050614a075f8301846149c7565b92915050565b5f5f60408385031215614a2357614a226139ef565b5b5f614a3085828601613a19565b9250506020614a4185828601614474565b9150509250929050565b5f5f5f60408486031215614a6257614a616139ef565b5b5f614a6f86828701613e21565b935050602084013567ffffffffffffffff811115614a9057614a8f6139f3565b5b614a9c86828701613e35565b92509250509250925092565b5f5f60408385031215614abe57614abd6139ef565b5b5f614acb85828601613a19565b925050602083013567ffffffffffffffff811115614aec57614aeb6139f3565b5b614af88582860161461f565b9150509250929050565b5f5f5f60608486031215614b1957614b186139ef565b5b5f614b2686828701613a19565b9350506020614b3786828701614734565b9250506040614b4886828701613b10565b9150509250925092565b5f5f5f5f60808587031215614b6a57614b696139ef565b5b5f614b7787828801613a19565b9450506020614b8887828801614474565b9350506040614b9987828801614734565b925050606085013567ffffffffffffffff811115614bba57614bb96139f3565b5b614bc68782880161461f565b91505092959194509250565b5f5f5f60608486031215614be957614be86139ef565b5b5f614bf686828701613e21565b9350506020614c0786828701613a19565b9250506040614c1886828701614734565b9150509250925092565b5f614c2c82613bab565b9050919050565b614c3c81614c22565b82525050565b5f602082019050614c555f830184614c33565b92915050565b5f5f60408385031215614c7157614c706139ef565b5b5f614c7e85828601613a19565b9250506020614c8f85828601614734565b9150509250929050565b5f5f5f5f60808587031215614cb157614cb06139ef565b5b5f614cbe87828801613a19565b9450506020614ccf87828801614734565b9350506040614ce087828801613e21565b9250506060614cf187828801613b10565b91505092959194509250565b5f5f60408385031215614d1357614d126139ef565b5b5f614d2085828601614474565b9250506020614d3185828601614474565b9150509250929050565b5f82825260208201905092915050565b7f5374616b6552656769737472792e6d6f646966795374726174656779506172615f8201527f6d733a206e6f20737472617465677920696e64696365732070726f7669646564602082015250565b5f614da5604083614d3b565b9150614db082614d4b565b604082019050919050565b5f6020820190508181035f830152614dd281614d99565b9050919050565b7f5374616b6552656769737472792e6d6f646966795374726174656779506172615f8201527f6d733a20696e707574206c656e677468206d69736d6174636800000000000000602082015250565b5f614e33603983614d3b565b9150614e3e82614dd9565b604082019050919050565b5f6020820190508181035f830152614e6081614e27565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215614ea957614ea86139ef565b5b5f614eb684828501614474565b91505092915050565b5f614ed9614ed4614ecf84613bdc565b613b81565b613ac9565b9050919050565b614ee981614ebf565b82525050565b5f604082019050614f025f830185613bcd565b614f0f6020830184614ee0565b9392505050565b7f5374616b6552656769737472792e72656769737465724f70657261746f723a205f8201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360208201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000604082015250565b5f614f96605b83614d3b565b9150614fa182614f16565b606082019050919050565b5f6020820190508181035f830152614fc381614f8a565b9050919050565b7f5374616b6552656769737472792e72656d6f7665537472617465676965733a205f8201527f6e6f20696e646963657320746f2072656d6f76652070726f7669646564000000602082015250565b5f615024603d83614d3b565b915061502f82614fca565b604082019050919050565b5f6020820190508181035f83015261505181615018565b9050919050565b5f819050919050565b5f61507b61507661507184615058565b613b81565b613ac9565b9050919050565b61508b81615061565b82525050565b5f6040820190506150a45f830185613bcd565b6150b16020830184615082565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f6150ef82613ac9565b91506150fa83613ac9565b9250828203905081811115615112576151116150b8565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b7f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a205f8201527f71756f72756d20616c7265616479206578697374730000000000000000000000602082015250565b5f61519f603583614d3b565b91506151aa82615145565b604082019050919050565b5f6020820190508181035f8301526151cc81615193565b9050919050565b7f5374616b6552656769737472792e676574546f74616c5374616b65496e6469635f8201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360208201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000604082015250565b5f615253605b83614d3b565b915061525e826151d3565b606082019050919050565b5f6020820190508181035f83015261528081615247565b9050919050565b7f5374616b6552656769737472792e71756f72756d4578697374733a2071756f725f8201527f756d20646f6573206e6f74206578697374000000000000000000000000000000602082015250565b5f6152e1603183614d3b565b91506152ec82615287565b604082019050919050565b5f6020820190508181035f83015261530e816152d5565b9050919050565b5f61531f82613ac9565b915061532a83613ac9565b925082820261533881613ac9565b9150828204841483151761534f5761534e6150b8565b5b5092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61538d82613ac9565b915061539883613ac9565b9250826153a8576153a7615356565b5b828204905092915050565b5f6153bd82613bdc565b91506153c883613bdc565b925082820190506bffffffffffffffffffffffff8111156153ec576153eb6150b8565b5b92915050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f61542983836149b8565b60208301905092915050565b5f815f1c9050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61547161546c83615435565b615440565b9050919050565b5f615483825461545f565b9050919050565b5f600182019050919050565b5f6154a0826153f2565b6154aa81856153fc565b93506154b58361540c565b805f5b838110156154ec576154c982615478565b6154d3888261541e565b97506154de8361548a565b9250506001810190506154b8565b5085935050505092915050565b5f60408201905061550c5f830185614436565b818103602083015261551e8184615496565b90509392505050565b5f8151905061553581613afa565b92915050565b5f61554d6155488461420c565b6141f2565b905080838252602082019050602084028301858111156155705761556f613cc3565b5b835b8181101561559957806155858882615527565b845260208401935050602081019050615572565b5050509392505050565b5f82601f8301126155b7576155b6613cbb565b5b81516155c784826020860161553b565b91505092915050565b5f602082840312156155e5576155e46139ef565b5b5f82015167ffffffffffffffff811115615602576156016139f3565b5b61560e848285016155a3565b91505092915050565b5f8151905061562581613c3a565b92915050565b5f602082840312156156405761563f6139ef565b5b5f61564d84828501615617565b91505092915050565b7f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e5f8201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f66207460208201527f6865207265676973747279436f6f7264696e61746f7200000000000000000000604082015250565b5f6156d6605683614d3b565b91506156e182615656565b606082019050919050565b5f6020820190508181035f830152615703816156ca565b9050919050565b7f5374616b6552656769737472792e6f6e6c795265676973747279436f6f7264695f8201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260208201527f79436f6f7264696e61746f720000000000000000000000000000000000000000604082015250565b5f61578a604c83614d3b565b91506157958261570a565b606082019050919050565b5f6020820190508181035f8301526157b78161577e565b9050919050565b5f6040820190506157d15f8301856146b8565b6157de6020830184613bf3565b9392505050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a206e6f20737472617465676965732070726f76696465640000000000000000602082015250565b5f61583f603883614d3b565b915061584a826157e5565b604082019050919050565b5f6020820190508181035f83015261586c81615833565b9050919050565b5f61587d82613ac9565b915061588883613ac9565b92508282019050808211156158a05761589f6150b8565b5b92915050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60208201527f454e475448000000000000000000000000000000000000000000000000000000604082015250565b5f615926604583614d3b565b9150615931826158a6565b606082019050919050565b5f6020820190508181035f8301526159538161591a565b9050919050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a2063616e6e6f74206164642073616d65207374726174656779203278000000602082015250565b5f6159b4603d83614d3b565b91506159bf8261595a565b604082019050919050565b5f6020820190508181035f8301526159e1816159a8565b9050919050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f2060208201527f7765696768740000000000000000000000000000000000000000000000000000604082015250565b5f615a68604683614d3b565b9150615a73826159e8565b606082019050919050565b5f6020820190508181035f830152615a9581615a5c565b9050919050565b5f615aa682613a2d565b91507f80000000000000000000000000000000000000000000000000000000000000008203615ad857615ad76150b8565b5b815f039050919050565b5f615aec82613bdc565b9150615af783613bdc565b925082820390506bffffffffffffffffffffffff811115615b1b57615b1a6150b8565b5b92915050565b7f5374616b6552656769737472792e5f76616c69646174655374616b65557064615f8201527f74654174426c6f636b4e756d6265723a207374616b655570646174652069732060208201527f66726f6d20616674657220626c6f636b4e756d62657200000000000000000000604082015250565b5f615ba1605683614d3b565b9150615bac82615b21565b606082019050919050565b5f6020820190508181035f830152615bce81615b95565b9050919050565b7f5374616b6552656769737472792e5f76616c69646174655374616b65557064615f8201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560208201527f72207374616b6555706461746520617661696c61626c65206265666f7265206260408201527f6c6f636b4e756d62657200000000000000000000000000000000000000000000606082015250565b5f615c7b606a83614d3b565b9150615c8682615bd5565b608082019050919050565b5f6020820190508181035f830152615ca881615c6f565b9050919050565b5f604082019050615cc25f8301856148ce565b615ccf60208301846148ce565b9392505050565b5f615ce082613ac9565b91505f8203615cf257615cf16150b8565b5b600182039050919050565b7f5374616b6552656769737472792e5f6765745374616b65557064617465496e645f8201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360208201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460408201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560608201527f7200000000000000000000000000000000000000000000000000000000000000608082015250565b5f615dc9608183614d3b565b9150615dd482615cfd565b60a082019050919050565b5f6020820190508181035f830152615df681615dbd565b9050919050565b5f615e0782613a2d565b9150615e1283613a2d565b925082820390508181125f8412168282135f851215161715615e3757615e366150b8565b5b92915050565b615e4681613c29565b82525050565b604082015f820151615e605f850182615e3d565b506020820151615e73602085018261405d565b50505050565b5f81519050919050565b5f819050602082019050919050565b5f615e9d8383615e3d565b60208301905092915050565b5f602082019050919050565b5f615ebf82615e79565b615ec981856153fc565b9350615ed483615e83565b805f5b83811015615f04578151615eeb8882615e92565b9750615ef683615ea9565b925050600181019050615ed7565b5085935050505092915050565b5f60a082019050615f245f830187615e4c565b8181036040830152615f368186615eb5565b90508181036060830152615f4a8185615496565b9050615f5960808301846148ce565b95945050505050565b5f67ffffffffffffffff821115615f7c57615f7b614194565b5b602082029050602081019050919050565b5f615f9f615f9a84615f62565b6141f2565b90508083825260208201905060208402830185811115615fc257615fc1613cc3565b5b835b8181101561600957805167ffffffffffffffff811115615fe757615fe6613cbb565b5b808601615ff489826155a3565b85526020850194505050602081019050615fc4565b5050509392505050565b5f82601f83011261602757616026613cbb565b5b8151616037848260208601615f8d565b91505092915050565b5f60208284031215616055576160546139ef565b5b5f82015167ffffffffffffffff811115616072576160716139f3565b5b61607e84828501616013565b9150509291505056fea26469706673582212203f7b3c1ef2a299fecb4293eba34b760ee66ba6a887256eff2ffb3b5ec10b61ed64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x02\\W_5`\xE0\x1C\x80c\x81\xC0u\x02\x11a\x01DW\x80c\xC6\x01R}\x11a\0\xC1W\x80c\xDF\\\xF7#\x11a\0\x85W\x80c\xDF\\\xF7#\x14a\x08\x14W\x80c\xE0\x86\xAD\xB3\x14a\x082W\x80c\xF2\xBE\x94\xAE\x14a\x08NW\x80c\xF5\tU\x1A\x14a\x08~W\x80c\xF8Q\xE1\x98\x14a\x08\xAEW\x80c\xFA(\xC6'\x14a\x08\xDEWa\x02\\V[\x80c\xC6\x01R}\x14a\x07LW\x80c\xC8)LV\x14a\x07hW\x80c\xCCZ| \x14a\x07\x98W\x80c\xD5\xEC\xCC\x05\x14a\x07\xB4W\x80c\xDD\x98F\xB9\x14a\x07\xE4Wa\x02\\V[\x80c\xAD\xC8\x04\xDA\x11a\x01\x08W\x80c\xAD\xC8\x04\xDA\x14a\x06\x84W\x80c\xB6\x90Kx\x14a\x06\xB4W\x80c\xBC\x9A@\xC3\x14a\x06\xE4W\x80c\xBD)\xB8\xCD\x14a\x07\0W\x80c\xC4gx\xA5\x14a\x07\x1CWa\x02\\V[\x80c\x81\xC0u\x02\x14a\x05\xA8W\x80c\x86\xC0hV\x14a\x05\xD8W\x80c\x9A\xB4\xD6\xFF\x14a\x05\xF4W\x80c\x9F<\xCFe\x14a\x06$W\x80c\xACk\xFB\x03\x14a\x06TWa\x02\\V[\x80cT\x01\xED'\x11a\x01\xDDW\x80ck:\xA7.\x11a\x01\xA1W\x80ck:\xA7.\x14a\x04\xD2W\x80cm\x14\xA9\x87\x14a\x04\xF0W\x80ctELm\x14a\x05\x0EW\x80cu\xD4\x17:\x14a\x05>W\x80c|\x17#G\x14a\x05ZW\x80c\x7FB\x98\"\x14a\x05xWa\x02\\V[\x80cT\x01\xED'\x14a\x04\x08W\x80c^Zgu\x14a\x048W\x80c_\x1F-w\x14a\x04VW\x80cf\xAC\xFE\xFE\x14a\x04rW\x80ci\x7F\xBD\x93\x14a\x04\xA2Wa\x02\\V[\x80c%PGw\x11a\x02$W\x80c%PGw\x14a\x03)W\x80c,\xD9Y@\x14a\x03ZW\x80c9\x98\xFD\xD3\x14a\x03\x8AW\x80c<\xA5\xA5\xF5\x14a\x03\xA8W\x80cK\xD2n\t\x14a\x03\xD8Wa\x02\\V[\x80c\x03\x90\xA4\xD5\x14a\x02`W\x80c\x04\x91\xB4\x1C\x14a\x02|W\x80c\x08s$a\x14a\x02\xACW\x80c\x1F\x9Bt\xE0\x14a\x02\xDDW\x80c \xB6b\x98\x14a\x03\rW[__\xFD[a\x02z`\x04\x806\x03\x81\x01\x90a\x02u\x91\x90a:`V[a\t\x0EV[\0[a\x02\x96`\x04\x806\x03\x81\x01\x90a\x02\x91\x91\x90a:\x9EV[a\t\x1DV[`@Qa\x02\xA3\x91\x90a:\xE1V[`@Q\x80\x91\x03\x90\xF3[a\x02\xC6`\x04\x806\x03\x81\x01\x90a\x02\xC1\x91\x90a;$V[a\t@V[`@Qa\x02\xD4\x92\x91\x90a<\x02V[`@Q\x80\x91\x03\x90\xF3[a\x02\xF7`\x04\x806\x03\x81\x01\x90a\x02\xF2\x91\x90a<dV[a\t\xACV[`@Qa\x03\x04\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x03'`\x04\x806\x03\x81\x01\x90a\x03\"\x91\x90a=qV[a\t\xD0V[\0[a\x03C`\x04\x806\x03\x81\x01\x90a\x03>\x91\x90a>\x8AV[a\x0B\xF4V[`@Qa\x03Q\x92\x91\x90a?\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x03t`\x04\x806\x03\x81\x01\x90a\x03o\x91\x90a?\xE7V[a\r\xDFV[`@Qa\x03\x81\x91\x90aA+V[`@Q\x80\x91\x03\x90\xF3[a\x03\x92a\x0E\xE4V[`@Qa\x03\x9F\x91\x90aAkV[`@Q\x80\x91\x03\x90\xF3[a\x03\xC2`\x04\x806\x03\x81\x01\x90a\x03\xBD\x91\x90a:\x9EV[a\x0F\x08V[`@Qa\x03\xCF\x91\x90a:\xE1V[`@Q\x80\x91\x03\x90\xF3[a\x03\xF2`\x04\x806\x03\x81\x01\x90a\x03\xED\x91\x90a?\xE7V[a\x0F+V[`@Qa\x03\xFF\x91\x90a:\xE1V[`@Q\x80\x91\x03\x90\xF3[a\x04\"`\x04\x806\x03\x81\x01\x90a\x04\x1D\x91\x90a?\xE7V[a\x0F^V[`@Qa\x04/\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x04@a\x0FzV[`@Qa\x04M\x91\x90a:\xE1V[`@Q\x80\x91\x03\x90\xF3[a\x04p`\x04\x806\x03\x81\x01\x90a\x04k\x91\x90aB\xCCV[a\x0F\x86V[\0[a\x04\x8C`\x04\x806\x03\x81\x01\x90a\x04\x87\x91\x90a>\x8AV[a\x13\xD4V[`@Qa\x04\x99\x91\x90aCXV[`@Q\x80\x91\x03\x90\xF3[a\x04\xBC`\x04\x806\x03\x81\x01\x90a\x04\xB7\x91\x90a:\x9EV[a\x14\x9BV[`@Qa\x04\xC9\x91\x90aC\xE4V[`@Q\x80\x91\x03\x90\xF3[a\x04\xDAa\x14\xB8V[`@Qa\x04\xE7\x91\x90aD\x1DV[`@Q\x80\x91\x03\x90\xF3[a\x04\xF8a\x14\xDCV[`@Qa\x05\x05\x91\x90aDEV[`@Q\x80\x91\x03\x90\xF3[a\x05(`\x04\x806\x03\x81\x01\x90a\x05#\x91\x90aD\x88V[a\x15\0V[`@Qa\x055\x91\x90aD\xE7V[`@Q\x80\x91\x03\x90\xF3[a\x05X`\x04\x806\x03\x81\x01\x90a\x05S\x91\x90aFLV[a\x15\x15V[\0[a\x05ba\x16yV[`@Qa\x05o\x91\x90aF\xC7V[`@Q\x80\x91\x03\x90\xF3[a\x05\x92`\x04\x806\x03\x81\x01\x90a\x05\x8D\x91\x90aF\xE0V[a\x16~V[`@Qa\x05\x9F\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x05\xC2`\x04\x806\x03\x81\x01\x90a\x05\xBD\x91\x90aGHV[a\x16\x91V[`@Qa\x05\xCF\x91\x90aHMV[`@Q\x80\x91\x03\x90\xF3[a\x05\xF2`\x04\x806\x03\x81\x01\x90a\x05\xED\x91\x90aH\x90V[a\x18\xB9V[\0[a\x06\x0E`\x04\x806\x03\x81\x01\x90a\x06\t\x91\x90a:\x9EV[a\x18\xCFV[`@Qa\x06\x1B\x91\x90aH\xDDV[`@Q\x80\x91\x03\x90\xF3[a\x06>`\x04\x806\x03\x81\x01\x90a\x069\x91\x90a;$V[a\x18\xEFV[`@Qa\x06K\x91\x90aH\xF6V[`@Q\x80\x91\x03\x90\xF3[a\x06n`\x04\x806\x03\x81\x01\x90a\x06i\x91\x90aI\x0FV[a\x197V[`@Qa\x06{\x91\x90aI\x9FV[`@Q\x80\x91\x03\x90\xF3[a\x06\x9E`\x04\x806\x03\x81\x01\x90a\x06\x99\x91\x90a;$V[a\x1A\x19V[`@Qa\x06\xAB\x91\x90aI\xF4V[`@Q\x80\x91\x03\x90\xF3[a\x06\xCE`\x04\x806\x03\x81\x01\x90a\x06\xC9\x91\x90a;$V[a\x1A\xF6V[`@Qa\x06\xDB\x91\x90aI\x9FV[`@Q\x80\x91\x03\x90\xF3[a\x06\xFE`\x04\x806\x03\x81\x01\x90a\x06\xF9\x91\x90aJ\rV[a\x1B\xC8V[\0[a\x07\x1A`\x04\x806\x03\x81\x01\x90a\x07\x15\x91\x90aJKV[a\x1B\xE9V[\0[a\x076`\x04\x806\x03\x81\x01\x90a\x071\x91\x90a:\x9EV[a\x1C[V[`@Qa\x07C\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x07f`\x04\x806\x03\x81\x01\x90a\x07a\x91\x90aJ\xA8V[a\x1C\x82V[\0[a\x07\x82`\x04\x806\x03\x81\x01\x90a\x07}\x91\x90aK\x02V[a\x1C\xA3V[`@Qa\x07\x8F\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x07\xB2`\x04\x806\x03\x81\x01\x90a\x07\xAD\x91\x90aKRV[a\x1D\x82V[\0[a\x07\xCE`\x04\x806\x03\x81\x01\x90a\x07\xC9\x91\x90a:\x9EV[a\x1E\xF2V[`@Qa\x07\xDB\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x07\xFE`\x04\x806\x03\x81\x01\x90a\x07\xF9\x91\x90aK\xD2V[a\x1FkV[`@Qa\x08\x0B\x91\x90aH\xDDV[`@Q\x80\x91\x03\x90\xF3[a\x08\x1Ca\x1F\x80V[`@Qa\x08)\x91\x90aLBV[`@Q\x80\x91\x03\x90\xF3[a\x08L`\x04\x806\x03\x81\x01\x90a\x08G\x91\x90aL[V[a\x1F\xA4V[\0[a\x08h`\x04\x806\x03\x81\x01\x90a\x08c\x91\x90aL\x99V[a\x1F\xBAV[`@Qa\x08u\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\x08\x98`\x04\x806\x03\x81\x01\x90a\x08\x93\x91\x90aL\xFDV[a \xA9V[`@Qa\x08\xA5\x91\x90aD\xE7V[`@Q\x80\x91\x03\x90\xF3[a\x08\xC8`\x04\x806\x03\x81\x01\x90a\x08\xC3\x91\x90a?\xE7V[a \xBCV[`@Qa\x08\xD5\x91\x90aI\x9FV[`@Q\x80\x91\x03\x90\xF3[a\x08\xF8`\x04\x806\x03\x81\x01\x90a\x08\xF3\x91\x90aK\xD2V[a!\xF5V[`@Qa\t\x05\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xF3[a\t\x18\x82\x82a\"jV[PPPV[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\tYW_\x80\xFD[\x90_R` _ \x01_\x91P\x91PP\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82V[_\x82a\t\xB7\x81a$\x99V[_a\t\xC2\x85\x85a$\xE4V[P\x90P\x80\x92PPP\x92\x91PPV[a\t\xD8a)>V[\x84a\t\xE2\x81a$\x99V[_\x85\x85\x90P\x90P_\x81\x11a\n+W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\n\"\x90aM\xBBV[`@Q\x80\x91\x03\x90\xFD[\x80\x84\x84\x90P\x14a\npW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\ng\x90aNIV[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x89`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P__\x90P[\x82\x81\x10\x15a\x0B\xE9W\x85\x85\x82\x81\x81\x10a\n\xAAWa\n\xA9aNgV[[\x90P` \x02\x01` \x81\x01\x90a\n\xBF\x91\x90aN\x94V[\x82\x89\x89\x84\x81\x81\x10a\n\xD3Wa\n\xD2aNgV[[\x90P` \x02\x015\x81T\x81\x10a\n\xEBWa\n\xEAaNgV[[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\x0B\\Wa\x0B[aNgV[[\x90P` \x02\x015\x81T\x81\x10a\x0BtWa\x0BsaNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x88\x85\x81\x81\x10a\x0B\xB1Wa\x0B\xB0aNgV[[\x90P` \x02\x01` \x81\x01\x90a\x0B\xC6\x91\x90aN\x94V[`@Qa\x0B\xD4\x92\x91\x90aN\xEFV[`@Q\x80\x91\x03\x90\xA2\x80\x80`\x01\x01\x91PPa\n\x8FV[PPPPPPPPPV[``\x80a\x0B\xFFa*:V[_\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0C\x1DWa\x0C\x1CaA\x94V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0CKW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x85\x85\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0ClWa\x0CkaA\x94V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0C\x9AW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x86\x86\x90P\x81\x10\x15a\r\xCDW_\x87\x87\x83\x81\x81\x10a\x0C\xC1Wa\x0C\xC0aNgV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x0C\xD9\x81a$\x99V[__a\x0C\xE5\x83\x8Da$\xE4V[\x91P\x91P\x80a\r)W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\r \x90aO\xACV[`@Q\x80\x91\x03\x90\xFD[_a\r5\x8C\x85\x85a*\xCAV[\x90P\x82\x87\x86\x81Q\x81\x10a\rKWa\rJaNgV[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\r\x7F\x84\x82a\"jV[\x86\x86\x81Q\x81\x10a\r\x92Wa\r\x91aNgV[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPP\x80\x80`\x01\x01\x91PPa\x0C\xA2V[P\x81\x81\x93P\x93PPP\x94P\x94\x92PPPV[```\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\x0E\xD8W\x83\x82\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81R` \x01\x90`\x01\x01\x90a\x0E&V[PPPP\x90P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x92\x91PPV[__a\x0Fj\x84\x84a \xBCV[\x90P\x80`@\x01Q\x91PP\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x0F\x8Ea)>V[\x81a\x0F\x98\x81a$\x99V[_\x82Q\x90P_\x81\x11a\x0F\xDFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0F\xD6\x90aP:V[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P_`\x04_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P__\x90P[\x83\x81\x10\x15a\x13\xCBW\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x10YWa\x10XaNgV[[` \x02` \x01\x01Q\x81T\x81\x10a\x10rWa\x10qaNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa\x10\xA9\x91\x90aH\xF6V[`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x10\xEAWa\x10\xE9aNgV[[` \x02` \x01\x01Q\x81T\x81\x10a\x11\x03Wa\x11\x02aNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_`@Qa\x11<\x92\x91\x90aP\x91V[`@Q\x80\x91\x03\x90\xA2\x82`\x01\x84\x80T\x90Pa\x11V\x91\x90aP\xE5V[\x81T\x81\x10a\x11gWa\x11faNgV[[\x90_R` _ \x01\x83\x87\x83\x81Q\x81\x10a\x11\x83Wa\x11\x82aNgV[[` \x02` \x01\x01Q\x81T\x81\x10a\x11\x9CWa\x11\x9BaNgV[[\x90_R` _ \x01_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x90PP\x82\x80T\x80a\x12hWa\x12gaQ\x18V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01__\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U_\x82\x01`\x14a\x01\0\n\x81T\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UPP\x90U\x81`\x01\x83\x80T\x90Pa\x12\xD0\x91\x90aP\xE5V[\x81T\x81\x10a\x12\xE1Wa\x12\xE0aNgV[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x87\x83\x81Q\x81\x10a\x13\x1DWa\x13\x1CaNgV[[` \x02` \x01\x01Q\x81T\x81\x10a\x136Wa\x135aNgV[[\x90_R` _ \x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x80T\x80a\x13\x8CWa\x13\x8BaQ\x18V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x90U\x80\x80`\x01\x01\x91PPa\x10\x18V[PPPPPPPV[_a\x13\xDDa*:V[____\x90P[\x85\x85\x90P\x81\x10\x15a\x14\x8DW_\x86\x86\x83\x81\x81\x10a\x14\x03Wa\x14\x02aNgV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x14\x1B\x81a$\x99V[__a\x14'\x83\x8Ca$\xE4V[\x91P\x91P\x80a\x14cW_\x91Pa\x14`\x83\x87w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a.o\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P[_a\x14o\x8B\x85\x85a*\xCAV[\x90Pa\x14{\x84\x82a\"jV[PPPPP\x80\x80`\x01\x01\x91PPa\x13\xE4V[P\x81\x92PPP\x94\x93PPPPV[`\x05` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_a\x15\x0C\x84\x84\x84a*\xCAV[\x90P\x93\x92PPPV[a\x15\x1Da*:V[a\x15&\x83a.\x82V[\x15a\x15fW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x15]\x90aQ\xB5V[`@Q\x80\x91\x03\x90\xFD[a\x15p\x83\x82a.\xA8V[a\x15z\x83\x83a3$V[a\x15\x84\x83_a3\xA7V[`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[` \x81V[_a\x16\x89\x83\x83a4 V[\x90P\x92\x91PPV[``_\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x16\xB1Wa\x16\xB0aA\x94V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x16\xDFW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x84\x84\x90P\x81\x10\x15a\x18\xADW_\x85\x85\x83\x81\x81\x10a\x17\x06Wa\x17\x05aNgV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x17\x1E\x81a$\x99V[\x86c\xFF\xFF\xFF\xFF\x16`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x81\x10a\x17NWa\x17MaNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11\x15a\x17\xAFW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x17\xA6\x90aRiV[`@Q\x80\x91\x03\x90\xFD[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P__\x90P[\x81\x81\x10\x15a\x18\x9DW\x88c\xFF\xFF\xFF\xFF\x16`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83\x85a\x18\x06\x91\x90aP\xE5V[a\x18\x10\x91\x90aP\xE5V[\x81T\x81\x10a\x18!Wa\x18 aNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a\x18\x90W`\x01\x81\x83a\x18T\x91\x90aP\xE5V[a\x18^\x91\x90aP\xE5V[\x85\x85\x81Q\x81\x10a\x18qWa\x18paNgV[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x18\x9DV[\x80\x80`\x01\x01\x91PPa\x17\xD2V[PPP\x80\x80`\x01\x01\x91PPa\x16\xE7V[P\x80\x91PP\x93\x92PPPV[a\x18\xC1a)>V[a\x18\xCB\x82\x82a3\xA7V[PPV[`\x06` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x19\x08W_\x80\xFD[\x90_R` _ \x01_\x91P\x91P\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x19?a9rV[`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x19wWa\x19vaNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x93\x92PPPV[a\x1A!a9\xAAV[`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x1AJWa\x1AIaNgV[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x1A\xFEa9rV[`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x1B'Wa\x1B&aNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x1B\xD0a)>V[\x81a\x1B\xDA\x81a$\x99V[a\x1B\xE4\x83\x83a3$V[PPPV[a\x1B\xF1a*:V[__\x90P[\x82\x82\x90P\x81\x10\x15a\x1CUW_\x83\x83\x83\x81\x81\x10a\x1C\x15Wa\x1C\x14aNgV[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x1C-\x81a$\x99V[_a\x1C9\x86\x83_a*\xCAV[\x90Pa\x1CE\x82\x82a\"jV[PPP\x80\x80`\x01\x01\x91PPa\x1B\xF6V[PPPPV[_` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x1C\x8Aa)>V[\x81a\x1C\x94\x81a$\x99V[a\x1C\x9E\x83\x83a.\xA8V[PPPV[__`\x01_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x1C\xCEWa\x1C\xCDaNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90Pa\x1Ds\x81\x85a4ZV[\x80`@\x01Q\x91PP\x93\x92PPPV[a\x1D\x8Aa*:V[a\x1D\x93\x84a.\x82V[\x15a\x1D\xD3W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1D\xCA\x90aQ\xB5V[`@Q\x80\x91\x03\x90\xFD[a\x1D\xDD\x84\x82a.\xA8V[a\x1D\xE7\x84\x84a3$V[a\x1D\xF2\x84`\x01a3\xA7V[a\x1D\xFC\x84\x83a5\x16V[`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPV[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x80_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa\x1F0\x91\x90aP\xE5V[\x81T\x81\x10a\x1FAWa\x1F@aNgV[[\x90_R` _ \x01_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[_a\x1Fw\x84\x84\x84a5\xB3V[\x90P\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x1F\xACa)>V[a\x1F\xB6\x82\x82a5\x16V[PPV[__`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x1F\xF4Wa\x1F\xF3aNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90Pa \x99\x81\x86a4ZV[\x80`@\x01Q\x91PP\x94\x93PPPPV[_a \xB4\x83\x83a6\xCAV[\x90P\x92\x91PPV[a \xC4a9rV[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90Pa \xF9a9rV[_\x82\x03a!\nW\x80\x92PPPa!\xEFV[`\x02_\x86\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a!=\x91\x90aP\xE5V[\x81T\x81\x10a!NWa!MaNgV[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x80\x92PPP[\x92\x91PPV[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ a\"'\x85\x85\x85a5\xB3V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\">Wa\"=aNgV[[\x90_R` _ \x01_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x93\x92PPPV[__`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_`\x01_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a\"\xAE\x91\x90aP\xE5V[\x81T\x81\x10a\"\xBFWa\"\xBEaNgV[[\x90_R` _ \x01\x90P_\x84\x03a\"\xF5W\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92PPPa$\x93V[_a#\x1B\x82_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86a4 V[\x90PCc\xFF\xFF\xFF\xFF\x16\x82_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a#yW\x80\x82_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa$\x8CV[C\x82_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[\x80\x93PPPP[\x92\x91PPV[a$\xA2\x81a.\x82V[a$\xE1W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a$\xD8\x90aR\xF7V[`@Q\x80\x91\x03\x90\xFD[PV[____a$\xF1\x86a\x0F\x08V[\x90Pa$\xFBa9\xAAV[```\x01\x80\x81\x11\x15a%\x10Wa%\x0FaCqV[[`\x05_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a%FWa%EaCqV[[\x03a&\xC1Wa%U\x88\x88a6\xFBV[\x90P__\x90P[\x83\x81\x10\x15a&\xBBW`\x03_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a%\x8DWa%\x8CaNgV[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x92P_\x82\x82\x81Q\x81\x10a&GWa&FaNgV[[` \x02` \x01\x01Q\x11\x15a&\xAEWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10a&\x84Wa&\x83aNgV[[` \x02` \x01\x01Qa&\x96\x91\x90aS\x15V[a&\xA0\x91\x90aS\x83V[\x85a&\xAB\x91\x90aS\xB3V[\x94P[\x80\x80`\x01\x01\x91PPa%\\V[Pa(\xDCV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\x04\x13G\x88`\x04_\x8C`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a'2\x92\x91\x90aT\xF9V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a'LW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a't\x91\x90aU\xD0V[\x90P__\x90P[\x83\x81\x10\x15a(\xDAW`\x03_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a'\xACWa'\xABaNgV[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x92P_\x82\x82\x81Q\x81\x10a(fWa(eaNgV[[` \x02` \x01\x01Q\x11\x15a(\xCDWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10a(\xA3Wa(\xA2aNgV[[` \x02` \x01\x01Qa(\xB5\x91\x90aS\x15V[a(\xBF\x91\x90aS\x83V[\x85a(\xCA\x91\x90aS\xB3V[\x94P[\x80\x80`\x01\x01\x91PPa'{V[P[___\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15\x90P\x84\x81\x96P\x96PPPPPP\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a)\xA7W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a)\xCB\x91\x90aV+V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a*8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*/\x90aV\xECV[`@Q\x80\x91\x03\x90\xFD[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a*\xC8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a*\xBF\x90aW\xA0V[`@Q\x80\x91\x03\x90\xFD[V[___`\x02_\x87\x81R` \x01\x90\x81R` \x01_ _\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a,\x04W`\x02_\x87\x81R` \x01\x90\x81R` \x01_ _\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPa.\x1FV[_`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a,8\x91\x90aP\xE5V[\x81T\x81\x10a,IWa,HaNgV[[\x90_R` _ \x01\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92P\x84k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a,\x9FW_\x93PPPPa.hV[Cc\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a,\xFBW\x84\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa.\x1DV[C\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[P[\x85\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x86\x86`@Qa.Q\x92\x91\x90aW\xBEV[`@Q\x80\x91\x03\x90\xA2a.c\x82\x85a6\xCAV[\x92PPP[\x93\x92PPPV[_\x81`\xFF\x16`\x01\x90\x1B\x83\x17\x90P\x92\x91PPV[__`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15\x90P\x91\x90PV[_\x81Q\x11a.\xEBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.\xE2\x90aXUV[`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P_`\x03_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P` `\xFF\x16\x82\x82a/\x1F\x91\x90aXsV[\x11\x15a/`W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/W\x90aY<V[`@Q\x80\x91\x03\x90\xFD[__\x90P[\x82\x81\x10\x15a3\x1DW__\x90P[\x81\x83a/~\x91\x90aXsV[\x81\x10\x15a0oW\x84\x82\x81Q\x81\x10a/\x98Wa/\x97aNgV[[` \x02` \x01\x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a/\xE2Wa/\xE1aNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a0bW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0Y\x90aY\xCAV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa/rV[P_\x84\x82\x81Q\x81\x10a0\x84Wa0\x83aNgV[[` \x02` \x01\x01Q` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a0\xDEW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a0\xD5\x90aZ~V[`@Q\x80\x91\x03\x90\xFD[`\x03_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x84\x82\x81Q\x81\x10a1\x08Wa1\x07aNgV[[` \x02` \x01\x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP`\x04_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x84\x82\x81Q\x81\x10a1\xD8Wa1\xD7aNgV[[` \x02` \x01\x01Q_\x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\xFF\x16\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x85\x83\x81Q\x81\x10a2xWa2waNgV[[` \x02` \x01\x01Q_\x01Q`@Qa2\x90\x91\x90aH\xF6V[`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a2\xD0Wa2\xCFaNgV[[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a2\xEEWa2\xEDaNgV[[` \x02` \x01\x01Q` \x01Q`@Qa3\x08\x92\x91\x90aN\xEFV[`@Q\x80\x91\x03\x90\xA2\x80\x80`\x01\x01\x91PPa/eV[PPPPPV[\x80__\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\xFF\x16\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x82`@Qa3\x9B\x91\x90a<\xA2V[`@Q\x80\x91\x03\x90\xA2PPV[\x80`\x05_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a3\xE0Wa3\xDFaCqV[[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa4\x14\x91\x90aC\xE4V[`@Q\x80\x91\x03\x90\xA1PPV[__\x82\x12\x15a4EW\x81a43\x90aZ\x9CV[\x83a4>\x91\x90aZ\xE2V[\x90Pa4TV[\x81\x83a4Q\x91\x90aS\xB3V[\x90P[\x92\x91PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a4\xACW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a4\xA3\x90a[\xB7V[`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x14\x80a4\xD3WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a5\x12W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5\t\x90a\\\x91V[`@Q\x80\x91\x03\x90\xFD[PPV[_`\x06_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x81`\x06_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x81\x83`@Qa5\xA6\x92\x91\x90a\\\xAFV[`@Q\x80\x91\x03\x90\xA1PPPV[__`\x02_\x86\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x90P[_\x81\x11\x15a6\x87W\x83c\xFF\xFF\xFF\xFF\x16`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a6(\x91\x90aP\xE5V[\x81T\x81\x10a69Wa68aNgV[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a6tW`\x01\x81a6k\x91\x90aP\xE5V[\x92PPPa6\xC3V[\x80\x80a6\x7F\x90a\\\xD6V[\x91PPa5\xE6V[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a6\xBA\x90a]\xDFV[`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[_\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a6\xF3\x91\x90a]\xFDV[\x90P\x92\x91PPV[``_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a7\x19Wa7\x18aA\x94V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a7GW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x82\x81_\x81Q\x81\x10a7^Wa7]aNgV[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_`\x06_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16Ba7\xD1\x91\x90aXsV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x8A\xA7\xC7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a8=W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a8a\x91\x90aV+V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c+\xAB,J`@Q\x80`@\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RP\x85`\x04_\x8B`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a9\x08\x94\x93\x92\x91\x90a_\x11V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a9\"W=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a9J\x91\x90a`@V[\x90P\x80_\x81Q\x81\x10a9_Wa9^aNgV[[` \x02` \x01\x01Q\x93PPPP\x92\x91PPV[`@Q\x80``\x01`@R\x80_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80`@\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a:\x0C\x81a9\xF7V[\x81\x14a:\x16W__\xFD[PV[_\x815\x90Pa:'\x81a:\x03V[\x92\x91PPV[_\x81\x90P\x91\x90PV[a:?\x81a:-V[\x81\x14a:IW__\xFD[PV[_\x815\x90Pa:Z\x81a:6V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a:vWa:ua9\xEFV[[_a:\x83\x85\x82\x86\x01a:\x19V[\x92PP` a:\x94\x85\x82\x86\x01a:LV[\x91PP\x92P\x92\x90PV[_` \x82\x84\x03\x12\x15a:\xB3Wa:\xB2a9\xEFV[[_a:\xC0\x84\x82\x85\x01a:\x19V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a:\xDB\x81a:\xC9V[\x82RPPV[_` \x82\x01\x90Pa:\xF4_\x83\x01\x84a:\xD2V[\x92\x91PPV[a;\x03\x81a:\xC9V[\x81\x14a;\rW__\xFD[PV[_\x815\x90Pa;\x1E\x81a:\xFAV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a;:Wa;9a9\xEFV[[_a;G\x85\x82\x86\x01a:\x19V[\x92PP` a;X\x85\x82\x86\x01a;\x10V[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a;\xA4a;\x9Fa;\x9A\x84a;bV[a;\x81V[a;bV[\x90P\x91\x90PV[_a;\xB5\x82a;\x8AV[\x90P\x91\x90PV[_a;\xC6\x82a;\xABV[\x90P\x91\x90PV[a;\xD6\x81a;\xBCV[\x82RPPV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a;\xFC\x81a;\xDCV[\x82RPPV[_`@\x82\x01\x90Pa<\x15_\x83\x01\x85a;\xCDV[a<\"` \x83\x01\x84a;\xF3V[\x93\x92PPPV[_a<3\x82a;bV[\x90P\x91\x90PV[a<C\x81a<)V[\x81\x14a<MW__\xFD[PV[_\x815\x90Pa<^\x81a<:V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a<zWa<ya9\xEFV[[_a<\x87\x85\x82\x86\x01a:\x19V[\x92PP` a<\x98\x85\x82\x86\x01a<PV[\x91PP\x92P\x92\x90PV[_` \x82\x01\x90Pa<\xB5_\x83\x01\x84a;\xF3V[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a<\xDCWa<\xDBa<\xBBV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xF9Wa<\xF8a<\xBFV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a=\x15Wa=\x14a<\xC3V[[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a=1Wa=0a<\xBBV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=NWa=Ma<\xBFV[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a=jWa=ia<\xC3V[[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15a=\x8AWa=\x89a9\xEFV[[_a=\x97\x88\x82\x89\x01a:\x19V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xB8Wa=\xB7a9\xF3V[[a=\xC4\x88\x82\x89\x01a<\xC7V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=\xE7Wa=\xE6a9\xF3V[[a=\xF3\x88\x82\x89\x01a=\x1CV[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x81\x90P\x91\x90PV[a>\x14\x81a>\x02V[\x81\x14a>\x1EW__\xFD[PV[_\x815\x90Pa>/\x81a>\x0BV[\x92\x91PPV[__\x83`\x1F\x84\x01\x12a>JWa>Ia<\xBBV[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>gWa>fa<\xBFV[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a>\x83Wa>\x82a<\xC3V[[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a>\xA2Wa>\xA1a9\xEFV[[_a>\xAF\x87\x82\x88\x01a<PV[\x94PP` a>\xC0\x87\x82\x88\x01a>!V[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a>\xE1Wa>\xE0a9\xF3V[[a>\xED\x87\x82\x88\x01a>5V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a?-\x81a;\xDCV[\x82RPPV[_a?>\x83\x83a?$V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a?`\x82a>\xFBV[a?j\x81\x85a?\x05V[\x93Pa?u\x83a?\x15V[\x80_[\x83\x81\x10\x15a?\xA5W\x81Qa?\x8C\x88\x82a?3V[\x97Pa?\x97\x83a?JV[\x92PP`\x01\x81\x01\x90Pa?xV[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra?\xCA\x81\x85a?VV[\x90P\x81\x81\x03` \x83\x01Ra?\xDE\x81\x84a?VV[\x90P\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a?\xFDWa?\xFCa9\xEFV[[_a@\n\x85\x82\x86\x01a>!V[\x92PP` a@\x1B\x85\x82\x86\x01a:\x19V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a@f\x81a@NV[\x82RPPV[``\x82\x01_\x82\x01Qa@\x80_\x85\x01\x82a@]V[P` \x82\x01Qa@\x93` \x85\x01\x82a@]V[P`@\x82\x01Qa@\xA6`@\x85\x01\x82a?$V[PPPPV[_a@\xB7\x83\x83a@lV[``\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a@\xD9\x82a@%V[a@\xE3\x81\x85a@/V[\x93Pa@\xEE\x83a@?V[\x80_[\x83\x81\x10\x15aA\x1EW\x81QaA\x05\x88\x82a@\xACV[\x97PaA\x10\x83a@\xC3V[\x92PP`\x01\x81\x01\x90Pa@\xF1V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaAC\x81\x84a@\xCFV[\x90P\x92\x91PPV[_aAU\x82a;\xABV[\x90P\x91\x90PV[aAe\x81aAKV[\x82RPPV[_` \x82\x01\x90PaA~_\x83\x01\x84aA\\V[\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[aA\xCA\x82aA\x84V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15aA\xE9WaA\xE8aA\x94V[[\x80`@RPPPV[_aA\xFBa9\xE6V[\x90PaB\x07\x82\x82aA\xC1V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aB&WaB%aA\x94V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_aBIaBD\x84aB\x0CV[aA\xF2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aBlWaBka<\xC3V[[\x83[\x81\x81\x10\x15aB\x95W\x80aB\x81\x88\x82a;\x10V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaBnV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aB\xB3WaB\xB2a<\xBBV[[\x815aB\xC3\x84\x82` \x86\x01aB7V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15aB\xE2WaB\xE1a9\xEFV[[_aB\xEF\x85\x82\x86\x01a:\x19V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aC\x10WaC\x0Fa9\xF3V[[aC\x1C\x85\x82\x86\x01aB\x9FV[\x91PP\x92P\x92\x90PV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aCR\x81aC&V[\x82RPPV[_` \x82\x01\x90PaCk_\x83\x01\x84aCIV[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aC\xAFWaC\xAEaCqV[[PV[_\x81\x90PaC\xBF\x82aC\x9EV[\x91\x90PV[_aC\xCE\x82aC\xB2V[\x90P\x91\x90PV[aC\xDE\x81aC\xC4V[\x82RPPV[_` \x82\x01\x90PaC\xF7_\x83\x01\x84aC\xD5V[\x92\x91PPV[_aD\x07\x82a;\xABV[\x90P\x91\x90PV[aD\x17\x81aC\xFDV[\x82RPPV[_` \x82\x01\x90PaD0_\x83\x01\x84aD\x0EV[\x92\x91PPV[aD?\x81a<)V[\x82RPPV[_` \x82\x01\x90PaDX_\x83\x01\x84aD6V[\x92\x91PPV[aDg\x81a;\xDCV[\x81\x14aDqW__\xFD[PV[_\x815\x90PaD\x82\x81aD^V[\x92\x91PPV[___``\x84\x86\x03\x12\x15aD\x9FWaD\x9Ea9\xEFV[[_aD\xAC\x86\x82\x87\x01a>!V[\x93PP` aD\xBD\x86\x82\x87\x01a:\x19V[\x92PP`@aD\xCE\x86\x82\x87\x01aDtV[\x91PP\x92P\x92P\x92V[aD\xE1\x81a:-V[\x82RPPV[_` \x82\x01\x90PaD\xFA_\x83\x01\x84aD\xD8V[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aE\x1AWaE\x19aA\x94V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_aE9\x82a<)V[\x90P\x91\x90PV[aEI\x81aE/V[\x81\x14aESW__\xFD[PV[_\x815\x90PaEd\x81aE@V[\x92\x91PPV[_`@\x82\x84\x03\x12\x15aE\x7FWaE~aE+V[[aE\x89`@aA\xF2V[\x90P_aE\x98\x84\x82\x85\x01aEVV[_\x83\x01RP` aE\xAB\x84\x82\x85\x01aDtV[` \x83\x01RP\x92\x91PPV[_aE\xC9aE\xC4\x84aE\0V[aA\xF2V[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15aE\xECWaE\xEBa<\xC3V[[\x83[\x81\x81\x10\x15aF\x15W\x80aF\x01\x88\x82aEjV[\x84R` \x84\x01\x93PP`@\x81\x01\x90PaE\xEEV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aF3WaF2a<\xBBV[[\x815aFC\x84\x82` \x86\x01aE\xB7V[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15aFcWaFba9\xEFV[[_aFp\x86\x82\x87\x01a:\x19V[\x93PP` aF\x81\x86\x82\x87\x01aDtV[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aF\xA2WaF\xA1a9\xF3V[[aF\xAE\x86\x82\x87\x01aF\x1FV[\x91PP\x92P\x92P\x92V[aF\xC1\x81a9\xF7V[\x82RPPV[_` \x82\x01\x90PaF\xDA_\x83\x01\x84aF\xB8V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aF\xF6WaF\xF5a9\xEFV[[_aG\x03\x85\x82\x86\x01aDtV[\x92PP` aG\x14\x85\x82\x86\x01a:LV[\x91PP\x92P\x92\x90PV[aG'\x81a@NV[\x81\x14aG1W__\xFD[PV[_\x815\x90PaGB\x81aG\x1EV[\x92\x91PPV[___`@\x84\x86\x03\x12\x15aG_WaG^a9\xEFV[[_aGl\x86\x82\x87\x01aG4V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aG\x8DWaG\x8Ca9\xF3V[[aG\x99\x86\x82\x87\x01a>5V[\x92P\x92PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_aG\xD9\x83\x83a@]V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aG\xFB\x82aG\xA5V[aH\x05\x81\x85aG\xAFV[\x93PaH\x10\x83aG\xBFV[\x80_[\x83\x81\x10\x15aH@W\x81QaH'\x88\x82aG\xCEV[\x97PaH2\x83aG\xE5V[\x92PP`\x01\x81\x01\x90PaH\x13V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaHe\x81\x84aG\xF1V[\x90P\x92\x91PPV[`\x02\x81\x10aHyW__\xFD[PV[_\x815\x90PaH\x8A\x81aHmV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aH\xA6WaH\xA5a9\xEFV[[_aH\xB3\x85\x82\x86\x01a:\x19V[\x92PP` aH\xC4\x85\x82\x86\x01aH|V[\x91PP\x92P\x92\x90PV[aH\xD7\x81a@NV[\x82RPPV[_` \x82\x01\x90PaH\xF0_\x83\x01\x84aH\xCEV[\x92\x91PPV[_` \x82\x01\x90PaI\t_\x83\x01\x84a;\xCDV[\x92\x91PPV[___``\x84\x86\x03\x12\x15aI&WaI%a9\xEFV[[_aI3\x86\x82\x87\x01a:\x19V[\x93PP` aID\x86\x82\x87\x01a>!V[\x92PP`@aIU\x86\x82\x87\x01a;\x10V[\x91PP\x92P\x92P\x92V[``\x82\x01_\x82\x01QaIs_\x85\x01\x82a@]V[P` \x82\x01QaI\x86` \x85\x01\x82a@]V[P`@\x82\x01QaI\x99`@\x85\x01\x82a?$V[PPPPV[_``\x82\x01\x90PaI\xB2_\x83\x01\x84aI_V[\x92\x91PPV[aI\xC1\x81a;\xBCV[\x82RPPV[`@\x82\x01_\x82\x01QaI\xDB_\x85\x01\x82aI\xB8V[P` \x82\x01QaI\xEE` \x85\x01\x82a?$V[PPPPV[_`@\x82\x01\x90PaJ\x07_\x83\x01\x84aI\xC7V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aJ#WaJ\"a9\xEFV[[_aJ0\x85\x82\x86\x01a:\x19V[\x92PP` aJA\x85\x82\x86\x01aDtV[\x91PP\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aJbWaJaa9\xEFV[[_aJo\x86\x82\x87\x01a>!V[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\x90WaJ\x8Fa9\xF3V[[aJ\x9C\x86\x82\x87\x01a>5V[\x92P\x92PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aJ\xBEWaJ\xBDa9\xEFV[[_aJ\xCB\x85\x82\x86\x01a:\x19V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aJ\xECWaJ\xEBa9\xF3V[[aJ\xF8\x85\x82\x86\x01aF\x1FV[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15aK\x19WaK\x18a9\xEFV[[_aK&\x86\x82\x87\x01a:\x19V[\x93PP` aK7\x86\x82\x87\x01aG4V[\x92PP`@aKH\x86\x82\x87\x01a;\x10V[\x91PP\x92P\x92P\x92V[____`\x80\x85\x87\x03\x12\x15aKjWaKia9\xEFV[[_aKw\x87\x82\x88\x01a:\x19V[\x94PP` aK\x88\x87\x82\x88\x01aDtV[\x93PP`@aK\x99\x87\x82\x88\x01aG4V[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aK\xBAWaK\xB9a9\xF3V[[aK\xC6\x87\x82\x88\x01aF\x1FV[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15aK\xE9WaK\xE8a9\xEFV[[_aK\xF6\x86\x82\x87\x01a>!V[\x93PP` aL\x07\x86\x82\x87\x01a:\x19V[\x92PP`@aL\x18\x86\x82\x87\x01aG4V[\x91PP\x92P\x92P\x92V[_aL,\x82a;\xABV[\x90P\x91\x90PV[aL<\x81aL\"V[\x82RPPV[_` \x82\x01\x90PaLU_\x83\x01\x84aL3V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aLqWaLpa9\xEFV[[_aL~\x85\x82\x86\x01a:\x19V[\x92PP` aL\x8F\x85\x82\x86\x01aG4V[\x91PP\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15aL\xB1WaL\xB0a9\xEFV[[_aL\xBE\x87\x82\x88\x01a:\x19V[\x94PP` aL\xCF\x87\x82\x88\x01aG4V[\x93PP`@aL\xE0\x87\x82\x88\x01a>!V[\x92PP``aL\xF1\x87\x82\x88\x01a;\x10V[\x91PP\x92\x95\x91\x94P\x92PV[__`@\x83\x85\x03\x12\x15aM\x13WaM\x12a9\xEFV[[_aM \x85\x82\x86\x01aDtV[\x92PP` aM1\x85\x82\x86\x01aDtV[\x91PP\x92P\x92\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FStakeRegistry.modifyStrategyPara_\x82\x01R\x7Fms: no strategy indices provided` \x82\x01RPV[_aM\xA5`@\x83aM;V[\x91PaM\xB0\x82aMKV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaM\xD2\x81aM\x99V[\x90P\x91\x90PV[\x7FStakeRegistry.modifyStrategyPara_\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0` \x82\x01RPV[_aN3`9\x83aM;V[\x91PaN>\x82aM\xD9V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaN`\x81aN'V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aN\xA9WaN\xA8a9\xEFV[[_aN\xB6\x84\x82\x85\x01aDtV[\x91PP\x92\x91PPV[_aN\xD9aN\xD4aN\xCF\x84a;\xDCV[a;\x81V[a:\xC9V[\x90P\x91\x90PV[aN\xE9\x81aN\xBFV[\x82RPPV[_`@\x82\x01\x90PaO\x02_\x83\x01\x85a;\xCDV[aO\x0F` \x83\x01\x84aN\xE0V[\x93\x92PPPV[\x7FStakeRegistry.registerOperator: _\x82\x01R\x7FOperator does not meet minimum s` \x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`@\x82\x01RPV[_aO\x96`[\x83aM;V[\x91PaO\xA1\x82aO\x16V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaO\xC3\x81aO\x8AV[\x90P\x91\x90PV[\x7FStakeRegistry.removeStrategies: _\x82\x01R\x7Fno indices to remove provided\0\0\0` \x82\x01RPV[_aP$`=\x83aM;V[\x91PaP/\x82aO\xCAV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaPQ\x81aP\x18V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_aP{aPvaPq\x84aPXV[a;\x81V[a:\xC9V[\x90P\x91\x90PV[aP\x8B\x81aPaV[\x82RPPV[_`@\x82\x01\x90PaP\xA4_\x83\x01\x85a;\xCDV[aP\xB1` \x83\x01\x84aP\x82V[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aP\xEF\x82a:\xC9V[\x91PaP\xFA\x83a:\xC9V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aQ\x12WaQ\x11aP\xB8V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[\x7FStakeRegistry.initializeQuorum: _\x82\x01R\x7Fquorum already exists\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aQ\x9F`5\x83aM;V[\x91PaQ\xAA\x82aQEV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaQ\xCC\x81aQ\x93V[\x90P\x91\x90PV[\x7FStakeRegistry.getTotalStakeIndic_\x82\x01R\x7FesAtBlockNumber: quorum has no s` \x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`@\x82\x01RPV[_aRS`[\x83aM;V[\x91PaR^\x82aQ\xD3V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaR\x80\x81aRGV[\x90P\x91\x90PV[\x7FStakeRegistry.quorumExists: quor_\x82\x01R\x7Fum does not exist\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aR\xE1`1\x83aM;V[\x91PaR\xEC\x82aR\x87V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaS\x0E\x81aR\xD5V[\x90P\x91\x90PV[_aS\x1F\x82a:\xC9V[\x91PaS*\x83a:\xC9V[\x92P\x82\x82\x02aS8\x81a:\xC9V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aSOWaSNaP\xB8V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_aS\x8D\x82a:\xC9V[\x91PaS\x98\x83a:\xC9V[\x92P\x82aS\xA8WaS\xA7aSVV[[\x82\x82\x04\x90P\x92\x91PPV[_aS\xBD\x82a;\xDCV[\x91PaS\xC8\x83a;\xDCV[\x92P\x82\x82\x01\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS\xECWaS\xEBaP\xB8V[[\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_aT)\x83\x83aI\xB8V[` \x83\x01\x90P\x92\x91PPV[_\x81_\x1C\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aTqaTl\x83aT5V[aT@V[\x90P\x91\x90PV[_aT\x83\x82TaT_V[\x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_aT\xA0\x82aS\xF2V[aT\xAA\x81\x85aS\xFCV[\x93PaT\xB5\x83aT\x0CV[\x80_[\x83\x81\x10\x15aT\xECWaT\xC9\x82aTxV[aT\xD3\x88\x82aT\x1EV[\x97PaT\xDE\x83aT\x8AV[\x92PP`\x01\x81\x01\x90PaT\xB8V[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90PaU\x0C_\x83\x01\x85aD6V[\x81\x81\x03` \x83\x01RaU\x1E\x81\x84aT\x96V[\x90P\x93\x92PPPV[_\x81Q\x90PaU5\x81a:\xFAV[\x92\x91PPV[_aUMaUH\x84aB\x0CV[aA\xF2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aUpWaUoa<\xC3V[[\x83[\x81\x81\x10\x15aU\x99W\x80aU\x85\x88\x82aU'V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaUrV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aU\xB7WaU\xB6a<\xBBV[[\x81QaU\xC7\x84\x82` \x86\x01aU;V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aU\xE5WaU\xE4a9\xEFV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aV\x02WaV\x01a9\xF3V[[aV\x0E\x84\x82\x85\x01aU\xA3V[\x91PP\x92\x91PPV[_\x81Q\x90PaV%\x81a<:V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aV@WaV?a9\xEFV[[_aVM\x84\x82\x85\x01aV\x17V[\x91PP\x92\x91PPV[\x7FStakeRegistry.onlyCoordinatorOwn_\x82\x01R\x7Fer: caller is not the owner of t` \x82\x01R\x7Fhe registryCoordinator\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aV\xD6`V\x83aM;V[\x91PaV\xE1\x82aVVV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\x03\x81aV\xCAV[\x90P\x91\x90PV[\x7FStakeRegistry.onlyRegistryCoordi_\x82\x01R\x7Fnator: caller is not the Registr` \x82\x01R\x7FyCoordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aW\x8A`L\x83aM;V[\x91PaW\x95\x82aW\nV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaW\xB7\x81aW~V[\x90P\x91\x90PV[_`@\x82\x01\x90PaW\xD1_\x83\x01\x85aF\xB8V[aW\xDE` \x83\x01\x84a;\xF3V[\x93\x92PPPV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0` \x82\x01RPV[_aX?`8\x83aM;V[\x91PaXJ\x82aW\xE5V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaXl\x81aX3V[\x90P\x91\x90PV[_aX}\x82a:\xC9V[\x91PaX\x88\x83a:\xC9V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aX\xA0WaX\x9FaP\xB8V[[\x92\x91PPV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L` \x82\x01R\x7FENGTH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aY&`E\x83aM;V[\x91PaY1\x82aX\xA6V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaYS\x81aY\x1AV[\x90P\x91\x90PV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: cannot add same strategy 2x\0\0\0` \x82\x01RPV[_aY\xB4`=\x83aM;V[\x91PaY\xBF\x82aYZV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaY\xE1\x81aY\xA8V[\x90P\x91\x90PV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: cannot add strategy with zero ` \x82\x01R\x7Fweight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aZh`F\x83aM;V[\x91PaZs\x82aY\xE8V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaZ\x95\x81aZ\\V[\x90P\x91\x90PV[_aZ\xA6\x82a:-V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03aZ\xD8WaZ\xD7aP\xB8V[[\x81_\x03\x90P\x91\x90PV[_aZ\xEC\x82a;\xDCV[\x91PaZ\xF7\x83a;\xDCV[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a[\x1BWa[\x1AaP\xB8V[[\x92\x91PPV[\x7FStakeRegistry._validateStakeUpda_\x82\x01R\x7FteAtBlockNumber: stakeUpdate is ` \x82\x01R\x7Ffrom after blockNumber\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_a[\xA1`V\x83aM;V[\x91Pa[\xAC\x82a[!V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra[\xCE\x81a[\x95V[\x90P\x91\x90PV[\x7FStakeRegistry._validateStakeUpda_\x82\x01R\x7FteAtBlockNumber: there is a newe` \x82\x01R\x7Fr stakeUpdate available before b`@\x82\x01R\x7FlockNumber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_a\\{`j\x83aM;V[\x91Pa\\\x86\x82a[\xD5V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra\\\xA8\x81a\\oV[\x90P\x91\x90PV[_`@\x82\x01\x90Pa\\\xC2_\x83\x01\x85aH\xCEV[a\\\xCF` \x83\x01\x84aH\xCEV[\x93\x92PPPV[_a\\\xE0\x82a:\xC9V[\x91P_\x82\x03a\\\xF2Wa\\\xF1aP\xB8V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStakeRegistry._getStakeUpdateInd_\x82\x01R\x7FexForOperatorAtBlockNumber: no s` \x82\x01R\x7Ftake update found for operatorId`@\x82\x01R\x7F and quorumNumber at block numbe``\x82\x01R\x7Fr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01RPV[_a]\xC9`\x81\x83aM;V[\x91Pa]\xD4\x82a\\\xFDV[`\xA0\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra]\xF6\x81a]\xBDV[\x90P\x91\x90PV[_a^\x07\x82a:-V[\x91Pa^\x12\x83a:-V[\x92P\x82\x82\x03\x90P\x81\x81\x12_\x84\x12\x16\x82\x82\x13_\x85\x12\x15\x16\x17\x15a^7Wa^6aP\xB8V[[\x92\x91PPV[a^F\x81a<)V[\x82RPPV[`@\x82\x01_\x82\x01Qa^`_\x85\x01\x82a^=V[P` \x82\x01Qa^s` \x85\x01\x82a@]V[PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a^\x9D\x83\x83a^=V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a^\xBF\x82a^yV[a^\xC9\x81\x85aS\xFCV[\x93Pa^\xD4\x83a^\x83V[\x80_[\x83\x81\x10\x15a_\x04W\x81Qa^\xEB\x88\x82a^\x92V[\x97Pa^\xF6\x83a^\xA9V[\x92PP`\x01\x81\x01\x90Pa^\xD7V[P\x85\x93PPPP\x92\x91PPV[_`\xA0\x82\x01\x90Pa_$_\x83\x01\x87a^LV[\x81\x81\x03`@\x83\x01Ra_6\x81\x86a^\xB5V[\x90P\x81\x81\x03``\x83\x01Ra_J\x81\x85aT\x96V[\x90Pa_Y`\x80\x83\x01\x84aH\xCEV[\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a_|Wa_{aA\x94V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a_\x9Fa_\x9A\x84a_bV[aA\xF2V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a_\xC2Wa_\xC1a<\xC3V[[\x83[\x81\x81\x10\x15a`\tW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_\xE7Wa_\xE6a<\xBBV[[\x80\x86\x01a_\xF4\x89\x82aU\xA3V[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa_\xC4V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a`'Wa`&a<\xBBV[[\x81Qa`7\x84\x82` \x86\x01a_\x8DV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a`UWa`Ta9\xEFV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`rWa`qa9\xF3V[[a`~\x84\x82\x85\x01a`\x13V[\x91PP\x92\x91PPV\xFE\xA2dipfsX\"\x12 ?{<\x1E\xF2\xA2\x99\xFE\xCBB\x93\xEB\xA3Kv\x0E\xE6k\xA6\xA8\x87%n\xFF/\xFB;^\xC1\x0Ba\xEDdsolcC\0\x08\x1B\x003",
    );
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct StakeType(u8);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<StakeType> for u8 {
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
        impl StakeType {
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
        impl alloy_sol_types::SolType for StakeType {
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
        impl alloy_sol_types::EventTopic for StakeType {
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
    /**Event with signature `LookAheadPeriodChanged(uint32,uint32)` and selector `0x28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7`.
```solidity
event LookAheadPeriodChanged(uint32 oldLookAheadDays, uint32 newLookAheadDays);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct LookAheadPeriodChanged {
        #[allow(missing_docs)]
        pub oldLookAheadDays: u32,
        #[allow(missing_docs)]
        pub newLookAheadDays: u32,
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
        impl alloy_sol_types::SolEvent for LookAheadPeriodChanged {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "LookAheadPeriodChanged(uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                40u8,
                215u8,
                53u8,
                139u8,
                121u8,
                240u8,
                45u8,
                33u8,
                184u8,
                183u8,
                225u8,
                122u8,
                239u8,
                196u8,
                24u8,
                90u8,
                100u8,
                48u8,
                138u8,
                163u8,
                116u8,
                6u8,
                250u8,
                91u8,
                239u8,
                192u8,
                91u8,
                145u8,
                147u8,
                44u8,
                57u8,
                199u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    oldLookAheadDays: data.0,
                    newLookAheadDays: data.1,
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
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.oldLookAheadDays),
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.newLookAheadDays),
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
        impl alloy_sol_types::private::IntoLogData for LookAheadPeriodChanged {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&LookAheadPeriodChanged> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &LookAheadPeriodChanged) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
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
    /**Event with signature `StakeTypeSet(uint8)` and selector `0x7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d`.
```solidity
event StakeTypeSet(StakeType newStakeType);
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style
    )]
    #[derive(Clone)]
    pub struct StakeTypeSet {
        #[allow(missing_docs)]
        pub newStakeType: <StakeType as alloy::sol_types::SolType>::RustType,
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
        impl alloy_sol_types::SolEvent for StakeTypeSet {
            type DataTuple<'a> = (StakeType,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StakeTypeSet(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                124u8,
                17u8,
                46u8,
                134u8,
                60u8,
                207u8,
                0u8,
                120u8,
                98u8,
                226u8,
                201u8,
                226u8,
                88u8,
                25u8,
                201u8,
                51u8,
                254u8,
                219u8,
                201u8,
                53u8,
                10u8,
                100u8,
                67u8,
                66u8,
                59u8,
                74u8,
                133u8,
                153u8,
                194u8,
                232u8,
                165u8,
                45u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self { newStakeType: data.0 }
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
                (<StakeType as alloy_sol_types::SolType>::tokenize(&self.newStakeType),)
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
        impl alloy_sol_types::private::IntoLogData for StakeTypeSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&StakeTypeSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &StakeTypeSet) -> alloy_sol_types::private::LogData {
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
constructor(address _registryCoordinator, address _delegationManager, address _avsDirectory, address _serviceManager);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct constructorCall {
        pub _registryCoordinator: alloy::sol_types::private::Address,
        pub _delegationManager: alloy::sol_types::private::Address,
        pub _avsDirectory: alloy::sol_types::private::Address,
        pub _serviceManager: alloy::sol_types::private::Address,
    }
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
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
                    (
                        value._registryCoordinator,
                        value._delegationManager,
                        value._avsDirectory,
                        value._serviceManager,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for constructorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _registryCoordinator: tuple.0,
                        _delegationManager: tuple.1,
                        _avsDirectory: tuple.2,
                        _serviceManager: tuple.3,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolConstructor for constructorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
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
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._avsDirectory,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self._serviceManager,
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
    /**Function with signature `applyDelta(uint96,int256)` and selector `0x7f429822`.
```solidity
function applyDelta(uint96 value, int256 delta) external pure returns (uint96);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct applyDeltaCall {
        pub value: alloy::sol_types::private::primitives::aliases::U96,
        pub delta: alloy::sol_types::private::primitives::aliases::I256,
    }
    ///Container type for the return parameters of the [`applyDelta(uint96,int256)`](applyDeltaCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct applyDeltaReturn {
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
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Int<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<applyDeltaCall> for UnderlyingRustTuple<'_> {
                fn from(value: applyDeltaCall) -> Self {
                    (value.value, value.delta)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for applyDeltaCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        value: tuple.0,
                        delta: tuple.1,
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
            impl ::core::convert::From<applyDeltaReturn> for UnderlyingRustTuple<'_> {
                fn from(value: applyDeltaReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for applyDeltaReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for applyDeltaCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Int<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = applyDeltaReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "applyDelta(uint96,int256)";
            const SELECTOR: [u8; 4] = [127u8, 66u8, 152u8, 34u8];
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
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.value),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.delta),
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
    /**Function with signature `avsDirectory()` and selector `0x6b3aa72e`.
```solidity
function avsDirectory() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryCall {}
    ///Container type for the return parameters of the [`avsDirectory()`](avsDirectoryCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct avsDirectoryReturn {
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
            impl ::core::convert::From<avsDirectoryCall> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryCall {
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
            impl ::core::convert::From<avsDirectoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: avsDirectoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for avsDirectoryReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for avsDirectoryCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "avsDirectory()";
            const SELECTOR: [u8; 4] = [107u8, 58u8, 167u8, 46u8];
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
    /**Function with signature `calculateDelta(uint96,uint96)` and selector `0xf509551a`.
```solidity
function calculateDelta(uint96 prev, uint96 cur) external pure returns (int256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDeltaCall {
        pub prev: alloy::sol_types::private::primitives::aliases::U96,
        pub cur: alloy::sol_types::private::primitives::aliases::U96,
    }
    ///Container type for the return parameters of the [`calculateDelta(uint96,uint96)`](calculateDeltaCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct calculateDeltaReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U96,
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
            impl ::core::convert::From<calculateDeltaCall> for UnderlyingRustTuple<'_> {
                fn from(value: calculateDeltaCall) -> Self {
                    (value.prev, value.cur)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for calculateDeltaCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        prev: tuple.0,
                        cur: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<calculateDeltaReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: calculateDeltaReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for calculateDeltaReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for calculateDeltaCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = calculateDeltaReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "calculateDelta(uint96,uint96)";
            const SELECTOR: [u8; 4] = [245u8, 9u8, 85u8, 26u8];
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
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.prev),
                    <alloy::sol_types::sol_data::Uint<
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.cur),
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
    /**Function with signature `initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])` and selector `0x75d4173a`.
```solidity
function initializeDelegatedStakeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeDelegatedStakeQuorumCall {
        pub quorumNumber: u8,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])`](initializeDelegatedStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeDelegatedStakeQuorumReturn {}
    #[allow(
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
            impl ::core::convert::From<initializeDelegatedStakeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeDelegatedStakeQuorumCall) -> Self {
                    (value.quorumNumber, value.minimumStake, value._strategyParams)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeDelegatedStakeQuorumCall {
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
            impl ::core::convert::From<initializeDelegatedStakeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeDelegatedStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeDelegatedStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeDelegatedStakeQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeDelegatedStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [117u8, 212u8, 23u8, 58u8];
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
    /**Function with signature `initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])` and selector `0xcc5a7c20`.
```solidity
function initializeSlashableStakeQuorum(uint8 quorumNumber, uint96 minimumStake, uint32 lookAheadPeriod, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeSlashableStakeQuorumCall {
        pub quorumNumber: u8,
        pub minimumStake: alloy::sol_types::private::primitives::aliases::U96,
        pub lookAheadPeriod: u32,
        pub _strategyParams: alloy::sol_types::private::Vec<
            <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
        >,
    }
    ///Container type for the return parameters of the [`initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])`](initializeSlashableStakeQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct initializeSlashableStakeQuorumReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::U96,
                u32,
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
            impl ::core::convert::From<initializeSlashableStakeQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeSlashableStakeQuorumCall) -> Self {
                    (
                        value.quorumNumber,
                        value.minimumStake,
                        value.lookAheadPeriod,
                        value._strategyParams,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeSlashableStakeQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        minimumStake: tuple.1,
                        lookAheadPeriod: tuple.2,
                        _strategyParams: tuple.3,
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
            impl ::core::convert::From<initializeSlashableStakeQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: initializeSlashableStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for initializeSlashableStakeQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeSlashableStakeQuorumCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
                alloy::sol_types::sol_data::Uint<32>,
                alloy::sol_types::sol_data::Array<IStakeRegistry::StrategyParams>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeSlashableStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])";
            const SELECTOR: [u8; 4] = [204u8, 90u8, 124u8, 32u8];
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
                    <alloy::sol_types::sol_data::Uint<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.lookAheadPeriod),
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
    /**Function with signature `recordOperatorStakeUpdate(bytes32,uint8,uint96)` and selector `0x74454c6d`.
```solidity
function recordOperatorStakeUpdate(bytes32 operatorId, uint8 quorumNumber, uint96 newStake) external returns (int256);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recordOperatorStakeUpdateCall {
        pub operatorId: alloy::sol_types::private::FixedBytes<32>,
        pub quorumNumber: u8,
        pub newStake: alloy::sol_types::private::primitives::aliases::U96,
    }
    ///Container type for the return parameters of the [`recordOperatorStakeUpdate(bytes32,uint8,uint96)`](recordOperatorStakeUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recordOperatorStakeUpdateReturn {
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
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
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
            impl ::core::convert::From<recordOperatorStakeUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: recordOperatorStakeUpdateCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.newStake)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for recordOperatorStakeUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operatorId: tuple.0,
                        quorumNumber: tuple.1,
                        newStake: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<recordOperatorStakeUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: recordOperatorStakeUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for recordOperatorStakeUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for recordOperatorStakeUpdateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = recordOperatorStakeUpdateReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "recordOperatorStakeUpdate(bytes32,uint8,uint96)";
            const SELECTOR: [u8; 4] = [116u8, 69u8, 76u8, 109u8];
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
                        96,
                    > as alloy_sol_types::SolType>::tokenize(&self.newStake),
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
    /**Function with signature `recordTotalStakeUpdate(uint8,int256)` and selector `0x0390a4d5`.
```solidity
function recordTotalStakeUpdate(uint8 quorumNumber, int256 stakeDelta) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recordTotalStakeUpdateCall {
        pub quorumNumber: u8,
        pub stakeDelta: alloy::sol_types::private::primitives::aliases::I256,
    }
    ///Container type for the return parameters of the [`recordTotalStakeUpdate(uint8,int256)`](recordTotalStakeUpdateCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct recordTotalStakeUpdateReturn {}
    #[allow(
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
                alloy::sol_types::sol_data::Int<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                alloy::sol_types::private::primitives::aliases::I256,
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
            impl ::core::convert::From<recordTotalStakeUpdateCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: recordTotalStakeUpdateCall) -> Self {
                    (value.quorumNumber, value.stakeDelta)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for recordTotalStakeUpdateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        stakeDelta: tuple.1,
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
            impl ::core::convert::From<recordTotalStakeUpdateReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: recordTotalStakeUpdateReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for recordTotalStakeUpdateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for recordTotalStakeUpdateCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Int<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = recordTotalStakeUpdateReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "recordTotalStakeUpdate(uint8,int256)";
            const SELECTOR: [u8; 4] = [3u8, 144u8, 164u8, 213u8];
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
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.stakeDelta),
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
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<serviceManagerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: serviceManagerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for serviceManagerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for serviceManagerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = serviceManagerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `setSlashableStakeLookahead(uint8,uint32)` and selector `0xe086adb3`.
```solidity
function setSlashableStakeLookahead(uint8 quorumNumber, uint32 _lookAheadPeriod) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSlashableStakeLookaheadCall {
        pub quorumNumber: u8,
        pub _lookAheadPeriod: u32,
    }
    ///Container type for the return parameters of the [`setSlashableStakeLookahead(uint8,uint32)`](setSlashableStakeLookaheadCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setSlashableStakeLookaheadReturn {}
    #[allow(
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
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, u32);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setSlashableStakeLookaheadCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSlashableStakeLookaheadCall) -> Self {
                    (value.quorumNumber, value._lookAheadPeriod)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSlashableStakeLookaheadCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _lookAheadPeriod: tuple.1,
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
            impl ::core::convert::From<setSlashableStakeLookaheadReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setSlashableStakeLookaheadReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setSlashableStakeLookaheadReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setSlashableStakeLookaheadCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Uint<8>,
                alloy::sol_types::sol_data::Uint<32>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSlashableStakeLookaheadReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setSlashableStakeLookahead(uint8,uint32)";
            const SELECTOR: [u8; 4] = [224u8, 134u8, 173u8, 179u8];
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
                    > as alloy_sol_types::SolType>::tokenize(&self._lookAheadPeriod),
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
    /**Function with signature `setStakeType(uint8,uint8)` and selector `0x86c06856`.
```solidity
function setStakeType(uint8 quorumNumber, StakeType _stakeType) external;
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStakeTypeCall {
        pub quorumNumber: u8,
        pub _stakeType: <StakeType as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`setStakeType(uint8,uint8)`](setStakeTypeCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct setStakeTypeReturn {}
    #[allow(
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
                StakeType,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                u8,
                <StakeType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<setStakeTypeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setStakeTypeCall) -> Self {
                    (value.quorumNumber, value._stakeType)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStakeTypeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                        _stakeType: tuple.1,
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
            impl ::core::convert::From<setStakeTypeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setStakeTypeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setStakeTypeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setStakeTypeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>, StakeType);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setStakeTypeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setStakeType(uint8,uint8)";
            const SELECTOR: [u8; 4] = [134u8, 192u8, 104u8, 86u8];
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
                    <StakeType as alloy_sol_types::SolType>::tokenize(&self._stakeType),
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
    /**Function with signature `slashableStakeLookAheadPerQuorum(uint8)` and selector `0x9ab4d6ff`.
```solidity
function slashableStakeLookAheadPerQuorum(uint8) external view returns (uint32);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashableStakeLookAheadPerQuorumCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`slashableStakeLookAheadPerQuorum(uint8)`](slashableStakeLookAheadPerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct slashableStakeLookAheadPerQuorumReturn {
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
            impl ::core::convert::From<slashableStakeLookAheadPerQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashableStakeLookAheadPerQuorumCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashableStakeLookAheadPerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
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
            impl ::core::convert::From<slashableStakeLookAheadPerQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: slashableStakeLookAheadPerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for slashableStakeLookAheadPerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashableStakeLookAheadPerQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashableStakeLookAheadPerQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "slashableStakeLookAheadPerQuorum(uint8)";
            const SELECTOR: [u8; 4] = [154u8, 180u8, 214u8, 255u8];
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
    /**Function with signature `stakeTypePerQuorum(uint8)` and selector `0x697fbd93`.
```solidity
function stakeTypePerQuorum(uint8) external view returns (StakeType);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeTypePerQuorumCall {
        pub _0: u8,
    }
    ///Container type for the return parameters of the [`stakeTypePerQuorum(uint8)`](stakeTypePerQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct stakeTypePerQuorumReturn {
        pub _0: <StakeType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<stakeTypePerQuorumCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakeTypePerQuorumCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakeTypePerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (StakeType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <StakeType as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<stakeTypePerQuorumReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: stakeTypePerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for stakeTypePerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeTypePerQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeTypePerQuorumReturn;
            type ReturnTuple<'a> = (StakeType,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "stakeTypePerQuorum(uint8)";
            const SELECTOR: [u8; 4] = [105u8, 127u8, 189u8, 147u8];
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
    ///Container for all the [`StakeRegistryHarness`](self) function calls.
    pub enum StakeRegistryHarnessCalls {
        MAX_WEIGHING_FUNCTION_LENGTH(MAX_WEIGHING_FUNCTION_LENGTHCall),
        WEIGHTING_DIVISOR(WEIGHTING_DIVISORCall),
        addStrategies(addStrategiesCall),
        applyDelta(applyDeltaCall),
        avsDirectory(avsDirectoryCall),
        calculateDelta(calculateDeltaCall),
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
        initializeDelegatedStakeQuorum(initializeDelegatedStakeQuorumCall),
        initializeSlashableStakeQuorum(initializeSlashableStakeQuorumCall),
        minimumStakeForQuorum(minimumStakeForQuorumCall),
        modifyStrategyParams(modifyStrategyParamsCall),
        recordOperatorStakeUpdate(recordOperatorStakeUpdateCall),
        recordTotalStakeUpdate(recordTotalStakeUpdateCall),
        registerOperator(registerOperatorCall),
        registryCoordinator(registryCoordinatorCall),
        removeStrategies(removeStrategiesCall),
        serviceManager(serviceManagerCall),
        setMinimumStakeForQuorum(setMinimumStakeForQuorumCall),
        setSlashableStakeLookahead(setSlashableStakeLookaheadCall),
        setStakeType(setStakeTypeCall),
        slashableStakeLookAheadPerQuorum(slashableStakeLookAheadPerQuorumCall),
        stakeTypePerQuorum(stakeTypePerQuorumCall),
        strategiesPerQuorum(strategiesPerQuorumCall),
        strategyParams(strategyParamsCall),
        strategyParamsByIndex(strategyParamsByIndexCall),
        strategyParamsLength(strategyParamsLengthCall),
        updateOperatorStake(updateOperatorStakeCall),
        weightOfOperatorForQuorum(weightOfOperatorForQuorumCall),
    }
    #[automatically_derived]
    impl StakeRegistryHarnessCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [3u8, 144u8, 164u8, 213u8],
            [4u8, 145u8, 180u8, 28u8],
            [8u8, 115u8, 36u8, 97u8],
            [31u8, 155u8, 116u8, 224u8],
            [32u8, 182u8, 98u8, 152u8],
            [37u8, 80u8, 71u8, 119u8],
            [44u8, 217u8, 89u8, 64u8],
            [57u8, 152u8, 253u8, 211u8],
            [60u8, 165u8, 165u8, 245u8],
            [75u8, 210u8, 110u8, 9u8],
            [84u8, 1u8, 237u8, 39u8],
            [94u8, 90u8, 103u8, 117u8],
            [95u8, 31u8, 45u8, 119u8],
            [102u8, 172u8, 254u8, 254u8],
            [105u8, 127u8, 189u8, 147u8],
            [107u8, 58u8, 167u8, 46u8],
            [109u8, 20u8, 169u8, 135u8],
            [116u8, 69u8, 76u8, 109u8],
            [117u8, 212u8, 23u8, 58u8],
            [124u8, 23u8, 35u8, 71u8],
            [127u8, 66u8, 152u8, 34u8],
            [129u8, 192u8, 117u8, 2u8],
            [134u8, 192u8, 104u8, 86u8],
            [154u8, 180u8, 214u8, 255u8],
            [159u8, 60u8, 207u8, 101u8],
            [172u8, 107u8, 251u8, 3u8],
            [173u8, 200u8, 4u8, 218u8],
            [182u8, 144u8, 75u8, 120u8],
            [188u8, 154u8, 64u8, 195u8],
            [189u8, 41u8, 184u8, 205u8],
            [196u8, 103u8, 120u8, 165u8],
            [198u8, 1u8, 82u8, 125u8],
            [200u8, 41u8, 76u8, 86u8],
            [204u8, 90u8, 124u8, 32u8],
            [213u8, 236u8, 204u8, 5u8],
            [221u8, 152u8, 70u8, 185u8],
            [223u8, 92u8, 247u8, 35u8],
            [224u8, 134u8, 173u8, 179u8],
            [242u8, 190u8, 148u8, 174u8],
            [245u8, 9u8, 85u8, 26u8],
            [248u8, 81u8, 225u8, 152u8],
            [250u8, 40u8, 198u8, 39u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StakeRegistryHarnessCalls {
        const NAME: &'static str = "StakeRegistryHarnessCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 42usize;
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
                Self::applyDelta(_) => {
                    <applyDeltaCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::calculateDelta(_) => {
                    <calculateDeltaCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::initializeDelegatedStakeQuorum(_) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initializeSlashableStakeQuorum(_) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::minimumStakeForQuorum(_) => {
                    <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::modifyStrategyParams(_) => {
                    <modifyStrategyParamsCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::recordOperatorStakeUpdate(_) => {
                    <recordOperatorStakeUpdateCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::recordTotalStakeUpdate(_) => {
                    <recordTotalStakeUpdateCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::serviceManager(_) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setMinimumStakeForQuorum(_) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setSlashableStakeLookahead(_) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setStakeType(_) => {
                    <setStakeTypeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::slashableStakeLookAheadPerQuorum(_) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::stakeTypePerQuorum(_) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::SELECTOR
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
            ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls>] = &[
                {
                    fn recordTotalStakeUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <recordTotalStakeUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::recordTotalStakeUpdate)
                    }
                    recordTotalStakeUpdate
                },
                {
                    fn getTotalStakeHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getTotalStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getTotalStakeHistoryLength)
                    }
                    getTotalStakeHistoryLength
                },
                {
                    fn strategyParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <strategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::strategyParams)
                    }
                    strategyParams
                },
                {
                    fn weightOfOperatorForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <weightOfOperatorForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::weightOfOperatorForQuorum)
                    }
                    weightOfOperatorForQuorum
                },
                {
                    fn modifyStrategyParams(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <modifyStrategyParamsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::modifyStrategyParams)
                    }
                    modifyStrategyParams
                },
                {
                    fn registerOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <registerOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::registerOperator)
                    }
                    registerOperator
                },
                {
                    fn getStakeHistory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getStakeHistoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getStakeHistory)
                    }
                    getStakeHistory
                },
                {
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::serviceManager)
                    }
                    serviceManager
                },
                {
                    fn strategyParamsLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <strategyParamsLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::strategyParamsLength)
                    }
                    strategyParamsLength
                },
                {
                    fn getStakeHistoryLength(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getStakeHistoryLengthCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getStakeHistoryLength)
                    }
                    getStakeHistoryLength
                },
                {
                    fn getCurrentStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getCurrentStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getCurrentStake)
                    }
                    getCurrentStake
                },
                {
                    fn WEIGHTING_DIVISOR(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::WEIGHTING_DIVISOR)
                    }
                    WEIGHTING_DIVISOR
                },
                {
                    fn removeStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <removeStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::removeStrategies)
                    }
                    removeStrategies
                },
                {
                    fn updateOperatorStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <updateOperatorStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::updateOperatorStake)
                    }
                    updateOperatorStake
                },
                {
                    fn stakeTypePerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::stakeTypePerQuorum)
                    }
                    stakeTypePerQuorum
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::avsDirectory)
                    }
                    avsDirectory
                },
                {
                    fn registryCoordinator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <registryCoordinatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::registryCoordinator)
                    }
                    registryCoordinator
                },
                {
                    fn recordOperatorStakeUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <recordOperatorStakeUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::recordOperatorStakeUpdate)
                    }
                    recordOperatorStakeUpdate
                },
                {
                    fn initializeDelegatedStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                StakeRegistryHarnessCalls::initializeDelegatedStakeQuorum,
                            )
                    }
                    initializeDelegatedStakeQuorum
                },
                {
                    fn MAX_WEIGHING_FUNCTION_LENGTH(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::MAX_WEIGHING_FUNCTION_LENGTH)
                    }
                    MAX_WEIGHING_FUNCTION_LENGTH
                },
                {
                    fn applyDelta(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <applyDeltaCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::applyDelta)
                    }
                    applyDelta
                },
                {
                    fn getTotalStakeIndicesAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getTotalStakeIndicesAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                StakeRegistryHarnessCalls::getTotalStakeIndicesAtBlockNumber,
                            )
                    }
                    getTotalStakeIndicesAtBlockNumber
                },
                {
                    fn setStakeType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <setStakeTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::setStakeType)
                    }
                    setStakeType
                },
                {
                    fn slashableStakeLookAheadPerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                StakeRegistryHarnessCalls::slashableStakeLookAheadPerQuorum,
                            )
                    }
                    slashableStakeLookAheadPerQuorum
                },
                {
                    fn strategiesPerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <strategiesPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::strategiesPerQuorum)
                    }
                    strategiesPerQuorum
                },
                {
                    fn getStakeUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getStakeUpdateAtIndex)
                    }
                    getStakeUpdateAtIndex
                },
                {
                    fn strategyParamsByIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <strategyParamsByIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::strategyParamsByIndex)
                    }
                    strategyParamsByIndex
                },
                {
                    fn getTotalStakeUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getTotalStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getTotalStakeUpdateAtIndex)
                    }
                    getTotalStakeUpdateAtIndex
                },
                {
                    fn setMinimumStakeForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::setMinimumStakeForQuorum)
                    }
                    setMinimumStakeForQuorum
                },
                {
                    fn deregisterOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <deregisterOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::deregisterOperator)
                    }
                    deregisterOperator
                },
                {
                    fn minimumStakeForQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <minimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::minimumStakeForQuorum)
                    }
                    minimumStakeForQuorum
                },
                {
                    fn addStrategies(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <addStrategiesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::addStrategies)
                    }
                    addStrategies
                },
                {
                    fn getTotalStakeAtBlockNumberFromIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getTotalStakeAtBlockNumberFromIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                StakeRegistryHarnessCalls::getTotalStakeAtBlockNumberFromIndex,
                            )
                    }
                    getTotalStakeAtBlockNumberFromIndex
                },
                {
                    fn initializeSlashableStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                StakeRegistryHarnessCalls::initializeSlashableStakeQuorum,
                            )
                    }
                    initializeSlashableStakeQuorum
                },
                {
                    fn getCurrentTotalStake(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getCurrentTotalStakeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getCurrentTotalStake)
                    }
                    getCurrentTotalStake
                },
                {
                    fn getStakeUpdateIndexAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getStakeUpdateIndexAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                StakeRegistryHarnessCalls::getStakeUpdateIndexAtBlockNumber,
                            )
                    }
                    getStakeUpdateIndexAtBlockNumber
                },
                {
                    fn delegation(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::delegation)
                    }
                    delegation
                },
                {
                    fn setSlashableStakeLookahead(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::setSlashableStakeLookahead)
                    }
                    setSlashableStakeLookahead
                },
                {
                    fn getStakeAtBlockNumberAndIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getStakeAtBlockNumberAndIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(
                                StakeRegistryHarnessCalls::getStakeAtBlockNumberAndIndex,
                            )
                    }
                    getStakeAtBlockNumberAndIndex
                },
                {
                    fn calculateDelta(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <calculateDeltaCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::calculateDelta)
                    }
                    calculateDelta
                },
                {
                    fn getLatestStakeUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getLatestStakeUpdateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getLatestStakeUpdate)
                    }
                    getLatestStakeUpdate
                },
                {
                    fn getStakeAtBlockNumber(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <getStakeAtBlockNumberCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::getStakeAtBlockNumber)
                    }
                    getStakeAtBlockNumber
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
                Self::applyDelta(inner) => {
                    <applyDeltaCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::calculateDelta(inner) => {
                    <calculateDeltaCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::initializeDelegatedStakeQuorum(inner) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::initializeSlashableStakeQuorum(inner) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::recordOperatorStakeUpdate(inner) => {
                    <recordOperatorStakeUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::recordTotalStakeUpdate(inner) => {
                    <recordTotalStakeUpdateCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setMinimumStakeForQuorum(inner) => {
                    <setMinimumStakeForQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setSlashableStakeLookahead(inner) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setStakeType(inner) => {
                    <setStakeTypeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::slashableStakeLookAheadPerQuorum(inner) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::stakeTypePerQuorum(inner) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::applyDelta(inner) => {
                    <applyDeltaCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::calculateDelta(inner) => {
                    <calculateDeltaCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::initializeDelegatedStakeQuorum(inner) => {
                    <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initializeSlashableStakeQuorum(inner) => {
                    <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::recordOperatorStakeUpdate(inner) => {
                    <recordOperatorStakeUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::recordTotalStakeUpdate(inner) => {
                    <recordTotalStakeUpdateCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::serviceManager(inner) => {
                    <serviceManagerCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                Self::setSlashableStakeLookahead(inner) => {
                    <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setStakeType(inner) => {
                    <setStakeTypeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::slashableStakeLookAheadPerQuorum(inner) => {
                    <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::stakeTypePerQuorum(inner) => {
                    <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`StakeRegistryHarness`](self) events.
    pub enum StakeRegistryHarnessEvents {
        LookAheadPeriodChanged(LookAheadPeriodChanged),
        MinimumStakeForQuorumUpdated(MinimumStakeForQuorumUpdated),
        OperatorStakeUpdate(OperatorStakeUpdate),
        QuorumCreated(QuorumCreated),
        StakeTypeSet(StakeTypeSet),
        StrategyAddedToQuorum(StrategyAddedToQuorum),
        StrategyMultiplierUpdated(StrategyMultiplierUpdated),
        StrategyRemovedFromQuorum(StrategyRemovedFromQuorum),
    }
    #[automatically_derived]
    impl StakeRegistryHarnessEvents {
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
                40u8,
                215u8,
                53u8,
                139u8,
                121u8,
                240u8,
                45u8,
                33u8,
                184u8,
                183u8,
                225u8,
                122u8,
                239u8,
                196u8,
                24u8,
                90u8,
                100u8,
                48u8,
                138u8,
                163u8,
                116u8,
                6u8,
                250u8,
                91u8,
                239u8,
                192u8,
                91u8,
                145u8,
                147u8,
                44u8,
                57u8,
                199u8,
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
                124u8,
                17u8,
                46u8,
                134u8,
                60u8,
                207u8,
                0u8,
                120u8,
                98u8,
                226u8,
                201u8,
                226u8,
                88u8,
                25u8,
                201u8,
                51u8,
                254u8,
                219u8,
                201u8,
                53u8,
                10u8,
                100u8,
                67u8,
                66u8,
                59u8,
                74u8,
                133u8,
                153u8,
                194u8,
                232u8,
                165u8,
                45u8,
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
    impl alloy_sol_types::SolEventInterface for StakeRegistryHarnessEvents {
        const NAME: &'static str = "StakeRegistryHarnessEvents";
        const COUNT: usize = 8usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(
                    <LookAheadPeriodChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <LookAheadPeriodChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::LookAheadPeriodChanged)
                }
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
                Some(<StakeTypeSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakeTypeSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::StakeTypeSet)
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
    impl alloy_sol_types::private::IntoLogData for StakeRegistryHarnessEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::LookAheadPeriodChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::MinimumStakeForQuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorStakeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::StakeTypeSet(inner) => {
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
                Self::LookAheadPeriodChanged(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::MinimumStakeForQuorumUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorStakeUpdate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::QuorumCreated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::StakeTypeSet(inner) => {
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
    /**Creates a new wrapper around an on-chain [`StakeRegistryHarness`](self) contract instance.

See the [wrapper's documentation](`StakeRegistryHarnessInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> StakeRegistryHarnessInstance<T, P, N> {
        StakeRegistryHarnessInstance::<T, P, N>::new(address, provider)
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
        _avsDirectory: alloy::sol_types::private::Address,
        _serviceManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<StakeRegistryHarnessInstance<T, P, N>>,
    > {
        StakeRegistryHarnessInstance::<
            T,
            P,
            N,
        >::deploy(
            provider,
            _registryCoordinator,
            _delegationManager,
            _avsDirectory,
            _serviceManager,
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
        _delegationManager: alloy::sol_types::private::Address,
        _avsDirectory: alloy::sol_types::private::Address,
        _serviceManager: alloy::sol_types::private::Address,
    ) -> alloy_contract::RawCallBuilder<T, P, N> {
        StakeRegistryHarnessInstance::<
            T,
            P,
            N,
        >::deploy_builder(
            provider,
            _registryCoordinator,
            _delegationManager,
            _avsDirectory,
            _serviceManager,
        )
    }
    /**A [`StakeRegistryHarness`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`StakeRegistryHarness`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct StakeRegistryHarnessInstance<
        T,
        P,
        N = alloy_contract::private::Ethereum,
    > {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for StakeRegistryHarnessInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("StakeRegistryHarnessInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > StakeRegistryHarnessInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`StakeRegistryHarness`](self) contract instance.

See the [wrapper's documentation](`StakeRegistryHarnessInstance`) for more details.*/
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
            _avsDirectory: alloy::sol_types::private::Address,
            _serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<StakeRegistryHarnessInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(
                provider,
                _registryCoordinator,
                _delegationManager,
                _avsDirectory,
                _serviceManager,
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
            _avsDirectory: alloy::sol_types::private::Address,
            _serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                [
                    &BYTECODE[..],
                    &alloy_sol_types::SolConstructor::abi_encode(
                        &constructorCall {
                            _registryCoordinator,
                            _delegationManager,
                            _avsDirectory,
                            _serviceManager,
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
    impl<T, P: ::core::clone::Clone, N> StakeRegistryHarnessInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> StakeRegistryHarnessInstance<T, P, N> {
            StakeRegistryHarnessInstance {
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
    > StakeRegistryHarnessInstance<T, P, N> {
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
        ///Creates a new call builder for the [`applyDelta`] function.
        pub fn applyDelta(
            &self,
            value: alloy::sol_types::private::primitives::aliases::U96,
            delta: alloy::sol_types::private::primitives::aliases::I256,
        ) -> alloy_contract::SolCallBuilder<T, &P, applyDeltaCall, N> {
            self.call_builder(&applyDeltaCall { value, delta })
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`calculateDelta`] function.
        pub fn calculateDelta(
            &self,
            prev: alloy::sol_types::private::primitives::aliases::U96,
            cur: alloy::sol_types::private::primitives::aliases::U96,
        ) -> alloy_contract::SolCallBuilder<T, &P, calculateDeltaCall, N> {
            self.call_builder(&calculateDeltaCall { prev, cur })
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
        ///Creates a new call builder for the [`initializeDelegatedStakeQuorum`] function.
        pub fn initializeDelegatedStakeQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            initializeDelegatedStakeQuorumCall,
            N,
        > {
            self.call_builder(
                &initializeDelegatedStakeQuorumCall {
                    quorumNumber,
                    minimumStake,
                    _strategyParams,
                },
            )
        }
        ///Creates a new call builder for the [`initializeSlashableStakeQuorum`] function.
        pub fn initializeSlashableStakeQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            lookAheadPeriod: u32,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            initializeSlashableStakeQuorumCall,
            N,
        > {
            self.call_builder(
                &initializeSlashableStakeQuorumCall {
                    quorumNumber,
                    minimumStake,
                    lookAheadPeriod,
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
        ///Creates a new call builder for the [`recordOperatorStakeUpdate`] function.
        pub fn recordOperatorStakeUpdate(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            newStake: alloy::sol_types::private::primitives::aliases::U96,
        ) -> alloy_contract::SolCallBuilder<T, &P, recordOperatorStakeUpdateCall, N> {
            self.call_builder(
                &recordOperatorStakeUpdateCall {
                    operatorId,
                    quorumNumber,
                    newStake,
                },
            )
        }
        ///Creates a new call builder for the [`recordTotalStakeUpdate`] function.
        pub fn recordTotalStakeUpdate(
            &self,
            quorumNumber: u8,
            stakeDelta: alloy::sol_types::private::primitives::aliases::I256,
        ) -> alloy_contract::SolCallBuilder<T, &P, recordTotalStakeUpdateCall, N> {
            self.call_builder(
                &recordTotalStakeUpdateCall {
                    quorumNumber,
                    stakeDelta,
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
        ///Creates a new call builder for the [`serviceManager`] function.
        pub fn serviceManager(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, serviceManagerCall, N> {
            self.call_builder(&serviceManagerCall {})
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
        ///Creates a new call builder for the [`setSlashableStakeLookahead`] function.
        pub fn setSlashableStakeLookahead(
            &self,
            quorumNumber: u8,
            _lookAheadPeriod: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setSlashableStakeLookaheadCall, N> {
            self.call_builder(
                &setSlashableStakeLookaheadCall {
                    quorumNumber,
                    _lookAheadPeriod,
                },
            )
        }
        ///Creates a new call builder for the [`setStakeType`] function.
        pub fn setStakeType(
            &self,
            quorumNumber: u8,
            _stakeType: <StakeType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setStakeTypeCall, N> {
            self.call_builder(
                &setStakeTypeCall {
                    quorumNumber,
                    _stakeType,
                },
            )
        }
        ///Creates a new call builder for the [`slashableStakeLookAheadPerQuorum`] function.
        pub fn slashableStakeLookAheadPerQuorum(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<
            T,
            &P,
            slashableStakeLookAheadPerQuorumCall,
            N,
        > {
            self.call_builder(
                &slashableStakeLookAheadPerQuorumCall {
                    _0,
                },
            )
        }
        ///Creates a new call builder for the [`stakeTypePerQuorum`] function.
        pub fn stakeTypePerQuorum(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, stakeTypePerQuorumCall, N> {
            self.call_builder(&stakeTypePerQuorumCall { _0 })
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
    > StakeRegistryHarnessInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`LookAheadPeriodChanged`] event.
        pub fn LookAheadPeriodChanged_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, LookAheadPeriodChanged, N> {
            self.event_filter::<LookAheadPeriodChanged>()
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
        ///Creates a new event filter for the [`StakeTypeSet`] event.
        pub fn StakeTypeSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, StakeTypeSet, N> {
            self.event_filter::<StakeTypeSet>()
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
