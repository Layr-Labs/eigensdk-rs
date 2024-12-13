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
    function avsDirectory() external view returns (address);
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
pub mod StakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100604052348015610010575f5ffd5b5060405161612c38038061612c83398181016040528101906100329190610233565b838383838373ffffffffffffffffffffffffffffffffffffffff1660e08173ffffffffffffffffffffffffffffffffffffffff16815250508273ffffffffffffffffffffffffffffffffffffffff1660808173ffffffffffffffffffffffffffffffffffffffff16815250508173ffffffffffffffffffffffffffffffffffffffff1660a08173ffffffffffffffffffffffffffffffffffffffff16815250508073ffffffffffffffffffffffffffffffffffffffff1660c08173ffffffffffffffffffffffffffffffffffffffff16815250505050505050505050610297565b5f5ffd5b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61014082610117565b9050919050565b5f61015182610136565b9050919050565b61016181610147565b811461016b575f5ffd5b50565b5f8151905061017c81610158565b92915050565b5f61018c82610136565b9050919050565b61019c81610182565b81146101a6575f5ffd5b50565b5f815190506101b781610193565b92915050565b5f6101c782610136565b9050919050565b6101d7816101bd565b81146101e1575f5ffd5b50565b5f815190506101f2816101ce565b92915050565b5f61020282610136565b9050919050565b610212816101f8565b811461021c575f5ffd5b50565b5f8151905061022d81610209565b92915050565b5f5f5f5f6080858703121561024b5761024a610113565b5b5f6102588782880161016e565b9450506020610269878288016101a9565b935050604061027a878288016101e4565b925050606061028b8782880161021f565b91505092959194509250565b60805160a05160c05160e051615e3f6102ed5f395f81816113f7015281816125ef01526126eb01525f8181610dff0152818161364901526136fc01525f6113d301525f8181611e7301526123720152615e3f5ff3fe608060405234801561000f575f5ffd5b5060043610610230575f3560e01c806386c068561161012e578063c601527d116100b6578063df5cf7231161007a578063df5cf7231461076c578063e086adb31461078a578063f2be94ae146107a6578063f851e198146107d6578063fa28c6271461080657610230565b8063c601527d146106a4578063c8294c56146106c0578063cc5a7c20146106f0578063d5eccc051461070c578063dd9846b91461073c57610230565b8063adc804da116100fd578063adc804da146105dc578063b6904b781461060c578063bc9a40c31461063c578063bd29b8cd14610658578063c46778a51461067457610230565b806386c06856146105305780639ab4d6ff1461054c5780639f3ccf651461057c578063ac6bfb03146105ac57610230565b80635401ed27116101bc5780636b3aa72e116101805780636b3aa72e1461048a5780636d14a987146104a857806375d4173a146104c65780637c172347146104e257806381c075021461050057610230565b80635401ed27146103c05780635e5a6775146103f05780635f1f2d771461040e57806366acfefe1461042a578063697fbd931461045a57610230565b8063255047771161020357806325504777146102e15780632cd95940146103125780633998fdd3146103425780633ca5a5f5146103605780634bd26e091461039057610230565b80630491b41c1461023457806308732461146102645780631f9b74e01461029557806320b66298146102c5575b5f5ffd5b61024e6004803603810190610249919061390b565b610836565b60405161025b919061394e565b60405180910390f35b61027e60048036038101906102799190613991565b610859565b60405161028c929190613a6f565b60405180910390f35b6102af60048036038101906102aa9190613ad1565b6108c5565b6040516102bc9190613b0f565b60405180910390f35b6102df60048036038101906102da9190613bde565b6108e9565b005b6102fb60048036038101906102f69190613cf7565b610b0d565b604051610309929190613e1f565b60405180910390f35b61032c60048036038101906103279190613e54565b610cf8565b6040516103399190613f98565b60405180910390f35b61034a610dfd565b6040516103579190613fd8565b60405180910390f35b61037a6004803603810190610375919061390b565b610e21565b604051610387919061394e565b60405180910390f35b6103aa60048036038101906103a59190613e54565b610e44565b6040516103b7919061394e565b60405180910390f35b6103da60048036038101906103d59190613e54565b610e77565b6040516103e79190613b0f565b60405180910390f35b6103f8610e93565b604051610405919061394e565b60405180910390f35b61042860048036038101906104239190614139565b610e9f565b005b610444600480360381019061043f9190613cf7565b6112ed565b60405161045191906141c5565b60405180910390f35b610474600480360381019061046f919061390b565b6113b4565b6040516104819190614251565b60405180910390f35b6104926113d1565b60405161049f919061428a565b60405180910390f35b6104b06113f5565b6040516104bd91906142b2565b60405180910390f35b6104e060048036038101906104db9190614441565b611419565b005b6104ea61157d565b6040516104f791906144bc565b60405180910390f35b61051a600480360381019061051591906144ff565b611582565b6040516105279190614604565b60405180910390f35b61054a60048036038101906105459190614647565b6117aa565b005b6105666004803603810190610561919061390b565b6117c0565b6040516105739190614694565b60405180910390f35b61059660048036038101906105919190613991565b6117e0565b6040516105a391906146ad565b60405180910390f35b6105c660048036038101906105c191906146c6565b611828565b6040516105d39190614756565b60405180910390f35b6105f660048036038101906105f19190613991565b61190a565b60405161060391906147ab565b60405180910390f35b61062660048036038101906106219190613991565b6119e7565b6040516106339190614756565b60405180910390f35b610656600480360381019061065191906147c4565b611ab9565b005b610672600480360381019061066d9190614802565b611ada565b005b61068e6004803603810190610689919061390b565b611b4c565b60405161069b9190613b0f565b60405180910390f35b6106be60048036038101906106b9919061485f565b611b73565b005b6106da60048036038101906106d591906148b9565b611b94565b6040516106e79190613b0f565b60405180910390f35b61070a60048036038101906107059190614909565b611c73565b005b6107266004803603810190610721919061390b565b611de3565b6040516107339190613b0f565b60405180910390f35b61075660048036038101906107519190614989565b611e5c565b6040516107639190614694565b60405180910390f35b610774611e71565b60405161078191906149f9565b60405180910390f35b6107a4600480360381019061079f9190614a12565b611e95565b005b6107c060048036038101906107bb9190614a50565b611eab565b6040516107cd9190613b0f565b60405180910390f35b6107f060048036038101906107eb9190613e54565b611f9a565b6040516107fd9190614756565b60405180910390f35b610820600480360381019061081b9190614989565b6120d3565b60405161082d9190613b0f565b60405180910390f35b5f60015f8360ff1660ff1681526020019081526020015f20805490509050919050565b6003602052815f5260405f208181548110610872575f80fd5b905f5260205f20015f9150915050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690805f0160149054906101000a90046bffffffffffffffffffffffff16905082565b5f826108d081612148565b5f6108db8585612193565b509050809250505092915050565b6108f16125ed565b846108fb81612148565b5f8585905090505f8111610944576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161093b90614b34565b60405180910390fd5b808484905014610989576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161098090614bc2565b60405180910390fd5b5f60035f8960ff1660ff1681526020019081526020015f2090505f5f90505b82811015610b02578585828181106109c3576109c2614be0565b5b90506020020160208101906109d89190614c0d565b828989848181106109ec576109eb614be0565b5b9050602002013581548110610a0457610a03614be0565b5b905f5260205f20015f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a85818110610a7557610a74614be0565b5b9050602002013581548110610a8d57610a8c614be0565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16888885818110610aca57610ac9614be0565b5b9050602002016020810190610adf9190614c0d565b604051610aed929190614c68565b60405180910390a280806001019150506109a8565b505050505050505050565b606080610b186126e9565b5f8484905067ffffffffffffffff811115610b3657610b35614001565b5b604051908082528060200260200182016040528015610b645781602001602082028036833780820191505090505b5090505f8585905067ffffffffffffffff811115610b8557610b84614001565b5b604051908082528060200260200182016040528015610bb35781602001602082028036833780820191505090505b5090505f5f90505b86869050811015610ce6575f878783818110610bda57610bd9614be0565b5b9050013560f81c60f81b60f81c9050610bf281612148565b5f5f610bfe838d612193565b9150915080610c42576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c3990614d25565b60405180910390fd5b5f610c4e8c8585612779565b905082878681518110610c6457610c63614be0565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff1681525050610c988482612b1e565b868681518110610cab57610caa614be0565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff1681525050505050508080600101915050610bbb565b50818193509350505094509492505050565b606060025f8481526020019081526020015f205f8360ff1660ff1681526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015610df1578382905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505081526020019060010190610d3f565b50505050905092915050565b7f000000000000000000000000000000000000000000000000000000000000000081565b5f60035f8360ff1660ff1681526020019081526020015f20805490509050919050565b5f60025f8481526020019081526020015f205f8360ff1660ff1681526020019081526020015f2080549050905092915050565b5f5f610e838484611f9a565b9050806040015191505092915050565b670de0b6b3a764000081565b610ea76125ed565b81610eb181612148565b5f825190505f8111610ef8576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610eef90614db3565b60405180910390fd5b5f60035f8660ff1660ff1681526020019081526020015f2090505f60045f8760ff1660ff1681526020019081526020015f2090505f5f90505b838110156112e4578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f784888481518110610f7257610f71614be0565b5b602002602001015181548110610f8b57610f8a614be0565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16604051610fc291906146ad565b60405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a758488848151811061100357611002614be0565b5b60200260200101518154811061101c5761101b614be0565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f604051611055929190614e0a565b60405180910390a2826001848054905061106f9190614e5e565b815481106110805761107f614be0565b5b905f5260205f20018387838151811061109c5761109b614be0565b5b6020026020010151815481106110b5576110b4614be0565b5b905f5260205f20015f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f820160149054906101000a90046bffffffffffffffffffffffff16815f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055509050508280548061118157611180614e91565b5b600190038181905f5260205f20015f5f82015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff02191690555f820160146101000a8154906bffffffffffffffffffffffff02191690555050905581600183805490506111e99190614e5e565b815481106111fa576111f9614be0565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168287838151811061123657611235614be0565b5b60200260200101518154811061124f5761124e614be0565b5b905f5260205f20015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550818054806112a5576112a4614e91565b5b600190038181905f5260205f20015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905590558080600101915050610f31565b50505050505050565b5f6112f66126e9565b5f5f5f5f90505b858590508110156113a6575f86868381811061131c5761131b614be0565b5b9050013560f81c60f81b60f81c905061133481612148565b5f5f611340838c612193565b915091508061137c575f9150611379838777ffffffffffffffffffffffffffffffffffffffffffffffff16612d4d90919063ffffffff16565b95505b5f6113888b8585612779565b90506113948482612b1e565b505050505080806001019150506112fd565b508192505050949350505050565b6005602052805f5260405f205f915054906101000a900460ff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b6114216126e9565b61142a83612d60565b1561146a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161146190614f2e565b60405180910390fd5b6114748382612d86565b61147e8383613202565b611488835f613285565b60015f8460ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505050505050565b602081565b60605f8383905067ffffffffffffffff8111156115a2576115a1614001565b5b6040519080825280602002602001820160405280156115d05781602001602082028036833780820191505090505b5090505f5f90505b8484905081101561179e575f8585838181106115f7576115f6614be0565b5b9050013560f81c60f81b60f81c905061160f81612148565b8663ffffffff1660015f8360ff1660ff1681526020019081526020015f205f8154811061163f5761163e614be0565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff1611156116a0576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161169790614fe2565b60405180910390fd5b5f60015f8360ff1660ff1681526020019081526020015f208054905090505f5f90505b8181101561178e578863ffffffff1660015f8560ff1660ff1681526020019081526020015f20600183856116f79190614e5e565b6117019190614e5e565b8154811061171257611711614be0565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff161161178157600181836117459190614e5e565b61174f9190614e5e565b85858151811061176257611761614be0565b5b602002602001019063ffffffff16908163ffffffff168152505061178e565b80806001019150506116c3565b50505080806001019150506115d8565b50809150509392505050565b6117b26125ed565b6117bc8282613285565b5050565b6006602052805f5260405f205f915054906101000a900463ffffffff1681565b6004602052815f5260405f2081815481106117f9575f80fd5b905f5260205f20015f915091509054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b611830613850565b60025f8481526020019081526020015f205f8560ff1660ff1681526020019081526020015f20828154811061186857611867614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505090509392505050565b611912613888565b60035f8460ff1660ff1681526020019081526020015f20828154811061193b5761193a614be0565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905092915050565b6119ef613850565b60015f8460ff1660ff1681526020019081526020015f208281548110611a1857611a17614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905092915050565b611ac16125ed565b81611acb81612148565b611ad58383613202565b505050565b611ae26126e9565b5f5f90505b82829050811015611b46575f838383818110611b0657611b05614be0565b5b9050013560f81c60f81b60f81c9050611b1e81612148565b5f611b2a86835f612779565b9050611b368282612b1e565b5050508080600101915050611ae7565b50505050565b5f602052805f5260405f205f915054906101000a90046bffffffffffffffffffffffff1681565b611b7b6125ed565b81611b8581612148565b611b8f8383612d86565b505050565b5f5f60015f8660ff1660ff1681526020019081526020015f208381548110611bbf57611bbe614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff16815250509050611c6481856132fe565b80604001519150509392505050565b611c7b6126e9565b611c8484612d60565b15611cc4576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611cbb90614f2e565b60405180910390fd5b611cce8482612d86565b611cd88484613202565b611ce3846001613285565b611ced84836133ba565b60015f8560ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550505050505050565b5f60015f8360ff1660ff1681526020019081526020015f206001805f8560ff1660ff1681526020019081526020015f2080549050611e219190614e5e565b81548110611e3257611e31614be0565b5b905f5260205f20015f0160089054906101000a90046bffffffffffffffffffffffff169050919050565b5f611e68848484613457565b90509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b611e9d6125ed565b611ea782826133ba565b5050565b5f5f60025f8581526020019081526020015f205f8760ff1660ff1681526020019081526020015f208381548110611ee557611ee4614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff16815250509050611f8a81866132fe565b8060400151915050949350505050565b611fa2613850565b5f60025f8581526020019081526020015f205f8460ff1660ff1681526020019081526020015f20805490509050611fd7613850565b5f8203611fe85780925050506120cd565b60025f8681526020019081526020015f205f8560ff1660ff1681526020019081526020015f2060018361201b9190614e5e565b8154811061202c5761202b614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905080925050505b92915050565b5f60025f8581526020019081526020015f205f8460ff1660ff1681526020019081526020015f20612105858585613457565b63ffffffff168154811061211c5761211b614be0565b5b905f5260205f20015f0160089054906101000a90046bffffffffffffffffffffffff1690509392505050565b61215181612d60565b612190576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161218790615070565b60405180910390fd5b50565b5f5f5f5f6121a086610e21565b90506121aa613888565b60606001808111156121bf576121be6141de565b5b60055f8a60ff1660ff1681526020019081526020015f205f9054906101000a900460ff1660018111156121f5576121f46141de565b5b0361237057612204888861356e565b90505f5f90505b8381101561236a5760035f8a60ff1660ff1681526020019081526020015f20818154811061223c5761223b614be0565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505092505f8282815181106122f6576122f5614be0565b5b6020026020010151111561235d57670de0b6b3a764000083602001516bffffffffffffffffffffffff1683838151811061233357612332614be0565b5b6020026020010151612345919061508e565b61234f91906150fc565b8561235a919061512c565b94505b808060010191505061220b565b5061258b565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663900413478860045f8c60ff1660ff1681526020019081526020015f206040518363ffffffff1660e01b81526004016123e1929190615272565b5f60405180830381865afa1580156123fb573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906124239190615349565b90505f5f90505b838110156125895760035f8a60ff1660ff1681526020019081526020015f20818154811061245b5761245a614be0565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505092505f82828151811061251557612514614be0565b5b6020026020010151111561257c57670de0b6b3a764000083602001516bffffffffffffffffffffffff1683838151811061255257612551614be0565b5b6020026020010151612564919061508e565b61256e91906150fc565b85612579919061512c565b94505b808060010191505061242a565b505b5f5f5f8a60ff1660ff1681526020019081526020015f205f9054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff16856bffffffffffffffffffffffff161015905084819650965050505050509250929050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612656573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061267a91906153a4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146126e7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016126de90615465565b60405180910390fd5b565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612777576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161276e90615519565b60405180910390fd5b565b5f5f5f60025f8781526020019081526020015f205f8660ff1660ff1681526020019081526020015f208054905090505f81036128b35760025f8781526020019081526020015f205f8660ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001866bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505050612ace565b5f60025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f206001836128e79190614e5e565b815481106128f8576128f7614be0565b5b905f5260205f20019050805f0160089054906101000a90046bffffffffffffffffffffffff169250846bffffffffffffffffffffffff16836bffffffffffffffffffffffff160361294e575f9350505050612b17565b4363ffffffff16815f015f9054906101000a900463ffffffff1663ffffffff16036129aa5784815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550612acc565b43815f0160046101000a81548163ffffffff021916908363ffffffff16021790555060025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001876bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555050505b505b857f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d8686604051612b00929190615537565b60405180910390a2612b1282856137e5565b925050505b9392505050565b5f5f60015f8560ff1660ff1681526020019081526020015f208054905090505f60015f8660ff1660ff1681526020019081526020015f20600183612b629190614e5e565b81548110612b7357612b72614be0565b5b905f5260205f200190505f8403612ba957805f0160089054906101000a90046bffffffffffffffffffffffff1692505050612d47565b5f612bcf825f0160089054906101000a90046bffffffffffffffffffffffff1686613816565b90504363ffffffff16825f015f9054906101000a900463ffffffff1663ffffffff1603612c2d5780825f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550612d40565b43825f0160046101000a81548163ffffffff021916908363ffffffff16021790555060015f8760ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001836bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555050505b8093505050505b92915050565b5f8160ff166001901b8317905092915050565b5f5f60015f8460ff1660ff1681526020019081526020015f208054905014159050919050565b5f815111612dc9576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612dc0906155ce565b60405180910390fd5b5f815190505f60035f8560ff1660ff1681526020019081526020015f20805490509050602060ff168282612dfd91906155ec565b1115612e3e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612e35906156b5565b60405180910390fd5b5f5f90505b828110156131fb575f5f90505b8183612e5c91906155ec565b811015612f4d57848281518110612e7657612e75614be0565b5b60200260200101515f015173ffffffffffffffffffffffffffffffffffffffff1660035f8860ff1660ff1681526020019081526020015f208281548110612ec057612ebf614be0565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603612f40576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612f3790615743565b60405180910390fd5b8080600101915050612e50565b505f848281518110612f6257612f61614be0565b5b6020026020010151602001516bffffffffffffffffffffffff1611612fbc576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612fb3906157f7565b60405180910390fd5b60035f8660ff1660ff1681526020019081526020015f20848281518110612fe657612fe5614be0565b5b6020026020010151908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151815f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550505060045f8660ff1660ff1681526020019081526020015f208482815181106130b6576130b5614be0565b5b60200260200101515f0151908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508460ff167f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540485838151811061315657613155614be0565b5b60200260200101515f015160405161316e91906146ad565b60405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a758583815181106131ae576131ad614be0565b5b60200260200101515f01518684815181106131cc576131cb614be0565b5b6020026020010151602001516040516131e6929190614c68565b60405180910390a28080600101915050612e43565b5050505050565b805f5f8460ff1660ff1681526020019081526020015f205f6101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055508160ff167f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf826040516132799190613b0f565b60405180910390a25050565b8060055f8460ff1660ff1681526020019081526020015f205f6101000a81548160ff021916908360018111156132be576132bd6141de565b5b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d816040516132f29190614251565b60405180910390a15050565b815f015163ffffffff168163ffffffff161015613350576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401613347906158ab565b60405180910390fd5b5f826020015163ffffffff1614806133775750816020015163ffffffff168163ffffffff16105b6133b6576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016133ad90615985565b60405180910390fd5b5050565b5f60065f8460ff1660ff1681526020019081526020015f205f9054906101000a900463ffffffff1690508160065f8560ff1660ff1681526020019081526020015f205f6101000a81548163ffffffff021916908363ffffffff1602179055507f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7818360405161344a9291906159a3565b60405180910390a1505050565b5f5f60025f8681526020019081526020015f205f8560ff1660ff1681526020019081526020015f208054905090505f8190505b5f81111561352b578363ffffffff1660025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f206001836134cc9190614e5e565b815481106134dd576134dc614be0565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff16116135185760018161350f9190614e5e565b92505050613567565b8080613523906159ca565b91505061348a565b506040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161355e90615ad3565b60405180910390fd5b9392505050565b60605f600167ffffffffffffffff81111561358c5761358b614001565b5b6040519080825280602002602001820160405280156135ba5781602001602082028036833780820191505090505b50905082815f815181106135d1576135d0614be0565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250505f60065f8660ff1660ff1681526020019081526020015f205f9054906101000a900463ffffffff1663ffffffff164261364491906155ec565b90505f7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663ca8aa7c76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156136b0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136d491906153a4565b73ffffffffffffffffffffffffffffffffffffffff16632bab2c4a60405180604001604052807f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1681526020018960ff1663ffffffff168152508560045f8b60ff1660ff1681526020019081526020015f20866040518563ffffffff1660e01b815260040161377b9493929190615bc5565b5f60405180830381865afa158015613795573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906137bd9190615cf4565b9050805f815181106137d2576137d1614be0565b5b6020026020010151935050505092915050565b5f826bffffffffffffffffffffffff16826bffffffffffffffffffffffff1661380e9190615d44565b905092915050565b5f5f82121561383b578161382990615d84565b836138349190615dca565b905061384a565b8183613847919061512c565b90505b92915050565b60405180606001604052805f63ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff1681525090565b60405180604001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f6bffffffffffffffffffffffff1681525090565b5f604051905090565b5f5ffd5b5f5ffd5b5f60ff82169050919050565b6138ea816138d5565b81146138f4575f5ffd5b50565b5f81359050613905816138e1565b92915050565b5f602082840312156139205761391f6138cd565b5b5f61392d848285016138f7565b91505092915050565b5f819050919050565b61394881613936565b82525050565b5f6020820190506139615f83018461393f565b92915050565b61397081613936565b811461397a575f5ffd5b50565b5f8135905061398b81613967565b92915050565b5f5f604083850312156139a7576139a66138cd565b5b5f6139b4858286016138f7565b92505060206139c58582860161397d565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f819050919050565b5f613a11613a0c613a07846139cf565b6139ee565b6139cf565b9050919050565b5f613a22826139f7565b9050919050565b5f613a3382613a18565b9050919050565b613a4381613a29565b82525050565b5f6bffffffffffffffffffffffff82169050919050565b613a6981613a49565b82525050565b5f604082019050613a825f830185613a3a565b613a8f6020830184613a60565b9392505050565b5f613aa0826139cf565b9050919050565b613ab081613a96565b8114613aba575f5ffd5b50565b5f81359050613acb81613aa7565b92915050565b5f5f60408385031215613ae757613ae66138cd565b5b5f613af4858286016138f7565b9250506020613b0585828601613abd565b9150509250929050565b5f602082019050613b225f830184613a60565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112613b4957613b48613b28565b5b8235905067ffffffffffffffff811115613b6657613b65613b2c565b5b602083019150836020820283011115613b8257613b81613b30565b5b9250929050565b5f5f83601f840112613b9e57613b9d613b28565b5b8235905067ffffffffffffffff811115613bbb57613bba613b2c565b5b602083019150836020820283011115613bd757613bd6613b30565b5b9250929050565b5f5f5f5f5f60608688031215613bf757613bf66138cd565b5b5f613c04888289016138f7565b955050602086013567ffffffffffffffff811115613c2557613c246138d1565b5b613c3188828901613b34565b9450945050604086013567ffffffffffffffff811115613c5457613c536138d1565b5b613c6088828901613b89565b92509250509295509295909350565b5f819050919050565b613c8181613c6f565b8114613c8b575f5ffd5b50565b5f81359050613c9c81613c78565b92915050565b5f5f83601f840112613cb757613cb6613b28565b5b8235905067ffffffffffffffff811115613cd457613cd3613b2c565b5b602083019150836001820283011115613cf057613cef613b30565b5b9250929050565b5f5f5f5f60608587031215613d0f57613d0e6138cd565b5b5f613d1c87828801613abd565b9450506020613d2d87828801613c8e565b935050604085013567ffffffffffffffff811115613d4e57613d4d6138d1565b5b613d5a87828801613ca2565b925092505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613d9a81613a49565b82525050565b5f613dab8383613d91565b60208301905092915050565b5f602082019050919050565b5f613dcd82613d68565b613dd78185613d72565b9350613de283613d82565b805f5b83811015613e12578151613df98882613da0565b9750613e0483613db7565b925050600181019050613de5565b5085935050505092915050565b5f6040820190508181035f830152613e378185613dc3565b90508181036020830152613e4b8184613dc3565b90509392505050565b5f5f60408385031215613e6a57613e696138cd565b5b5f613e7785828601613c8e565b9250506020613e88858286016138f7565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f63ffffffff82169050919050565b613ed381613ebb565b82525050565b606082015f820151613eed5f850182613eca565b506020820151613f006020850182613eca565b506040820151613f136040850182613d91565b50505050565b5f613f248383613ed9565b60608301905092915050565b5f602082019050919050565b5f613f4682613e92565b613f508185613e9c565b9350613f5b83613eac565b805f5b83811015613f8b578151613f728882613f19565b9750613f7d83613f30565b925050600181019050613f5e565b5085935050505092915050565b5f6020820190508181035f830152613fb08184613f3c565b905092915050565b5f613fc282613a18565b9050919050565b613fd281613fb8565b82525050565b5f602082019050613feb5f830184613fc9565b92915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61403782613ff1565b810181811067ffffffffffffffff8211171561405657614055614001565b5b80604052505050565b5f6140686138c4565b9050614074828261402e565b919050565b5f67ffffffffffffffff82111561409357614092614001565b5b602082029050602081019050919050565b5f6140b66140b184614079565b61405f565b905080838252602082019050602084028301858111156140d9576140d8613b30565b5b835b8181101561410257806140ee888261397d565b8452602084019350506020810190506140db565b5050509392505050565b5f82601f8301126141205761411f613b28565b5b81356141308482602086016140a4565b91505092915050565b5f5f6040838503121561414f5761414e6138cd565b5b5f61415c858286016138f7565b925050602083013567ffffffffffffffff81111561417d5761417c6138d1565b5b6141898582860161410c565b9150509250929050565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff82169050919050565b6141bf81614193565b82525050565b5f6020820190506141d85f8301846141b6565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061421c5761421b6141de565b5b50565b5f81905061422c8261420b565b919050565b5f61423b8261421f565b9050919050565b61424b81614231565b82525050565b5f6020820190506142645f830184614242565b92915050565b5f61427482613a18565b9050919050565b6142848161426a565b82525050565b5f60208201905061429d5f83018461427b565b92915050565b6142ac81613a96565b82525050565b5f6020820190506142c55f8301846142a3565b92915050565b6142d481613a49565b81146142de575f5ffd5b50565b5f813590506142ef816142cb565b92915050565b5f67ffffffffffffffff82111561430f5761430e614001565b5b602082029050602081019050919050565b5f5ffd5b5f61432e82613a96565b9050919050565b61433e81614324565b8114614348575f5ffd5b50565b5f8135905061435981614335565b92915050565b5f6040828403121561437457614373614320565b5b61437e604061405f565b90505f61438d8482850161434b565b5f8301525060206143a0848285016142e1565b60208301525092915050565b5f6143be6143b9846142f5565b61405f565b905080838252602082019050604084028301858111156143e1576143e0613b30565b5b835b8181101561440a57806143f6888261435f565b8452602084019350506040810190506143e3565b5050509392505050565b5f82601f83011261442857614427613b28565b5b81356144388482602086016143ac565b91505092915050565b5f5f5f60608486031215614458576144576138cd565b5b5f614465868287016138f7565b9350506020614476868287016142e1565b925050604084013567ffffffffffffffff811115614497576144966138d1565b5b6144a386828701614414565b9150509250925092565b6144b6816138d5565b82525050565b5f6020820190506144cf5f8301846144ad565b92915050565b6144de81613ebb565b81146144e8575f5ffd5b50565b5f813590506144f9816144d5565b92915050565b5f5f5f60408486031215614516576145156138cd565b5b5f614523868287016144eb565b935050602084013567ffffffffffffffff811115614544576145436138d1565b5b61455086828701613ca2565b92509250509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6145908383613eca565b60208301905092915050565b5f602082019050919050565b5f6145b28261455c565b6145bc8185614566565b93506145c783614576565b805f5b838110156145f75781516145de8882614585565b97506145e98361459c565b9250506001810190506145ca565b5085935050505092915050565b5f6020820190508181035f83015261461c81846145a8565b905092915050565b60028110614630575f5ffd5b50565b5f8135905061464181614624565b92915050565b5f5f6040838503121561465d5761465c6138cd565b5b5f61466a858286016138f7565b925050602061467b85828601614633565b9150509250929050565b61468e81613ebb565b82525050565b5f6020820190506146a75f830184614685565b92915050565b5f6020820190506146c05f830184613a3a565b92915050565b5f5f5f606084860312156146dd576146dc6138cd565b5b5f6146ea868287016138f7565b93505060206146fb86828701613c8e565b925050604061470c8682870161397d565b9150509250925092565b606082015f82015161472a5f850182613eca565b50602082015161473d6020850182613eca565b5060408201516147506040850182613d91565b50505050565b5f6060820190506147695f830184614716565b92915050565b61477881613a29565b82525050565b604082015f8201516147925f85018261476f565b5060208201516147a56020850182613d91565b50505050565b5f6040820190506147be5f83018461477e565b92915050565b5f5f604083850312156147da576147d96138cd565b5b5f6147e7858286016138f7565b92505060206147f8858286016142e1565b9150509250929050565b5f5f5f60408486031215614819576148186138cd565b5b5f61482686828701613c8e565b935050602084013567ffffffffffffffff811115614847576148466138d1565b5b61485386828701613ca2565b92509250509250925092565b5f5f60408385031215614875576148746138cd565b5b5f614882858286016138f7565b925050602083013567ffffffffffffffff8111156148a3576148a26138d1565b5b6148af85828601614414565b9150509250929050565b5f5f5f606084860312156148d0576148cf6138cd565b5b5f6148dd868287016138f7565b93505060206148ee868287016144eb565b92505060406148ff8682870161397d565b9150509250925092565b5f5f5f5f60808587031215614921576149206138cd565b5b5f61492e878288016138f7565b945050602061493f878288016142e1565b9350506040614950878288016144eb565b925050606085013567ffffffffffffffff811115614971576149706138d1565b5b61497d87828801614414565b91505092959194509250565b5f5f5f606084860312156149a05761499f6138cd565b5b5f6149ad86828701613c8e565b93505060206149be868287016138f7565b92505060406149cf868287016144eb565b9150509250925092565b5f6149e382613a18565b9050919050565b6149f3816149d9565b82525050565b5f602082019050614a0c5f8301846149ea565b92915050565b5f5f60408385031215614a2857614a276138cd565b5b5f614a35858286016138f7565b9250506020614a46858286016144eb565b9150509250929050565b5f5f5f5f60808587031215614a6857614a676138cd565b5b5f614a75878288016138f7565b9450506020614a86878288016144eb565b9350506040614a9787828801613c8e565b9250506060614aa88782880161397d565b91505092959194509250565b5f82825260208201905092915050565b7f5374616b6552656769737472792e6d6f646966795374726174656779506172615f8201527f6d733a206e6f20737472617465677920696e64696365732070726f7669646564602082015250565b5f614b1e604083614ab4565b9150614b2982614ac4565b604082019050919050565b5f6020820190508181035f830152614b4b81614b12565b9050919050565b7f5374616b6552656769737472792e6d6f646966795374726174656779506172615f8201527f6d733a20696e707574206c656e677468206d69736d6174636800000000000000602082015250565b5f614bac603983614ab4565b9150614bb782614b52565b604082019050919050565b5f6020820190508181035f830152614bd981614ba0565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215614c2257614c216138cd565b5b5f614c2f848285016142e1565b91505092915050565b5f614c52614c4d614c4884613a49565b6139ee565b613936565b9050919050565b614c6281614c38565b82525050565b5f604082019050614c7b5f830185613a3a565b614c886020830184614c59565b9392505050565b7f5374616b6552656769737472792e72656769737465724f70657261746f723a205f8201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360208201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000604082015250565b5f614d0f605b83614ab4565b9150614d1a82614c8f565b606082019050919050565b5f6020820190508181035f830152614d3c81614d03565b9050919050565b7f5374616b6552656769737472792e72656d6f7665537472617465676965733a205f8201527f6e6f20696e646963657320746f2072656d6f76652070726f7669646564000000602082015250565b5f614d9d603d83614ab4565b9150614da882614d43565b604082019050919050565b5f6020820190508181035f830152614dca81614d91565b9050919050565b5f819050919050565b5f614df4614def614dea84614dd1565b6139ee565b613936565b9050919050565b614e0481614dda565b82525050565b5f604082019050614e1d5f830185613a3a565b614e2a6020830184614dfb565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f614e6882613936565b9150614e7383613936565b9250828203905081811115614e8b57614e8a614e31565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b7f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a205f8201527f71756f72756d20616c7265616479206578697374730000000000000000000000602082015250565b5f614f18603583614ab4565b9150614f2382614ebe565b604082019050919050565b5f6020820190508181035f830152614f4581614f0c565b9050919050565b7f5374616b6552656769737472792e676574546f74616c5374616b65496e6469635f8201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360208201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000604082015250565b5f614fcc605b83614ab4565b9150614fd782614f4c565b606082019050919050565b5f6020820190508181035f830152614ff981614fc0565b9050919050565b7f5374616b6552656769737472792e71756f72756d4578697374733a2071756f725f8201527f756d20646f6573206e6f74206578697374000000000000000000000000000000602082015250565b5f61505a603183614ab4565b915061506582615000565b604082019050919050565b5f6020820190508181035f8301526150878161504e565b9050919050565b5f61509882613936565b91506150a383613936565b92508282026150b181613936565b915082820484148315176150c8576150c7614e31565b5b5092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61510682613936565b915061511183613936565b925082615121576151206150cf565b5b828204905092915050565b5f61513682613a49565b915061514183613a49565b925082820190506bffffffffffffffffffffffff81111561516557615164614e31565b5b92915050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f6151a2838361476f565b60208301905092915050565b5f815f1c9050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6151ea6151e5836151ae565b6151b9565b9050919050565b5f6151fc82546151d8565b9050919050565b5f600182019050919050565b5f6152198261516b565b6152238185615175565b935061522e83615185565b805f5b8381101561526557615242826151f1565b61524c8882615197565b975061525783615203565b925050600181019050615231565b5085935050505092915050565b5f6040820190506152855f8301856142a3565b8181036020830152615297818461520f565b90509392505050565b5f815190506152ae81613967565b92915050565b5f6152c66152c184614079565b61405f565b905080838252602082019050602084028301858111156152e9576152e8613b30565b5b835b8181101561531257806152fe88826152a0565b8452602084019350506020810190506152eb565b5050509392505050565b5f82601f8301126153305761532f613b28565b5b81516153408482602086016152b4565b91505092915050565b5f6020828403121561535e5761535d6138cd565b5b5f82015167ffffffffffffffff81111561537b5761537a6138d1565b5b6153878482850161531c565b91505092915050565b5f8151905061539e81613aa7565b92915050565b5f602082840312156153b9576153b86138cd565b5b5f6153c684828501615390565b91505092915050565b7f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e5f8201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f66207460208201527f6865207265676973747279436f6f7264696e61746f7200000000000000000000604082015250565b5f61544f605683614ab4565b915061545a826153cf565b606082019050919050565b5f6020820190508181035f83015261547c81615443565b9050919050565b7f5374616b6552656769737472792e6f6e6c795265676973747279436f6f7264695f8201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260208201527f79436f6f7264696e61746f720000000000000000000000000000000000000000604082015250565b5f615503604c83614ab4565b915061550e82615483565b606082019050919050565b5f6020820190508181035f830152615530816154f7565b9050919050565b5f60408201905061554a5f8301856144ad565b6155576020830184613a60565b9392505050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a206e6f20737472617465676965732070726f76696465640000000000000000602082015250565b5f6155b8603883614ab4565b91506155c38261555e565b604082019050919050565b5f6020820190508181035f8301526155e5816155ac565b9050919050565b5f6155f682613936565b915061560183613936565b925082820190508082111561561957615618614e31565b5b92915050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60208201527f454e475448000000000000000000000000000000000000000000000000000000604082015250565b5f61569f604583614ab4565b91506156aa8261561f565b606082019050919050565b5f6020820190508181035f8301526156cc81615693565b9050919050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a2063616e6e6f74206164642073616d65207374726174656779203278000000602082015250565b5f61572d603d83614ab4565b9150615738826156d3565b604082019050919050565b5f6020820190508181035f83015261575a81615721565b9050919050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f2060208201527f7765696768740000000000000000000000000000000000000000000000000000604082015250565b5f6157e1604683614ab4565b91506157ec82615761565b606082019050919050565b5f6020820190508181035f83015261580e816157d5565b9050919050565b7f5374616b6552656769737472792e5f76616c69646174655374616b65557064615f8201527f74654174426c6f636b4e756d6265723a207374616b655570646174652069732060208201527f66726f6d20616674657220626c6f636b4e756d62657200000000000000000000604082015250565b5f615895605683614ab4565b91506158a082615815565b606082019050919050565b5f6020820190508181035f8301526158c281615889565b9050919050565b7f5374616b6552656769737472792e5f76616c69646174655374616b65557064615f8201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560208201527f72207374616b6555706461746520617661696c61626c65206265666f7265206260408201527f6c6f636b4e756d62657200000000000000000000000000000000000000000000606082015250565b5f61596f606a83614ab4565b915061597a826158c9565b608082019050919050565b5f6020820190508181035f83015261599c81615963565b9050919050565b5f6040820190506159b65f830185614685565b6159c36020830184614685565b9392505050565b5f6159d482613936565b91505f82036159e6576159e5614e31565b5b600182039050919050565b7f5374616b6552656769737472792e5f6765745374616b65557064617465496e645f8201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360208201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460408201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560608201527f7200000000000000000000000000000000000000000000000000000000000000608082015250565b5f615abd608183614ab4565b9150615ac8826159f1565b60a082019050919050565b5f6020820190508181035f830152615aea81615ab1565b9050919050565b615afa81613a96565b82525050565b604082015f820151615b145f850182615af1565b506020820151615b276020850182613eca565b50505050565b5f81519050919050565b5f819050602082019050919050565b5f615b518383615af1565b60208301905092915050565b5f602082019050919050565b5f615b7382615b2d565b615b7d8185615175565b9350615b8883615b37565b805f5b83811015615bb8578151615b9f8882615b46565b9750615baa83615b5d565b925050600181019050615b8b565b5085935050505092915050565b5f60a082019050615bd85f830187615b00565b8181036040830152615bea8186615b69565b90508181036060830152615bfe818561520f565b9050615c0d6080830184614685565b95945050505050565b5f67ffffffffffffffff821115615c3057615c2f614001565b5b602082029050602081019050919050565b5f615c53615c4e84615c16565b61405f565b90508083825260208201905060208402830185811115615c7657615c75613b30565b5b835b81811015615cbd57805167ffffffffffffffff811115615c9b57615c9a613b28565b5b808601615ca8898261531c565b85526020850194505050602081019050615c78565b5050509392505050565b5f82601f830112615cdb57615cda613b28565b5b8151615ceb848260208601615c41565b91505092915050565b5f60208284031215615d0957615d086138cd565b5b5f82015167ffffffffffffffff811115615d2657615d256138d1565b5b615d3284828501615cc7565b91505092915050565b5f819050919050565b5f615d4e82615d3b565b9150615d5983615d3b565b925082820390508181125f8412168282135f851215161715615d7e57615d7d614e31565b5b92915050565b5f615d8e82615d3b565b91507f80000000000000000000000000000000000000000000000000000000000000008203615dc057615dbf614e31565b5b815f039050919050565b5f615dd482613a49565b9150615ddf83613a49565b925082820390506bffffffffffffffffffffffff811115615e0357615e02614e31565b5b9291505056fea2646970667358221220966ff81bfd20dfab0b22dbd7cff1df7a118e413703bffaafb68611fc9634933a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qaa,8\x03\x80aa,\x839\x81\x81\x01`@R\x81\x01\x90a\x002\x91\x90a\x023V[\x83\x83\x83\x83\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xE0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xA0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\xC0\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPPPPPPa\x02\x97V[__\xFD[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_a\x01@\x82a\x01\x17V[\x90P\x91\x90PV[_a\x01Q\x82a\x016V[\x90P\x91\x90PV[a\x01a\x81a\x01GV[\x81\x14a\x01kW__\xFD[PV[_\x81Q\x90Pa\x01|\x81a\x01XV[\x92\x91PPV[_a\x01\x8C\x82a\x016V[\x90P\x91\x90PV[a\x01\x9C\x81a\x01\x82V[\x81\x14a\x01\xA6W__\xFD[PV[_\x81Q\x90Pa\x01\xB7\x81a\x01\x93V[\x92\x91PPV[_a\x01\xC7\x82a\x016V[\x90P\x91\x90PV[a\x01\xD7\x81a\x01\xBDV[\x81\x14a\x01\xE1W__\xFD[PV[_\x81Q\x90Pa\x01\xF2\x81a\x01\xCEV[\x92\x91PPV[_a\x02\x02\x82a\x016V[\x90P\x91\x90PV[a\x02\x12\x81a\x01\xF8V[\x81\x14a\x02\x1CW__\xFD[PV[_\x81Q\x90Pa\x02-\x81a\x02\tV[\x92\x91PPV[____`\x80\x85\x87\x03\x12\x15a\x02KWa\x02Ja\x01\x13V[[_a\x02X\x87\x82\x88\x01a\x01nV[\x94PP` a\x02i\x87\x82\x88\x01a\x01\xA9V[\x93PP`@a\x02z\x87\x82\x88\x01a\x01\xE4V[\x92PP``a\x02\x8B\x87\x82\x88\x01a\x02\x1FV[\x91PP\x92\x95\x91\x94P\x92PV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa^?a\x02\xED_9_\x81\x81a\x13\xF7\x01R\x81\x81a%\xEF\x01Ra&\xEB\x01R_\x81\x81a\r\xFF\x01R\x81\x81a6I\x01Ra6\xFC\x01R_a\x13\xD3\x01R_\x81\x81a\x1Es\x01Ra#r\x01Ra^?_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x020W_5`\xE0\x1C\x80c\x86\xC0hV\x11a\x01.W\x80c\xC6\x01R}\x11a\0\xB6W\x80c\xDF\\\xF7#\x11a\0zW\x80c\xDF\\\xF7#\x14a\x07lW\x80c\xE0\x86\xAD\xB3\x14a\x07\x8AW\x80c\xF2\xBE\x94\xAE\x14a\x07\xA6W\x80c\xF8Q\xE1\x98\x14a\x07\xD6W\x80c\xFA(\xC6'\x14a\x08\x06Wa\x020V[\x80c\xC6\x01R}\x14a\x06\xA4W\x80c\xC8)LV\x14a\x06\xC0W\x80c\xCCZ| \x14a\x06\xF0W\x80c\xD5\xEC\xCC\x05\x14a\x07\x0CW\x80c\xDD\x98F\xB9\x14a\x07<Wa\x020V[\x80c\xAD\xC8\x04\xDA\x11a\0\xFDW\x80c\xAD\xC8\x04\xDA\x14a\x05\xDCW\x80c\xB6\x90Kx\x14a\x06\x0CW\x80c\xBC\x9A@\xC3\x14a\x06<W\x80c\xBD)\xB8\xCD\x14a\x06XW\x80c\xC4gx\xA5\x14a\x06tWa\x020V[\x80c\x86\xC0hV\x14a\x050W\x80c\x9A\xB4\xD6\xFF\x14a\x05LW\x80c\x9F<\xCFe\x14a\x05|W\x80c\xACk\xFB\x03\x14a\x05\xACWa\x020V[\x80cT\x01\xED'\x11a\x01\xBCW\x80ck:\xA7.\x11a\x01\x80W\x80ck:\xA7.\x14a\x04\x8AW\x80cm\x14\xA9\x87\x14a\x04\xA8W\x80cu\xD4\x17:\x14a\x04\xC6W\x80c|\x17#G\x14a\x04\xE2W\x80c\x81\xC0u\x02\x14a\x05\0Wa\x020V[\x80cT\x01\xED'\x14a\x03\xC0W\x80c^Zgu\x14a\x03\xF0W\x80c_\x1F-w\x14a\x04\x0EW\x80cf\xAC\xFE\xFE\x14a\x04*W\x80ci\x7F\xBD\x93\x14a\x04ZWa\x020V[\x80c%PGw\x11a\x02\x03W\x80c%PGw\x14a\x02\xE1W\x80c,\xD9Y@\x14a\x03\x12W\x80c9\x98\xFD\xD3\x14a\x03BW\x80c<\xA5\xA5\xF5\x14a\x03`W\x80cK\xD2n\t\x14a\x03\x90Wa\x020V[\x80c\x04\x91\xB4\x1C\x14a\x024W\x80c\x08s$a\x14a\x02dW\x80c\x1F\x9Bt\xE0\x14a\x02\x95W\x80c \xB6b\x98\x14a\x02\xC5W[__\xFD[a\x02N`\x04\x806\x03\x81\x01\x90a\x02I\x91\x90a9\x0BV[a\x086V[`@Qa\x02[\x91\x90a9NV[`@Q\x80\x91\x03\x90\xF3[a\x02~`\x04\x806\x03\x81\x01\x90a\x02y\x91\x90a9\x91V[a\x08YV[`@Qa\x02\x8C\x92\x91\x90a:oV[`@Q\x80\x91\x03\x90\xF3[a\x02\xAF`\x04\x806\x03\x81\x01\x90a\x02\xAA\x91\x90a:\xD1V[a\x08\xC5V[`@Qa\x02\xBC\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x02\xDF`\x04\x806\x03\x81\x01\x90a\x02\xDA\x91\x90a;\xDEV[a\x08\xE9V[\0[a\x02\xFB`\x04\x806\x03\x81\x01\x90a\x02\xF6\x91\x90a<\xF7V[a\x0B\rV[`@Qa\x03\t\x92\x91\x90a>\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x03,`\x04\x806\x03\x81\x01\x90a\x03'\x91\x90a>TV[a\x0C\xF8V[`@Qa\x039\x91\x90a?\x98V[`@Q\x80\x91\x03\x90\xF3[a\x03Ja\r\xFDV[`@Qa\x03W\x91\x90a?\xD8V[`@Q\x80\x91\x03\x90\xF3[a\x03z`\x04\x806\x03\x81\x01\x90a\x03u\x91\x90a9\x0BV[a\x0E!V[`@Qa\x03\x87\x91\x90a9NV[`@Q\x80\x91\x03\x90\xF3[a\x03\xAA`\x04\x806\x03\x81\x01\x90a\x03\xA5\x91\x90a>TV[a\x0EDV[`@Qa\x03\xB7\x91\x90a9NV[`@Q\x80\x91\x03\x90\xF3[a\x03\xDA`\x04\x806\x03\x81\x01\x90a\x03\xD5\x91\x90a>TV[a\x0EwV[`@Qa\x03\xE7\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x03\xF8a\x0E\x93V[`@Qa\x04\x05\x91\x90a9NV[`@Q\x80\x91\x03\x90\xF3[a\x04(`\x04\x806\x03\x81\x01\x90a\x04#\x91\x90aA9V[a\x0E\x9FV[\0[a\x04D`\x04\x806\x03\x81\x01\x90a\x04?\x91\x90a<\xF7V[a\x12\xEDV[`@Qa\x04Q\x91\x90aA\xC5V[`@Q\x80\x91\x03\x90\xF3[a\x04t`\x04\x806\x03\x81\x01\x90a\x04o\x91\x90a9\x0BV[a\x13\xB4V[`@Qa\x04\x81\x91\x90aBQV[`@Q\x80\x91\x03\x90\xF3[a\x04\x92a\x13\xD1V[`@Qa\x04\x9F\x91\x90aB\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x04\xB0a\x13\xF5V[`@Qa\x04\xBD\x91\x90aB\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x04\xE0`\x04\x806\x03\x81\x01\x90a\x04\xDB\x91\x90aDAV[a\x14\x19V[\0[a\x04\xEAa\x15}V[`@Qa\x04\xF7\x91\x90aD\xBCV[`@Q\x80\x91\x03\x90\xF3[a\x05\x1A`\x04\x806\x03\x81\x01\x90a\x05\x15\x91\x90aD\xFFV[a\x15\x82V[`@Qa\x05'\x91\x90aF\x04V[`@Q\x80\x91\x03\x90\xF3[a\x05J`\x04\x806\x03\x81\x01\x90a\x05E\x91\x90aFGV[a\x17\xAAV[\0[a\x05f`\x04\x806\x03\x81\x01\x90a\x05a\x91\x90a9\x0BV[a\x17\xC0V[`@Qa\x05s\x91\x90aF\x94V[`@Q\x80\x91\x03\x90\xF3[a\x05\x96`\x04\x806\x03\x81\x01\x90a\x05\x91\x91\x90a9\x91V[a\x17\xE0V[`@Qa\x05\xA3\x91\x90aF\xADV[`@Q\x80\x91\x03\x90\xF3[a\x05\xC6`\x04\x806\x03\x81\x01\x90a\x05\xC1\x91\x90aF\xC6V[a\x18(V[`@Qa\x05\xD3\x91\x90aGVV[`@Q\x80\x91\x03\x90\xF3[a\x05\xF6`\x04\x806\x03\x81\x01\x90a\x05\xF1\x91\x90a9\x91V[a\x19\nV[`@Qa\x06\x03\x91\x90aG\xABV[`@Q\x80\x91\x03\x90\xF3[a\x06&`\x04\x806\x03\x81\x01\x90a\x06!\x91\x90a9\x91V[a\x19\xE7V[`@Qa\x063\x91\x90aGVV[`@Q\x80\x91\x03\x90\xF3[a\x06V`\x04\x806\x03\x81\x01\x90a\x06Q\x91\x90aG\xC4V[a\x1A\xB9V[\0[a\x06r`\x04\x806\x03\x81\x01\x90a\x06m\x91\x90aH\x02V[a\x1A\xDAV[\0[a\x06\x8E`\x04\x806\x03\x81\x01\x90a\x06\x89\x91\x90a9\x0BV[a\x1BLV[`@Qa\x06\x9B\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x06\xBE`\x04\x806\x03\x81\x01\x90a\x06\xB9\x91\x90aH_V[a\x1BsV[\0[a\x06\xDA`\x04\x806\x03\x81\x01\x90a\x06\xD5\x91\x90aH\xB9V[a\x1B\x94V[`@Qa\x06\xE7\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x07\n`\x04\x806\x03\x81\x01\x90a\x07\x05\x91\x90aI\tV[a\x1CsV[\0[a\x07&`\x04\x806\x03\x81\x01\x90a\x07!\x91\x90a9\x0BV[a\x1D\xE3V[`@Qa\x073\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x07V`\x04\x806\x03\x81\x01\x90a\x07Q\x91\x90aI\x89V[a\x1E\\V[`@Qa\x07c\x91\x90aF\x94V[`@Q\x80\x91\x03\x90\xF3[a\x07ta\x1EqV[`@Qa\x07\x81\x91\x90aI\xF9V[`@Q\x80\x91\x03\x90\xF3[a\x07\xA4`\x04\x806\x03\x81\x01\x90a\x07\x9F\x91\x90aJ\x12V[a\x1E\x95V[\0[a\x07\xC0`\x04\x806\x03\x81\x01\x90a\x07\xBB\x91\x90aJPV[a\x1E\xABV[`@Qa\x07\xCD\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x07\xF0`\x04\x806\x03\x81\x01\x90a\x07\xEB\x91\x90a>TV[a\x1F\x9AV[`@Qa\x07\xFD\x91\x90aGVV[`@Q\x80\x91\x03\x90\xF3[a\x08 `\x04\x806\x03\x81\x01\x90a\x08\x1B\x91\x90aI\x89V[a \xD3V[`@Qa\x08-\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\x08rW_\x80\xFD[\x90_R` _ \x01_\x91P\x91PP\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82V[_\x82a\x08\xD0\x81a!HV[_a\x08\xDB\x85\x85a!\x93V[P\x90P\x80\x92PPP\x92\x91PPV[a\x08\xF1a%\xEDV[\x84a\x08\xFB\x81a!HV[_\x85\x85\x90P\x90P_\x81\x11a\tDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t;\x90aK4V[`@Q\x80\x91\x03\x90\xFD[\x80\x84\x84\x90P\x14a\t\x89W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x80\x90aK\xC2V[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x89`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P__\x90P[\x82\x81\x10\x15a\x0B\x02W\x85\x85\x82\x81\x81\x10a\t\xC3Wa\t\xC2aK\xE0V[[\x90P` \x02\x01` \x81\x01\x90a\t\xD8\x91\x90aL\rV[\x82\x89\x89\x84\x81\x81\x10a\t\xECWa\t\xEBaK\xE0V[[\x90P` \x02\x015\x81T\x81\x10a\n\x04Wa\n\x03aK\xE0V[[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\nuWa\ntaK\xE0V[[\x90P` \x02\x015\x81T\x81\x10a\n\x8DWa\n\x8CaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x88\x85\x81\x81\x10a\n\xCAWa\n\xC9aK\xE0V[[\x90P` \x02\x01` \x81\x01\x90a\n\xDF\x91\x90aL\rV[`@Qa\n\xED\x92\x91\x90aLhV[`@Q\x80\x91\x03\x90\xA2\x80\x80`\x01\x01\x91PPa\t\xA8V[PPPPPPPPPV[``\x80a\x0B\x18a&\xE9V[_\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B6Wa\x0B5a@\x01V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BdW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x85\x85\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x85Wa\x0B\x84a@\x01V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xB3W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x86\x86\x90P\x81\x10\x15a\x0C\xE6W_\x87\x87\x83\x81\x81\x10a\x0B\xDAWa\x0B\xD9aK\xE0V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x0B\xF2\x81a!HV[__a\x0B\xFE\x83\x8Da!\x93V[\x91P\x91P\x80a\x0CBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C9\x90aM%V[`@Q\x80\x91\x03\x90\xFD[_a\x0CN\x8C\x85\x85a'yV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0CdWa\x0CcaK\xE0V[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x0C\x98\x84\x82a+\x1EV[\x86\x86\x81Q\x81\x10a\x0C\xABWa\x0C\xAAaK\xE0V[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPP\x80\x80`\x01\x01\x91PPa\x0B\xBBV[P\x81\x81\x93P\x93PPP\x94P\x94\x92PPPV[```\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\r\xF1W\x83\x82\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81R` \x01\x90`\x01\x01\x90a\r?V[PPPP\x90P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x92\x91PPV[__a\x0E\x83\x84\x84a\x1F\x9AV[\x90P\x80`@\x01Q\x91PP\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x0E\xA7a%\xEDV[\x81a\x0E\xB1\x81a!HV[_\x82Q\x90P_\x81\x11a\x0E\xF8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xEF\x90aM\xB3V[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P_`\x04_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P__\x90P[\x83\x81\x10\x15a\x12\xE4W\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x0FrWa\x0FqaK\xE0V[[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\x8BWa\x0F\x8AaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa\x0F\xC2\x91\x90aF\xADV[`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x10\x03Wa\x10\x02aK\xE0V[[` \x02` \x01\x01Q\x81T\x81\x10a\x10\x1CWa\x10\x1BaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_`@Qa\x10U\x92\x91\x90aN\nV[`@Q\x80\x91\x03\x90\xA2\x82`\x01\x84\x80T\x90Pa\x10o\x91\x90aN^V[\x81T\x81\x10a\x10\x80Wa\x10\x7FaK\xE0V[[\x90_R` _ \x01\x83\x87\x83\x81Q\x81\x10a\x10\x9CWa\x10\x9BaK\xE0V[[` \x02` \x01\x01Q\x81T\x81\x10a\x10\xB5Wa\x10\xB4aK\xE0V[[\x90_R` _ \x01_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x90PP\x82\x80T\x80a\x11\x81Wa\x11\x80aN\x91V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01__\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U_\x82\x01`\x14a\x01\0\n\x81T\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UPP\x90U\x81`\x01\x83\x80T\x90Pa\x11\xE9\x91\x90aN^V[\x81T\x81\x10a\x11\xFAWa\x11\xF9aK\xE0V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x87\x83\x81Q\x81\x10a\x126Wa\x125aK\xE0V[[` \x02` \x01\x01Q\x81T\x81\x10a\x12OWa\x12NaK\xE0V[[\x90_R` _ \x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x80T\x80a\x12\xA5Wa\x12\xA4aN\x91V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x90U\x80\x80`\x01\x01\x91PPa\x0F1V[PPPPPPPV[_a\x12\xF6a&\xE9V[____\x90P[\x85\x85\x90P\x81\x10\x15a\x13\xA6W_\x86\x86\x83\x81\x81\x10a\x13\x1CWa\x13\x1BaK\xE0V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x134\x81a!HV[__a\x13@\x83\x8Ca!\x93V[\x91P\x91P\x80a\x13|W_\x91Pa\x13y\x83\x87w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a-M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P[_a\x13\x88\x8B\x85\x85a'yV[\x90Pa\x13\x94\x84\x82a+\x1EV[PPPPP\x80\x80`\x01\x01\x91PPa\x12\xFDV[P\x81\x92PPP\x94\x93PPPPV[`\x05` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x14!a&\xE9V[a\x14*\x83a-`V[\x15a\x14jW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14a\x90aO.V[`@Q\x80\x91\x03\x90\xFD[a\x14t\x83\x82a-\x86V[a\x14~\x83\x83a2\x02V[a\x14\x88\x83_a2\x85V[`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[` \x81V[``_\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xA2Wa\x15\xA1a@\x01V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xD0W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x84\x84\x90P\x81\x10\x15a\x17\x9EW_\x85\x85\x83\x81\x81\x10a\x15\xF7Wa\x15\xF6aK\xE0V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x16\x0F\x81a!HV[\x86c\xFF\xFF\xFF\xFF\x16`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x81\x10a\x16?Wa\x16>aK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\xA0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\x97\x90aO\xE2V[`@Q\x80\x91\x03\x90\xFD[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P__\x90P[\x81\x81\x10\x15a\x17\x8EW\x88c\xFF\xFF\xFF\xFF\x16`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83\x85a\x16\xF7\x91\x90aN^V[a\x17\x01\x91\x90aN^V[\x81T\x81\x10a\x17\x12Wa\x17\x11aK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a\x17\x81W`\x01\x81\x83a\x17E\x91\x90aN^V[a\x17O\x91\x90aN^V[\x85\x85\x81Q\x81\x10a\x17bWa\x17aaK\xE0V[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x17\x8EV[\x80\x80`\x01\x01\x91PPa\x16\xC3V[PPP\x80\x80`\x01\x01\x91PPa\x15\xD8V[P\x80\x91PP\x93\x92PPPV[a\x17\xB2a%\xEDV[a\x17\xBC\x82\x82a2\x85V[PPV[`\x06` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x17\xF9W_\x80\xFD[\x90_R` _ \x01_\x91P\x91P\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x180a8PV[`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x18hWa\x18gaK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x93\x92PPPV[a\x19\x12a8\x88V[`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x19;Wa\x19:aK\xE0V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x19\xEFa8PV[`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x1A\x18Wa\x1A\x17aK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x1A\xC1a%\xEDV[\x81a\x1A\xCB\x81a!HV[a\x1A\xD5\x83\x83a2\x02V[PPPV[a\x1A\xE2a&\xE9V[__\x90P[\x82\x82\x90P\x81\x10\x15a\x1BFW_\x83\x83\x83\x81\x81\x10a\x1B\x06Wa\x1B\x05aK\xE0V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x1B\x1E\x81a!HV[_a\x1B*\x86\x83_a'yV[\x90Pa\x1B6\x82\x82a+\x1EV[PPP\x80\x80`\x01\x01\x91PPa\x1A\xE7V[PPPPV[_` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x1B{a%\xEDV[\x81a\x1B\x85\x81a!HV[a\x1B\x8F\x83\x83a-\x86V[PPPV[__`\x01_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x1B\xBFWa\x1B\xBEaK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90Pa\x1Cd\x81\x85a2\xFEV[\x80`@\x01Q\x91PP\x93\x92PPPV[a\x1C{a&\xE9V[a\x1C\x84\x84a-`V[\x15a\x1C\xC4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xBB\x90aO.V[`@Q\x80\x91\x03\x90\xFD[a\x1C\xCE\x84\x82a-\x86V[a\x1C\xD8\x84\x84a2\x02V[a\x1C\xE3\x84`\x01a2\x85V[a\x1C\xED\x84\x83a3\xBAV[`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPV[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x80_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa\x1E!\x91\x90aN^V[\x81T\x81\x10a\x1E2Wa\x1E1aK\xE0V[[\x90_R` _ \x01_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[_a\x1Eh\x84\x84\x84a4WV[\x90P\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x1E\x9Da%\xEDV[a\x1E\xA7\x82\x82a3\xBAV[PPV[__`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x1E\xE5Wa\x1E\xE4aK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90Pa\x1F\x8A\x81\x86a2\xFEV[\x80`@\x01Q\x91PP\x94\x93PPPPV[a\x1F\xA2a8PV[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90Pa\x1F\xD7a8PV[_\x82\x03a\x1F\xE8W\x80\x92PPPa \xCDV[`\x02_\x86\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a \x1B\x91\x90aN^V[\x81T\x81\x10a ,Wa +aK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x80\x92PPP[\x92\x91PPV[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ a!\x05\x85\x85\x85a4WV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a!\x1CWa!\x1BaK\xE0V[[\x90_R` _ \x01_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x93\x92PPPV[a!Q\x81a-`V[a!\x90W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\x87\x90aPpV[`@Q\x80\x91\x03\x90\xFD[PV[____a!\xA0\x86a\x0E!V[\x90Pa!\xAAa8\x88V[```\x01\x80\x81\x11\x15a!\xBFWa!\xBEaA\xDEV[[`\x05_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a!\xF5Wa!\xF4aA\xDEV[[\x03a#pWa\"\x04\x88\x88a5nV[\x90P__\x90P[\x83\x81\x10\x15a#jW`\x03_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a\"<Wa\";aK\xE0V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x92P_\x82\x82\x81Q\x81\x10a\"\xF6Wa\"\xF5aK\xE0V[[` \x02` \x01\x01Q\x11\x15a#]Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10a#3Wa#2aK\xE0V[[` \x02` \x01\x01Qa#E\x91\x90aP\x8EV[a#O\x91\x90aP\xFCV[\x85a#Z\x91\x90aQ,V[\x94P[\x80\x80`\x01\x01\x91PPa\"\x0BV[Pa%\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\x04\x13G\x88`\x04_\x8C`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\xE1\x92\x91\x90aRrV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xFBW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$#\x91\x90aSIV[\x90P__\x90P[\x83\x81\x10\x15a%\x89W`\x03_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a$[Wa$ZaK\xE0V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x92P_\x82\x82\x81Q\x81\x10a%\x15Wa%\x14aK\xE0V[[` \x02` \x01\x01Q\x11\x15a%|Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10a%RWa%QaK\xE0V[[` \x02` \x01\x01Qa%d\x91\x90aP\x8EV[a%n\x91\x90aP\xFCV[\x85a%y\x91\x90aQ,V[\x94P[\x80\x80`\x01\x01\x91PPa$*V[P[___\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15\x90P\x84\x81\x96P\x96PPPPPP\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&z\x91\x90aS\xA4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\xE7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xDE\x90aTeV[`@Q\x80\x91\x03\x90\xFD[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a'wW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'n\x90aU\x19V[`@Q\x80\x91\x03\x90\xFD[V[___`\x02_\x87\x81R` \x01\x90\x81R` \x01_ _\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a(\xB3W`\x02_\x87\x81R` \x01\x90\x81R` \x01_ _\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPa*\xCEV[_`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a(\xE7\x91\x90aN^V[\x81T\x81\x10a(\xF8Wa(\xF7aK\xE0V[[\x90_R` _ \x01\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92P\x84k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a)NW_\x93PPPPa+\x17V[Cc\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a)\xAAW\x84\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa*\xCCV[C\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[P[\x85\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x86\x86`@Qa+\0\x92\x91\x90aU7V[`@Q\x80\x91\x03\x90\xA2a+\x12\x82\x85a7\xE5V[\x92PPP[\x93\x92PPPV[__`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_`\x01_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a+b\x91\x90aN^V[\x81T\x81\x10a+sWa+raK\xE0V[[\x90_R` _ \x01\x90P_\x84\x03a+\xA9W\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92PPPa-GV[_a+\xCF\x82_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86a8\x16V[\x90PCc\xFF\xFF\xFF\xFF\x16\x82_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a,-W\x80\x82_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa-@V[C\x82_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[\x80\x93PPPP[\x92\x91PPV[_\x81`\xFF\x16`\x01\x90\x1B\x83\x17\x90P\x92\x91PPV[__`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15\x90P\x91\x90PV[_\x81Q\x11a-\xC9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xC0\x90aU\xCEV[`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P_`\x03_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P` `\xFF\x16\x82\x82a-\xFD\x91\x90aU\xECV[\x11\x15a.>W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.5\x90aV\xB5V[`@Q\x80\x91\x03\x90\xFD[__\x90P[\x82\x81\x10\x15a1\xFBW__\x90P[\x81\x83a.\\\x91\x90aU\xECV[\x81\x10\x15a/MW\x84\x82\x81Q\x81\x10a.vWa.uaK\xE0V[[` \x02` \x01\x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a.\xC0Wa.\xBFaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a/@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/7\x90aWCV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa.PV[P_\x84\x82\x81Q\x81\x10a/bWa/aaK\xE0V[[` \x02` \x01\x01Q` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a/\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\xB3\x90aW\xF7V[`@Q\x80\x91\x03\x90\xFD[`\x03_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x84\x82\x81Q\x81\x10a/\xE6Wa/\xE5aK\xE0V[[` \x02` \x01\x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP`\x04_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x84\x82\x81Q\x81\x10a0\xB6Wa0\xB5aK\xE0V[[` \x02` \x01\x01Q_\x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\xFF\x16\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x85\x83\x81Q\x81\x10a1VWa1UaK\xE0V[[` \x02` \x01\x01Q_\x01Q`@Qa1n\x91\x90aF\xADV[`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a1\xAEWa1\xADaK\xE0V[[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a1\xCCWa1\xCBaK\xE0V[[` \x02` \x01\x01Q` \x01Q`@Qa1\xE6\x92\x91\x90aLhV[`@Q\x80\x91\x03\x90\xA2\x80\x80`\x01\x01\x91PPa.CV[PPPPPV[\x80__\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\xFF\x16\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x82`@Qa2y\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xA2PPV[\x80`\x05_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a2\xBEWa2\xBDaA\xDEV[[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa2\xF2\x91\x90aBQV[`@Q\x80\x91\x03\x90\xA1PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a3PW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3G\x90aX\xABV[`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x14\x80a3wWP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a3\xB6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\xAD\x90aY\x85V[`@Q\x80\x91\x03\x90\xFD[PPV[_`\x06_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x81`\x06_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x81\x83`@Qa4J\x92\x91\x90aY\xA3V[`@Q\x80\x91\x03\x90\xA1PPPV[__`\x02_\x86\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x90P[_\x81\x11\x15a5+W\x83c\xFF\xFF\xFF\xFF\x16`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a4\xCC\x91\x90aN^V[\x81T\x81\x10a4\xDDWa4\xDCaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a5\x18W`\x01\x81a5\x0F\x91\x90aN^V[\x92PPPa5gV[\x80\x80a5#\x90aY\xCAV[\x91PPa4\x8AV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5^\x90aZ\xD3V[`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[``_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x8CWa5\x8Ba@\x01V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\xBAW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x82\x81_\x81Q\x81\x10a5\xD1Wa5\xD0aK\xE0V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_`\x06_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16Ba6D\x91\x90aU\xECV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x8A\xA7\xC7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xB0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xD4\x91\x90aS\xA4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c+\xAB,J`@Q\x80`@\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RP\x85`\x04_\x8B`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7{\x94\x93\x92\x91\x90a[\xC5V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x95W=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xBD\x91\x90a\\\xF4V[\x90P\x80_\x81Q\x81\x10a7\xD2Wa7\xD1aK\xE0V[[` \x02` \x01\x01Q\x93PPPP\x92\x91PPV[_\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a8\x0E\x91\x90a]DV[\x90P\x92\x91PPV[__\x82\x12\x15a8;W\x81a8)\x90a]\x84V[\x83a84\x91\x90a]\xCAV[\x90Pa8JV[\x81\x83a8G\x91\x90aQ,V[\x90P[\x92\x91PPV[`@Q\x80``\x01`@R\x80_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80`@\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a8\xEA\x81a8\xD5V[\x81\x14a8\xF4W__\xFD[PV[_\x815\x90Pa9\x05\x81a8\xE1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a9 Wa9\x1Fa8\xCDV[[_a9-\x84\x82\x85\x01a8\xF7V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a9H\x81a96V[\x82RPPV[_` \x82\x01\x90Pa9a_\x83\x01\x84a9?V[\x92\x91PPV[a9p\x81a96V[\x81\x14a9zW__\xFD[PV[_\x815\x90Pa9\x8B\x81a9gV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a9\xA7Wa9\xA6a8\xCDV[[_a9\xB4\x85\x82\x86\x01a8\xF7V[\x92PP` a9\xC5\x85\x82\x86\x01a9}V[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a:\x11a:\x0Ca:\x07\x84a9\xCFV[a9\xEEV[a9\xCFV[\x90P\x91\x90PV[_a:\"\x82a9\xF7V[\x90P\x91\x90PV[_a:3\x82a:\x18V[\x90P\x91\x90PV[a:C\x81a:)V[\x82RPPV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a:i\x81a:IV[\x82RPPV[_`@\x82\x01\x90Pa:\x82_\x83\x01\x85a::V[a:\x8F` \x83\x01\x84a:`V[\x93\x92PPPV[_a:\xA0\x82a9\xCFV[\x90P\x91\x90PV[a:\xB0\x81a:\x96V[\x81\x14a:\xBAW__\xFD[PV[_\x815\x90Pa:\xCB\x81a:\xA7V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a:\xE7Wa:\xE6a8\xCDV[[_a:\xF4\x85\x82\x86\x01a8\xF7V[\x92PP` a;\x05\x85\x82\x86\x01a:\xBDV[\x91PP\x92P\x92\x90PV[_` \x82\x01\x90Pa;\"_\x83\x01\x84a:`V[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a;IWa;Ha;(V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;fWa;ea;,V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a;\x82Wa;\x81a;0V[[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a;\x9EWa;\x9Da;(V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xBBWa;\xBAa;,V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a;\xD7Wa;\xD6a;0V[[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15a;\xF7Wa;\xF6a8\xCDV[[_a<\x04\x88\x82\x89\x01a8\xF7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<%Wa<$a8\xD1V[[a<1\x88\x82\x89\x01a;4V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<TWa<Sa8\xD1V[[a<`\x88\x82\x89\x01a;\x89V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x81\x90P\x91\x90PV[a<\x81\x81a<oV[\x81\x14a<\x8BW__\xFD[PV[_\x815\x90Pa<\x9C\x81a<xV[\x92\x91PPV[__\x83`\x1F\x84\x01\x12a<\xB7Wa<\xB6a;(V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xD4Wa<\xD3a;,V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a<\xF0Wa<\xEFa;0V[[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a=\x0FWa=\x0Ea8\xCDV[[_a=\x1C\x87\x82\x88\x01a:\xBDV[\x94PP` a=-\x87\x82\x88\x01a<\x8EV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=NWa=Ma8\xD1V[[a=Z\x87\x82\x88\x01a<\xA2V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a=\x9A\x81a:IV[\x82RPPV[_a=\xAB\x83\x83a=\x91V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a=\xCD\x82a=hV[a=\xD7\x81\x85a=rV[\x93Pa=\xE2\x83a=\x82V[\x80_[\x83\x81\x10\x15a>\x12W\x81Qa=\xF9\x88\x82a=\xA0V[\x97Pa>\x04\x83a=\xB7V[\x92PP`\x01\x81\x01\x90Pa=\xE5V[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra>7\x81\x85a=\xC3V[\x90P\x81\x81\x03` \x83\x01Ra>K\x81\x84a=\xC3V[\x90P\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a>jWa>ia8\xCDV[[_a>w\x85\x82\x86\x01a<\x8EV[\x92PP` a>\x88\x85\x82\x86\x01a8\xF7V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a>\xD3\x81a>\xBBV[\x82RPPV[``\x82\x01_\x82\x01Qa>\xED_\x85\x01\x82a>\xCAV[P` \x82\x01Qa?\0` \x85\x01\x82a>\xCAV[P`@\x82\x01Qa?\x13`@\x85\x01\x82a=\x91V[PPPPV[_a?$\x83\x83a>\xD9V[``\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a?F\x82a>\x92V[a?P\x81\x85a>\x9CV[\x93Pa?[\x83a>\xACV[\x80_[\x83\x81\x10\x15a?\x8BW\x81Qa?r\x88\x82a?\x19V[\x97Pa?}\x83a?0V[\x92PP`\x01\x81\x01\x90Pa?^V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra?\xB0\x81\x84a?<V[\x90P\x92\x91PPV[_a?\xC2\x82a:\x18V[\x90P\x91\x90PV[a?\xD2\x81a?\xB8V[\x82RPPV[_` \x82\x01\x90Pa?\xEB_\x83\x01\x84a?\xC9V[\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a@7\x82a?\xF1V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a@VWa@Ua@\x01V[[\x80`@RPPPV[_a@ha8\xC4V[\x90Pa@t\x82\x82a@.V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a@\x93Wa@\x92a@\x01V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a@\xB6a@\xB1\x84a@yV[a@_V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a@\xD9Wa@\xD8a;0V[[\x83[\x81\x81\x10\x15aA\x02W\x80a@\xEE\x88\x82a9}V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa@\xDBV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aA WaA\x1Fa;(V[[\x815aA0\x84\x82` \x86\x01a@\xA4V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15aAOWaANa8\xCDV[[_aA\\\x85\x82\x86\x01a8\xF7V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA}WaA|a8\xD1V[[aA\x89\x85\x82\x86\x01aA\x0CV[\x91PP\x92P\x92\x90PV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aA\xBF\x81aA\x93V[\x82RPPV[_` \x82\x01\x90PaA\xD8_\x83\x01\x84aA\xB6V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aB\x1CWaB\x1BaA\xDEV[[PV[_\x81\x90PaB,\x82aB\x0BV[\x91\x90PV[_aB;\x82aB\x1FV[\x90P\x91\x90PV[aBK\x81aB1V[\x82RPPV[_` \x82\x01\x90PaBd_\x83\x01\x84aBBV[\x92\x91PPV[_aBt\x82a:\x18V[\x90P\x91\x90PV[aB\x84\x81aBjV[\x82RPPV[_` \x82\x01\x90PaB\x9D_\x83\x01\x84aB{V[\x92\x91PPV[aB\xAC\x81a:\x96V[\x82RPPV[_` \x82\x01\x90PaB\xC5_\x83\x01\x84aB\xA3V[\x92\x91PPV[aB\xD4\x81a:IV[\x81\x14aB\xDEW__\xFD[PV[_\x815\x90PaB\xEF\x81aB\xCBV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aC\x0FWaC\x0Ea@\x01V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_aC.\x82a:\x96V[\x90P\x91\x90PV[aC>\x81aC$V[\x81\x14aCHW__\xFD[PV[_\x815\x90PaCY\x81aC5V[\x92\x91PPV[_`@\x82\x84\x03\x12\x15aCtWaCsaC V[[aC~`@a@_V[\x90P_aC\x8D\x84\x82\x85\x01aCKV[_\x83\x01RP` aC\xA0\x84\x82\x85\x01aB\xE1V[` \x83\x01RP\x92\x91PPV[_aC\xBEaC\xB9\x84aB\xF5V[a@_V[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15aC\xE1WaC\xE0a;0V[[\x83[\x81\x81\x10\x15aD\nW\x80aC\xF6\x88\x82aC_V[\x84R` \x84\x01\x93PP`@\x81\x01\x90PaC\xE3V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aD(WaD'a;(V[[\x815aD8\x84\x82` \x86\x01aC\xACV[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15aDXWaDWa8\xCDV[[_aDe\x86\x82\x87\x01a8\xF7V[\x93PP` aDv\x86\x82\x87\x01aB\xE1V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x97WaD\x96a8\xD1V[[aD\xA3\x86\x82\x87\x01aD\x14V[\x91PP\x92P\x92P\x92V[aD\xB6\x81a8\xD5V[\x82RPPV[_` \x82\x01\x90PaD\xCF_\x83\x01\x84aD\xADV[\x92\x91PPV[aD\xDE\x81a>\xBBV[\x81\x14aD\xE8W__\xFD[PV[_\x815\x90PaD\xF9\x81aD\xD5V[\x92\x91PPV[___`@\x84\x86\x03\x12\x15aE\x16WaE\x15a8\xCDV[[_aE#\x86\x82\x87\x01aD\xEBV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aEDWaECa8\xD1V[[aEP\x86\x82\x87\x01a<\xA2V[\x92P\x92PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_aE\x90\x83\x83a>\xCAV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aE\xB2\x82aE\\V[aE\xBC\x81\x85aEfV[\x93PaE\xC7\x83aEvV[\x80_[\x83\x81\x10\x15aE\xF7W\x81QaE\xDE\x88\x82aE\x85V[\x97PaE\xE9\x83aE\x9CV[\x92PP`\x01\x81\x01\x90PaE\xCAV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaF\x1C\x81\x84aE\xA8V[\x90P\x92\x91PPV[`\x02\x81\x10aF0W__\xFD[PV[_\x815\x90PaFA\x81aF$V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aF]WaF\\a8\xCDV[[_aFj\x85\x82\x86\x01a8\xF7V[\x92PP` aF{\x85\x82\x86\x01aF3V[\x91PP\x92P\x92\x90PV[aF\x8E\x81a>\xBBV[\x82RPPV[_` \x82\x01\x90PaF\xA7_\x83\x01\x84aF\x85V[\x92\x91PPV[_` \x82\x01\x90PaF\xC0_\x83\x01\x84a::V[\x92\x91PPV[___``\x84\x86\x03\x12\x15aF\xDDWaF\xDCa8\xCDV[[_aF\xEA\x86\x82\x87\x01a8\xF7V[\x93PP` aF\xFB\x86\x82\x87\x01a<\x8EV[\x92PP`@aG\x0C\x86\x82\x87\x01a9}V[\x91PP\x92P\x92P\x92V[``\x82\x01_\x82\x01QaG*_\x85\x01\x82a>\xCAV[P` \x82\x01QaG=` \x85\x01\x82a>\xCAV[P`@\x82\x01QaGP`@\x85\x01\x82a=\x91V[PPPPV[_``\x82\x01\x90PaGi_\x83\x01\x84aG\x16V[\x92\x91PPV[aGx\x81a:)V[\x82RPPV[`@\x82\x01_\x82\x01QaG\x92_\x85\x01\x82aGoV[P` \x82\x01QaG\xA5` \x85\x01\x82a=\x91V[PPPPV[_`@\x82\x01\x90PaG\xBE_\x83\x01\x84aG~V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aG\xDAWaG\xD9a8\xCDV[[_aG\xE7\x85\x82\x86\x01a8\xF7V[\x92PP` aG\xF8\x85\x82\x86\x01aB\xE1V[\x91PP\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aH\x19WaH\x18a8\xCDV[[_aH&\x86\x82\x87\x01a<\x8EV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aHGWaHFa8\xD1V[[aHS\x86\x82\x87\x01a<\xA2V[\x92P\x92PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aHuWaHta8\xCDV[[_aH\x82\x85\x82\x86\x01a8\xF7V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xA3WaH\xA2a8\xD1V[[aH\xAF\x85\x82\x86\x01aD\x14V[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15aH\xD0WaH\xCFa8\xCDV[[_aH\xDD\x86\x82\x87\x01a8\xF7V[\x93PP` aH\xEE\x86\x82\x87\x01aD\xEBV[\x92PP`@aH\xFF\x86\x82\x87\x01a9}V[\x91PP\x92P\x92P\x92V[____`\x80\x85\x87\x03\x12\x15aI!WaI a8\xCDV[[_aI.\x87\x82\x88\x01a8\xF7V[\x94PP` aI?\x87\x82\x88\x01aB\xE1V[\x93PP`@aIP\x87\x82\x88\x01aD\xEBV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aIqWaIpa8\xD1V[[aI}\x87\x82\x88\x01aD\x14V[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15aI\xA0WaI\x9Fa8\xCDV[[_aI\xAD\x86\x82\x87\x01a<\x8EV[\x93PP` aI\xBE\x86\x82\x87\x01a8\xF7V[\x92PP`@aI\xCF\x86\x82\x87\x01aD\xEBV[\x91PP\x92P\x92P\x92V[_aI\xE3\x82a:\x18V[\x90P\x91\x90PV[aI\xF3\x81aI\xD9V[\x82RPPV[_` \x82\x01\x90PaJ\x0C_\x83\x01\x84aI\xEAV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aJ(WaJ'a8\xCDV[[_aJ5\x85\x82\x86\x01a8\xF7V[\x92PP` aJF\x85\x82\x86\x01aD\xEBV[\x91PP\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15aJhWaJga8\xCDV[[_aJu\x87\x82\x88\x01a8\xF7V[\x94PP` aJ\x86\x87\x82\x88\x01aD\xEBV[\x93PP`@aJ\x97\x87\x82\x88\x01a<\x8EV[\x92PP``aJ\xA8\x87\x82\x88\x01a9}V[\x91PP\x92\x95\x91\x94P\x92PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FStakeRegistry.modifyStrategyPara_\x82\x01R\x7Fms: no strategy indices provided` \x82\x01RPV[_aK\x1E`@\x83aJ\xB4V[\x91PaK)\x82aJ\xC4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaKK\x81aK\x12V[\x90P\x91\x90PV[\x7FStakeRegistry.modifyStrategyPara_\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0` \x82\x01RPV[_aK\xAC`9\x83aJ\xB4V[\x91PaK\xB7\x82aKRV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaK\xD9\x81aK\xA0V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aL\"WaL!a8\xCDV[[_aL/\x84\x82\x85\x01aB\xE1V[\x91PP\x92\x91PPV[_aLRaLMaLH\x84a:IV[a9\xEEV[a96V[\x90P\x91\x90PV[aLb\x81aL8V[\x82RPPV[_`@\x82\x01\x90PaL{_\x83\x01\x85a::V[aL\x88` \x83\x01\x84aLYV[\x93\x92PPPV[\x7FStakeRegistry.registerOperator: _\x82\x01R\x7FOperator does not meet minimum s` \x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`@\x82\x01RPV[_aM\x0F`[\x83aJ\xB4V[\x91PaM\x1A\x82aL\x8FV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaM<\x81aM\x03V[\x90P\x91\x90PV[\x7FStakeRegistry.removeStrategies: _\x82\x01R\x7Fno indices to remove provided\0\0\0` \x82\x01RPV[_aM\x9D`=\x83aJ\xB4V[\x91PaM\xA8\x82aMCV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaM\xCA\x81aM\x91V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_aM\xF4aM\xEFaM\xEA\x84aM\xD1V[a9\xEEV[a96V[\x90P\x91\x90PV[aN\x04\x81aM\xDAV[\x82RPPV[_`@\x82\x01\x90PaN\x1D_\x83\x01\x85a::V[aN*` \x83\x01\x84aM\xFBV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aNh\x82a96V[\x91PaNs\x83a96V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aN\x8BWaN\x8AaN1V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[\x7FStakeRegistry.initializeQuorum: _\x82\x01R\x7Fquorum already exists\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aO\x18`5\x83aJ\xB4V[\x91PaO#\x82aN\xBEV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaOE\x81aO\x0CV[\x90P\x91\x90PV[\x7FStakeRegistry.getTotalStakeIndic_\x82\x01R\x7FesAtBlockNumber: quorum has no s` \x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`@\x82\x01RPV[_aO\xCC`[\x83aJ\xB4V[\x91PaO\xD7\x82aOLV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaO\xF9\x81aO\xC0V[\x90P\x91\x90PV[\x7FStakeRegistry.quorumExists: quor_\x82\x01R\x7Fum does not exist\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aPZ`1\x83aJ\xB4V[\x91PaPe\x82aP\0V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaP\x87\x81aPNV[\x90P\x91\x90PV[_aP\x98\x82a96V[\x91PaP\xA3\x83a96V[\x92P\x82\x82\x02aP\xB1\x81a96V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aP\xC8WaP\xC7aN1V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_aQ\x06\x82a96V[\x91PaQ\x11\x83a96V[\x92P\x82aQ!WaQ aP\xCFV[[\x82\x82\x04\x90P\x92\x91PPV[_aQ6\x82a:IV[\x91PaQA\x83a:IV[\x92P\x82\x82\x01\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQeWaQdaN1V[[\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_aQ\xA2\x83\x83aGoV[` \x83\x01\x90P\x92\x91PPV[_\x81_\x1C\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aQ\xEAaQ\xE5\x83aQ\xAEV[aQ\xB9V[\x90P\x91\x90PV[_aQ\xFC\x82TaQ\xD8V[\x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_aR\x19\x82aQkV[aR#\x81\x85aQuV[\x93PaR.\x83aQ\x85V[\x80_[\x83\x81\x10\x15aReWaRB\x82aQ\xF1V[aRL\x88\x82aQ\x97V[\x97PaRW\x83aR\x03V[\x92PP`\x01\x81\x01\x90PaR1V[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90PaR\x85_\x83\x01\x85aB\xA3V[\x81\x81\x03` \x83\x01RaR\x97\x81\x84aR\x0FV[\x90P\x93\x92PPPV[_\x81Q\x90PaR\xAE\x81a9gV[\x92\x91PPV[_aR\xC6aR\xC1\x84a@yV[a@_V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aR\xE9WaR\xE8a;0V[[\x83[\x81\x81\x10\x15aS\x12W\x80aR\xFE\x88\x82aR\xA0V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaR\xEBV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aS0WaS/a;(V[[\x81QaS@\x84\x82` \x86\x01aR\xB4V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aS^WaS]a8\xCDV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS{WaSza8\xD1V[[aS\x87\x84\x82\x85\x01aS\x1CV[\x91PP\x92\x91PPV[_\x81Q\x90PaS\x9E\x81a:\xA7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aS\xB9WaS\xB8a8\xCDV[[_aS\xC6\x84\x82\x85\x01aS\x90V[\x91PP\x92\x91PPV[\x7FStakeRegistry.onlyCoordinatorOwn_\x82\x01R\x7Fer: caller is not the owner of t` \x82\x01R\x7Fhe registryCoordinator\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aTO`V\x83aJ\xB4V[\x91PaTZ\x82aS\xCFV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT|\x81aTCV[\x90P\x91\x90PV[\x7FStakeRegistry.onlyRegistryCoordi_\x82\x01R\x7Fnator: caller is not the Registr` \x82\x01R\x7FyCoordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aU\x03`L\x83aJ\xB4V[\x91PaU\x0E\x82aT\x83V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaU0\x81aT\xF7V[\x90P\x91\x90PV[_`@\x82\x01\x90PaUJ_\x83\x01\x85aD\xADV[aUW` \x83\x01\x84a:`V[\x93\x92PPPV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0` \x82\x01RPV[_aU\xB8`8\x83aJ\xB4V[\x91PaU\xC3\x82aU^V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaU\xE5\x81aU\xACV[\x90P\x91\x90PV[_aU\xF6\x82a96V[\x91PaV\x01\x83a96V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aV\x19WaV\x18aN1V[[\x92\x91PPV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L` \x82\x01R\x7FENGTH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aV\x9F`E\x83aJ\xB4V[\x91PaV\xAA\x82aV\x1FV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaV\xCC\x81aV\x93V[\x90P\x91\x90PV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: cannot add same strategy 2x\0\0\0` \x82\x01RPV[_aW-`=\x83aJ\xB4V[\x91PaW8\x82aV\xD3V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaWZ\x81aW!V[\x90P\x91\x90PV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: cannot add strategy with zero ` \x82\x01R\x7Fweight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aW\xE1`F\x83aJ\xB4V[\x91PaW\xEC\x82aWaV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaX\x0E\x81aW\xD5V[\x90P\x91\x90PV[\x7FStakeRegistry._validateStakeUpda_\x82\x01R\x7FteAtBlockNumber: stakeUpdate is ` \x82\x01R\x7Ffrom after blockNumber\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aX\x95`V\x83aJ\xB4V[\x91PaX\xA0\x82aX\x15V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaX\xC2\x81aX\x89V[\x90P\x91\x90PV[\x7FStakeRegistry._validateStakeUpda_\x82\x01R\x7FteAtBlockNumber: there is a newe` \x82\x01R\x7Fr stakeUpdate available before b`@\x82\x01R\x7FlockNumber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_aYo`j\x83aJ\xB4V[\x91PaYz\x82aX\xC9V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaY\x9C\x81aYcV[\x90P\x91\x90PV[_`@\x82\x01\x90PaY\xB6_\x83\x01\x85aF\x85V[aY\xC3` \x83\x01\x84aF\x85V[\x93\x92PPPV[_aY\xD4\x82a96V[\x91P_\x82\x03aY\xE6WaY\xE5aN1V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStakeRegistry._getStakeUpdateInd_\x82\x01R\x7FexForOperatorAtBlockNumber: no s` \x82\x01R\x7Ftake update found for operatorId`@\x82\x01R\x7F and quorumNumber at block numbe``\x82\x01R\x7Fr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01RPV[_aZ\xBD`\x81\x83aJ\xB4V[\x91PaZ\xC8\x82aY\xF1V[`\xA0\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaZ\xEA\x81aZ\xB1V[\x90P\x91\x90PV[aZ\xFA\x81a:\x96V[\x82RPPV[`@\x82\x01_\x82\x01Qa[\x14_\x85\x01\x82aZ\xF1V[P` \x82\x01Qa['` \x85\x01\x82a>\xCAV[PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a[Q\x83\x83aZ\xF1V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a[s\x82a[-V[a[}\x81\x85aQuV[\x93Pa[\x88\x83a[7V[\x80_[\x83\x81\x10\x15a[\xB8W\x81Qa[\x9F\x88\x82a[FV[\x97Pa[\xAA\x83a[]V[\x92PP`\x01\x81\x01\x90Pa[\x8BV[P\x85\x93PPPP\x92\x91PPV[_`\xA0\x82\x01\x90Pa[\xD8_\x83\x01\x87a[\0V[\x81\x81\x03`@\x83\x01Ra[\xEA\x81\x86a[iV[\x90P\x81\x81\x03``\x83\x01Ra[\xFE\x81\x85aR\x0FV[\x90Pa\\\r`\x80\x83\x01\x84aF\x85V[\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\\0Wa\\/a@\x01V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a\\Sa\\N\x84a\\\x16V[a@_V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\\vWa\\ua;0V[[\x83[\x81\x81\x10\x15a\\\xBDW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\x9BWa\\\x9Aa;(V[[\x80\x86\x01a\\\xA8\x89\x82aS\x1CV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\\xV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\\\xDBWa\\\xDAa;(V[[\x81Qa\\\xEB\x84\x82` \x86\x01a\\AV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a]\tWa]\x08a8\xCDV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]&Wa]%a8\xD1V[[a]2\x84\x82\x85\x01a\\\xC7V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a]N\x82a];V[\x91Pa]Y\x83a];V[\x92P\x82\x82\x03\x90P\x81\x81\x12_\x84\x12\x16\x82\x82\x13_\x85\x12\x15\x16\x17\x15a]~Wa]}aN1V[[\x92\x91PPV[_a]\x8E\x82a];V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a]\xC0Wa]\xBFaN1V[[\x81_\x03\x90P\x91\x90PV[_a]\xD4\x82a:IV[\x91Pa]\xDF\x83a:IV[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\x03Wa^\x02aN1V[[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x96o\xF8\x1B\xFD \xDF\xAB\x0B\"\xDB\xD7\xCF\xF1\xDFz\x11\x8EA7\x03\xBF\xFA\xAF\xB6\x86\x11\xFC\x964\x93:dsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610230575f3560e01c806386c068561161012e578063c601527d116100b6578063df5cf7231161007a578063df5cf7231461076c578063e086adb31461078a578063f2be94ae146107a6578063f851e198146107d6578063fa28c6271461080657610230565b8063c601527d146106a4578063c8294c56146106c0578063cc5a7c20146106f0578063d5eccc051461070c578063dd9846b91461073c57610230565b8063adc804da116100fd578063adc804da146105dc578063b6904b781461060c578063bc9a40c31461063c578063bd29b8cd14610658578063c46778a51461067457610230565b806386c06856146105305780639ab4d6ff1461054c5780639f3ccf651461057c578063ac6bfb03146105ac57610230565b80635401ed27116101bc5780636b3aa72e116101805780636b3aa72e1461048a5780636d14a987146104a857806375d4173a146104c65780637c172347146104e257806381c075021461050057610230565b80635401ed27146103c05780635e5a6775146103f05780635f1f2d771461040e57806366acfefe1461042a578063697fbd931461045a57610230565b8063255047771161020357806325504777146102e15780632cd95940146103125780633998fdd3146103425780633ca5a5f5146103605780634bd26e091461039057610230565b80630491b41c1461023457806308732461146102645780631f9b74e01461029557806320b66298146102c5575b5f5ffd5b61024e6004803603810190610249919061390b565b610836565b60405161025b919061394e565b60405180910390f35b61027e60048036038101906102799190613991565b610859565b60405161028c929190613a6f565b60405180910390f35b6102af60048036038101906102aa9190613ad1565b6108c5565b6040516102bc9190613b0f565b60405180910390f35b6102df60048036038101906102da9190613bde565b6108e9565b005b6102fb60048036038101906102f69190613cf7565b610b0d565b604051610309929190613e1f565b60405180910390f35b61032c60048036038101906103279190613e54565b610cf8565b6040516103399190613f98565b60405180910390f35b61034a610dfd565b6040516103579190613fd8565b60405180910390f35b61037a6004803603810190610375919061390b565b610e21565b604051610387919061394e565b60405180910390f35b6103aa60048036038101906103a59190613e54565b610e44565b6040516103b7919061394e565b60405180910390f35b6103da60048036038101906103d59190613e54565b610e77565b6040516103e79190613b0f565b60405180910390f35b6103f8610e93565b604051610405919061394e565b60405180910390f35b61042860048036038101906104239190614139565b610e9f565b005b610444600480360381019061043f9190613cf7565b6112ed565b60405161045191906141c5565b60405180910390f35b610474600480360381019061046f919061390b565b6113b4565b6040516104819190614251565b60405180910390f35b6104926113d1565b60405161049f919061428a565b60405180910390f35b6104b06113f5565b6040516104bd91906142b2565b60405180910390f35b6104e060048036038101906104db9190614441565b611419565b005b6104ea61157d565b6040516104f791906144bc565b60405180910390f35b61051a600480360381019061051591906144ff565b611582565b6040516105279190614604565b60405180910390f35b61054a60048036038101906105459190614647565b6117aa565b005b6105666004803603810190610561919061390b565b6117c0565b6040516105739190614694565b60405180910390f35b61059660048036038101906105919190613991565b6117e0565b6040516105a391906146ad565b60405180910390f35b6105c660048036038101906105c191906146c6565b611828565b6040516105d39190614756565b60405180910390f35b6105f660048036038101906105f19190613991565b61190a565b60405161060391906147ab565b60405180910390f35b61062660048036038101906106219190613991565b6119e7565b6040516106339190614756565b60405180910390f35b610656600480360381019061065191906147c4565b611ab9565b005b610672600480360381019061066d9190614802565b611ada565b005b61068e6004803603810190610689919061390b565b611b4c565b60405161069b9190613b0f565b60405180910390f35b6106be60048036038101906106b9919061485f565b611b73565b005b6106da60048036038101906106d591906148b9565b611b94565b6040516106e79190613b0f565b60405180910390f35b61070a60048036038101906107059190614909565b611c73565b005b6107266004803603810190610721919061390b565b611de3565b6040516107339190613b0f565b60405180910390f35b61075660048036038101906107519190614989565b611e5c565b6040516107639190614694565b60405180910390f35b610774611e71565b60405161078191906149f9565b60405180910390f35b6107a4600480360381019061079f9190614a12565b611e95565b005b6107c060048036038101906107bb9190614a50565b611eab565b6040516107cd9190613b0f565b60405180910390f35b6107f060048036038101906107eb9190613e54565b611f9a565b6040516107fd9190614756565b60405180910390f35b610820600480360381019061081b9190614989565b6120d3565b60405161082d9190613b0f565b60405180910390f35b5f60015f8360ff1660ff1681526020019081526020015f20805490509050919050565b6003602052815f5260405f208181548110610872575f80fd5b905f5260205f20015f9150915050805f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1690805f0160149054906101000a90046bffffffffffffffffffffffff16905082565b5f826108d081612148565b5f6108db8585612193565b509050809250505092915050565b6108f16125ed565b846108fb81612148565b5f8585905090505f8111610944576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161093b90614b34565b60405180910390fd5b808484905014610989576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161098090614bc2565b60405180910390fd5b5f60035f8960ff1660ff1681526020019081526020015f2090505f5f90505b82811015610b02578585828181106109c3576109c2614be0565b5b90506020020160208101906109d89190614c0d565b828989848181106109ec576109eb614be0565b5b9050602002013581548110610a0457610a03614be0565b5b905f5260205f20015f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a85818110610a7557610a74614be0565b5b9050602002013581548110610a8d57610a8c614be0565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16888885818110610aca57610ac9614be0565b5b9050602002016020810190610adf9190614c0d565b604051610aed929190614c68565b60405180910390a280806001019150506109a8565b505050505050505050565b606080610b186126e9565b5f8484905067ffffffffffffffff811115610b3657610b35614001565b5b604051908082528060200260200182016040528015610b645781602001602082028036833780820191505090505b5090505f8585905067ffffffffffffffff811115610b8557610b84614001565b5b604051908082528060200260200182016040528015610bb35781602001602082028036833780820191505090505b5090505f5f90505b86869050811015610ce6575f878783818110610bda57610bd9614be0565b5b9050013560f81c60f81b60f81c9050610bf281612148565b5f5f610bfe838d612193565b9150915080610c42576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610c3990614d25565b60405180910390fd5b5f610c4e8c8585612779565b905082878681518110610c6457610c63614be0565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff1681525050610c988482612b1e565b868681518110610cab57610caa614be0565b5b60200260200101906bffffffffffffffffffffffff1690816bffffffffffffffffffffffff1681525050505050508080600101915050610bbb565b50818193509350505094509492505050565b606060025f8481526020019081526020015f205f8360ff1660ff1681526020019081526020015f20805480602002602001604051908101604052809291908181526020015f905b82821015610df1578382905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505081526020019060010190610d3f565b50505050905092915050565b7f000000000000000000000000000000000000000000000000000000000000000081565b5f60035f8360ff1660ff1681526020019081526020015f20805490509050919050565b5f60025f8481526020019081526020015f205f8360ff1660ff1681526020019081526020015f2080549050905092915050565b5f5f610e838484611f9a565b9050806040015191505092915050565b670de0b6b3a764000081565b610ea76125ed565b81610eb181612148565b5f825190505f8111610ef8576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401610eef90614db3565b60405180910390fd5b5f60035f8660ff1660ff1681526020019081526020015f2090505f60045f8760ff1660ff1681526020019081526020015f2090505f5f90505b838110156112e4578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f784888481518110610f7257610f71614be0565b5b602002602001015181548110610f8b57610f8a614be0565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16604051610fc291906146ad565b60405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a758488848151811061100357611002614be0565b5b60200260200101518154811061101c5761101b614be0565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff165f604051611055929190614e0a565b60405180910390a2826001848054905061106f9190614e5e565b815481106110805761107f614be0565b5b905f5260205f20018387838151811061109c5761109b614be0565b5b6020026020010151815481106110b5576110b4614be0565b5b905f5260205f20015f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff16815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055505f820160149054906101000a90046bffffffffffffffffffffffff16815f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055509050508280548061118157611180614e91565b5b600190038181905f5260205f20015f5f82015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff02191690555f820160146101000a8154906bffffffffffffffffffffffff02191690555050905581600183805490506111e99190614e5e565b815481106111fa576111f9614be0565b5b905f5260205f20015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff168287838151811061123657611235614be0565b5b60200260200101518154811061124f5761124e614be0565b5b905f5260205f20015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550818054806112a5576112a4614e91565b5b600190038181905f5260205f20015f6101000a81549073ffffffffffffffffffffffffffffffffffffffff021916905590558080600101915050610f31565b50505050505050565b5f6112f66126e9565b5f5f5f5f90505b858590508110156113a6575f86868381811061131c5761131b614be0565b5b9050013560f81c60f81b60f81c905061133481612148565b5f5f611340838c612193565b915091508061137c575f9150611379838777ffffffffffffffffffffffffffffffffffffffffffffffff16612d4d90919063ffffffff16565b95505b5f6113888b8585612779565b90506113948482612b1e565b505050505080806001019150506112fd565b508192505050949350505050565b6005602052805f5260405f205f915054906101000a900460ff1681565b7f000000000000000000000000000000000000000000000000000000000000000081565b7f000000000000000000000000000000000000000000000000000000000000000081565b6114216126e9565b61142a83612d60565b1561146a576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161146190614f2e565b60405180910390fd5b6114748382612d86565b61147e8383613202565b611488835f613285565b60015f8460ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505050505050565b602081565b60605f8383905067ffffffffffffffff8111156115a2576115a1614001565b5b6040519080825280602002602001820160405280156115d05781602001602082028036833780820191505090505b5090505f5f90505b8484905081101561179e575f8585838181106115f7576115f6614be0565b5b9050013560f81c60f81b60f81c905061160f81612148565b8663ffffffff1660015f8360ff1660ff1681526020019081526020015f205f8154811061163f5761163e614be0565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff1611156116a0576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161169790614fe2565b60405180910390fd5b5f60015f8360ff1660ff1681526020019081526020015f208054905090505f5f90505b8181101561178e578863ffffffff1660015f8560ff1660ff1681526020019081526020015f20600183856116f79190614e5e565b6117019190614e5e565b8154811061171257611711614be0565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff161161178157600181836117459190614e5e565b61174f9190614e5e565b85858151811061176257611761614be0565b5b602002602001019063ffffffff16908163ffffffff168152505061178e565b80806001019150506116c3565b50505080806001019150506115d8565b50809150509392505050565b6117b26125ed565b6117bc8282613285565b5050565b6006602052805f5260405f205f915054906101000a900463ffffffff1681565b6004602052815f5260405f2081815481106117f9575f80fd5b905f5260205f20015f915091509054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b611830613850565b60025f8481526020019081526020015f205f8560ff1660ff1681526020019081526020015f20828154811061186857611867614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505090509392505050565b611912613888565b60035f8460ff1660ff1681526020019081526020015f20828154811061193b5761193a614be0565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905092915050565b6119ef613850565b60015f8460ff1660ff1681526020019081526020015f208281548110611a1857611a17614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905092915050565b611ac16125ed565b81611acb81612148565b611ad58383613202565b505050565b611ae26126e9565b5f5f90505b82829050811015611b46575f838383818110611b0657611b05614be0565b5b9050013560f81c60f81b60f81c9050611b1e81612148565b5f611b2a86835f612779565b9050611b368282612b1e565b5050508080600101915050611ae7565b50505050565b5f602052805f5260405f205f915054906101000a90046bffffffffffffffffffffffff1681565b611b7b6125ed565b81611b8581612148565b611b8f8383612d86565b505050565b5f5f60015f8660ff1660ff1681526020019081526020015f208381548110611bbf57611bbe614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff16815250509050611c6481856132fe565b80604001519150509392505050565b611c7b6126e9565b611c8484612d60565b15611cc4576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401611cbb90614f2e565b60405180910390fd5b611cce8482612d86565b611cd88484613202565b611ce3846001613285565b611ced84836133ba565b60015f8560ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550505050505050565b5f60015f8360ff1660ff1681526020019081526020015f206001805f8560ff1660ff1681526020019081526020015f2080549050611e219190614e5e565b81548110611e3257611e31614be0565b5b905f5260205f20015f0160089054906101000a90046bffffffffffffffffffffffff169050919050565b5f611e68848484613457565b90509392505050565b7f000000000000000000000000000000000000000000000000000000000000000081565b611e9d6125ed565b611ea782826133ba565b5050565b5f5f60025f8581526020019081526020015f205f8760ff1660ff1681526020019081526020015f208381548110611ee557611ee4614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff16815250509050611f8a81866132fe565b8060400151915050949350505050565b611fa2613850565b5f60025f8581526020019081526020015f205f8460ff1660ff1681526020019081526020015f20805490509050611fd7613850565b5f8203611fe85780925050506120cd565b60025f8681526020019081526020015f205f8560ff1660ff1681526020019081526020015f2060018361201b9190614e5e565b8154811061202c5761202b614be0565b5b905f5260205f20016040518060600160405290815f82015f9054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160049054906101000a900463ffffffff1663ffffffff1663ffffffff1681526020015f820160089054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff1681525050905080925050505b92915050565b5f60025f8581526020019081526020015f205f8460ff1660ff1681526020019081526020015f20612105858585613457565b63ffffffff168154811061211c5761211b614be0565b5b905f5260205f20015f0160089054906101000a90046bffffffffffffffffffffffff1690509392505050565b61215181612d60565b612190576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161218790615070565b60405180910390fd5b50565b5f5f5f5f6121a086610e21565b90506121aa613888565b60606001808111156121bf576121be6141de565b5b60055f8a60ff1660ff1681526020019081526020015f205f9054906101000a900460ff1660018111156121f5576121f46141de565b5b0361237057612204888861356e565b90505f5f90505b8381101561236a5760035f8a60ff1660ff1681526020019081526020015f20818154811061223c5761223b614be0565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505092505f8282815181106122f6576122f5614be0565b5b6020026020010151111561235d57670de0b6b3a764000083602001516bffffffffffffffffffffffff1683838151811061233357612332614be0565b5b6020026020010151612345919061508e565b61234f91906150fc565b8561235a919061512c565b94505b808060010191505061220b565b5061258b565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663900413478860045f8c60ff1660ff1681526020019081526020015f206040518363ffffffff1660e01b81526004016123e1929190615272565b5f60405180830381865afa1580156123fb573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906124239190615349565b90505f5f90505b838110156125895760035f8a60ff1660ff1681526020019081526020015f20818154811061245b5761245a614be0565b5b905f5260205f20016040518060400160405290815f82015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020015f820160149054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff166bffffffffffffffffffffffff168152505092505f82828151811061251557612514614be0565b5b6020026020010151111561257c57670de0b6b3a764000083602001516bffffffffffffffffffffffff1683838151811061255257612551614be0565b5b6020026020010151612564919061508e565b61256e91906150fc565b85612579919061512c565b94505b808060010191505061242a565b505b5f5f5f8a60ff1660ff1681526020019081526020015f205f9054906101000a90046bffffffffffffffffffffffff166bffffffffffffffffffffffff16856bffffffffffffffffffffffff161015905084819650965050505050509250929050565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015612656573d5f5f3e3d5ffd5b505050506040513d601f19601f8201168201806040525081019061267a91906153a4565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16146126e7576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016126de90615465565b60405180910390fd5b565b7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614612777576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161276e90615519565b60405180910390fd5b565b5f5f5f60025f8781526020019081526020015f205f8660ff1660ff1681526020019081526020015f208054905090505f81036128b35760025f8781526020019081526020015f205f8660ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001866bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055505050612ace565b5f60025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f206001836128e79190614e5e565b815481106128f8576128f7614be0565b5b905f5260205f20019050805f0160089054906101000a90046bffffffffffffffffffffffff169250846bffffffffffffffffffffffff16836bffffffffffffffffffffffff160361294e575f9350505050612b17565b4363ffffffff16815f015f9054906101000a900463ffffffff1663ffffffff16036129aa5784815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550612acc565b43815f0160046101000a81548163ffffffff021916908363ffffffff16021790555060025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001876bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555050505b505b857f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d8686604051612b00929190615537565b60405180910390a2612b1282856137e5565b925050505b9392505050565b5f5f60015f8560ff1660ff1681526020019081526020015f208054905090505f60015f8660ff1660ff1681526020019081526020015f20600183612b629190614e5e565b81548110612b7357612b72614be0565b5b905f5260205f200190505f8403612ba957805f0160089054906101000a90046bffffffffffffffffffffffff1692505050612d47565b5f612bcf825f0160089054906101000a90046bffffffffffffffffffffffff1686613816565b90504363ffffffff16825f015f9054906101000a900463ffffffff1663ffffffff1603612c2d5780825f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550612d40565b43825f0160046101000a81548163ffffffff021916908363ffffffff16021790555060015f8760ff1660ff1681526020019081526020015f2060405180606001604052804363ffffffff1681526020015f63ffffffff168152602001836bffffffffffffffffffffffff16815250908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548163ffffffff021916908363ffffffff1602179055506020820151815f0160046101000a81548163ffffffff021916908363ffffffff1602179055506040820151815f0160086101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff16021790555050505b8093505050505b92915050565b5f8160ff166001901b8317905092915050565b5f5f60015f8460ff1660ff1681526020019081526020015f208054905014159050919050565b5f815111612dc9576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612dc0906155ce565b60405180910390fd5b5f815190505f60035f8560ff1660ff1681526020019081526020015f20805490509050602060ff168282612dfd91906155ec565b1115612e3e576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612e35906156b5565b60405180910390fd5b5f5f90505b828110156131fb575f5f90505b8183612e5c91906155ec565b811015612f4d57848281518110612e7657612e75614be0565b5b60200260200101515f015173ffffffffffffffffffffffffffffffffffffffff1660035f8860ff1660ff1681526020019081526020015f208281548110612ec057612ebf614be0565b5b905f5260205f20015f015f9054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1603612f40576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612f3790615743565b60405180910390fd5b8080600101915050612e50565b505f848281518110612f6257612f61614be0565b5b6020026020010151602001516bffffffffffffffffffffffff1611612fbc576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401612fb3906157f7565b60405180910390fd5b60035f8660ff1660ff1681526020019081526020015f20848281518110612fe657612fe5614be0565b5b6020026020010151908060018154018082558091505060019003905f5260205f20015f909190919091505f820151815f015f6101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506020820151815f0160146101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff160217905550505060045f8660ff1660ff1681526020019081526020015f208482815181106130b6576130b5614be0565b5b60200260200101515f0151908060018154018082558091505060019003905f5260205f20015f9091909190916101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055508460ff167f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540485838151811061315657613155614be0565b5b60200260200101515f015160405161316e91906146ad565b60405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a758583815181106131ae576131ad614be0565b5b60200260200101515f01518684815181106131cc576131cb614be0565b5b6020026020010151602001516040516131e6929190614c68565b60405180910390a28080600101915050612e43565b5050505050565b805f5f8460ff1660ff1681526020019081526020015f205f6101000a8154816bffffffffffffffffffffffff02191690836bffffffffffffffffffffffff1602179055508160ff167f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf826040516132799190613b0f565b60405180910390a25050565b8060055f8460ff1660ff1681526020019081526020015f205f6101000a81548160ff021916908360018111156132be576132bd6141de565b5b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d816040516132f29190614251565b60405180910390a15050565b815f015163ffffffff168163ffffffff161015613350576040517f08c379a0000000000000000000000000000000000000000000000000000000008152600401613347906158ab565b60405180910390fd5b5f826020015163ffffffff1614806133775750816020015163ffffffff168163ffffffff16105b6133b6576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004016133ad90615985565b60405180910390fd5b5050565b5f60065f8460ff1660ff1681526020019081526020015f205f9054906101000a900463ffffffff1690508160065f8560ff1660ff1681526020019081526020015f205f6101000a81548163ffffffff021916908363ffffffff1602179055507f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7818360405161344a9291906159a3565b60405180910390a1505050565b5f5f60025f8681526020019081526020015f205f8560ff1660ff1681526020019081526020015f208054905090505f8190505b5f81111561352b578363ffffffff1660025f8881526020019081526020015f205f8760ff1660ff1681526020019081526020015f206001836134cc9190614e5e565b815481106134dd576134dc614be0565b5b905f5260205f20015f015f9054906101000a900463ffffffff1663ffffffff16116135185760018161350f9190614e5e565b92505050613567565b8080613523906159ca565b91505061348a565b506040517f08c379a000000000000000000000000000000000000000000000000000000000815260040161355e90615ad3565b60405180910390fd5b9392505050565b60605f600167ffffffffffffffff81111561358c5761358b614001565b5b6040519080825280602002602001820160405280156135ba5781602001602082028036833780820191505090505b50905082815f815181106135d1576135d0614be0565b5b602002602001019073ffffffffffffffffffffffffffffffffffffffff16908173ffffffffffffffffffffffffffffffffffffffff16815250505f60065f8660ff1660ff1681526020019081526020015f205f9054906101000a900463ffffffff1663ffffffff164261364491906155ec565b90505f7f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1663ca8aa7c76040518163ffffffff1660e01b8152600401602060405180830381865afa1580156136b0573d5f5f3e3d5ffd5b505050506040513d601f19601f820116820180604052508101906136d491906153a4565b73ffffffffffffffffffffffffffffffffffffffff16632bab2c4a60405180604001604052807f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff1681526020018960ff1663ffffffff168152508560045f8b60ff1660ff1681526020019081526020015f20866040518563ffffffff1660e01b815260040161377b9493929190615bc5565b5f60405180830381865afa158015613795573d5f5f3e3d5ffd5b505050506040513d5f823e3d601f19601f820116820180604052508101906137bd9190615cf4565b9050805f815181106137d2576137d1614be0565b5b6020026020010151935050505092915050565b5f826bffffffffffffffffffffffff16826bffffffffffffffffffffffff1661380e9190615d44565b905092915050565b5f5f82121561383b578161382990615d84565b836138349190615dca565b905061384a565b8183613847919061512c565b90505b92915050565b60405180606001604052805f63ffffffff1681526020015f63ffffffff1681526020015f6bffffffffffffffffffffffff1681525090565b60405180604001604052805f73ffffffffffffffffffffffffffffffffffffffff1681526020015f6bffffffffffffffffffffffff1681525090565b5f604051905090565b5f5ffd5b5f5ffd5b5f60ff82169050919050565b6138ea816138d5565b81146138f4575f5ffd5b50565b5f81359050613905816138e1565b92915050565b5f602082840312156139205761391f6138cd565b5b5f61392d848285016138f7565b91505092915050565b5f819050919050565b61394881613936565b82525050565b5f6020820190506139615f83018461393f565b92915050565b61397081613936565b811461397a575f5ffd5b50565b5f8135905061398b81613967565b92915050565b5f5f604083850312156139a7576139a66138cd565b5b5f6139b4858286016138f7565b92505060206139c58582860161397d565b9150509250929050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f819050919050565b5f613a11613a0c613a07846139cf565b6139ee565b6139cf565b9050919050565b5f613a22826139f7565b9050919050565b5f613a3382613a18565b9050919050565b613a4381613a29565b82525050565b5f6bffffffffffffffffffffffff82169050919050565b613a6981613a49565b82525050565b5f604082019050613a825f830185613a3a565b613a8f6020830184613a60565b9392505050565b5f613aa0826139cf565b9050919050565b613ab081613a96565b8114613aba575f5ffd5b50565b5f81359050613acb81613aa7565b92915050565b5f5f60408385031215613ae757613ae66138cd565b5b5f613af4858286016138f7565b9250506020613b0585828601613abd565b9150509250929050565b5f602082019050613b225f830184613a60565b92915050565b5f5ffd5b5f5ffd5b5f5ffd5b5f5f83601f840112613b4957613b48613b28565b5b8235905067ffffffffffffffff811115613b6657613b65613b2c565b5b602083019150836020820283011115613b8257613b81613b30565b5b9250929050565b5f5f83601f840112613b9e57613b9d613b28565b5b8235905067ffffffffffffffff811115613bbb57613bba613b2c565b5b602083019150836020820283011115613bd757613bd6613b30565b5b9250929050565b5f5f5f5f5f60608688031215613bf757613bf66138cd565b5b5f613c04888289016138f7565b955050602086013567ffffffffffffffff811115613c2557613c246138d1565b5b613c3188828901613b34565b9450945050604086013567ffffffffffffffff811115613c5457613c536138d1565b5b613c6088828901613b89565b92509250509295509295909350565b5f819050919050565b613c8181613c6f565b8114613c8b575f5ffd5b50565b5f81359050613c9c81613c78565b92915050565b5f5f83601f840112613cb757613cb6613b28565b5b8235905067ffffffffffffffff811115613cd457613cd3613b2c565b5b602083019150836001820283011115613cf057613cef613b30565b5b9250929050565b5f5f5f5f60608587031215613d0f57613d0e6138cd565b5b5f613d1c87828801613abd565b9450506020613d2d87828801613c8e565b935050604085013567ffffffffffffffff811115613d4e57613d4d6138d1565b5b613d5a87828801613ca2565b925092505092959194509250565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b613d9a81613a49565b82525050565b5f613dab8383613d91565b60208301905092915050565b5f602082019050919050565b5f613dcd82613d68565b613dd78185613d72565b9350613de283613d82565b805f5b83811015613e12578151613df98882613da0565b9750613e0483613db7565b925050600181019050613de5565b5085935050505092915050565b5f6040820190508181035f830152613e378185613dc3565b90508181036020830152613e4b8184613dc3565b90509392505050565b5f5f60408385031215613e6a57613e696138cd565b5b5f613e7785828601613c8e565b9250506020613e88858286016138f7565b9150509250929050565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f63ffffffff82169050919050565b613ed381613ebb565b82525050565b606082015f820151613eed5f850182613eca565b506020820151613f006020850182613eca565b506040820151613f136040850182613d91565b50505050565b5f613f248383613ed9565b60608301905092915050565b5f602082019050919050565b5f613f4682613e92565b613f508185613e9c565b9350613f5b83613eac565b805f5b83811015613f8b578151613f728882613f19565b9750613f7d83613f30565b925050600181019050613f5e565b5085935050505092915050565b5f6020820190508181035f830152613fb08184613f3c565b905092915050565b5f613fc282613a18565b9050919050565b613fd281613fb8565b82525050565b5f602082019050613feb5f830184613fc9565b92915050565b5f601f19601f8301169050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b61403782613ff1565b810181811067ffffffffffffffff8211171561405657614055614001565b5b80604052505050565b5f6140686138c4565b9050614074828261402e565b919050565b5f67ffffffffffffffff82111561409357614092614001565b5b602082029050602081019050919050565b5f6140b66140b184614079565b61405f565b905080838252602082019050602084028301858111156140d9576140d8613b30565b5b835b8181101561410257806140ee888261397d565b8452602084019350506020810190506140db565b5050509392505050565b5f82601f8301126141205761411f613b28565b5b81356141308482602086016140a4565b91505092915050565b5f5f6040838503121561414f5761414e6138cd565b5b5f61415c858286016138f7565b925050602083013567ffffffffffffffff81111561417d5761417c6138d1565b5b6141898582860161410c565b9150509250929050565b5f77ffffffffffffffffffffffffffffffffffffffffffffffff82169050919050565b6141bf81614193565b82525050565b5f6020820190506141d85f8301846141b6565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52602160045260245ffd5b6002811061421c5761421b6141de565b5b50565b5f81905061422c8261420b565b919050565b5f61423b8261421f565b9050919050565b61424b81614231565b82525050565b5f6020820190506142645f830184614242565b92915050565b5f61427482613a18565b9050919050565b6142848161426a565b82525050565b5f60208201905061429d5f83018461427b565b92915050565b6142ac81613a96565b82525050565b5f6020820190506142c55f8301846142a3565b92915050565b6142d481613a49565b81146142de575f5ffd5b50565b5f813590506142ef816142cb565b92915050565b5f67ffffffffffffffff82111561430f5761430e614001565b5b602082029050602081019050919050565b5f5ffd5b5f61432e82613a96565b9050919050565b61433e81614324565b8114614348575f5ffd5b50565b5f8135905061435981614335565b92915050565b5f6040828403121561437457614373614320565b5b61437e604061405f565b90505f61438d8482850161434b565b5f8301525060206143a0848285016142e1565b60208301525092915050565b5f6143be6143b9846142f5565b61405f565b905080838252602082019050604084028301858111156143e1576143e0613b30565b5b835b8181101561440a57806143f6888261435f565b8452602084019350506040810190506143e3565b5050509392505050565b5f82601f83011261442857614427613b28565b5b81356144388482602086016143ac565b91505092915050565b5f5f5f60608486031215614458576144576138cd565b5b5f614465868287016138f7565b9350506020614476868287016142e1565b925050604084013567ffffffffffffffff811115614497576144966138d1565b5b6144a386828701614414565b9150509250925092565b6144b6816138d5565b82525050565b5f6020820190506144cf5f8301846144ad565b92915050565b6144de81613ebb565b81146144e8575f5ffd5b50565b5f813590506144f9816144d5565b92915050565b5f5f5f60408486031215614516576145156138cd565b5b5f614523868287016144eb565b935050602084013567ffffffffffffffff811115614544576145436138d1565b5b61455086828701613ca2565b92509250509250925092565b5f81519050919050565b5f82825260208201905092915050565b5f819050602082019050919050565b5f6145908383613eca565b60208301905092915050565b5f602082019050919050565b5f6145b28261455c565b6145bc8185614566565b93506145c783614576565b805f5b838110156145f75781516145de8882614585565b97506145e98361459c565b9250506001810190506145ca565b5085935050505092915050565b5f6020820190508181035f83015261461c81846145a8565b905092915050565b60028110614630575f5ffd5b50565b5f8135905061464181614624565b92915050565b5f5f6040838503121561465d5761465c6138cd565b5b5f61466a858286016138f7565b925050602061467b85828601614633565b9150509250929050565b61468e81613ebb565b82525050565b5f6020820190506146a75f830184614685565b92915050565b5f6020820190506146c05f830184613a3a565b92915050565b5f5f5f606084860312156146dd576146dc6138cd565b5b5f6146ea868287016138f7565b93505060206146fb86828701613c8e565b925050604061470c8682870161397d565b9150509250925092565b606082015f82015161472a5f850182613eca565b50602082015161473d6020850182613eca565b5060408201516147506040850182613d91565b50505050565b5f6060820190506147695f830184614716565b92915050565b61477881613a29565b82525050565b604082015f8201516147925f85018261476f565b5060208201516147a56020850182613d91565b50505050565b5f6040820190506147be5f83018461477e565b92915050565b5f5f604083850312156147da576147d96138cd565b5b5f6147e7858286016138f7565b92505060206147f8858286016142e1565b9150509250929050565b5f5f5f60408486031215614819576148186138cd565b5b5f61482686828701613c8e565b935050602084013567ffffffffffffffff811115614847576148466138d1565b5b61485386828701613ca2565b92509250509250925092565b5f5f60408385031215614875576148746138cd565b5b5f614882858286016138f7565b925050602083013567ffffffffffffffff8111156148a3576148a26138d1565b5b6148af85828601614414565b9150509250929050565b5f5f5f606084860312156148d0576148cf6138cd565b5b5f6148dd868287016138f7565b93505060206148ee868287016144eb565b92505060406148ff8682870161397d565b9150509250925092565b5f5f5f5f60808587031215614921576149206138cd565b5b5f61492e878288016138f7565b945050602061493f878288016142e1565b9350506040614950878288016144eb565b925050606085013567ffffffffffffffff811115614971576149706138d1565b5b61497d87828801614414565b91505092959194509250565b5f5f5f606084860312156149a05761499f6138cd565b5b5f6149ad86828701613c8e565b93505060206149be868287016138f7565b92505060406149cf868287016144eb565b9150509250925092565b5f6149e382613a18565b9050919050565b6149f3816149d9565b82525050565b5f602082019050614a0c5f8301846149ea565b92915050565b5f5f60408385031215614a2857614a276138cd565b5b5f614a35858286016138f7565b9250506020614a46858286016144eb565b9150509250929050565b5f5f5f5f60808587031215614a6857614a676138cd565b5b5f614a75878288016138f7565b9450506020614a86878288016144eb565b9350506040614a9787828801613c8e565b9250506060614aa88782880161397d565b91505092959194509250565b5f82825260208201905092915050565b7f5374616b6552656769737472792e6d6f646966795374726174656779506172615f8201527f6d733a206e6f20737472617465677920696e64696365732070726f7669646564602082015250565b5f614b1e604083614ab4565b9150614b2982614ac4565b604082019050919050565b5f6020820190508181035f830152614b4b81614b12565b9050919050565b7f5374616b6552656769737472792e6d6f646966795374726174656779506172615f8201527f6d733a20696e707574206c656e677468206d69736d6174636800000000000000602082015250565b5f614bac603983614ab4565b9150614bb782614b52565b604082019050919050565b5f6020820190508181035f830152614bd981614ba0565b9050919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603260045260245ffd5b5f60208284031215614c2257614c216138cd565b5b5f614c2f848285016142e1565b91505092915050565b5f614c52614c4d614c4884613a49565b6139ee565b613936565b9050919050565b614c6281614c38565b82525050565b5f604082019050614c7b5f830185613a3a565b614c886020830184614c59565b9392505050565b7f5374616b6552656769737472792e72656769737465724f70657261746f723a205f8201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360208201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000604082015250565b5f614d0f605b83614ab4565b9150614d1a82614c8f565b606082019050919050565b5f6020820190508181035f830152614d3c81614d03565b9050919050565b7f5374616b6552656769737472792e72656d6f7665537472617465676965733a205f8201527f6e6f20696e646963657320746f2072656d6f76652070726f7669646564000000602082015250565b5f614d9d603d83614ab4565b9150614da882614d43565b604082019050919050565b5f6020820190508181035f830152614dca81614d91565b9050919050565b5f819050919050565b5f614df4614def614dea84614dd1565b6139ee565b613936565b9050919050565b614e0481614dda565b82525050565b5f604082019050614e1d5f830185613a3a565b614e2a6020830184614dfb565b9392505050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f614e6882613936565b9150614e7383613936565b9250828203905081811115614e8b57614e8a614e31565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52603160045260245ffd5b7f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a205f8201527f71756f72756d20616c7265616479206578697374730000000000000000000000602082015250565b5f614f18603583614ab4565b9150614f2382614ebe565b604082019050919050565b5f6020820190508181035f830152614f4581614f0c565b9050919050565b7f5374616b6552656769737472792e676574546f74616c5374616b65496e6469635f8201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360208201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000604082015250565b5f614fcc605b83614ab4565b9150614fd782614f4c565b606082019050919050565b5f6020820190508181035f830152614ff981614fc0565b9050919050565b7f5374616b6552656769737472792e71756f72756d4578697374733a2071756f725f8201527f756d20646f6573206e6f74206578697374000000000000000000000000000000602082015250565b5f61505a603183614ab4565b915061506582615000565b604082019050919050565b5f6020820190508181035f8301526150878161504e565b9050919050565b5f61509882613936565b91506150a383613936565b92508282026150b181613936565b915082820484148315176150c8576150c7614e31565b5b5092915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f61510682613936565b915061511183613936565b925082615121576151206150cf565b5b828204905092915050565b5f61513682613a49565b915061514183613a49565b925082820190506bffffffffffffffffffffffff81111561516557615164614e31565b5b92915050565b5f81549050919050565b5f82825260208201905092915050565b5f819050815f5260205f209050919050565b5f6151a2838361476f565b60208301905092915050565b5f815f1c9050919050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f6151ea6151e5836151ae565b6151b9565b9050919050565b5f6151fc82546151d8565b9050919050565b5f600182019050919050565b5f6152198261516b565b6152238185615175565b935061522e83615185565b805f5b8381101561526557615242826151f1565b61524c8882615197565b975061525783615203565b925050600181019050615231565b5085935050505092915050565b5f6040820190506152855f8301856142a3565b8181036020830152615297818461520f565b90509392505050565b5f815190506152ae81613967565b92915050565b5f6152c66152c184614079565b61405f565b905080838252602082019050602084028301858111156152e9576152e8613b30565b5b835b8181101561531257806152fe88826152a0565b8452602084019350506020810190506152eb565b5050509392505050565b5f82601f8301126153305761532f613b28565b5b81516153408482602086016152b4565b91505092915050565b5f6020828403121561535e5761535d6138cd565b5b5f82015167ffffffffffffffff81111561537b5761537a6138d1565b5b6153878482850161531c565b91505092915050565b5f8151905061539e81613aa7565b92915050565b5f602082840312156153b9576153b86138cd565b5b5f6153c684828501615390565b91505092915050565b7f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e5f8201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f66207460208201527f6865207265676973747279436f6f7264696e61746f7200000000000000000000604082015250565b5f61544f605683614ab4565b915061545a826153cf565b606082019050919050565b5f6020820190508181035f83015261547c81615443565b9050919050565b7f5374616b6552656769737472792e6f6e6c795265676973747279436f6f7264695f8201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260208201527f79436f6f7264696e61746f720000000000000000000000000000000000000000604082015250565b5f615503604c83614ab4565b915061550e82615483565b606082019050919050565b5f6020820190508181035f830152615530816154f7565b9050919050565b5f60408201905061554a5f8301856144ad565b6155576020830184613a60565b9392505050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a206e6f20737472617465676965732070726f76696465640000000000000000602082015250565b5f6155b8603883614ab4565b91506155c38261555e565b604082019050919050565b5f6020820190508181035f8301526155e5816155ac565b9050919050565b5f6155f682613936565b915061560183613936565b925082820190508082111561561957615618614e31565b5b92915050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60208201527f454e475448000000000000000000000000000000000000000000000000000000604082015250565b5f61569f604583614ab4565b91506156aa8261561f565b606082019050919050565b5f6020820190508181035f8301526156cc81615693565b9050919050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a2063616e6e6f74206164642073616d65207374726174656779203278000000602082015250565b5f61572d603d83614ab4565b9150615738826156d3565b604082019050919050565b5f6020820190508181035f83015261575a81615721565b9050919050565b7f5374616b6552656769737472792e5f6164645374726174656779506172616d735f8201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f2060208201527f7765696768740000000000000000000000000000000000000000000000000000604082015250565b5f6157e1604683614ab4565b91506157ec82615761565b606082019050919050565b5f6020820190508181035f83015261580e816157d5565b9050919050565b7f5374616b6552656769737472792e5f76616c69646174655374616b65557064615f8201527f74654174426c6f636b4e756d6265723a207374616b655570646174652069732060208201527f66726f6d20616674657220626c6f636b4e756d62657200000000000000000000604082015250565b5f615895605683614ab4565b91506158a082615815565b606082019050919050565b5f6020820190508181035f8301526158c281615889565b9050919050565b7f5374616b6552656769737472792e5f76616c69646174655374616b65557064615f8201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560208201527f72207374616b6555706461746520617661696c61626c65206265666f7265206260408201527f6c6f636b4e756d62657200000000000000000000000000000000000000000000606082015250565b5f61596f606a83614ab4565b915061597a826158c9565b608082019050919050565b5f6020820190508181035f83015261599c81615963565b9050919050565b5f6040820190506159b65f830185614685565b6159c36020830184614685565b9392505050565b5f6159d482613936565b91505f82036159e6576159e5614e31565b5b600182039050919050565b7f5374616b6552656769737472792e5f6765745374616b65557064617465496e645f8201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360208201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460408201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560608201527f7200000000000000000000000000000000000000000000000000000000000000608082015250565b5f615abd608183614ab4565b9150615ac8826159f1565b60a082019050919050565b5f6020820190508181035f830152615aea81615ab1565b9050919050565b615afa81613a96565b82525050565b604082015f820151615b145f850182615af1565b506020820151615b276020850182613eca565b50505050565b5f81519050919050565b5f819050602082019050919050565b5f615b518383615af1565b60208301905092915050565b5f602082019050919050565b5f615b7382615b2d565b615b7d8185615175565b9350615b8883615b37565b805f5b83811015615bb8578151615b9f8882615b46565b9750615baa83615b5d565b925050600181019050615b8b565b5085935050505092915050565b5f60a082019050615bd85f830187615b00565b8181036040830152615bea8186615b69565b90508181036060830152615bfe818561520f565b9050615c0d6080830184614685565b95945050505050565b5f67ffffffffffffffff821115615c3057615c2f614001565b5b602082029050602081019050919050565b5f615c53615c4e84615c16565b61405f565b90508083825260208201905060208402830185811115615c7657615c75613b30565b5b835b81811015615cbd57805167ffffffffffffffff811115615c9b57615c9a613b28565b5b808601615ca8898261531c565b85526020850194505050602081019050615c78565b5050509392505050565b5f82601f830112615cdb57615cda613b28565b5b8151615ceb848260208601615c41565b91505092915050565b5f60208284031215615d0957615d086138cd565b5b5f82015167ffffffffffffffff811115615d2657615d256138d1565b5b615d3284828501615cc7565b91505092915050565b5f819050919050565b5f615d4e82615d3b565b9150615d5983615d3b565b925082820390508181125f8412168282135f851215161715615d7e57615d7d614e31565b5b92915050565b5f615d8e82615d3b565b91507f80000000000000000000000000000000000000000000000000000000000000008203615dc057615dbf614e31565b5b815f039050919050565b5f615dd482613a49565b9150615ddf83613a49565b925082820390506bffffffffffffffffffffffff811115615e0357615e02614e31565b5b9291505056fea2646970667358221220966ff81bfd20dfab0b22dbd7cff1df7a118e413703bffaafb68611fc9634933a64736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x020W_5`\xE0\x1C\x80c\x86\xC0hV\x11a\x01.W\x80c\xC6\x01R}\x11a\0\xB6W\x80c\xDF\\\xF7#\x11a\0zW\x80c\xDF\\\xF7#\x14a\x07lW\x80c\xE0\x86\xAD\xB3\x14a\x07\x8AW\x80c\xF2\xBE\x94\xAE\x14a\x07\xA6W\x80c\xF8Q\xE1\x98\x14a\x07\xD6W\x80c\xFA(\xC6'\x14a\x08\x06Wa\x020V[\x80c\xC6\x01R}\x14a\x06\xA4W\x80c\xC8)LV\x14a\x06\xC0W\x80c\xCCZ| \x14a\x06\xF0W\x80c\xD5\xEC\xCC\x05\x14a\x07\x0CW\x80c\xDD\x98F\xB9\x14a\x07<Wa\x020V[\x80c\xAD\xC8\x04\xDA\x11a\0\xFDW\x80c\xAD\xC8\x04\xDA\x14a\x05\xDCW\x80c\xB6\x90Kx\x14a\x06\x0CW\x80c\xBC\x9A@\xC3\x14a\x06<W\x80c\xBD)\xB8\xCD\x14a\x06XW\x80c\xC4gx\xA5\x14a\x06tWa\x020V[\x80c\x86\xC0hV\x14a\x050W\x80c\x9A\xB4\xD6\xFF\x14a\x05LW\x80c\x9F<\xCFe\x14a\x05|W\x80c\xACk\xFB\x03\x14a\x05\xACWa\x020V[\x80cT\x01\xED'\x11a\x01\xBCW\x80ck:\xA7.\x11a\x01\x80W\x80ck:\xA7.\x14a\x04\x8AW\x80cm\x14\xA9\x87\x14a\x04\xA8W\x80cu\xD4\x17:\x14a\x04\xC6W\x80c|\x17#G\x14a\x04\xE2W\x80c\x81\xC0u\x02\x14a\x05\0Wa\x020V[\x80cT\x01\xED'\x14a\x03\xC0W\x80c^Zgu\x14a\x03\xF0W\x80c_\x1F-w\x14a\x04\x0EW\x80cf\xAC\xFE\xFE\x14a\x04*W\x80ci\x7F\xBD\x93\x14a\x04ZWa\x020V[\x80c%PGw\x11a\x02\x03W\x80c%PGw\x14a\x02\xE1W\x80c,\xD9Y@\x14a\x03\x12W\x80c9\x98\xFD\xD3\x14a\x03BW\x80c<\xA5\xA5\xF5\x14a\x03`W\x80cK\xD2n\t\x14a\x03\x90Wa\x020V[\x80c\x04\x91\xB4\x1C\x14a\x024W\x80c\x08s$a\x14a\x02dW\x80c\x1F\x9Bt\xE0\x14a\x02\x95W\x80c \xB6b\x98\x14a\x02\xC5W[__\xFD[a\x02N`\x04\x806\x03\x81\x01\x90a\x02I\x91\x90a9\x0BV[a\x086V[`@Qa\x02[\x91\x90a9NV[`@Q\x80\x91\x03\x90\xF3[a\x02~`\x04\x806\x03\x81\x01\x90a\x02y\x91\x90a9\x91V[a\x08YV[`@Qa\x02\x8C\x92\x91\x90a:oV[`@Q\x80\x91\x03\x90\xF3[a\x02\xAF`\x04\x806\x03\x81\x01\x90a\x02\xAA\x91\x90a:\xD1V[a\x08\xC5V[`@Qa\x02\xBC\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x02\xDF`\x04\x806\x03\x81\x01\x90a\x02\xDA\x91\x90a;\xDEV[a\x08\xE9V[\0[a\x02\xFB`\x04\x806\x03\x81\x01\x90a\x02\xF6\x91\x90a<\xF7V[a\x0B\rV[`@Qa\x03\t\x92\x91\x90a>\x1FV[`@Q\x80\x91\x03\x90\xF3[a\x03,`\x04\x806\x03\x81\x01\x90a\x03'\x91\x90a>TV[a\x0C\xF8V[`@Qa\x039\x91\x90a?\x98V[`@Q\x80\x91\x03\x90\xF3[a\x03Ja\r\xFDV[`@Qa\x03W\x91\x90a?\xD8V[`@Q\x80\x91\x03\x90\xF3[a\x03z`\x04\x806\x03\x81\x01\x90a\x03u\x91\x90a9\x0BV[a\x0E!V[`@Qa\x03\x87\x91\x90a9NV[`@Q\x80\x91\x03\x90\xF3[a\x03\xAA`\x04\x806\x03\x81\x01\x90a\x03\xA5\x91\x90a>TV[a\x0EDV[`@Qa\x03\xB7\x91\x90a9NV[`@Q\x80\x91\x03\x90\xF3[a\x03\xDA`\x04\x806\x03\x81\x01\x90a\x03\xD5\x91\x90a>TV[a\x0EwV[`@Qa\x03\xE7\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x03\xF8a\x0E\x93V[`@Qa\x04\x05\x91\x90a9NV[`@Q\x80\x91\x03\x90\xF3[a\x04(`\x04\x806\x03\x81\x01\x90a\x04#\x91\x90aA9V[a\x0E\x9FV[\0[a\x04D`\x04\x806\x03\x81\x01\x90a\x04?\x91\x90a<\xF7V[a\x12\xEDV[`@Qa\x04Q\x91\x90aA\xC5V[`@Q\x80\x91\x03\x90\xF3[a\x04t`\x04\x806\x03\x81\x01\x90a\x04o\x91\x90a9\x0BV[a\x13\xB4V[`@Qa\x04\x81\x91\x90aBQV[`@Q\x80\x91\x03\x90\xF3[a\x04\x92a\x13\xD1V[`@Qa\x04\x9F\x91\x90aB\x8AV[`@Q\x80\x91\x03\x90\xF3[a\x04\xB0a\x13\xF5V[`@Qa\x04\xBD\x91\x90aB\xB2V[`@Q\x80\x91\x03\x90\xF3[a\x04\xE0`\x04\x806\x03\x81\x01\x90a\x04\xDB\x91\x90aDAV[a\x14\x19V[\0[a\x04\xEAa\x15}V[`@Qa\x04\xF7\x91\x90aD\xBCV[`@Q\x80\x91\x03\x90\xF3[a\x05\x1A`\x04\x806\x03\x81\x01\x90a\x05\x15\x91\x90aD\xFFV[a\x15\x82V[`@Qa\x05'\x91\x90aF\x04V[`@Q\x80\x91\x03\x90\xF3[a\x05J`\x04\x806\x03\x81\x01\x90a\x05E\x91\x90aFGV[a\x17\xAAV[\0[a\x05f`\x04\x806\x03\x81\x01\x90a\x05a\x91\x90a9\x0BV[a\x17\xC0V[`@Qa\x05s\x91\x90aF\x94V[`@Q\x80\x91\x03\x90\xF3[a\x05\x96`\x04\x806\x03\x81\x01\x90a\x05\x91\x91\x90a9\x91V[a\x17\xE0V[`@Qa\x05\xA3\x91\x90aF\xADV[`@Q\x80\x91\x03\x90\xF3[a\x05\xC6`\x04\x806\x03\x81\x01\x90a\x05\xC1\x91\x90aF\xC6V[a\x18(V[`@Qa\x05\xD3\x91\x90aGVV[`@Q\x80\x91\x03\x90\xF3[a\x05\xF6`\x04\x806\x03\x81\x01\x90a\x05\xF1\x91\x90a9\x91V[a\x19\nV[`@Qa\x06\x03\x91\x90aG\xABV[`@Q\x80\x91\x03\x90\xF3[a\x06&`\x04\x806\x03\x81\x01\x90a\x06!\x91\x90a9\x91V[a\x19\xE7V[`@Qa\x063\x91\x90aGVV[`@Q\x80\x91\x03\x90\xF3[a\x06V`\x04\x806\x03\x81\x01\x90a\x06Q\x91\x90aG\xC4V[a\x1A\xB9V[\0[a\x06r`\x04\x806\x03\x81\x01\x90a\x06m\x91\x90aH\x02V[a\x1A\xDAV[\0[a\x06\x8E`\x04\x806\x03\x81\x01\x90a\x06\x89\x91\x90a9\x0BV[a\x1BLV[`@Qa\x06\x9B\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x06\xBE`\x04\x806\x03\x81\x01\x90a\x06\xB9\x91\x90aH_V[a\x1BsV[\0[a\x06\xDA`\x04\x806\x03\x81\x01\x90a\x06\xD5\x91\x90aH\xB9V[a\x1B\x94V[`@Qa\x06\xE7\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x07\n`\x04\x806\x03\x81\x01\x90a\x07\x05\x91\x90aI\tV[a\x1CsV[\0[a\x07&`\x04\x806\x03\x81\x01\x90a\x07!\x91\x90a9\x0BV[a\x1D\xE3V[`@Qa\x073\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x07V`\x04\x806\x03\x81\x01\x90a\x07Q\x91\x90aI\x89V[a\x1E\\V[`@Qa\x07c\x91\x90aF\x94V[`@Q\x80\x91\x03\x90\xF3[a\x07ta\x1EqV[`@Qa\x07\x81\x91\x90aI\xF9V[`@Q\x80\x91\x03\x90\xF3[a\x07\xA4`\x04\x806\x03\x81\x01\x90a\x07\x9F\x91\x90aJ\x12V[a\x1E\x95V[\0[a\x07\xC0`\x04\x806\x03\x81\x01\x90a\x07\xBB\x91\x90aJPV[a\x1E\xABV[`@Qa\x07\xCD\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[a\x07\xF0`\x04\x806\x03\x81\x01\x90a\x07\xEB\x91\x90a>TV[a\x1F\x9AV[`@Qa\x07\xFD\x91\x90aGVV[`@Q\x80\x91\x03\x90\xF3[a\x08 `\x04\x806\x03\x81\x01\x90a\x08\x1B\x91\x90aI\x89V[a \xD3V[`@Qa\x08-\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xF3[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\x08rW_\x80\xFD[\x90_R` _ \x01_\x91P\x91PP\x80_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x80_\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x82V[_\x82a\x08\xD0\x81a!HV[_a\x08\xDB\x85\x85a!\x93V[P\x90P\x80\x92PPP\x92\x91PPV[a\x08\xF1a%\xEDV[\x84a\x08\xFB\x81a!HV[_\x85\x85\x90P\x90P_\x81\x11a\tDW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t;\x90aK4V[`@Q\x80\x91\x03\x90\xFD[\x80\x84\x84\x90P\x14a\t\x89W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\t\x80\x90aK\xC2V[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x89`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P__\x90P[\x82\x81\x10\x15a\x0B\x02W\x85\x85\x82\x81\x81\x10a\t\xC3Wa\t\xC2aK\xE0V[[\x90P` \x02\x01` \x81\x01\x90a\t\xD8\x91\x90aL\rV[\x82\x89\x89\x84\x81\x81\x10a\t\xECWa\t\xEBaK\xE0V[[\x90P` \x02\x015\x81T\x81\x10a\n\x04Wa\n\x03aK\xE0V[[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\nuWa\ntaK\xE0V[[\x90P` \x02\x015\x81T\x81\x10a\n\x8DWa\n\x8CaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88\x88\x85\x81\x81\x10a\n\xCAWa\n\xC9aK\xE0V[[\x90P` \x02\x01` \x81\x01\x90a\n\xDF\x91\x90aL\rV[`@Qa\n\xED\x92\x91\x90aLhV[`@Q\x80\x91\x03\x90\xA2\x80\x80`\x01\x01\x91PPa\t\xA8V[PPPPPPPPPV[``\x80a\x0B\x18a&\xE9V[_\x84\x84\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B6Wa\x0B5a@\x01V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0BdW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P_\x85\x85\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x0B\x85Wa\x0B\x84a@\x01V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x0B\xB3W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x86\x86\x90P\x81\x10\x15a\x0C\xE6W_\x87\x87\x83\x81\x81\x10a\x0B\xDAWa\x0B\xD9aK\xE0V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x0B\xF2\x81a!HV[__a\x0B\xFE\x83\x8Da!\x93V[\x91P\x91P\x80a\x0CBW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0C9\x90aM%V[`@Q\x80\x91\x03\x90\xFD[_a\x0CN\x8C\x85\x85a'yV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0CdWa\x0CcaK\xE0V[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPa\x0C\x98\x84\x82a+\x1EV[\x86\x86\x81Q\x81\x10a\x0C\xABWa\x0C\xAAaK\xE0V[[` \x02` \x01\x01\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPPPPPP\x80\x80`\x01\x01\x91PPa\x0B\xBBV[P\x81\x81\x93P\x93PPP\x94P\x94\x92PPPV[```\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x80` \x02` \x01`@Q\x90\x81\x01`@R\x80\x92\x91\x90\x81\x81R` \x01_\x90[\x82\x82\x10\x15a\r\xF1W\x83\x82\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x81R` \x01\x90`\x01\x01\x90a\r?V[PPPP\x90P\x92\x91PPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[_`\x03_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x91\x90PV[_`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P\x92\x91PPV[__a\x0E\x83\x84\x84a\x1F\x9AV[\x90P\x80`@\x01Q\x91PP\x92\x91PPV[g\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x0E\xA7a%\xEDV[\x81a\x0E\xB1\x81a!HV[_\x82Q\x90P_\x81\x11a\x0E\xF8W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x0E\xEF\x90aM\xB3V[`@Q\x80\x91\x03\x90\xFD[_`\x03_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P_`\x04_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x90P__\x90P[\x83\x81\x10\x15a\x12\xE4W\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x0FrWa\x0FqaK\xE0V[[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\x8BWa\x0F\x8AaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@Qa\x0F\xC2\x91\x90aF\xADV[`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x10\x03Wa\x10\x02aK\xE0V[[` \x02` \x01\x01Q\x81T\x81\x10a\x10\x1CWa\x10\x1BaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_`@Qa\x10U\x92\x91\x90aN\nV[`@Q\x80\x91\x03\x90\xA2\x82`\x01\x84\x80T\x90Pa\x10o\x91\x90aN^V[\x81T\x81\x10a\x10\x80Wa\x10\x7FaK\xE0V[[\x90_R` _ \x01\x83\x87\x83\x81Q\x81\x10a\x10\x9CWa\x10\x9BaK\xE0V[[` \x02` \x01\x01Q\x81T\x81\x10a\x10\xB5Wa\x10\xB4aK\xE0V[[\x90_R` _ \x01_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x90PP\x82\x80T\x80a\x11\x81Wa\x11\x80aN\x91V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01__\x82\x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U_\x82\x01`\x14a\x01\0\n\x81T\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90UPP\x90U\x81`\x01\x83\x80T\x90Pa\x11\xE9\x91\x90aN^V[\x81T\x81\x10a\x11\xFAWa\x11\xF9aK\xE0V[[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82\x87\x83\x81Q\x81\x10a\x126Wa\x125aK\xE0V[[` \x02` \x01\x01Q\x81T\x81\x10a\x12OWa\x12NaK\xE0V[[\x90_R` _ \x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81\x80T\x80a\x12\xA5Wa\x12\xA4aN\x91V[[`\x01\x90\x03\x81\x81\x90_R` _ \x01_a\x01\0\n\x81T\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90U\x90U\x80\x80`\x01\x01\x91PPa\x0F1V[PPPPPPPV[_a\x12\xF6a&\xE9V[____\x90P[\x85\x85\x90P\x81\x10\x15a\x13\xA6W_\x86\x86\x83\x81\x81\x10a\x13\x1CWa\x13\x1BaK\xE0V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x134\x81a!HV[__a\x13@\x83\x8Ca!\x93V[\x91P\x91P\x80a\x13|W_\x91Pa\x13y\x83\x87w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a-M\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[\x95P[_a\x13\x88\x8B\x85\x85a'yV[\x90Pa\x13\x94\x84\x82a+\x1EV[PPPPP\x80\x80`\x01\x01\x91PPa\x12\xFDV[P\x81\x92PPP\x94\x93PPPPV[`\x05` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04`\xFF\x16\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x14!a&\xE9V[a\x14*\x83a-`V[\x15a\x14jW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x14a\x90aO.V[`@Q\x80\x91\x03\x90\xFD[a\x14t\x83\x82a-\x86V[a\x14~\x83\x83a2\x02V[a\x14\x88\x83_a2\x85V[`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPV[` \x81V[``_\x83\x83\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\x15\xA2Wa\x15\xA1a@\x01V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x15\xD0W\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P__\x90P[\x84\x84\x90P\x81\x10\x15a\x17\x9EW_\x85\x85\x83\x81\x81\x10a\x15\xF7Wa\x15\xF6aK\xE0V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x16\x0F\x81a!HV[\x86c\xFF\xFF\xFF\xFF\x16`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x81T\x81\x10a\x16?Wa\x16>aK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11\x15a\x16\xA0W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x16\x97\x90aO\xE2V[`@Q\x80\x91\x03\x90\xFD[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P__\x90P[\x81\x81\x10\x15a\x17\x8EW\x88c\xFF\xFF\xFF\xFF\x16`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83\x85a\x16\xF7\x91\x90aN^V[a\x17\x01\x91\x90aN^V[\x81T\x81\x10a\x17\x12Wa\x17\x11aK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a\x17\x81W`\x01\x81\x83a\x17E\x91\x90aN^V[a\x17O\x91\x90aN^V[\x85\x85\x81Q\x81\x10a\x17bWa\x17aaK\xE0V[[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x17\x8EV[\x80\x80`\x01\x01\x91PPa\x16\xC3V[PPP\x80\x80`\x01\x01\x91PPa\x15\xD8V[P\x80\x91PP\x93\x92PPPV[a\x17\xB2a%\xEDV[a\x17\xBC\x82\x82a2\x85V[PPV[`\x06` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x81V[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x17\xF9W_\x80\xFD[\x90_R` _ \x01_\x91P\x91P\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x180a8PV[`\x02_\x84\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x18hWa\x18gaK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x93\x92PPPV[a\x19\x12a8\x88V[`\x03_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x19;Wa\x19:aK\xE0V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x19\xEFa8PV[`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a\x1A\x18Wa\x1A\x17aK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x92\x91PPV[a\x1A\xC1a%\xEDV[\x81a\x1A\xCB\x81a!HV[a\x1A\xD5\x83\x83a2\x02V[PPPV[a\x1A\xE2a&\xE9V[__\x90P[\x82\x82\x90P\x81\x10\x15a\x1BFW_\x83\x83\x83\x81\x81\x10a\x1B\x06Wa\x1B\x05aK\xE0V[[\x90P\x015`\xF8\x1C`\xF8\x1B`\xF8\x1C\x90Pa\x1B\x1E\x81a!HV[_a\x1B*\x86\x83_a'yV[\x90Pa\x1B6\x82\x82a+\x1EV[PPP\x80\x80`\x01\x01\x91PPa\x1A\xE7V[PPPPV[_` R\x80_R`@_ _\x91PT\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[a\x1B{a%\xEDV[\x81a\x1B\x85\x81a!HV[a\x1B\x8F\x83\x83a-\x86V[PPPV[__`\x01_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x1B\xBFWa\x1B\xBEaK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90Pa\x1Cd\x81\x85a2\xFEV[\x80`@\x01Q\x91PP\x93\x92PPPV[a\x1C{a&\xE9V[a\x1C\x84\x84a-`V[\x15a\x1C\xC4W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a\x1C\xBB\x90aO.V[`@Q\x80\x91\x03\x90\xFD[a\x1C\xCE\x84\x82a-\x86V[a\x1C\xD8\x84\x84a2\x02V[a\x1C\xE3\x84`\x01a2\x85V[a\x1C\xED\x84\x83a3\xBAV[`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPPPPPV[_`\x01_\x83`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x80_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90Pa\x1E!\x91\x90aN^V[\x81T\x81\x10a\x1E2Wa\x1E1aK\xE0V[[\x90_R` _ \x01_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x91\x90PV[_a\x1Eh\x84\x84\x84a4WV[\x90P\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x1E\x9Da%\xEDV[a\x1E\xA7\x82\x82a3\xBAV[PPV[__`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x83\x81T\x81\x10a\x1E\xE5Wa\x1E\xE4aK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90Pa\x1F\x8A\x81\x86a2\xFEV[\x80`@\x01Q\x91PP\x94\x93PPPPV[a\x1F\xA2a8PV[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90Pa\x1F\xD7a8PV[_\x82\x03a\x1F\xE8W\x80\x92PPPa \xCDV[`\x02_\x86\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a \x1B\x91\x90aN^V[\x81T\x81\x10a ,Wa +aK\xE0V[[\x90_R` _ \x01`@Q\x80``\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x04\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x90P\x80\x92PPP[\x92\x91PPV[_`\x02_\x85\x81R` \x01\x90\x81R` \x01_ _\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ a!\x05\x85\x85\x85a4WV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a!\x1CWa!\x1BaK\xE0V[[\x90_R` _ \x01_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90P\x93\x92PPPV[a!Q\x81a-`V[a!\x90W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a!\x87\x90aPpV[`@Q\x80\x91\x03\x90\xFD[PV[____a!\xA0\x86a\x0E!V[\x90Pa!\xAAa8\x88V[```\x01\x80\x81\x11\x15a!\xBFWa!\xBEaA\xDEV[[`\x05_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04`\xFF\x16`\x01\x81\x11\x15a!\xF5Wa!\xF4aA\xDEV[[\x03a#pWa\"\x04\x88\x88a5nV[\x90P__\x90P[\x83\x81\x10\x15a#jW`\x03_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a\"<Wa\";aK\xE0V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x92P_\x82\x82\x81Q\x81\x10a\"\xF6Wa\"\xF5aK\xE0V[[` \x02` \x01\x01Q\x11\x15a#]Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10a#3Wa#2aK\xE0V[[` \x02` \x01\x01Qa#E\x91\x90aP\x8EV[a#O\x91\x90aP\xFCV[\x85a#Z\x91\x90aQ,V[\x94P[\x80\x80`\x01\x01\x91PPa\"\x0BV[Pa%\x8BV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x90\x04\x13G\x88`\x04_\x8C`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x83c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a#\xE1\x92\x91\x90aRrV[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a#\xFBW=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a$#\x91\x90aSIV[\x90P__\x90P[\x83\x81\x10\x15a%\x89W`\x03_\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x81\x81T\x81\x10a$[Wa$ZaK\xE0V[[\x90_R` _ \x01`@Q\x80`@\x01`@R\x90\x81_\x82\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_\x82\x01`\x14\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP\x92P_\x82\x82\x81Q\x81\x10a%\x15Wa%\x14aK\xE0V[[` \x02` \x01\x01Q\x11\x15a%|Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x83\x81Q\x81\x10a%RWa%QaK\xE0V[[` \x02` \x01\x01Qa%d\x91\x90aP\x8EV[a%n\x91\x90aP\xFCV[\x85a%y\x91\x90aQ,V[\x94P[\x80\x80`\x01\x01\x91PPa$*V[P[___\x8A`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10\x15\x90P\x84\x81\x96P\x96PPPPPP\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a&VW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a&z\x91\x90aS\xA4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a&\xE7W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a&\xDE\x90aTeV[`@Q\x80\x91\x03\x90\xFD[V[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14a'wW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a'n\x90aU\x19V[`@Q\x80\x91\x03\x90\xFD[V[___`\x02_\x87\x81R` \x01\x90\x81R` \x01_ _\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x03a(\xB3W`\x02_\x87\x81R` \x01\x90\x81R` \x01_ _\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x86k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPPa*\xCEV[_`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a(\xE7\x91\x90aN^V[\x81T\x81\x10a(\xF8Wa(\xF7aK\xE0V[[\x90_R` _ \x01\x90P\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92P\x84k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a)NW_\x93PPPPa+\x17V[Cc\xFF\xFF\xFF\xFF\x16\x81_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a)\xAAW\x84\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa*\xCCV[C\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x87k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[P[\x85\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x86\x86`@Qa+\0\x92\x91\x90aU7V[`@Q\x80\x91\x03\x90\xA2a+\x12\x82\x85a7\xE5V[\x92PPP[\x93\x92PPPV[__`\x01_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_`\x01_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a+b\x91\x90aN^V[\x81T\x81\x10a+sWa+raK\xE0V[[\x90_R` _ \x01\x90P_\x84\x03a+\xA9W\x80_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x92PPPa-GV[_a+\xCF\x82_\x01`\x08\x90T\x90a\x01\0\n\x90\x04k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x86a8\x16V[\x90PCc\xFF\xFF\xFF\xFF\x16\x82_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x03a,-W\x80\x82_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPa-@V[C\x82_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`\x01_\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `@Q\x80``\x01`@R\x80Cc\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x04a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP`@\x82\x01Q\x81_\x01`\x08a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP[\x80\x93PPPP[\x92\x91PPV[_\x81`\xFF\x16`\x01\x90\x1B\x83\x17\x90P\x92\x91PPV[__`\x01_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x14\x15\x90P\x91\x90PV[_\x81Q\x11a-\xC9W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a-\xC0\x90aU\xCEV[`@Q\x80\x91\x03\x90\xFD[_\x81Q\x90P_`\x03_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P` `\xFF\x16\x82\x82a-\xFD\x91\x90aU\xECV[\x11\x15a.>W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a.5\x90aV\xB5V[`@Q\x80\x91\x03\x90\xFD[__\x90P[\x82\x81\x10\x15a1\xFBW__\x90P[\x81\x83a.\\\x91\x90aU\xECV[\x81\x10\x15a/MW\x84\x82\x81Q\x81\x10a.vWa.uaK\xE0V[[` \x02` \x01\x01Q_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a.\xC0Wa.\xBFaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03a/@W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/7\x90aWCV[`@Q\x80\x91\x03\x90\xFD[\x80\x80`\x01\x01\x91PPa.PV[P_\x84\x82\x81Q\x81\x10a/bWa/aaK\xE0V[[` \x02` \x01\x01Q` \x01Qk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11a/\xBCW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a/\xB3\x90aW\xF7V[`@Q\x80\x91\x03\x90\xFD[`\x03_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x84\x82\x81Q\x81\x10a/\xE6Wa/\xE5aK\xE0V[[` \x02` \x01\x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91P_\x82\x01Q\x81_\x01_a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP` \x82\x01Q\x81_\x01`\x14a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UPPP`\x04_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x84\x82\x81Q\x81\x10a0\xB6Wa0\xB5aK\xE0V[[` \x02` \x01\x01Q_\x01Q\x90\x80`\x01\x81T\x01\x80\x82U\x80\x91PP`\x01\x90\x03\x90_R` _ \x01_\x90\x91\x90\x91\x90\x91a\x01\0\n\x81T\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x84`\xFF\x16\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x85\x83\x81Q\x81\x10a1VWa1UaK\xE0V[[` \x02` \x01\x01Q_\x01Q`@Qa1n\x91\x90aF\xADV[`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a1\xAEWa1\xADaK\xE0V[[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a1\xCCWa1\xCBaK\xE0V[[` \x02` \x01\x01Q` \x01Q`@Qa1\xE6\x92\x91\x90aLhV[`@Q\x80\x91\x03\x90\xA2\x80\x80`\x01\x01\x91PPa.CV[PPPPPV[\x80__\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x81`\xFF\x16\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x82`@Qa2y\x91\x90a;\x0FV[`@Q\x80\x91\x03\x90\xA2PPV[\x80`\x05_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81`\xFF\x02\x19\x16\x90\x83`\x01\x81\x11\x15a2\xBEWa2\xBDaA\xDEV[[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa2\xF2\x91\x90aBQV[`@Q\x80\x91\x03\x90\xA1PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a3PW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3G\x90aX\xABV[`@Q\x80\x91\x03\x90\xFD[_\x82` \x01Qc\xFF\xFF\xFF\xFF\x16\x14\x80a3wWP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a3\xB6W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a3\xAD\x90aY\x85V[`@Q\x80\x91\x03\x90\xFD[PPV[_`\x06_\x84`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16\x90P\x81`\x06_\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _a\x01\0\n\x81T\x81c\xFF\xFF\xFF\xFF\x02\x19\x16\x90\x83c\xFF\xFF\xFF\xFF\x16\x02\x17\x90UP\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x81\x83`@Qa4J\x92\x91\x90aY\xA3V[`@Q\x80\x91\x03\x90\xA1PPPV[__`\x02_\x86\x81R` \x01\x90\x81R` \x01_ _\x85`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x80T\x90P\x90P_\x81\x90P[_\x81\x11\x15a5+W\x83c\xFF\xFF\xFF\xFF\x16`\x02_\x88\x81R` \x01\x90\x81R` \x01_ _\x87`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ `\x01\x83a4\xCC\x91\x90aN^V[\x81T\x81\x10a4\xDDWa4\xDCaK\xE0V[[\x90_R` _ \x01_\x01_\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16\x11a5\x18W`\x01\x81a5\x0F\x91\x90aN^V[\x92PPPa5gV[\x80\x80a5#\x90aY\xCAV[\x91PPa4\x8AV[P`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01a5^\x90aZ\xD3V[`@Q\x80\x91\x03\x90\xFD[\x93\x92PPPV[``_`\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a5\x8CWa5\x8Ba@\x01V[[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a5\xBAW\x81` \x01` \x82\x02\x806\x837\x80\x82\x01\x91PP\x90P[P\x90P\x82\x81_\x81Q\x81\x10a5\xD1Wa5\xD0aK\xE0V[[` \x02` \x01\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x90\x81s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RPP_`\x06_\x86`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ _\x90T\x90a\x01\0\n\x90\x04c\xFF\xFF\xFF\xFF\x16c\xFF\xFF\xFF\xFF\x16Ba6D\x91\x90aU\xECV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c\xCA\x8A\xA7\xC7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a6\xB0W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a6\xD4\x91\x90aS\xA4V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16c+\xAB,J`@Q\x80`@\x01`@R\x80\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x89`\xFF\x16c\xFF\xFF\xFF\xFF\x16\x81RP\x85`\x04_\x8B`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x86`@Q\x85c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01a7{\x94\x93\x92\x91\x90a[\xC5V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a7\x95W=__>=_\xFD[PPPP`@Q=_\x82>=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a7\xBD\x91\x90a\\\xF4V[\x90P\x80_\x81Q\x81\x10a7\xD2Wa7\xD1aK\xE0V[[` \x02` \x01\x01Q\x93PPPP\x92\x91PPV[_\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a8\x0E\x91\x90a]DV[\x90P\x92\x91PPV[__\x82\x12\x15a8;W\x81a8)\x90a]\x84V[\x83a84\x91\x90a]\xCAV[\x90Pa8JV[\x81\x83a8G\x91\x90aQ,V[\x90P[\x92\x91PPV[`@Q\x80``\x01`@R\x80_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_c\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[`@Q\x80`@\x01`@R\x80_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81RP\x90V[_`@Q\x90P\x90V[__\xFD[__\xFD[_`\xFF\x82\x16\x90P\x91\x90PV[a8\xEA\x81a8\xD5V[\x81\x14a8\xF4W__\xFD[PV[_\x815\x90Pa9\x05\x81a8\xE1V[\x92\x91PPV[_` \x82\x84\x03\x12\x15a9 Wa9\x1Fa8\xCDV[[_a9-\x84\x82\x85\x01a8\xF7V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[a9H\x81a96V[\x82RPPV[_` \x82\x01\x90Pa9a_\x83\x01\x84a9?V[\x92\x91PPV[a9p\x81a96V[\x81\x14a9zW__\xFD[PV[_\x815\x90Pa9\x8B\x81a9gV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a9\xA7Wa9\xA6a8\xCDV[[_a9\xB4\x85\x82\x86\x01a8\xF7V[\x92PP` a9\xC5\x85\x82\x86\x01a9}V[\x91PP\x92P\x92\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_a:\x11a:\x0Ca:\x07\x84a9\xCFV[a9\xEEV[a9\xCFV[\x90P\x91\x90PV[_a:\"\x82a9\xF7V[\x90P\x91\x90PV[_a:3\x82a:\x18V[\x90P\x91\x90PV[a:C\x81a:)V[\x82RPPV[_k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a:i\x81a:IV[\x82RPPV[_`@\x82\x01\x90Pa:\x82_\x83\x01\x85a::V[a:\x8F` \x83\x01\x84a:`V[\x93\x92PPPV[_a:\xA0\x82a9\xCFV[\x90P\x91\x90PV[a:\xB0\x81a:\x96V[\x81\x14a:\xBAW__\xFD[PV[_\x815\x90Pa:\xCB\x81a:\xA7V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15a:\xE7Wa:\xE6a8\xCDV[[_a:\xF4\x85\x82\x86\x01a8\xF7V[\x92PP` a;\x05\x85\x82\x86\x01a:\xBDV[\x91PP\x92P\x92\x90PV[_` \x82\x01\x90Pa;\"_\x83\x01\x84a:`V[\x92\x91PPV[__\xFD[__\xFD[__\xFD[__\x83`\x1F\x84\x01\x12a;IWa;Ha;(V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;fWa;ea;,V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a;\x82Wa;\x81a;0V[[\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a;\x9EWa;\x9Da;(V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a;\xBBWa;\xBAa;,V[[` \x83\x01\x91P\x83` \x82\x02\x83\x01\x11\x15a;\xD7Wa;\xD6a;0V[[\x92P\x92\x90PV[_____``\x86\x88\x03\x12\x15a;\xF7Wa;\xF6a8\xCDV[[_a<\x04\x88\x82\x89\x01a8\xF7V[\x95PP` \x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<%Wa<$a8\xD1V[[a<1\x88\x82\x89\x01a;4V[\x94P\x94PP`@\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<TWa<Sa8\xD1V[[a<`\x88\x82\x89\x01a;\x89V[\x92P\x92PP\x92\x95P\x92\x95\x90\x93PV[_\x81\x90P\x91\x90PV[a<\x81\x81a<oV[\x81\x14a<\x8BW__\xFD[PV[_\x815\x90Pa<\x9C\x81a<xV[\x92\x91PPV[__\x83`\x1F\x84\x01\x12a<\xB7Wa<\xB6a;(V[[\x825\x90Pg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a<\xD4Wa<\xD3a;,V[[` \x83\x01\x91P\x83`\x01\x82\x02\x83\x01\x11\x15a<\xF0Wa<\xEFa;0V[[\x92P\x92\x90PV[____``\x85\x87\x03\x12\x15a=\x0FWa=\x0Ea8\xCDV[[_a=\x1C\x87\x82\x88\x01a:\xBDV[\x94PP` a=-\x87\x82\x88\x01a<\x8EV[\x93PP`@\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a=NWa=Ma8\xD1V[[a=Z\x87\x82\x88\x01a<\xA2V[\x92P\x92PP\x92\x95\x91\x94P\x92PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[a=\x9A\x81a:IV[\x82RPPV[_a=\xAB\x83\x83a=\x91V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a=\xCD\x82a=hV[a=\xD7\x81\x85a=rV[\x93Pa=\xE2\x83a=\x82V[\x80_[\x83\x81\x10\x15a>\x12W\x81Qa=\xF9\x88\x82a=\xA0V[\x97Pa>\x04\x83a=\xB7V[\x92PP`\x01\x81\x01\x90Pa=\xE5V[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90P\x81\x81\x03_\x83\x01Ra>7\x81\x85a=\xC3V[\x90P\x81\x81\x03` \x83\x01Ra>K\x81\x84a=\xC3V[\x90P\x93\x92PPPV[__`@\x83\x85\x03\x12\x15a>jWa>ia8\xCDV[[_a>w\x85\x82\x86\x01a<\x8EV[\x92PP` a>\x88\x85\x82\x86\x01a8\xF7V[\x91PP\x92P\x92\x90PV[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_c\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[a>\xD3\x81a>\xBBV[\x82RPPV[``\x82\x01_\x82\x01Qa>\xED_\x85\x01\x82a>\xCAV[P` \x82\x01Qa?\0` \x85\x01\x82a>\xCAV[P`@\x82\x01Qa?\x13`@\x85\x01\x82a=\x91V[PPPPV[_a?$\x83\x83a>\xD9V[``\x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a?F\x82a>\x92V[a?P\x81\x85a>\x9CV[\x93Pa?[\x83a>\xACV[\x80_[\x83\x81\x10\x15a?\x8BW\x81Qa?r\x88\x82a?\x19V[\x97Pa?}\x83a?0V[\x92PP`\x01\x81\x01\x90Pa?^V[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01Ra?\xB0\x81\x84a?<V[\x90P\x92\x91PPV[_a?\xC2\x82a:\x18V[\x90P\x91\x90PV[a?\xD2\x81a?\xB8V[\x82RPPV[_` \x82\x01\x90Pa?\xEB_\x83\x01\x84a?\xC9V[\x92\x91PPV[_`\x1F\x19`\x1F\x83\x01\x16\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[a@7\x82a?\xF1V[\x81\x01\x81\x81\x10g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x17\x15a@VWa@Ua@\x01V[[\x80`@RPPPV[_a@ha8\xC4V[\x90Pa@t\x82\x82a@.V[\x91\x90PV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a@\x93Wa@\x92a@\x01V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a@\xB6a@\xB1\x84a@yV[a@_V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a@\xD9Wa@\xD8a;0V[[\x83[\x81\x81\x10\x15aA\x02W\x80a@\xEE\x88\x82a9}V[\x84R` \x84\x01\x93PP` \x81\x01\x90Pa@\xDBV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aA WaA\x1Fa;(V[[\x815aA0\x84\x82` \x86\x01a@\xA4V[\x91PP\x92\x91PPV[__`@\x83\x85\x03\x12\x15aAOWaANa8\xCDV[[_aA\\\x85\x82\x86\x01a8\xF7V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aA}WaA|a8\xD1V[[aA\x89\x85\x82\x86\x01aA\x0CV[\x91PP\x92P\x92\x90PV[_w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[aA\xBF\x81aA\x93V[\x82RPPV[_` \x82\x01\x90PaA\xD8_\x83\x01\x84aA\xB6V[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`!`\x04R`$_\xFD[`\x02\x81\x10aB\x1CWaB\x1BaA\xDEV[[PV[_\x81\x90PaB,\x82aB\x0BV[\x91\x90PV[_aB;\x82aB\x1FV[\x90P\x91\x90PV[aBK\x81aB1V[\x82RPPV[_` \x82\x01\x90PaBd_\x83\x01\x84aBBV[\x92\x91PPV[_aBt\x82a:\x18V[\x90P\x91\x90PV[aB\x84\x81aBjV[\x82RPPV[_` \x82\x01\x90PaB\x9D_\x83\x01\x84aB{V[\x92\x91PPV[aB\xAC\x81a:\x96V[\x82RPPV[_` \x82\x01\x90PaB\xC5_\x83\x01\x84aB\xA3V[\x92\x91PPV[aB\xD4\x81a:IV[\x81\x14aB\xDEW__\xFD[PV[_\x815\x90PaB\xEF\x81aB\xCBV[\x92\x91PPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15aC\x0FWaC\x0Ea@\x01V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[__\xFD[_aC.\x82a:\x96V[\x90P\x91\x90PV[aC>\x81aC$V[\x81\x14aCHW__\xFD[PV[_\x815\x90PaCY\x81aC5V[\x92\x91PPV[_`@\x82\x84\x03\x12\x15aCtWaCsaC V[[aC~`@a@_V[\x90P_aC\x8D\x84\x82\x85\x01aCKV[_\x83\x01RP` aC\xA0\x84\x82\x85\x01aB\xE1V[` \x83\x01RP\x92\x91PPV[_aC\xBEaC\xB9\x84aB\xF5V[a@_V[\x90P\x80\x83\x82R` \x82\x01\x90P`@\x84\x02\x83\x01\x85\x81\x11\x15aC\xE1WaC\xE0a;0V[[\x83[\x81\x81\x10\x15aD\nW\x80aC\xF6\x88\x82aC_V[\x84R` \x84\x01\x93PP`@\x81\x01\x90PaC\xE3V[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aD(WaD'a;(V[[\x815aD8\x84\x82` \x86\x01aC\xACV[\x91PP\x92\x91PPV[___``\x84\x86\x03\x12\x15aDXWaDWa8\xCDV[[_aDe\x86\x82\x87\x01a8\xF7V[\x93PP` aDv\x86\x82\x87\x01aB\xE1V[\x92PP`@\x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aD\x97WaD\x96a8\xD1V[[aD\xA3\x86\x82\x87\x01aD\x14V[\x91PP\x92P\x92P\x92V[aD\xB6\x81a8\xD5V[\x82RPPV[_` \x82\x01\x90PaD\xCF_\x83\x01\x84aD\xADV[\x92\x91PPV[aD\xDE\x81a>\xBBV[\x81\x14aD\xE8W__\xFD[PV[_\x815\x90PaD\xF9\x81aD\xD5V[\x92\x91PPV[___`@\x84\x86\x03\x12\x15aE\x16WaE\x15a8\xCDV[[_aE#\x86\x82\x87\x01aD\xEBV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aEDWaECa8\xD1V[[aEP\x86\x82\x87\x01a<\xA2V[\x92P\x92PP\x92P\x92P\x92V[_\x81Q\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_aE\x90\x83\x83a>\xCAV[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_aE\xB2\x82aE\\V[aE\xBC\x81\x85aEfV[\x93PaE\xC7\x83aEvV[\x80_[\x83\x81\x10\x15aE\xF7W\x81QaE\xDE\x88\x82aE\x85V[\x97PaE\xE9\x83aE\x9CV[\x92PP`\x01\x81\x01\x90PaE\xCAV[P\x85\x93PPPP\x92\x91PPV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaF\x1C\x81\x84aE\xA8V[\x90P\x92\x91PPV[`\x02\x81\x10aF0W__\xFD[PV[_\x815\x90PaFA\x81aF$V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aF]WaF\\a8\xCDV[[_aFj\x85\x82\x86\x01a8\xF7V[\x92PP` aF{\x85\x82\x86\x01aF3V[\x91PP\x92P\x92\x90PV[aF\x8E\x81a>\xBBV[\x82RPPV[_` \x82\x01\x90PaF\xA7_\x83\x01\x84aF\x85V[\x92\x91PPV[_` \x82\x01\x90PaF\xC0_\x83\x01\x84a::V[\x92\x91PPV[___``\x84\x86\x03\x12\x15aF\xDDWaF\xDCa8\xCDV[[_aF\xEA\x86\x82\x87\x01a8\xF7V[\x93PP` aF\xFB\x86\x82\x87\x01a<\x8EV[\x92PP`@aG\x0C\x86\x82\x87\x01a9}V[\x91PP\x92P\x92P\x92V[``\x82\x01_\x82\x01QaG*_\x85\x01\x82a>\xCAV[P` \x82\x01QaG=` \x85\x01\x82a>\xCAV[P`@\x82\x01QaGP`@\x85\x01\x82a=\x91V[PPPPV[_``\x82\x01\x90PaGi_\x83\x01\x84aG\x16V[\x92\x91PPV[aGx\x81a:)V[\x82RPPV[`@\x82\x01_\x82\x01QaG\x92_\x85\x01\x82aGoV[P` \x82\x01QaG\xA5` \x85\x01\x82a=\x91V[PPPPV[_`@\x82\x01\x90PaG\xBE_\x83\x01\x84aG~V[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aG\xDAWaG\xD9a8\xCDV[[_aG\xE7\x85\x82\x86\x01a8\xF7V[\x92PP` aG\xF8\x85\x82\x86\x01aB\xE1V[\x91PP\x92P\x92\x90PV[___`@\x84\x86\x03\x12\x15aH\x19WaH\x18a8\xCDV[[_aH&\x86\x82\x87\x01a<\x8EV[\x93PP` \x84\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aHGWaHFa8\xD1V[[aHS\x86\x82\x87\x01a<\xA2V[\x92P\x92PP\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15aHuWaHta8\xCDV[[_aH\x82\x85\x82\x86\x01a8\xF7V[\x92PP` \x83\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aH\xA3WaH\xA2a8\xD1V[[aH\xAF\x85\x82\x86\x01aD\x14V[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15aH\xD0WaH\xCFa8\xCDV[[_aH\xDD\x86\x82\x87\x01a8\xF7V[\x93PP` aH\xEE\x86\x82\x87\x01aD\xEBV[\x92PP`@aH\xFF\x86\x82\x87\x01a9}V[\x91PP\x92P\x92P\x92V[____`\x80\x85\x87\x03\x12\x15aI!WaI a8\xCDV[[_aI.\x87\x82\x88\x01a8\xF7V[\x94PP` aI?\x87\x82\x88\x01aB\xE1V[\x93PP`@aIP\x87\x82\x88\x01aD\xEBV[\x92PP``\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aIqWaIpa8\xD1V[[aI}\x87\x82\x88\x01aD\x14V[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15aI\xA0WaI\x9Fa8\xCDV[[_aI\xAD\x86\x82\x87\x01a<\x8EV[\x93PP` aI\xBE\x86\x82\x87\x01a8\xF7V[\x92PP`@aI\xCF\x86\x82\x87\x01aD\xEBV[\x91PP\x92P\x92P\x92V[_aI\xE3\x82a:\x18V[\x90P\x91\x90PV[aI\xF3\x81aI\xD9V[\x82RPPV[_` \x82\x01\x90PaJ\x0C_\x83\x01\x84aI\xEAV[\x92\x91PPV[__`@\x83\x85\x03\x12\x15aJ(WaJ'a8\xCDV[[_aJ5\x85\x82\x86\x01a8\xF7V[\x92PP` aJF\x85\x82\x86\x01aD\xEBV[\x91PP\x92P\x92\x90PV[____`\x80\x85\x87\x03\x12\x15aJhWaJga8\xCDV[[_aJu\x87\x82\x88\x01a8\xF7V[\x94PP` aJ\x86\x87\x82\x88\x01aD\xEBV[\x93PP`@aJ\x97\x87\x82\x88\x01a<\x8EV[\x92PP``aJ\xA8\x87\x82\x88\x01a9}V[\x91PP\x92\x95\x91\x94P\x92PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[\x7FStakeRegistry.modifyStrategyPara_\x82\x01R\x7Fms: no strategy indices provided` \x82\x01RPV[_aK\x1E`@\x83aJ\xB4V[\x91PaK)\x82aJ\xC4V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaKK\x81aK\x12V[\x90P\x91\x90PV[\x7FStakeRegistry.modifyStrategyPara_\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0` \x82\x01RPV[_aK\xAC`9\x83aJ\xB4V[\x91PaK\xB7\x82aKRV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaK\xD9\x81aK\xA0V[\x90P\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15aL\"WaL!a8\xCDV[[_aL/\x84\x82\x85\x01aB\xE1V[\x91PP\x92\x91PPV[_aLRaLMaLH\x84a:IV[a9\xEEV[a96V[\x90P\x91\x90PV[aLb\x81aL8V[\x82RPPV[_`@\x82\x01\x90PaL{_\x83\x01\x85a::V[aL\x88` \x83\x01\x84aLYV[\x93\x92PPPV[\x7FStakeRegistry.registerOperator: _\x82\x01R\x7FOperator does not meet minimum s` \x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`@\x82\x01RPV[_aM\x0F`[\x83aJ\xB4V[\x91PaM\x1A\x82aL\x8FV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaM<\x81aM\x03V[\x90P\x91\x90PV[\x7FStakeRegistry.removeStrategies: _\x82\x01R\x7Fno indices to remove provided\0\0\0` \x82\x01RPV[_aM\x9D`=\x83aJ\xB4V[\x91PaM\xA8\x82aMCV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaM\xCA\x81aM\x91V[\x90P\x91\x90PV[_\x81\x90P\x91\x90PV[_aM\xF4aM\xEFaM\xEA\x84aM\xD1V[a9\xEEV[a96V[\x90P\x91\x90PV[aN\x04\x81aM\xDAV[\x82RPPV[_`@\x82\x01\x90PaN\x1D_\x83\x01\x85a::V[aN*` \x83\x01\x84aM\xFBV[\x93\x92PPPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[_aNh\x82a96V[\x91PaNs\x83a96V[\x92P\x82\x82\x03\x90P\x81\x81\x11\x15aN\x8BWaN\x8AaN1V[[\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`1`\x04R`$_\xFD[\x7FStakeRegistry.initializeQuorum: _\x82\x01R\x7Fquorum already exists\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aO\x18`5\x83aJ\xB4V[\x91PaO#\x82aN\xBEV[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaOE\x81aO\x0CV[\x90P\x91\x90PV[\x7FStakeRegistry.getTotalStakeIndic_\x82\x01R\x7FesAtBlockNumber: quorum has no s` \x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`@\x82\x01RPV[_aO\xCC`[\x83aJ\xB4V[\x91PaO\xD7\x82aOLV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaO\xF9\x81aO\xC0V[\x90P\x91\x90PV[\x7FStakeRegistry.quorumExists: quor_\x82\x01R\x7Fum does not exist\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0` \x82\x01RPV[_aPZ`1\x83aJ\xB4V[\x91PaPe\x82aP\0V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaP\x87\x81aPNV[\x90P\x91\x90PV[_aP\x98\x82a96V[\x91PaP\xA3\x83a96V[\x92P\x82\x82\x02aP\xB1\x81a96V[\x91P\x82\x82\x04\x84\x14\x83\x15\x17aP\xC8WaP\xC7aN1V[[P\x92\x91PPV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_aQ\x06\x82a96V[\x91PaQ\x11\x83a96V[\x92P\x82aQ!WaQ aP\xCFV[[\x82\x82\x04\x90P\x92\x91PPV[_aQ6\x82a:IV[\x91PaQA\x83a:IV[\x92P\x82\x82\x01\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aQeWaQdaN1V[[\x92\x91PPV[_\x81T\x90P\x91\x90PV[_\x82\x82R` \x82\x01\x90P\x92\x91PPV[_\x81\x90P\x81_R` _ \x90P\x91\x90PV[_aQ\xA2\x83\x83aGoV[` \x83\x01\x90P\x92\x91PPV[_\x81_\x1C\x90P\x91\x90PV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x90P\x91\x90PV[_aQ\xEAaQ\xE5\x83aQ\xAEV[aQ\xB9V[\x90P\x91\x90PV[_aQ\xFC\x82TaQ\xD8V[\x90P\x91\x90PV[_`\x01\x82\x01\x90P\x91\x90PV[_aR\x19\x82aQkV[aR#\x81\x85aQuV[\x93PaR.\x83aQ\x85V[\x80_[\x83\x81\x10\x15aReWaRB\x82aQ\xF1V[aRL\x88\x82aQ\x97V[\x97PaRW\x83aR\x03V[\x92PP`\x01\x81\x01\x90PaR1V[P\x85\x93PPPP\x92\x91PPV[_`@\x82\x01\x90PaR\x85_\x83\x01\x85aB\xA3V[\x81\x81\x03` \x83\x01RaR\x97\x81\x84aR\x0FV[\x90P\x93\x92PPPV[_\x81Q\x90PaR\xAE\x81a9gV[\x92\x91PPV[_aR\xC6aR\xC1\x84a@yV[a@_V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15aR\xE9WaR\xE8a;0V[[\x83[\x81\x81\x10\x15aS\x12W\x80aR\xFE\x88\x82aR\xA0V[\x84R` \x84\x01\x93PP` \x81\x01\x90PaR\xEBV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12aS0WaS/a;(V[[\x81QaS@\x84\x82` \x86\x01aR\xB4V[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15aS^WaS]a8\xCDV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15aS{WaSza8\xD1V[[aS\x87\x84\x82\x85\x01aS\x1CV[\x91PP\x92\x91PPV[_\x81Q\x90PaS\x9E\x81a:\xA7V[\x92\x91PPV[_` \x82\x84\x03\x12\x15aS\xB9WaS\xB8a8\xCDV[[_aS\xC6\x84\x82\x85\x01aS\x90V[\x91PP\x92\x91PPV[\x7FStakeRegistry.onlyCoordinatorOwn_\x82\x01R\x7Fer: caller is not the owner of t` \x82\x01R\x7Fhe registryCoordinator\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aTO`V\x83aJ\xB4V[\x91PaTZ\x82aS\xCFV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaT|\x81aTCV[\x90P\x91\x90PV[\x7FStakeRegistry.onlyRegistryCoordi_\x82\x01R\x7Fnator: caller is not the Registr` \x82\x01R\x7FyCoordinator\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aU\x03`L\x83aJ\xB4V[\x91PaU\x0E\x82aT\x83V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaU0\x81aT\xF7V[\x90P\x91\x90PV[_`@\x82\x01\x90PaUJ_\x83\x01\x85aD\xADV[aUW` \x83\x01\x84a:`V[\x93\x92PPPV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0` \x82\x01RPV[_aU\xB8`8\x83aJ\xB4V[\x91PaU\xC3\x82aU^V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaU\xE5\x81aU\xACV[\x90P\x91\x90PV[_aU\xF6\x82a96V[\x91PaV\x01\x83a96V[\x92P\x82\x82\x01\x90P\x80\x82\x11\x15aV\x19WaV\x18aN1V[[\x92\x91PPV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L` \x82\x01R\x7FENGTH\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aV\x9F`E\x83aJ\xB4V[\x91PaV\xAA\x82aV\x1FV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaV\xCC\x81aV\x93V[\x90P\x91\x90PV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: cannot add same strategy 2x\0\0\0` \x82\x01RPV[_aW-`=\x83aJ\xB4V[\x91PaW8\x82aV\xD3V[`@\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaWZ\x81aW!V[\x90P\x91\x90PV[\x7FStakeRegistry._addStrategyParams_\x82\x01R\x7F: cannot add strategy with zero ` \x82\x01R\x7Fweight\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aW\xE1`F\x83aJ\xB4V[\x91PaW\xEC\x82aWaV[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaX\x0E\x81aW\xD5V[\x90P\x91\x90PV[\x7FStakeRegistry._validateStakeUpda_\x82\x01R\x7FteAtBlockNumber: stakeUpdate is ` \x82\x01R\x7Ffrom after blockNumber\0\0\0\0\0\0\0\0\0\0`@\x82\x01RPV[_aX\x95`V\x83aJ\xB4V[\x91PaX\xA0\x82aX\x15V[``\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaX\xC2\x81aX\x89V[\x90P\x91\x90PV[\x7FStakeRegistry._validateStakeUpda_\x82\x01R\x7FteAtBlockNumber: there is a newe` \x82\x01R\x7Fr stakeUpdate available before b`@\x82\x01R\x7FlockNumber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0``\x82\x01RPV[_aYo`j\x83aJ\xB4V[\x91PaYz\x82aX\xC9V[`\x80\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaY\x9C\x81aYcV[\x90P\x91\x90PV[_`@\x82\x01\x90PaY\xB6_\x83\x01\x85aF\x85V[aY\xC3` \x83\x01\x84aF\x85V[\x93\x92PPPV[_aY\xD4\x82a96V[\x91P_\x82\x03aY\xE6WaY\xE5aN1V[[`\x01\x82\x03\x90P\x91\x90PV[\x7FStakeRegistry._getStakeUpdateInd_\x82\x01R\x7FexForOperatorAtBlockNumber: no s` \x82\x01R\x7Ftake update found for operatorId`@\x82\x01R\x7F and quorumNumber at block numbe``\x82\x01R\x7Fr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x80\x82\x01RPV[_aZ\xBD`\x81\x83aJ\xB4V[\x91PaZ\xC8\x82aY\xF1V[`\xA0\x82\x01\x90P\x91\x90PV[_` \x82\x01\x90P\x81\x81\x03_\x83\x01RaZ\xEA\x81aZ\xB1V[\x90P\x91\x90PV[aZ\xFA\x81a:\x96V[\x82RPPV[`@\x82\x01_\x82\x01Qa[\x14_\x85\x01\x82aZ\xF1V[P` \x82\x01Qa['` \x85\x01\x82a>\xCAV[PPPPV[_\x81Q\x90P\x91\x90PV[_\x81\x90P` \x82\x01\x90P\x91\x90PV[_a[Q\x83\x83aZ\xF1V[` \x83\x01\x90P\x92\x91PPV[_` \x82\x01\x90P\x91\x90PV[_a[s\x82a[-V[a[}\x81\x85aQuV[\x93Pa[\x88\x83a[7V[\x80_[\x83\x81\x10\x15a[\xB8W\x81Qa[\x9F\x88\x82a[FV[\x97Pa[\xAA\x83a[]V[\x92PP`\x01\x81\x01\x90Pa[\x8BV[P\x85\x93PPPP\x92\x91PPV[_`\xA0\x82\x01\x90Pa[\xD8_\x83\x01\x87a[\0V[\x81\x81\x03`@\x83\x01Ra[\xEA\x81\x86a[iV[\x90P\x81\x81\x03``\x83\x01Ra[\xFE\x81\x85aR\x0FV[\x90Pa\\\r`\x80\x83\x01\x84aF\x85V[\x95\x94PPPPPV[_g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x11\x15a\\0Wa\\/a@\x01V[[` \x82\x02\x90P` \x81\x01\x90P\x91\x90PV[_a\\Sa\\N\x84a\\\x16V[a@_V[\x90P\x80\x83\x82R` \x82\x01\x90P` \x84\x02\x83\x01\x85\x81\x11\x15a\\vWa\\ua;0V[[\x83[\x81\x81\x10\x15a\\\xBDW\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\x9BWa\\\x9Aa;(V[[\x80\x86\x01a\\\xA8\x89\x82aS\x1CV[\x85R` \x85\x01\x94PPP` \x81\x01\x90Pa\\xV[PPP\x93\x92PPPV[_\x82`\x1F\x83\x01\x12a\\\xDBWa\\\xDAa;(V[[\x81Qa\\\xEB\x84\x82` \x86\x01a\\AV[\x91PP\x92\x91PPV[_` \x82\x84\x03\x12\x15a]\tWa]\x08a8\xCDV[[_\x82\x01Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]&Wa]%a8\xD1V[[a]2\x84\x82\x85\x01a\\\xC7V[\x91PP\x92\x91PPV[_\x81\x90P\x91\x90PV[_a]N\x82a];V[\x91Pa]Y\x83a];V[\x92P\x82\x82\x03\x90P\x81\x81\x12_\x84\x12\x16\x82\x82\x13_\x85\x12\x15\x16\x17\x15a]~Wa]}aN1V[[\x92\x91PPV[_a]\x8E\x82a];V[\x91P\x7F\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x03a]\xC0Wa]\xBFaN1V[[\x81_\x03\x90P\x91\x90PV[_a]\xD4\x82a:IV[\x91Pa]\xDF\x83a:IV[\x92P\x82\x82\x03\x90Pk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a^\x03Wa^\x02aN1V[[\x92\x91PPV\xFE\xA2dipfsX\"\x12 \x96o\xF8\x1B\xFD \xDF\xAB\x0B\"\xDB\xD7\xCF\xF1\xDFz\x11\x8EA7\x03\xBF\xFA\xAF\xB6\x86\x11\xFC\x964\x93:dsolcC\0\x08\x1B\x003",
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
    ///Container for all the [`StakeRegistry`](self) function calls.
    pub enum StakeRegistryCalls {
        MAX_WEIGHING_FUNCTION_LENGTH(MAX_WEIGHING_FUNCTION_LENGTHCall),
        WEIGHTING_DIVISOR(WEIGHTING_DIVISORCall),
        addStrategies(addStrategiesCall),
        avsDirectory(avsDirectoryCall),
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
            [117u8, 212u8, 23u8, 58u8],
            [124u8, 23u8, 35u8, 71u8],
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
            [248u8, 81u8, 225u8, 152u8],
            [250u8, 40u8, 198u8, 39u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StakeRegistryCalls {
        const NAME: &'static str = "StakeRegistryCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 38usize;
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
                Self::avsDirectory(_) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR
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
                    fn serviceManager(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <serviceManagerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::serviceManager)
                    }
                    serviceManager
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
                    fn stakeTypePerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <stakeTypePerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::stakeTypePerQuorum)
                    }
                    stakeTypePerQuorum
                },
                {
                    fn avsDirectory(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <avsDirectoryCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::avsDirectory)
                    }
                    avsDirectory
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
                    fn initializeDelegatedStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeDelegatedStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::initializeDelegatedStakeQuorum)
                    }
                    initializeDelegatedStakeQuorum
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
                    fn setStakeType(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setStakeTypeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::setStakeType)
                    }
                    setStakeType
                },
                {
                    fn slashableStakeLookAheadPerQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <slashableStakeLookAheadPerQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::slashableStakeLookAheadPerQuorum)
                    }
                    slashableStakeLookAheadPerQuorum
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
                    fn initializeSlashableStakeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <initializeSlashableStakeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::initializeSlashableStakeQuorum)
                    }
                    initializeSlashableStakeQuorum
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
                    fn setSlashableStakeLookahead(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <setSlashableStakeLookaheadCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryCalls::setSlashableStakeLookahead)
                    }
                    setSlashableStakeLookahead
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
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::avsDirectory(inner) => {
                    <avsDirectoryCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`StakeRegistry`](self) events.
    pub enum StakeRegistryEvents {
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
    impl alloy_sol_types::SolEventInterface for StakeRegistryEvents {
        const NAME: &'static str = "StakeRegistryEvents";
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
    impl alloy_sol_types::private::IntoLogData for StakeRegistryEvents {
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
        _avsDirectory: alloy::sol_types::private::Address,
        _serviceManager: alloy::sol_types::private::Address,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<StakeRegistryInstance<T, P, N>>,
    > {
        StakeRegistryInstance::<
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
        StakeRegistryInstance::<
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
            _avsDirectory: alloy::sol_types::private::Address,
            _serviceManager: alloy::sol_types::private::Address,
        ) -> alloy_contract::Result<StakeRegistryInstance<T, P, N>> {
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
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
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
