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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                (
                    value.updateBlockNumber,
                    value.nextUpdateBlockNumber,
                    value.stake,
                )
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.updateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.nextUpdateBlockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.stake,
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
        impl alloy_sol_types::SolType for StakeUpdate {
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
        impl alloy_sol_types::SolStruct for StakeUpdate {
            const NAME: &'static str = "StakeUpdate";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StakeUpdate(uint32 updateBlockNumber,uint32 nextUpdateBlockNumber,uint96 stake)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
        fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
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
        impl alloy_sol_types::SolType for StrategyParams {
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
        impl alloy_sol_types::SolStruct for StrategyParams {
            const NAME: &'static str = "StrategyParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "StrategyParams(address strategy,uint96 multiplier)",
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
                out.reserve(<Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust));
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, &mut out);
                alloy_sol_types::abi::token::WordToken(alloy_sol_types::private::keccak256(out))
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
            f.debug_tuple("IStakeRegistryInstance")
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
        > IStakeRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`IStakeRegistry`](self) contract instance.

        See the [wrapper's documentation](`IStakeRegistryInstance`) for more details.*/
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
        > IStakeRegistryInstance<T, P, N>
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
        > IStakeRegistryInstance<T, P, N>
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
    function isOperatorSetQuorum(uint8 quorumNumber) external view returns (bool);
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
    "name": "isOperatorSetQuorum",
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
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
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
    ///0x610100346100f657601f6131c338819003918201601f19168301916001600160401b038311848410176100fa578084926080946040528339810103126100f65780516001600160a01b03811691908290036100f65760208101516001600160a01b03811681036100f6576040820151916001600160a01b03831683036100f65760600151926001600160a01b03841684036100f65760e05260805260a05260c0526040516130b4908161010f823960805181818161033f0152612210015260a05181610d37015260c0518181816112170152611f07015260e0518181816107e501528181610cf30152818161232c01526124210152f35b5f80fd5b634e487b7160e01b5f52604160045260245ffdfe6080806040526004361015610012575f80fd5b5f3560e01c9081630491b41c1461178857508063087324611461172b5780631f9b74e0146116d757806320b66298146114a957806325504777146113425780632cd95940146112465780633998fdd3146112025780633ca5a5f5146111d05780634bd26e09146111915780635401ed27146111695780635e5a6775146111475780635f1f2d7714610e3857806366acfefe14610da4578063697fbd9314610d665780636b3aa72e14610d225780636d14a98714610cde57806375d4173a14610c395780637c17234714610c1f57806381c07502146109e057806386c06856146109575780639ab4d6ff1461091f5780639f3ccf65146108c65780639f8aff26146107b2578063ac6bfb0314610764578063adc804da146106fc578063b6904b78146106bc578063bc9a40c314610681578063bd29b8cd14610603578063c46778a5146105c9578063c601527d14610576578063c8294c561461052b578063cc5a7c20146103f3578063d5eccc0514610395578063dd9846b91461036e578063df5cf7231461032a578063e086adb3146102ed578063f2be94ae1461027e578063f851e198146102205763fa28c627146101c9575f80fd5b3461021c5760206001600160601b0361020e6102086101e736611a2d565b90825f949394526002875260405f2060ff82165f52875260405f2093612eb0565b906117d6565b505460401c16604051908152f35b5f80fd5b3461021c57604036600319011261021c57606061024661023e6117c6565b600435611c9f565b61027c60405180926001600160601b036040809263ffffffff815116855263ffffffff6020820151166020860152015116910152565bf35b3461021c57608036600319011261021c5760206001600160601b0360406102a36117b6565b6102ab611a1a565b906044355f526002855260ff835f2091165f5284526102e26102db6102d5845f20606435906117d6565b50611ae1565b9182612cef565b015116604051908152f35b3461021c57604036600319011261021c576103286103096117b6565b610311611a1a565b9061031a612317565b61032381611d0f565b612e56565b005b3461021c575f36600319011261021c576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461021c57602061038761038136611a2d565b91612eb0565b63ffffffff60405191168152f35b3461021c57602036600319011261021c5760ff6103b06117b6565b165f90815260016020526040902080545f1981019081116103df5761020e6001600160601b03916020936117d6565b634e487b7160e01b5f52601160045260245ffd5b3461021c57608036600319011261021c5761040c6117b6565b61041461195f565b906044359163ffffffff8316830361021c57606435906001600160401b03821161021c5761047b61044c610481933690600401611975565b61045461241f565b61047561046f8660ff165f52600160205260405f2054151590565b15611b28565b84612904565b82612c99565b60ff811691825f52600560205260405f20600160ff1982541617905560405191602083016002600110156105175783807f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d9260016104e297520390a1612e56565b5f52600160205261032860405f20604051906104fd826118ca565b63ffffffff431682525f60208301525f6040830152611bc2565b634e487b7160e01b5f52602160045260245ffd5b3461021c57606036600319011261021c5760206001600160601b0360406105506117b6565b60ff61055a611a1a565b91165f52600184526102e26102db6102d5604435855f206117d6565b3461021c57604036600319011261021c5761058f6117b6565b602435906001600160401b03821161021c576105b2610328923690600401611975565b906105bb612317565b6105c481611d0f565b612904565b3461021c57602036600319011261021c5760ff6105e46117b6565b165f525f60205260206001600160601b0360405f205416604051908152f35b3461021c57604036600319011261021c576004356024356001600160401b03811161021c5761063690369060040161181b565b61064192919261241f565b5f5b81811061064c57005b8061067a61065d6001938588611ab4565b3560f81c61066a81611d0f565b61067481876124d1565b90612822565b5001610643565b3461021c57604036600319011261021c5761032861069d6117b6565b6106a561195f565b906106ae612317565b6106b781611d0f565b612c99565b3461021c57604036600319011261021c5760ff6106d76117b6565b6106df611c44565b50165f52600160205260606102466102d560243560405f206117d6565b3461021c57604036600319011261021c5760ff6107176117b6565b61071f611c62565b50165f526003602052604061074161073b602435835f206117d6565b50611c7a565b6001600160601b03602083519260018060a01b0381511684520151166020820152f35b3461021c57606036600319011261021c5761077d6117b6565b610785611c44565b506024355f52600260205260ff60405f2091165f5260205260606102466102d560405f20604435906117d6565b3461021c57602036600319011261021c576107cb6117b6565b60405163a4d7871f60e01b815260ff9190911660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316602082602481845afa908115610899576004925f926108a4575b506020906040519384809263cabbb17f60e01b82525afa908115610899576020925f9261086a575b5081610861575b506040519015158152f35b90501582610856565b61088b919250833d8511610892575b6108838183611914565b810190611c2c565b908361084f565b503d610879565b6040513d5f823e3d90fd5b60209192506108bf90823d8411610892576108838183611914565b9190610827565b3461021c57604036600319011261021c576108df6117b6565b60ff60243591165f52600460205260405f20805482101561021c57602091610906916117d6565b905460405160039290921b1c6001600160a01b03168152f35b3461021c57602036600319011261021c5760ff61093a6117b6565b165f526006602052602063ffffffff60405f205416604051908152f35b3461021c57604036600319011261021c576109706117b6565b60243590600282101561021c577f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d9160ff6109db926109ad612317565b6109b681611d0f565b165f52600560205260405f2060ff1981541660ff83161790556040519182918261194c565b0390a1005b3461021c57604036600319011261021c5760043563ffffffff811680910361021c576024356001600160401b03811161021c57610a2190369060040161181b565b90610a2b82611935565b92610a396040519485611914565b828452610a4583611935565b602085019390601f19013685375f5b818110610aa5578486604051918291602083019060208452518091526040830191905f5b818110610a86575050500390f35b825163ffffffff16845285945060209384019390920191600101610a78565b610ab0818386611ab4565b3560f81c610abd81611d0f565b805f52600160205260405f20805415610c0b575f528363ffffffff60205f20541611610b7a57805f52600160205260405f20545f5b818110610b05575b505050600101610a54565b825f52600160205260405f20610b1b8284611b1b565b5f1981019081116103df57610b3663ffffffff9189936117d6565b5054161115610b4757600101610af2565b90610b529250611b1b565b5f198101919082116103df5763ffffffff60019216610b718289611acd565b52908780610afa565b60405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360648201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000608482015260a490fd5b634e487b7160e01b5f52603260045260245ffd5b3461021c575f36600319011261021c576020604051818152f35b3461021c57606036600319011261021c57610c526117b6565b610c5a61195f565b906044356001600160401b03811161021c5760ff9261047b61044c610c83933690600401611975565b165f818152600560209081526040808320805460ff19169055519182527f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d91a15f52600160205261032860405f20604051906104fd826118ca565b3461021c575f36600319011261021c576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461021c575f36600319011261021c576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461021c57602036600319011261021c5760ff610d816117b6565b165f526005602052610da060ff60405f2054166040519182918261194c565b0390f35b3461021c57610db236611848565b929091610dbd61241f565b5f935f5b818110610ddc576040516001600160c01b0387168152602090f35b80610e15610ded6001938589611ab4565b3560f81c610dfa81611d0f565b610e048782611e5d565b15610e1c575b610674908288612675565b5001610dc1565b5083811b60c085901b859003908116991698909817975f610e0a565b3461021c57604036600319011261021c57610e516117b6565b602435906001600160401b03821161021c573660238301121561021c57816004013591610e7d83611935565b92610e8b6040519485611914565b8084526024602085019160051b8301019136831161021c57602401905b82821061113757505050610eba612317565b610ec381611d0f565b81519081156110cc5760ff1691825f52600360205260405f20835f52600460205260405f20935f5b848110610ef457005b817f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f76020610f2c610f258589611acd565b51876117d6565b50546040516001600160a01b039091168152a2817f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a756040610f70610f258589611acd565b505481516001600160a01b0390911681525f6020820152a282545f1981019081116103df57610f9f90846117d6565b50610fb4610fad8387611acd565b51856117d6565b61108b5781810361109e575b505082548015611077575f1901610fd781856117d6565b61108b575f9055835585545f1981019081116103df57610ffa61104491886117d6565b905460039190911b1c6001600160a01b03166110206110198488611acd565b51896117d6565b81546001600160a01b0393841660039290921b91821b9390911b1916919091179055565b8554908115611077576001915f190161105d81896117d6565b815490858060a01b039060031b1b19169055875501610eeb565b634e487b7160e01b5f52603160045260245ffd5b634e487b7160e01b5f525f60045260245ffd5b815481546001600160a01b039091166001600160a01b03199182168117835592541690911790558680610fc0565b60405162461bcd60e51b815260206004820152603d60248201527f5374616b6552656769737472792e72656d6f7665537472617465676965733a2060448201527f6e6f20696e646963657320746f2072656d6f76652070726f76696465640000006064820152608490fd5b8135815260209182019101610ea8565b3461021c575f36600319011261021c576020604051670de0b6b3a76400008152f35b3461021c57604036600319011261021c5760206001600160601b0360406102e261023e6117c6565b3461021c57604036600319011261021c576111aa6117c6565b6004355f52600260205260ff60405f2091165f52602052602060405f2054604051908152f35b3461021c57602036600319011261021c5760ff6111eb6117b6565b165f526003602052602060405f2054604051908152f35b3461021c575f36600319011261021c576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461021c57604036600319011261021c5761125f6117c6565b6004355f52600260205260ff60405f2091165f5260205260405f2080549061128682611935565b916112946040519384611914565b8083526020830180925f5260205f205f915b838310611325578486604051918291602083019060208452518091526040830191905f5b8181106112d8575050500390f35b91935091602060608261131760019488516001600160601b036040809263ffffffff815116855263ffffffff6020820151166020860152015116910152565b0194019101918493926112ca565b60016020819261133485611ae1565b8152019201920191906112a6565b3461021c5761135036611848565b9061135c93929361241f565b61136582611a82565b9261136f83611a82565b925f5b8181106113a35761139586610da08760405193849360408552604085019061188e565b90838203602085015261188e565b6113ae818386611ab4565b3560f81c906113bc82611d0f565b6113c68483611e5d565b929092156114185782816113e06001956113fc948d612675565b916001600160601b036113f3868d611acd565b91169052612822565b6001600160601b0361140e8389611acd565b9116905201611372565b60405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a2060448201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360648201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000608482015260a490fd5b3461021c57606036600319011261021c576114c26117b6565b6024356001600160401b03811161021c576114e19036906004016117eb565b916044356001600160401b03811161021c576115019036906004016117eb565b909161150b612317565b61151481611d0f565b841561166d578482036116025760ff1691825f52600360205260405f20935f5b86811061153d57005b8061158f6115566115516001948888611a5e565b611a6e565b61156b611564848c88611a5e565b358a6117d6565b5080546001600160a01b031660a09290921b6001600160a01b031916919091179055565b857f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a756115bf611564848c88611a5e565b50848060a01b039054166115d7611551858a8a611a5e565b604080516001600160a01b039390931683526001600160601b0391909116602083015290a201611534565b60405162461bcd60e51b815260206004820152603960248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a20696e707574206c656e677468206d69736d61746368000000000000006064820152608490fd5b608460405162461bcd60e51b815260206004820152604060248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a206e6f20737472617465677920696e64696365732070726f76696465646064820152fd5b3461021c57604036600319011261021c576116f06117b6565b602435906001600160a01b038216820361021c576020918161171461171993611d0f565b611e5d565b506001600160601b0360405191168152f35b3461021c57604036600319011261021c576117446117b6565b60ff60243591165f52600360205260405f20805482101561021c5760409161176b916117d6565b505481516001600160a01b038216815260a09190911c6020820152f35b3461021c57602036600319011261021c5760209060ff6117a66117b6565b165f526001825260405f20548152f35b6004359060ff8216820361021c57565b6024359060ff8216820361021c57565b8054821015610c0b575f5260205f2001905f90565b9181601f8401121561021c578235916001600160401b03831161021c576020808501948460051b01011161021c57565b9181601f8401121561021c578235916001600160401b03831161021c576020838186019501011161021c57565b606060031982011261021c576004356001600160a01b038116810361021c579160243591604435906001600160401b03821161021c5761188a9160040161181b565b9091565b90602080835192838152019201905f5b8181106118ab5750505090565b82516001600160601b031684526020938401939092019160010161189e565b606081019081106001600160401b038211176118e557604052565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176118e557604052565b90601f801991011681019081106001600160401b038211176118e557604052565b6001600160401b0381116118e55760051b60200190565b9190602083019260028210156105175752565b602435906001600160601b038216820361021c57565b81601f8201121561021c5780359061198c82611935565b9261199a6040519485611914565b82845260208085019360061b8301019181831161021c57602001925b8284106119c4575050505090565b60408483031261021c57604051906119db826118f9565b84356001600160a01b038116810361021c5782526020850135906001600160601b038216820361021c57826020928360409501528152019301926119b6565b6024359063ffffffff8216820361021c57565b606090600319011261021c576004359060243560ff8116810361021c579060443563ffffffff8116810361021c5790565b9190811015610c0b5760051b0190565b356001600160601b038116810361021c5790565b90611a8c82611935565b611a996040519182611914565b8281528092611aaa601f1991611935565b0190602036910137565b90821015610c0b570190565b805115610c0b5760200190565b8051821015610c0b5760209160051b010190565b90604051611aee816118ca565b60406001600160601b0382945463ffffffff8116845263ffffffff8160201c166020850152821c16910152565b919082039182116103df57565b15611b2f57565b60405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a2060448201527471756f72756d20616c72656164792065786973747360581b6064820152608490fd5b906bffffffffffffffffffffffff60401b82549160401b16906bffffffffffffffffffffffff60401b1916179055565b8054600160401b8110156118e557611bdf916001820181556117d6565b61108b578151815460208085015167ffffffff00000000911b1663ffffffff90921667ffffffffffffffff1990911617178155611c2a916001600160601b0390604001511690611b92565b565b9081602091031261021c5751801515810361021c5790565b60405190611c51826118ca565b5f6040838281528260208201520152565b60405190611c6f826118f9565b5f6020838281520152565b90604051611c87816118f9565b91546001600160a01b038116835260a01c6020830152565b90611ca8611c44565b50815f52600260205260405f2060ff82165f5260205260405f205490611ccc611c44565b9282611cd85750505090565b909192505f52600260205260ff60405f2091165f5260205260405f205f1982019182116103df57611d0c916102d5916117d6565b90565b611d279060ff165f52600160205260405f2054151590565b15611d2e57565b60405162461bcd60e51b815260206004820152603160248201527f5374616b6552656769737472792e71756f72756d4578697374733a2071756f726044820152701d5b48191bd95cc81b9bdd08195e1a5cdd607a1b6064820152608490fd5b9080601f8301121561021c578151611da481611935565b92611db26040519485611914565b81845260208085019260051b82010192831161021c57602001905b828210611dda5750505090565b8151815260209182019101611dcd565b90602082549182815201915f5260205f20905f5b818110611e0b5750505090565b82546001600160a01b0316845260209093019260019283019201611dfe565b818102929181159184041417156103df57565b906001600160601b03809116911601906001600160601b0382116103df57565b919060ff5f931690815f52600360205260405f205490604051611e7f816118f9565b5f81525f602082015250825f52600560205260ff60405f2054166002811015610517576001036121c7576040908151611eb88382611914565b600181526020810191601f198401368437611ed282611ac0565b9060018060a01b03169052845f52600660205263ffffffff611ef981855f205416426128f7565b845163ca8aa7c760e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169490939290911691602084600481885afa9384156121bd575f9461218c575b509063ffffffff949291865193611f64856118f9565b84526020840193898552895f526004602052875f209188519788966315d5962560e11b885260a488019360018060a01b0390511660048901525116602487015260a060448701525180915260c4850192905f5b81811061216a575050505f9492611fdc85938493600319858303016064860152611dea565b608483019190915203916001600160a01b03165afa8015612160575f906120ae575b6120089150611ac0565b51905f5b83811061203b57505050505b5f525f6020526001600160601b0360405f2054166001600160601b038316101590565b845f52600360205261205261073b82845f206117d6565b61205c8285611acd565b5161206b575b5060010161200c565b81976001600160601b03670de0b6b3a764000061209f6120a7948360206120946001998c611acd565b519201511690611e2a565b041690611e3d565b9690612062565b503d805f833e6120be8183611914565b81019060208183031261021c578051906001600160401b03821161021c57019080601f8301121561021c5781516120f481611935565b9261210185519485611914565b81845260208085019260051b8201019183831161021c5760208201905b83821061213357505050505061200890611ffe565b81516001600160401b03811161021c5760209161215587848094880101611d8d565b81520191019061211e565b82513d5f823e3d90fd5b82516001600160a01b0316855288965060209485019490920191600101611fb7565b6121af91945060203d6020116121b6575b6121a78183611914565b8101906122f8565b925f611f4e565b503d61219d565b86513d5f823e3d90fd5b5f8381526004602081905260408083208151639004134760e01b81526001600160a01b039095169285019290925260248401528290819061220c906044830190611dea565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610899575f916122bb575b505f5b82811061225a57505050612018565b835f52600360205261227261073b8260405f206117d6565b61227c8284611acd565b5161228b575b5060010161224b565b81966001600160601b03670de0b6b3a764000061209f6122b4948360206120946001998b611acd565b9590612282565b90503d805f833e6122cc8183611914565b810160208282031261021c5781516001600160401b03811161021c576122f29201611d8d565b5f612248565b9081602091031261021c57516001600160a01b038116810361021c5790565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610899575f91612400575b506001600160a01b0316330361237657565b60405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e60448201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f6620746064820152753432903932b3b4b9ba393ca1b7b7b93234b730ba37b960511b608482015260a490fd5b612419915060203d6020116121b6576121a78183611914565b5f612364565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316330361245157565b60405162461bcd60e51b815260206004820152604c60248201527f5374616b6552656769737472792e6f6e6c795265676973747279436f6f72646960448201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260648201526b3ca1b7b7b93234b730ba37b960a11b608482015260a490fd5b5f90805f52600260205260405f2060ff84165f5260205260405f205480155f1461257857505f51602061303f5f395f51905f5260406001600160601b0394835f526002602052815f2060ff82165f5260205261254e825f20835190612535826118ca565b63ffffffff431682525f60208301525f85830152611bc2565b60ff8251911681525f6020820152a2165f81810391125f82128116905f8313901516176103df5790565b908092505f52600260205260405f2060ff84165f5260205260405f20905f1981019081116103df576125a9916117d6565b50908154916001600160601b038360401c1692831561266c576001600160601b03945f51602061303f5f395f51905f529260409263ffffffff4381169116810361260e5750805473ffffffffffffffffffffffff00000000000000001916905561254e565b815467ffffffff000000001916602082901b67ffffffff00000000161790915561266790855f526002602052835f2060ff84165f52602052835f20845191612655836118ca565b82525f60208301525f85830152611bc2565b61254e565b50505050505f90565b9190915f90805f52600260205260405f2060ff85165f5260205260405f205480155f1461272457505f51602061303f5f395f51905f5260406001600160601b038095845f526002602052825f2060ff89165f526020526126f8835f208451906126dd826118ca565b63ffffffff431682525f602083015284841686830152611bc2565b60ff8351981688521695866020820152a216905f82820392128183128116918313901516176103df5790565b908092505f52600260205260405f2060ff85165f5260205260405f20905f1981019081116103df57612755916117d6565b50908154916001600160601b038360401c16926001600160601b0385169081851461281757855f51602061303f5f395f51905f52936001600160601b039763ffffffff6040958a9582431692839116145f146127bb5750506127b691611b92565b6126f8565b835467ffffffff000000001916602083901b67ffffffff0000000016179093556127b692909150875f526002602052855f2060ff8c165f52602052855f2090865192612806846118ca565b83525f602084015286830152611bc2565b505050505050505f90565b60ff165f81815260016020526040902080549192915f1981019081116103df5761284b916117d6565b509080156128e45763ffffffff6128708354926001600160601b038460401c16612ff5565b93849243831692168203612889575050611d0c91611b92565b835467ffffffff000000001916602083901b67ffffffff000000001617909355611d0c929091505f52600160205260405f20604051916128c8836118ca565b82525f60208301526001600160601b0384166040830152611bc2565b506001600160601b0391505460401c1690565b919082018092116103df57565b815115612c415760ff8251911691825f52600360205260405f205492602061292c84866128f7565b11612bdb575f925b808410612942575050505050565b90919293945f5b61295386886128f7565b8110156129ef57835f52600360205261296f8160405f206117d6565b50546001600160a01b03908116906129878888611acd565b5151161461299757600101612949565b60405162461bcd60e51b815260206004820152603d60248201525f51602061305f5f395f51905f5260448201527f3a2063616e6e6f74206164642073616d652073747261746567792032780000006064820152608490fd5b509493929190926001600160601b036020612a0a8386611acd565b5101511615612b7457815f52600360205260405f20612a298285611acd565b51908054600160401b8110156118e557612a48916001820181556117d6565b61108b5781516020929092015160a01b6001600160a01b0319166001600160a01b03929092169190911790555f828152600460205260409020906001600160a01b03612a948286611acd565b515116825490600160401b8210156118e5576110208260019586612aba950181556117d6565b827f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f54046020848060a01b03612aee8589611acd565b515116604051908152a2827f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838060a01b03612b2a8488611acd565b5151166001600160601b036020612b41868a611acd565b510151604080516001600160a01b0394909416845291166001600160601b03166020830152819081010390a20192612934565b60405162461bcd60e51b815260206004820152604660248201525f51602061305f5f395f51905f5260448201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f206064820152651dd95a59da1d60d21b608482015260a490fd5b60405162461bcd60e51b815260206004820152604560248201525f51602061305f5f395f51905f5260448201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60648201526408a9c8ea8960db1b608482015260a490fd5b60405162461bcd60e51b815260206004820152603860248201525f51602061305f5f395f51905f5260448201527f3a206e6f20737472617465676965732070726f766964656400000000000000006064820152608490fd5b602060ff7f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf921692835f525f82526001600160601b0360405f20911690816001600160601b0319825416179055604051908152a2565b63ffffffff808251169216918210612dcc576020015163ffffffff168015918215612dc2575b505015612d1e57565b60405162461bcd60e51b815260206004820152606a60248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560648201527f72207374616b6555706461746520617661696c61626c65206265666f726520626084820152693637b1b5a73ab6b132b960b11b60a482015260c490fd5b1090505f80612d15565b60405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a207374616b6555706461746520697320606482015275333937b69030b33a32b910313637b1b5a73ab6b132b960511b608482015260a490fd5b60ff165f90815260066020908152604091829020805463ffffffff94851663ffffffff1982168117909255835194168452908301527f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c791a1565b929190835f52600260205260405f2060ff82165f5260205260405f2054805b612f945760405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e490fd5b845f52600260205260405f2060ff83165f5260205260405f205f198201908282116103df57612fc88263ffffffff926117d6565b50541663ffffffff85161015612fe8575080156103df575f190180612ecf565b63ffffffff169450505050565b905f81121561302a57600160ff1b81146103df576001600160601b0380915f03169116036001600160601b0381116103df5790565b906001600160601b03611d0c921690611e3d56fe2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d5374616b6552656769737472792e5f6164645374726174656779506172616d73a2646970667358221220bb2094734430917001b4297013c31c80ee3f66305351dce8ee0dea1b714f3b5764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\x004a\0\xF6W`\x1Fa1\xC38\x81\x90\x03\x91\x82\x01`\x1F\x19\x16\x83\x01\x91`\x01`\x01`@\x1B\x03\x83\x11\x84\x84\x10\x17a\0\xFAW\x80\x84\x92`\x80\x94`@R\x839\x81\x01\x03\x12a\0\xF6W\x80Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x91\x90\x82\x90\x03a\0\xF6W` \x81\x01Q`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\0\xF6W`@\x82\x01Q\x91`\x01`\x01`\xA0\x1B\x03\x83\x16\x83\x03a\0\xF6W``\x01Q\x92`\x01`\x01`\xA0\x1B\x03\x84\x16\x84\x03a\0\xF6W`\xE0R`\x80R`\xA0R`\xC0R`@Qa0\xB4\x90\x81a\x01\x0F\x829`\x80Q\x81\x81\x81a\x03?\x01Ra\"\x10\x01R`\xA0Q\x81a\r7\x01R`\xC0Q\x81\x81\x81a\x12\x17\x01Ra\x1F\x07\x01R`\xE0Q\x81\x81\x81a\x07\xE5\x01R\x81\x81a\x0C\xF3\x01R\x81\x81a#,\x01Ra$!\x01R\xF3[_\x80\xFD[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD\xFE`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x04\x91\xB4\x1C\x14a\x17\x88WP\x80c\x08s$a\x14a\x17+W\x80c\x1F\x9Bt\xE0\x14a\x16\xD7W\x80c \xB6b\x98\x14a\x14\xA9W\x80c%PGw\x14a\x13BW\x80c,\xD9Y@\x14a\x12FW\x80c9\x98\xFD\xD3\x14a\x12\x02W\x80c<\xA5\xA5\xF5\x14a\x11\xD0W\x80cK\xD2n\t\x14a\x11\x91W\x80cT\x01\xED'\x14a\x11iW\x80c^Zgu\x14a\x11GW\x80c_\x1F-w\x14a\x0E8W\x80cf\xAC\xFE\xFE\x14a\r\xA4W\x80ci\x7F\xBD\x93\x14a\rfW\x80ck:\xA7.\x14a\r\"W\x80cm\x14\xA9\x87\x14a\x0C\xDEW\x80cu\xD4\x17:\x14a\x0C9W\x80c|\x17#G\x14a\x0C\x1FW\x80c\x81\xC0u\x02\x14a\t\xE0W\x80c\x86\xC0hV\x14a\tWW\x80c\x9A\xB4\xD6\xFF\x14a\t\x1FW\x80c\x9F<\xCFe\x14a\x08\xC6W\x80c\x9F\x8A\xFF&\x14a\x07\xB2W\x80c\xACk\xFB\x03\x14a\x07dW\x80c\xAD\xC8\x04\xDA\x14a\x06\xFCW\x80c\xB6\x90Kx\x14a\x06\xBCW\x80c\xBC\x9A@\xC3\x14a\x06\x81W\x80c\xBD)\xB8\xCD\x14a\x06\x03W\x80c\xC4gx\xA5\x14a\x05\xC9W\x80c\xC6\x01R}\x14a\x05vW\x80c\xC8)LV\x14a\x05+W\x80c\xCCZ| \x14a\x03\xF3W\x80c\xD5\xEC\xCC\x05\x14a\x03\x95W\x80c\xDD\x98F\xB9\x14a\x03nW\x80c\xDF\\\xF7#\x14a\x03*W\x80c\xE0\x86\xAD\xB3\x14a\x02\xEDW\x80c\xF2\xBE\x94\xAE\x14a\x02~W\x80c\xF8Q\xE1\x98\x14a\x02 Wc\xFA(\xC6'\x14a\x01\xC9W_\x80\xFD[4a\x02\x1CW` `\x01`\x01``\x1B\x03a\x02\x0Ea\x02\x08a\x01\xE76a\x1A-V[\x90\x82_\x94\x93\x94R`\x02\x87R`@_ `\xFF\x82\x16_R\x87R`@_ \x93a.\xB0V[\x90a\x17\xD6V[PT`@\x1C\x16`@Q\x90\x81R\xF3[_\x80\xFD[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW``a\x02Fa\x02>a\x17\xC6V[`\x045a\x1C\x9FV[a\x02|`@Q\x80\x92`\x01`\x01``\x1B\x03`@\x80\x92c\xFF\xFF\xFF\xFF\x81Q\x16\x85Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[\xF3[4a\x02\x1CW`\x806`\x03\x19\x01\x12a\x02\x1CW` `\x01`\x01``\x1B\x03`@a\x02\xA3a\x17\xB6V[a\x02\xABa\x1A\x1AV[\x90`D5_R`\x02\x85R`\xFF\x83_ \x91\x16_R\x84Ra\x02\xE2a\x02\xDBa\x02\xD5\x84_ `d5\x90a\x17\xD6V[Pa\x1A\xE1V[\x91\x82a,\xEFV[\x01Q\x16`@Q\x90\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x03(a\x03\ta\x17\xB6V[a\x03\x11a\x1A\x1AV[\x90a\x03\x1Aa#\x17V[a\x03#\x81a\x1D\x0FV[a.VV[\0[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x1CW` a\x03\x87a\x03\x816a\x1A-V[\x91a.\xB0V[c\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x03\xB0a\x17\xB6V[\x16_\x90\x81R`\x01` R`@\x90 \x80T_\x19\x81\x01\x90\x81\x11a\x03\xDFWa\x02\x0E`\x01`\x01``\x1B\x03\x91` \x93a\x17\xD6V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[4a\x02\x1CW`\x806`\x03\x19\x01\x12a\x02\x1CWa\x04\x0Ca\x17\xB6V[a\x04\x14a\x19_V[\x90`D5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x02\x1CW`d5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CWa\x04{a\x04La\x04\x81\x936\x90`\x04\x01a\x19uV[a\x04Ta$\x1FV[a\x04ua\x04o\x86`\xFF\x16_R`\x01` R`@_ T\x15\x15\x90V[\x15a\x1B(V[\x84a)\x04V[\x82a,\x99V[`\xFF\x81\x16\x91\x82_R`\x05` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x91` \x83\x01`\x02`\x01\x10\x15a\x05\x17W\x83\x80\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x92`\x01a\x04\xE2\x97R\x03\x90\xA1a.VV[_R`\x01` Ra\x03(`@_ `@Q\x90a\x04\xFD\x82a\x18\xCAV[c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01R_`@\x83\x01Ra\x1B\xC2V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[4a\x02\x1CW``6`\x03\x19\x01\x12a\x02\x1CW` `\x01`\x01``\x1B\x03`@a\x05Pa\x17\xB6V[`\xFFa\x05Za\x1A\x1AV[\x91\x16_R`\x01\x84Ra\x02\xE2a\x02\xDBa\x02\xD5`D5\x85_ a\x17\xD6V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x05\x8Fa\x17\xB6V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CWa\x05\xB2a\x03(\x926\x90`\x04\x01a\x19uV[\x90a\x05\xBBa#\x17V[a\x05\xC4\x81a\x1D\x0FV[a)\x04V[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x05\xE4a\x17\xB6V[\x16_R_` R` `\x01`\x01``\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\x066\x906\x90`\x04\x01a\x18\x1BV[a\x06A\x92\x91\x92a$\x1FV[_[\x81\x81\x10a\x06LW\0[\x80a\x06za\x06]`\x01\x93\x85\x88a\x1A\xB4V[5`\xF8\x1Ca\x06j\x81a\x1D\x0FV[a\x06t\x81\x87a$\xD1V[\x90a(\"V[P\x01a\x06CV[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x03(a\x06\x9Da\x17\xB6V[a\x06\xA5a\x19_V[\x90a\x06\xAEa#\x17V[a\x06\xB7\x81a\x1D\x0FV[a,\x99V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x06\xD7a\x17\xB6V[a\x06\xDFa\x1CDV[P\x16_R`\x01` R``a\x02Fa\x02\xD5`$5`@_ a\x17\xD6V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x07\x17a\x17\xB6V[a\x07\x1Fa\x1CbV[P\x16_R`\x03` R`@a\x07Aa\x07;`$5\x83_ a\x17\xD6V[Pa\x1CzV[`\x01`\x01``\x1B\x03` \x83Q\x92`\x01\x80`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x16` \x82\x01R\xF3[4a\x02\x1CW``6`\x03\x19\x01\x12a\x02\x1CWa\x07}a\x17\xB6V[a\x07\x85a\x1CDV[P`$5_R`\x02` R`\xFF`@_ \x91\x16_R` R``a\x02Fa\x02\xD5`@_ `D5\x90a\x17\xD6V[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CWa\x07\xCBa\x17\xB6V[`@Qc\xA4\xD7\x87\x1F`\xE0\x1B\x81R`\xFF\x91\x90\x91\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16` \x82`$\x81\x84Z\xFA\x90\x81\x15a\x08\x99W`\x04\x92_\x92a\x08\xA4W[P` \x90`@Q\x93\x84\x80\x92c\xCA\xBB\xB1\x7F`\xE0\x1B\x82RZ\xFA\x90\x81\x15a\x08\x99W` \x92_\x92a\x08jW[P\x81a\x08aW[P`@Q\x90\x15\x15\x81R\xF3[\x90P\x15\x82a\x08VV[a\x08\x8B\x91\x92P\x83=\x85\x11a\x08\x92W[a\x08\x83\x81\x83a\x19\x14V[\x81\x01\x90a\x1C,V[\x90\x83a\x08OV[P=a\x08yV[`@Q=_\x82>=\x90\xFD[` \x91\x92Pa\x08\xBF\x90\x82=\x84\x11a\x08\x92Wa\x08\x83\x81\x83a\x19\x14V[\x91\x90a\x08'V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x08\xDFa\x17\xB6V[`\xFF`$5\x91\x16_R`\x04` R`@_ \x80T\x82\x10\x15a\x02\x1CW` \x91a\t\x06\x91a\x17\xD6V[\x90T`@Q`\x03\x92\x90\x92\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16\x81R\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\t:a\x17\xB6V[\x16_R`\x06` R` c\xFF\xFF\xFF\xFF`@_ T\x16`@Q\x90\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\tpa\x17\xB6V[`$5\x90`\x02\x82\x10\x15a\x02\x1CW\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x91`\xFFa\t\xDB\x92a\t\xADa#\x17V[a\t\xB6\x81a\x1D\x0FV[\x16_R`\x05` R`@_ `\xFF\x19\x81T\x16`\xFF\x83\x16\x17\x90U`@Q\x91\x82\x91\x82a\x19LV[\x03\x90\xA1\0[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW`\x045c\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x02\x1CW`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\n!\x906\x90`\x04\x01a\x18\x1BV[\x90a\n+\x82a\x195V[\x92a\n9`@Q\x94\x85a\x19\x14V[\x82\x84Ra\nE\x83a\x195V[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x81\x81\x10a\n\xA5W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\n\x86WPPP\x03\x90\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\nxV[a\n\xB0\x81\x83\x86a\x1A\xB4V[5`\xF8\x1Ca\n\xBD\x81a\x1D\x0FV[\x80_R`\x01` R`@_ \x80T\x15a\x0C\x0BW_R\x83c\xFF\xFF\xFF\xFF` _ T\x16\x11a\x0BzW\x80_R`\x01` R`@_ T_[\x81\x81\x10a\x0B\x05W[PPP`\x01\x01a\nTV[\x82_R`\x01` R`@_ a\x0B\x1B\x82\x84a\x1B\x1BV[_\x19\x81\x01\x90\x81\x11a\x03\xDFWa\x0B6c\xFF\xFF\xFF\xFF\x91\x89\x93a\x17\xD6V[PT\x16\x11\x15a\x0BGW`\x01\x01a\n\xF2V[\x90a\x0BR\x92Pa\x1B\x1BV[_\x19\x81\x01\x91\x90\x82\x11a\x03\xDFWc\xFF\xFF\xFF\xFF`\x01\x92\x16a\x0Bq\x82\x89a\x1A\xCDV[R\x90\x87\x80a\n\xFAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW` `@Q\x81\x81R\xF3[4a\x02\x1CW``6`\x03\x19\x01\x12a\x02\x1CWa\x0CRa\x17\xB6V[a\x0CZa\x19_V[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CW`\xFF\x92a\x04{a\x04La\x0C\x83\x936\x90`\x04\x01a\x19uV[\x16_\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90UQ\x91\x82R\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x91\xA1_R`\x01` Ra\x03(`@_ `@Q\x90a\x04\xFD\x82a\x18\xCAV[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\r\x81a\x17\xB6V[\x16_R`\x05` Ra\r\xA0`\xFF`@_ T\x16`@Q\x91\x82\x91\x82a\x19LV[\x03\x90\xF3[4a\x02\x1CWa\r\xB26a\x18HV[\x92\x90\x91a\r\xBDa$\x1FV[_\x93_[\x81\x81\x10a\r\xDCW`@Q`\x01`\x01`\xC0\x1B\x03\x87\x16\x81R` \x90\xF3[\x80a\x0E\x15a\r\xED`\x01\x93\x85\x89a\x1A\xB4V[5`\xF8\x1Ca\r\xFA\x81a\x1D\x0FV[a\x0E\x04\x87\x82a\x1E]V[\x15a\x0E\x1CW[a\x06t\x90\x82\x88a&uV[P\x01a\r\xC1V[P\x83\x81\x1B`\xC0\x85\x90\x1B\x85\x90\x03\x90\x81\x16\x99\x16\x98\x90\x98\x17\x97_a\x0E\nV[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x0EQa\x17\xB6V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CW6`#\x83\x01\x12\x15a\x02\x1CW\x81`\x04\x015\x91a\x0E}\x83a\x195V[\x92a\x0E\x8B`@Q\x94\x85a\x19\x14V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x02\x1CW`$\x01\x90[\x82\x82\x10a\x117WPPPa\x0E\xBAa#\x17V[a\x0E\xC3\x81a\x1D\x0FV[\x81Q\x90\x81\x15a\x10\xCCW`\xFF\x16\x91\x82_R`\x03` R`@_ \x83_R`\x04` R`@_ \x93_[\x84\x81\x10a\x0E\xF4W\0[\x81\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7` a\x0F,a\x0F%\x85\x89a\x1A\xCDV[Q\x87a\x17\xD6V[PT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xA2\x81\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au`@a\x0Fpa\x0F%\x85\x89a\x1A\xCDV[PT\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R_` \x82\x01R\xA2\x82T_\x19\x81\x01\x90\x81\x11a\x03\xDFWa\x0F\x9F\x90\x84a\x17\xD6V[Pa\x0F\xB4a\x0F\xAD\x83\x87a\x1A\xCDV[Q\x85a\x17\xD6V[a\x10\x8BW\x81\x81\x03a\x10\x9EW[PP\x82T\x80\x15a\x10wW_\x19\x01a\x0F\xD7\x81\x85a\x17\xD6V[a\x10\x8BW_\x90U\x83U\x85T_\x19\x81\x01\x90\x81\x11a\x03\xDFWa\x0F\xFAa\x10D\x91\x88a\x17\xD6V[\x90T`\x03\x91\x90\x91\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16a\x10 a\x10\x19\x84\x88a\x1A\xCDV[Q\x89a\x17\xD6V[\x81T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x03\x92\x90\x92\x1B\x91\x82\x1B\x93\x90\x91\x1B\x19\x16\x91\x90\x91\x17\x90UV[\x85T\x90\x81\x15a\x10wW`\x01\x91_\x19\x01a\x10]\x81\x89a\x17\xD6V[\x81T\x90\x85\x80`\xA0\x1B\x03\x90`\x03\x1B\x1B\x19\x16\x90U\x87U\x01a\x0E\xEBV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x81T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x83U\x92T\x16\x90\x91\x17\x90U\x86\x80a\x0F\xC0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x90\xFD[\x815\x81R` \x91\x82\x01\x91\x01a\x0E\xA8V[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW` `@Qg\r\xE0\xB6\xB3\xA7d\0\0\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW` `\x01`\x01``\x1B\x03`@a\x02\xE2a\x02>a\x17\xC6V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x11\xAAa\x17\xC6V[`\x045_R`\x02` R`\xFF`@_ \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x11\xEBa\x17\xB6V[\x16_R`\x03` R` `@_ T`@Q\x90\x81R\xF3[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x12_a\x17\xC6V[`\x045_R`\x02` R`\xFF`@_ \x91\x16_R` R`@_ \x80T\x90a\x12\x86\x82a\x195V[\x91a\x12\x94`@Q\x93\x84a\x19\x14V[\x80\x83R` \x83\x01\x80\x92_R` _ _\x91[\x83\x83\x10a\x13%W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x12\xD8WPPP\x03\x90\xF3[\x91\x93P\x91` ``\x82a\x13\x17`\x01\x94\x88Q`\x01`\x01``\x1B\x03`@\x80\x92c\xFF\xFF\xFF\xFF\x81Q\x16\x85Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[\x01\x94\x01\x91\x01\x91\x84\x93\x92a\x12\xCAV[`\x01` \x81\x92a\x134\x85a\x1A\xE1V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x12\xA6V[4a\x02\x1CWa\x13P6a\x18HV[\x90a\x13\\\x93\x92\x93a$\x1FV[a\x13e\x82a\x1A\x82V[\x92a\x13o\x83a\x1A\x82V[\x92_[\x81\x81\x10a\x13\xA3Wa\x13\x95\x86a\r\xA0\x87`@Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x18\x8EV[\x90\x83\x82\x03` \x85\x01Ra\x18\x8EV[a\x13\xAE\x81\x83\x86a\x1A\xB4V[5`\xF8\x1C\x90a\x13\xBC\x82a\x1D\x0FV[a\x13\xC6\x84\x83a\x1E]V[\x92\x90\x92\x15a\x14\x18W\x82\x81a\x13\xE0`\x01\x95a\x13\xFC\x94\x8Da&uV[\x91`\x01`\x01``\x1B\x03a\x13\xF3\x86\x8Da\x1A\xCDV[\x91\x16\x90Ra(\"V[`\x01`\x01``\x1B\x03a\x14\x0E\x83\x89a\x1A\xCDV[\x91\x16\x90R\x01a\x13rV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[4a\x02\x1CW``6`\x03\x19\x01\x12a\x02\x1CWa\x14\xC2a\x17\xB6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\x14\xE1\x906\x90`\x04\x01a\x17\xEBV[\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\x15\x01\x906\x90`\x04\x01a\x17\xEBV[\x90\x91a\x15\x0Ba#\x17V[a\x15\x14\x81a\x1D\x0FV[\x84\x15a\x16mW\x84\x82\x03a\x16\x02W`\xFF\x16\x91\x82_R`\x03` R`@_ \x93_[\x86\x81\x10a\x15=W\0[\x80a\x15\x8Fa\x15Va\x15Q`\x01\x94\x88\x88a\x1A^V[a\x1AnV[a\x15ka\x15d\x84\x8C\x88a\x1A^V[5\x8Aa\x17\xD6V[P\x80T`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x92\x90\x92\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90UV[\x85\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Aua\x15\xBFa\x15d\x84\x8C\x88a\x1A^V[P\x84\x80`\xA0\x1B\x03\x90T\x16a\x15\xD7a\x15Q\x85\x8A\x8Aa\x1A^V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R`\x01`\x01``\x1B\x03\x91\x90\x91\x16` \x83\x01R\x90\xA2\x01a\x154V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R\xFD[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x16\xF0a\x17\xB6V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\x1CW` \x91\x81a\x17\x14a\x17\x19\x93a\x1D\x0FV[a\x1E]V[P`\x01`\x01``\x1B\x03`@Q\x91\x16\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x17Da\x17\xB6V[`\xFF`$5\x91\x16_R`\x03` R`@_ \x80T\x82\x10\x15a\x02\x1CW`@\x91a\x17k\x91a\x17\xD6V[PT\x81Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R`\xA0\x91\x90\x91\x1C` \x82\x01R\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW` \x90`\xFFa\x17\xA6a\x17\xB6V[\x16_R`\x01\x82R`@_ T\x81R\xF3[`\x045\x90`\xFF\x82\x16\x82\x03a\x02\x1CWV[`$5\x90`\xFF\x82\x16\x82\x03a\x02\x1CWV[\x80T\x82\x10\x15a\x0C\x0BW_R` _ \x01\x90_\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\x02\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\x1CW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x02\x1CWV[\x91\x81`\x1F\x84\x01\x12\x15a\x02\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\x1CW` \x83\x81\x86\x01\x95\x01\x01\x11a\x02\x1CWV[```\x03\x19\x82\x01\x12a\x02\x1CW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x1CW\x91`$5\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CWa\x18\x8A\x91`\x04\x01a\x18\x1BV[\x90\x91V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x18\xABWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x18\x9EV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\xE5W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\xE5W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\xE5W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x18\xE5W`\x05\x1B` \x01\x90V[\x91\x90` \x83\x01\x92`\x02\x82\x10\x15a\x05\x17WRV[`$5\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\x02\x1CWV[\x81`\x1F\x82\x01\x12\x15a\x02\x1CW\x805\x90a\x19\x8C\x82a\x195V[\x92a\x19\x9A`@Q\x94\x85a\x19\x14V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x02\x1CW` \x01\x92[\x82\x84\x10a\x19\xC4WPPPP\x90V[`@\x84\x83\x03\x12a\x02\x1CW`@Q\x90a\x19\xDB\x82a\x18\xF9V[\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x1CW\x82R` \x85\x015\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\x02\x1CW\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a\x19\xB6V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x1CWV[``\x90`\x03\x19\x01\x12a\x02\x1CW`\x045\x90`$5`\xFF\x81\x16\x81\x03a\x02\x1CW\x90`D5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x02\x1CW\x90V[\x91\x90\x81\x10\x15a\x0C\x0BW`\x05\x1B\x01\x90V[5`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x02\x1CW\x90V[\x90a\x1A\x8C\x82a\x195V[a\x1A\x99`@Q\x91\x82a\x19\x14V[\x82\x81R\x80\x92a\x1A\xAA`\x1F\x19\x91a\x195V[\x01\x90` 6\x91\x017V[\x90\x82\x10\x15a\x0C\x0BW\x01\x90V[\x80Q\x15a\x0C\x0BW` \x01\x90V[\x80Q\x82\x10\x15a\x0C\x0BW` \x91`\x05\x1B\x01\x01\x90V[\x90`@Qa\x1A\xEE\x81a\x18\xCAV[`@`\x01`\x01``\x1B\x03\x82\x94Tc\xFF\xFF\xFF\xFF\x81\x16\x84Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16` \x85\x01R\x82\x1C\x16\x91\x01RV[\x91\x90\x82\x03\x91\x82\x11a\x03\xDFWV[\x15a\x1B/WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x90\xFD[\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x82T\x91`@\x1B\x16\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19\x16\x17\x90UV[\x80T`\x01`@\x1B\x81\x10\x15a\x18\xE5Wa\x1B\xDF\x91`\x01\x82\x01\x81Ua\x17\xD6V[a\x10\x8BW\x81Q\x81T` \x80\x85\x01Qg\xFF\xFF\xFF\xFF\0\0\0\0\x91\x1B\x16c\xFF\xFF\xFF\xFF\x90\x92\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17\x17\x81Ua\x1C*\x91`\x01`\x01``\x1B\x03\x90`@\x01Q\x16\x90a\x1B\x92V[V[\x90\x81` \x91\x03\x12a\x02\x1CWQ\x80\x15\x15\x81\x03a\x02\x1CW\x90V[`@Q\x90a\x1CQ\x82a\x18\xCAV[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[`@Q\x90a\x1Co\x82a\x18\xF9V[_` \x83\x82\x81R\x01RV[\x90`@Qa\x1C\x87\x81a\x18\xF9V[\x91T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\xA0\x1C` \x83\x01RV[\x90a\x1C\xA8a\x1CDV[P\x81_R`\x02` R`@_ `\xFF\x82\x16_R` R`@_ T\x90a\x1C\xCCa\x1CDV[\x92\x82a\x1C\xD8WPPP\x90V[\x90\x91\x92P_R`\x02` R`\xFF`@_ \x91\x16_R` R`@_ _\x19\x82\x01\x91\x82\x11a\x03\xDFWa\x1D\x0C\x91a\x02\xD5\x91a\x17\xD6V[\x90V[a\x1D'\x90`\xFF\x16_R`\x01` R`@_ T\x15\x15\x90V[\x15a\x1D.WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FStakeRegistry.quorumExists: quor`D\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x1CW\x81Qa\x1D\xA4\x81a\x195V[\x92a\x1D\xB2`@Q\x94\x85a\x19\x14V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x1CW` \x01\x90[\x82\x82\x10a\x1D\xDAWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x1D\xCDV[\x90` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a\x1E\x0BWPPP\x90V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1D\xFEV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x03\xDFWV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x01\x90`\x01`\x01``\x1B\x03\x82\x11a\x03\xDFWV[\x91\x90`\xFF_\x93\x16\x90\x81_R`\x03` R`@_ T\x90`@Qa\x1E\x7F\x81a\x18\xF9V[_\x81R_` \x82\x01RP\x82_R`\x05` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x05\x17W`\x01\x03a!\xC7W`@\x90\x81Qa\x1E\xB8\x83\x82a\x19\x14V[`\x01\x81R` \x81\x01\x91`\x1F\x19\x84\x016\x847a\x1E\xD2\x82a\x1A\xC0V[\x90`\x01\x80`\xA0\x1B\x03\x16\x90R\x84_R`\x06` Rc\xFF\xFF\xFF\xFFa\x1E\xF9\x81\x85_ T\x16Ba(\xF7V[\x84Qc\xCA\x8A\xA7\xC7`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94\x90\x93\x92\x90\x91\x16\x91` \x84`\x04\x81\x88Z\xFA\x93\x84\x15a!\xBDW_\x94a!\x8CW[P\x90c\xFF\xFF\xFF\xFF\x94\x92\x91\x86Q\x93a\x1Fd\x85a\x18\xF9V[\x84R` \x84\x01\x93\x89\x85R\x89_R`\x04` R\x87_ \x91\x88Q\x97\x88\x96c\x15\xD5\x96%`\xE1\x1B\x88R`\xA4\x88\x01\x93`\x01\x80`\xA0\x1B\x03\x90Q\x16`\x04\x89\x01RQ\x16`$\x87\x01R`\xA0`D\x87\x01RQ\x80\x91R`\xC4\x85\x01\x92\x90_[\x81\x81\x10a!jWPPP_\x94\x92a\x1F\xDC\x85\x93\x84\x93`\x03\x19\x85\x83\x03\x01`d\x86\x01Ra\x1D\xEAV[`\x84\x83\x01\x91\x90\x91R\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a!`W_\x90a \xAEW[a \x08\x91Pa\x1A\xC0V[Q\x90_[\x83\x81\x10a ;WPPPP[_R_` R`\x01`\x01``\x1B\x03`@_ T\x16`\x01`\x01``\x1B\x03\x83\x16\x10\x15\x90V[\x84_R`\x03` Ra Ra\x07;\x82\x84_ a\x17\xD6V[a \\\x82\x85a\x1A\xCDV[Qa kW[P`\x01\x01a \x0CV[\x81\x97`\x01`\x01``\x1B\x03g\r\xE0\xB6\xB3\xA7d\0\0a \x9Fa \xA7\x94\x83` a \x94`\x01\x99\x8Ca\x1A\xCDV[Q\x92\x01Q\x16\x90a\x1E*V[\x04\x16\x90a\x1E=V[\x96\x90a bV[P=\x80_\x83>a \xBE\x81\x83a\x19\x14V[\x81\x01\x90` \x81\x83\x03\x12a\x02\x1CW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\x1CW\x81Qa \xF4\x81a\x195V[\x92a!\x01\x85Q\x94\x85a\x19\x14V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x02\x1CW` \x82\x01\x90[\x83\x82\x10a!3WPPPPPa \x08\x90a\x1F\xFEV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CW` \x91a!U\x87\x84\x80\x94\x88\x01\x01a\x1D\x8DV[\x81R\x01\x91\x01\x90a!\x1EV[\x82Q=_\x82>=\x90\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x96P` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a\x1F\xB7V[a!\xAF\x91\x94P` =` \x11a!\xB6W[a!\xA7\x81\x83a\x19\x14V[\x81\x01\x90a\"\xF8V[\x92_a\x1FNV[P=a!\x9DV[\x86Q=_\x82>=\x90\xFD[_\x83\x81R`\x04` \x81\x90R`@\x80\x83 \x81Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x90\x92R`$\x84\x01R\x82\x90\x81\x90a\"\x0C\x90`D\x83\x01\x90a\x1D\xEAV[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x08\x99W_\x91a\"\xBBW[P_[\x82\x81\x10a\"ZWPPPa \x18V[\x83_R`\x03` Ra\"ra\x07;\x82`@_ a\x17\xD6V[a\"|\x82\x84a\x1A\xCDV[Qa\"\x8BW[P`\x01\x01a\"KV[\x81\x96`\x01`\x01``\x1B\x03g\r\xE0\xB6\xB3\xA7d\0\0a \x9Fa\"\xB4\x94\x83` a \x94`\x01\x99\x8Ba\x1A\xCDV[\x95\x90a\"\x82V[\x90P=\x80_\x83>a\"\xCC\x81\x83a\x19\x14V[\x81\x01` \x82\x82\x03\x12a\x02\x1CW\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\"\xF2\x92\x01a\x1D\x8DV[_a\"HV[\x90\x81` \x91\x03\x12a\x02\x1CWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x1CW\x90V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x08\x99W_\x91a$\0W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a#vWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`D\x82\x01R\x7Fer: caller is not the owner of t`d\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x84\x82\x01R`\xA4\x90\xFD[a$\x19\x91P` =` \x11a!\xB6Wa!\xA7\x81\x83a\x19\x14V[_a#dV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a$QWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`D\x82\x01R\x7Fnator: caller is not the Registr`d\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[_\x90\x80_R`\x02` R`@_ `\xFF\x84\x16_R` R`@_ T\x80\x15_\x14a%xWP_Q` a0?_9_Q\x90_R`@`\x01`\x01``\x1B\x03\x94\x83_R`\x02` R\x81_ `\xFF\x82\x16_R` Ra%N\x82_ \x83Q\x90a%5\x82a\x18\xCAV[c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01R_\x85\x83\x01Ra\x1B\xC2V[`\xFF\x82Q\x91\x16\x81R_` \x82\x01R\xA2\x16_\x81\x81\x03\x91\x12_\x82\x12\x81\x16\x90_\x83\x13\x90\x15\x16\x17a\x03\xDFW\x90V[\x90\x80\x92P_R`\x02` R`@_ `\xFF\x84\x16_R` R`@_ \x90_\x19\x81\x01\x90\x81\x11a\x03\xDFWa%\xA9\x91a\x17\xD6V[P\x90\x81T\x91`\x01`\x01``\x1B\x03\x83`@\x1C\x16\x92\x83\x15a&lW`\x01`\x01``\x1B\x03\x94_Q` a0?_9_Q\x90_R\x92`@\x92c\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x81\x03a&\x0EWP\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16\x90Ua%NV[\x81Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x82\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x91Ua&g\x90\x85_R`\x02` R\x83_ `\xFF\x84\x16_R` R\x83_ \x84Q\x91a&U\x83a\x18\xCAV[\x82R_` \x83\x01R_\x85\x83\x01Ra\x1B\xC2V[a%NV[PPPPP_\x90V[\x91\x90\x91_\x90\x80_R`\x02` R`@_ `\xFF\x85\x16_R` R`@_ T\x80\x15_\x14a'$WP_Q` a0?_9_Q\x90_R`@`\x01`\x01``\x1B\x03\x80\x95\x84_R`\x02` R\x82_ `\xFF\x89\x16_R` Ra&\xF8\x83_ \x84Q\x90a&\xDD\x82a\x18\xCAV[c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01R\x84\x84\x16\x86\x83\x01Ra\x1B\xC2V[`\xFF\x83Q\x98\x16\x88R\x16\x95\x86` \x82\x01R\xA2\x16\x90_\x82\x82\x03\x92\x12\x81\x83\x12\x81\x16\x91\x83\x13\x90\x15\x16\x17a\x03\xDFW\x90V[\x90\x80\x92P_R`\x02` R`@_ `\xFF\x85\x16_R` R`@_ \x90_\x19\x81\x01\x90\x81\x11a\x03\xDFWa'U\x91a\x17\xD6V[P\x90\x81T\x91`\x01`\x01``\x1B\x03\x83`@\x1C\x16\x92`\x01`\x01``\x1B\x03\x85\x16\x90\x81\x85\x14a(\x17W\x85_Q` a0?_9_Q\x90_R\x93`\x01`\x01``\x1B\x03\x97c\xFF\xFF\xFF\xFF`@\x95\x8A\x95\x82C\x16\x92\x83\x91\x16\x14_\x14a'\xBBWPPa'\xB6\x91a\x1B\x92V[a&\xF8V[\x83Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x83\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x93Ua'\xB6\x92\x90\x91P\x87_R`\x02` R\x85_ `\xFF\x8C\x16_R` R\x85_ \x90\x86Q\x92a(\x06\x84a\x18\xCAV[\x83R_` \x84\x01R\x86\x83\x01Ra\x1B\xC2V[PPPPPPP_\x90V[`\xFF\x16_\x81\x81R`\x01` R`@\x90 \x80T\x91\x92\x91_\x19\x81\x01\x90\x81\x11a\x03\xDFWa(K\x91a\x17\xD6V[P\x90\x80\x15a(\xE4Wc\xFF\xFF\xFF\xFFa(p\x83T\x92`\x01`\x01``\x1B\x03\x84`@\x1C\x16a/\xF5V[\x93\x84\x92C\x83\x16\x92\x16\x82\x03a(\x89WPPa\x1D\x0C\x91a\x1B\x92V[\x83Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x83\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x93Ua\x1D\x0C\x92\x90\x91P_R`\x01` R`@_ `@Q\x91a(\xC8\x83a\x18\xCAV[\x82R_` \x83\x01R`\x01`\x01``\x1B\x03\x84\x16`@\x83\x01Ra\x1B\xC2V[P`\x01`\x01``\x1B\x03\x91PT`@\x1C\x16\x90V[\x91\x90\x82\x01\x80\x92\x11a\x03\xDFWV[\x81Q\x15a,AW`\xFF\x82Q\x91\x16\x91\x82_R`\x03` R`@_ T\x92` a),\x84\x86a(\xF7V[\x11a+\xDBW_\x92[\x80\x84\x10a)BWPPPPPV[\x90\x91\x92\x93\x94_[a)S\x86\x88a(\xF7V[\x81\x10\x15a)\xEFW\x83_R`\x03` Ra)o\x81`@_ a\x17\xD6V[PT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90a)\x87\x88\x88a\x1A\xCDV[QQ\x16\x14a)\x97W`\x01\x01a)IV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R_Q` a0__9_Q\x90_R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x90\xFD[P\x94\x93\x92\x91\x90\x92`\x01`\x01``\x1B\x03` a*\n\x83\x86a\x1A\xCDV[Q\x01Q\x16\x15a+tW\x81_R`\x03` R`@_ a*)\x82\x85a\x1A\xCDV[Q\x90\x80T`\x01`@\x1B\x81\x10\x15a\x18\xE5Wa*H\x91`\x01\x82\x01\x81Ua\x17\xD6V[a\x10\x8BW\x81Q` \x92\x90\x92\x01Q`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U_\x82\x81R`\x04` R`@\x90 \x90`\x01`\x01`\xA0\x1B\x03a*\x94\x82\x86a\x1A\xCDV[QQ\x16\x82T\x90`\x01`@\x1B\x82\x10\x15a\x18\xE5Wa\x10 \x82`\x01\x95\x86a*\xBA\x95\x01\x81Ua\x17\xD6V[\x82\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04` \x84\x80`\xA0\x1B\x03a*\xEE\x85\x89a\x1A\xCDV[QQ\x16`@Q\x90\x81R\xA2\x82\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x80`\xA0\x1B\x03a+*\x84\x88a\x1A\xCDV[QQ\x16`\x01`\x01``\x1B\x03` a+A\x86\x8Aa\x1A\xCDV[Q\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x84R\x91\x16`\x01`\x01``\x1B\x03\x16` \x83\x01R\x81\x90\x81\x01\x03\x90\xA2\x01\x92a)4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R_Q` a0__9_Q\x90_R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R_Q` a0__9_Q\x90_R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R_Q` a0__9_Q\x90_R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[` `\xFF\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x92\x16\x92\x83_R_\x82R`\x01`\x01``\x1B\x03`@_ \x91\x16\x90\x81`\x01`\x01``\x1B\x03\x19\x82T\x16\x17\x90U`@Q\x90\x81R\xA2V[c\xFF\xFF\xFF\xFF\x80\x82Q\x16\x92\x16\x91\x82\x10a-\xCCW` \x01Qc\xFF\xFF\xFF\xFF\x16\x80\x15\x91\x82\x15a-\xC2W[PP\x15a-\x1EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x10\x90P_\x80a-\x15V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x90\xFD[`\xFF\x16_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x94\x85\x16c\xFF\xFF\xFF\xFF\x19\x82\x16\x81\x17\x90\x92U\x83Q\x94\x16\x84R\x90\x83\x01R\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x91\xA1V[\x92\x91\x90\x83_R`\x02` R`@_ `\xFF\x82\x16_R` R`@_ T\x80[a/\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x90\xFD[\x84_R`\x02` R`@_ `\xFF\x83\x16_R` R`@_ _\x19\x82\x01\x90\x82\x82\x11a\x03\xDFWa/\xC8\x82c\xFF\xFF\xFF\xFF\x92a\x17\xD6V[PT\x16c\xFF\xFF\xFF\xFF\x85\x16\x10\x15a/\xE8WP\x80\x15a\x03\xDFW_\x19\x01\x80a.\xCFV[c\xFF\xFF\xFF\xFF\x16\x94PPPPV[\x90_\x81\x12\x15a0*W`\x01`\xFF\x1B\x81\x14a\x03\xDFW`\x01`\x01``\x1B\x03\x80\x91_\x03\x16\x91\x16\x03`\x01`\x01``\x1B\x03\x81\x11a\x03\xDFW\x90V[\x90`\x01`\x01``\x1B\x03a\x1D\x0C\x92\x16\x90a\x1E=V\xFE/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}StakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \xBB \x94sD0\x91p\x01\xB4)p\x13\xC3\x1C\x80\xEE?f0SQ\xDC\xE8\xEE\r\xEA\x1BqO;WdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080806040526004361015610012575f80fd5b5f3560e01c9081630491b41c1461178857508063087324611461172b5780631f9b74e0146116d757806320b66298146114a957806325504777146113425780632cd95940146112465780633998fdd3146112025780633ca5a5f5146111d05780634bd26e09146111915780635401ed27146111695780635e5a6775146111475780635f1f2d7714610e3857806366acfefe14610da4578063697fbd9314610d665780636b3aa72e14610d225780636d14a98714610cde57806375d4173a14610c395780637c17234714610c1f57806381c07502146109e057806386c06856146109575780639ab4d6ff1461091f5780639f3ccf65146108c65780639f8aff26146107b2578063ac6bfb0314610764578063adc804da146106fc578063b6904b78146106bc578063bc9a40c314610681578063bd29b8cd14610603578063c46778a5146105c9578063c601527d14610576578063c8294c561461052b578063cc5a7c20146103f3578063d5eccc0514610395578063dd9846b91461036e578063df5cf7231461032a578063e086adb3146102ed578063f2be94ae1461027e578063f851e198146102205763fa28c627146101c9575f80fd5b3461021c5760206001600160601b0361020e6102086101e736611a2d565b90825f949394526002875260405f2060ff82165f52875260405f2093612eb0565b906117d6565b505460401c16604051908152f35b5f80fd5b3461021c57604036600319011261021c57606061024661023e6117c6565b600435611c9f565b61027c60405180926001600160601b036040809263ffffffff815116855263ffffffff6020820151166020860152015116910152565bf35b3461021c57608036600319011261021c5760206001600160601b0360406102a36117b6565b6102ab611a1a565b906044355f526002855260ff835f2091165f5284526102e26102db6102d5845f20606435906117d6565b50611ae1565b9182612cef565b015116604051908152f35b3461021c57604036600319011261021c576103286103096117b6565b610311611a1a565b9061031a612317565b61032381611d0f565b612e56565b005b3461021c575f36600319011261021c576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461021c57602061038761038136611a2d565b91612eb0565b63ffffffff60405191168152f35b3461021c57602036600319011261021c5760ff6103b06117b6565b165f90815260016020526040902080545f1981019081116103df5761020e6001600160601b03916020936117d6565b634e487b7160e01b5f52601160045260245ffd5b3461021c57608036600319011261021c5761040c6117b6565b61041461195f565b906044359163ffffffff8316830361021c57606435906001600160401b03821161021c5761047b61044c610481933690600401611975565b61045461241f565b61047561046f8660ff165f52600160205260405f2054151590565b15611b28565b84612904565b82612c99565b60ff811691825f52600560205260405f20600160ff1982541617905560405191602083016002600110156105175783807f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d9260016104e297520390a1612e56565b5f52600160205261032860405f20604051906104fd826118ca565b63ffffffff431682525f60208301525f6040830152611bc2565b634e487b7160e01b5f52602160045260245ffd5b3461021c57606036600319011261021c5760206001600160601b0360406105506117b6565b60ff61055a611a1a565b91165f52600184526102e26102db6102d5604435855f206117d6565b3461021c57604036600319011261021c5761058f6117b6565b602435906001600160401b03821161021c576105b2610328923690600401611975565b906105bb612317565b6105c481611d0f565b612904565b3461021c57602036600319011261021c5760ff6105e46117b6565b165f525f60205260206001600160601b0360405f205416604051908152f35b3461021c57604036600319011261021c576004356024356001600160401b03811161021c5761063690369060040161181b565b61064192919261241f565b5f5b81811061064c57005b8061067a61065d6001938588611ab4565b3560f81c61066a81611d0f565b61067481876124d1565b90612822565b5001610643565b3461021c57604036600319011261021c5761032861069d6117b6565b6106a561195f565b906106ae612317565b6106b781611d0f565b612c99565b3461021c57604036600319011261021c5760ff6106d76117b6565b6106df611c44565b50165f52600160205260606102466102d560243560405f206117d6565b3461021c57604036600319011261021c5760ff6107176117b6565b61071f611c62565b50165f526003602052604061074161073b602435835f206117d6565b50611c7a565b6001600160601b03602083519260018060a01b0381511684520151166020820152f35b3461021c57606036600319011261021c5761077d6117b6565b610785611c44565b506024355f52600260205260ff60405f2091165f5260205260606102466102d560405f20604435906117d6565b3461021c57602036600319011261021c576107cb6117b6565b60405163a4d7871f60e01b815260ff9190911660048201527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316602082602481845afa908115610899576004925f926108a4575b506020906040519384809263cabbb17f60e01b82525afa908115610899576020925f9261086a575b5081610861575b506040519015158152f35b90501582610856565b61088b919250833d8511610892575b6108838183611914565b810190611c2c565b908361084f565b503d610879565b6040513d5f823e3d90fd5b60209192506108bf90823d8411610892576108838183611914565b9190610827565b3461021c57604036600319011261021c576108df6117b6565b60ff60243591165f52600460205260405f20805482101561021c57602091610906916117d6565b905460405160039290921b1c6001600160a01b03168152f35b3461021c57602036600319011261021c5760ff61093a6117b6565b165f526006602052602063ffffffff60405f205416604051908152f35b3461021c57604036600319011261021c576109706117b6565b60243590600282101561021c577f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d9160ff6109db926109ad612317565b6109b681611d0f565b165f52600560205260405f2060ff1981541660ff83161790556040519182918261194c565b0390a1005b3461021c57604036600319011261021c5760043563ffffffff811680910361021c576024356001600160401b03811161021c57610a2190369060040161181b565b90610a2b82611935565b92610a396040519485611914565b828452610a4583611935565b602085019390601f19013685375f5b818110610aa5578486604051918291602083019060208452518091526040830191905f5b818110610a86575050500390f35b825163ffffffff16845285945060209384019390920191600101610a78565b610ab0818386611ab4565b3560f81c610abd81611d0f565b805f52600160205260405f20805415610c0b575f528363ffffffff60205f20541611610b7a57805f52600160205260405f20545f5b818110610b05575b505050600101610a54565b825f52600160205260405f20610b1b8284611b1b565b5f1981019081116103df57610b3663ffffffff9189936117d6565b5054161115610b4757600101610af2565b90610b529250611b1b565b5f198101919082116103df5763ffffffff60019216610b718289611acd565b52908780610afa565b60405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360648201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000608482015260a490fd5b634e487b7160e01b5f52603260045260245ffd5b3461021c575f36600319011261021c576020604051818152f35b3461021c57606036600319011261021c57610c526117b6565b610c5a61195f565b906044356001600160401b03811161021c5760ff9261047b61044c610c83933690600401611975565b165f818152600560209081526040808320805460ff19169055519182527f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d91a15f52600160205261032860405f20604051906104fd826118ca565b3461021c575f36600319011261021c576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461021c575f36600319011261021c576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461021c57602036600319011261021c5760ff610d816117b6565b165f526005602052610da060ff60405f2054166040519182918261194c565b0390f35b3461021c57610db236611848565b929091610dbd61241f565b5f935f5b818110610ddc576040516001600160c01b0387168152602090f35b80610e15610ded6001938589611ab4565b3560f81c610dfa81611d0f565b610e048782611e5d565b15610e1c575b610674908288612675565b5001610dc1565b5083811b60c085901b859003908116991698909817975f610e0a565b3461021c57604036600319011261021c57610e516117b6565b602435906001600160401b03821161021c573660238301121561021c57816004013591610e7d83611935565b92610e8b6040519485611914565b8084526024602085019160051b8301019136831161021c57602401905b82821061113757505050610eba612317565b610ec381611d0f565b81519081156110cc5760ff1691825f52600360205260405f20835f52600460205260405f20935f5b848110610ef457005b817f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f76020610f2c610f258589611acd565b51876117d6565b50546040516001600160a01b039091168152a2817f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a756040610f70610f258589611acd565b505481516001600160a01b0390911681525f6020820152a282545f1981019081116103df57610f9f90846117d6565b50610fb4610fad8387611acd565b51856117d6565b61108b5781810361109e575b505082548015611077575f1901610fd781856117d6565b61108b575f9055835585545f1981019081116103df57610ffa61104491886117d6565b905460039190911b1c6001600160a01b03166110206110198488611acd565b51896117d6565b81546001600160a01b0393841660039290921b91821b9390911b1916919091179055565b8554908115611077576001915f190161105d81896117d6565b815490858060a01b039060031b1b19169055875501610eeb565b634e487b7160e01b5f52603160045260245ffd5b634e487b7160e01b5f525f60045260245ffd5b815481546001600160a01b039091166001600160a01b03199182168117835592541690911790558680610fc0565b60405162461bcd60e51b815260206004820152603d60248201527f5374616b6552656769737472792e72656d6f7665537472617465676965733a2060448201527f6e6f20696e646963657320746f2072656d6f76652070726f76696465640000006064820152608490fd5b8135815260209182019101610ea8565b3461021c575f36600319011261021c576020604051670de0b6b3a76400008152f35b3461021c57604036600319011261021c5760206001600160601b0360406102e261023e6117c6565b3461021c57604036600319011261021c576111aa6117c6565b6004355f52600260205260ff60405f2091165f52602052602060405f2054604051908152f35b3461021c57602036600319011261021c5760ff6111eb6117b6565b165f526003602052602060405f2054604051908152f35b3461021c575f36600319011261021c576040517f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03168152602090f35b3461021c57604036600319011261021c5761125f6117c6565b6004355f52600260205260ff60405f2091165f5260205260405f2080549061128682611935565b916112946040519384611914565b8083526020830180925f5260205f205f915b838310611325578486604051918291602083019060208452518091526040830191905f5b8181106112d8575050500390f35b91935091602060608261131760019488516001600160601b036040809263ffffffff815116855263ffffffff6020820151166020860152015116910152565b0194019101918493926112ca565b60016020819261133485611ae1565b8152019201920191906112a6565b3461021c5761135036611848565b9061135c93929361241f565b61136582611a82565b9261136f83611a82565b925f5b8181106113a35761139586610da08760405193849360408552604085019061188e565b90838203602085015261188e565b6113ae818386611ab4565b3560f81c906113bc82611d0f565b6113c68483611e5d565b929092156114185782816113e06001956113fc948d612675565b916001600160601b036113f3868d611acd565b91169052612822565b6001600160601b0361140e8389611acd565b9116905201611372565b60405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a2060448201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360648201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000608482015260a490fd5b3461021c57606036600319011261021c576114c26117b6565b6024356001600160401b03811161021c576114e19036906004016117eb565b916044356001600160401b03811161021c576115019036906004016117eb565b909161150b612317565b61151481611d0f565b841561166d578482036116025760ff1691825f52600360205260405f20935f5b86811061153d57005b8061158f6115566115516001948888611a5e565b611a6e565b61156b611564848c88611a5e565b358a6117d6565b5080546001600160a01b031660a09290921b6001600160a01b031916919091179055565b857f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a756115bf611564848c88611a5e565b50848060a01b039054166115d7611551858a8a611a5e565b604080516001600160a01b039390931683526001600160601b0391909116602083015290a201611534565b60405162461bcd60e51b815260206004820152603960248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a20696e707574206c656e677468206d69736d61746368000000000000006064820152608490fd5b608460405162461bcd60e51b815260206004820152604060248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a206e6f20737472617465677920696e64696365732070726f76696465646064820152fd5b3461021c57604036600319011261021c576116f06117b6565b602435906001600160a01b038216820361021c576020918161171461171993611d0f565b611e5d565b506001600160601b0360405191168152f35b3461021c57604036600319011261021c576117446117b6565b60ff60243591165f52600360205260405f20805482101561021c5760409161176b916117d6565b505481516001600160a01b038216815260a09190911c6020820152f35b3461021c57602036600319011261021c5760209060ff6117a66117b6565b165f526001825260405f20548152f35b6004359060ff8216820361021c57565b6024359060ff8216820361021c57565b8054821015610c0b575f5260205f2001905f90565b9181601f8401121561021c578235916001600160401b03831161021c576020808501948460051b01011161021c57565b9181601f8401121561021c578235916001600160401b03831161021c576020838186019501011161021c57565b606060031982011261021c576004356001600160a01b038116810361021c579160243591604435906001600160401b03821161021c5761188a9160040161181b565b9091565b90602080835192838152019201905f5b8181106118ab5750505090565b82516001600160601b031684526020938401939092019160010161189e565b606081019081106001600160401b038211176118e557604052565b634e487b7160e01b5f52604160045260245ffd5b604081019081106001600160401b038211176118e557604052565b90601f801991011681019081106001600160401b038211176118e557604052565b6001600160401b0381116118e55760051b60200190565b9190602083019260028210156105175752565b602435906001600160601b038216820361021c57565b81601f8201121561021c5780359061198c82611935565b9261199a6040519485611914565b82845260208085019360061b8301019181831161021c57602001925b8284106119c4575050505090565b60408483031261021c57604051906119db826118f9565b84356001600160a01b038116810361021c5782526020850135906001600160601b038216820361021c57826020928360409501528152019301926119b6565b6024359063ffffffff8216820361021c57565b606090600319011261021c576004359060243560ff8116810361021c579060443563ffffffff8116810361021c5790565b9190811015610c0b5760051b0190565b356001600160601b038116810361021c5790565b90611a8c82611935565b611a996040519182611914565b8281528092611aaa601f1991611935565b0190602036910137565b90821015610c0b570190565b805115610c0b5760200190565b8051821015610c0b5760209160051b010190565b90604051611aee816118ca565b60406001600160601b0382945463ffffffff8116845263ffffffff8160201c166020850152821c16910152565b919082039182116103df57565b15611b2f57565b60405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a2060448201527471756f72756d20616c72656164792065786973747360581b6064820152608490fd5b906bffffffffffffffffffffffff60401b82549160401b16906bffffffffffffffffffffffff60401b1916179055565b8054600160401b8110156118e557611bdf916001820181556117d6565b61108b578151815460208085015167ffffffff00000000911b1663ffffffff90921667ffffffffffffffff1990911617178155611c2a916001600160601b0390604001511690611b92565b565b9081602091031261021c5751801515810361021c5790565b60405190611c51826118ca565b5f6040838281528260208201520152565b60405190611c6f826118f9565b5f6020838281520152565b90604051611c87816118f9565b91546001600160a01b038116835260a01c6020830152565b90611ca8611c44565b50815f52600260205260405f2060ff82165f5260205260405f205490611ccc611c44565b9282611cd85750505090565b909192505f52600260205260ff60405f2091165f5260205260405f205f1982019182116103df57611d0c916102d5916117d6565b90565b611d279060ff165f52600160205260405f2054151590565b15611d2e57565b60405162461bcd60e51b815260206004820152603160248201527f5374616b6552656769737472792e71756f72756d4578697374733a2071756f726044820152701d5b48191bd95cc81b9bdd08195e1a5cdd607a1b6064820152608490fd5b9080601f8301121561021c578151611da481611935565b92611db26040519485611914565b81845260208085019260051b82010192831161021c57602001905b828210611dda5750505090565b8151815260209182019101611dcd565b90602082549182815201915f5260205f20905f5b818110611e0b5750505090565b82546001600160a01b0316845260209093019260019283019201611dfe565b818102929181159184041417156103df57565b906001600160601b03809116911601906001600160601b0382116103df57565b919060ff5f931690815f52600360205260405f205490604051611e7f816118f9565b5f81525f602082015250825f52600560205260ff60405f2054166002811015610517576001036121c7576040908151611eb88382611914565b600181526020810191601f198401368437611ed282611ac0565b9060018060a01b03169052845f52600660205263ffffffff611ef981855f205416426128f7565b845163ca8aa7c760e01b81527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03169490939290911691602084600481885afa9384156121bd575f9461218c575b509063ffffffff949291865193611f64856118f9565b84526020840193898552895f526004602052875f209188519788966315d5962560e11b885260a488019360018060a01b0390511660048901525116602487015260a060448701525180915260c4850192905f5b81811061216a575050505f9492611fdc85938493600319858303016064860152611dea565b608483019190915203916001600160a01b03165afa8015612160575f906120ae575b6120089150611ac0565b51905f5b83811061203b57505050505b5f525f6020526001600160601b0360405f2054166001600160601b038316101590565b845f52600360205261205261073b82845f206117d6565b61205c8285611acd565b5161206b575b5060010161200c565b81976001600160601b03670de0b6b3a764000061209f6120a7948360206120946001998c611acd565b519201511690611e2a565b041690611e3d565b9690612062565b503d805f833e6120be8183611914565b81019060208183031261021c578051906001600160401b03821161021c57019080601f8301121561021c5781516120f481611935565b9261210185519485611914565b81845260208085019260051b8201019183831161021c5760208201905b83821061213357505050505061200890611ffe565b81516001600160401b03811161021c5760209161215587848094880101611d8d565b81520191019061211e565b82513d5f823e3d90fd5b82516001600160a01b0316855288965060209485019490920191600101611fb7565b6121af91945060203d6020116121b6575b6121a78183611914565b8101906122f8565b925f611f4e565b503d61219d565b86513d5f823e3d90fd5b5f8381526004602081905260408083208151639004134760e01b81526001600160a01b039095169285019290925260248401528290819061220c906044830190611dea565b03817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610899575f916122bb575b505f5b82811061225a57505050612018565b835f52600360205261227261073b8260405f206117d6565b61227c8284611acd565b5161228b575b5060010161224b565b81966001600160601b03670de0b6b3a764000061209f6122b4948360206120946001998b611acd565b9590612282565b90503d805f833e6122cc8183611914565b810160208282031261021c5781516001600160401b03811161021c576122f29201611d8d565b5f612248565b9081602091031261021c57516001600160a01b038116810361021c5790565b604051638da5cb5b60e01b81526020816004817f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03165afa908115610899575f91612400575b506001600160a01b0316330361237657565b60405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e60448201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f6620746064820152753432903932b3b4b9ba393ca1b7b7b93234b730ba37b960511b608482015260a490fd5b612419915060203d6020116121b6576121a78183611914565b5f612364565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316330361245157565b60405162461bcd60e51b815260206004820152604c60248201527f5374616b6552656769737472792e6f6e6c795265676973747279436f6f72646960448201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260648201526b3ca1b7b7b93234b730ba37b960a11b608482015260a490fd5b5f90805f52600260205260405f2060ff84165f5260205260405f205480155f1461257857505f51602061303f5f395f51905f5260406001600160601b0394835f526002602052815f2060ff82165f5260205261254e825f20835190612535826118ca565b63ffffffff431682525f60208301525f85830152611bc2565b60ff8251911681525f6020820152a2165f81810391125f82128116905f8313901516176103df5790565b908092505f52600260205260405f2060ff84165f5260205260405f20905f1981019081116103df576125a9916117d6565b50908154916001600160601b038360401c1692831561266c576001600160601b03945f51602061303f5f395f51905f529260409263ffffffff4381169116810361260e5750805473ffffffffffffffffffffffff00000000000000001916905561254e565b815467ffffffff000000001916602082901b67ffffffff00000000161790915561266790855f526002602052835f2060ff84165f52602052835f20845191612655836118ca565b82525f60208301525f85830152611bc2565b61254e565b50505050505f90565b9190915f90805f52600260205260405f2060ff85165f5260205260405f205480155f1461272457505f51602061303f5f395f51905f5260406001600160601b038095845f526002602052825f2060ff89165f526020526126f8835f208451906126dd826118ca565b63ffffffff431682525f602083015284841686830152611bc2565b60ff8351981688521695866020820152a216905f82820392128183128116918313901516176103df5790565b908092505f52600260205260405f2060ff85165f5260205260405f20905f1981019081116103df57612755916117d6565b50908154916001600160601b038360401c16926001600160601b0385169081851461281757855f51602061303f5f395f51905f52936001600160601b039763ffffffff6040958a9582431692839116145f146127bb5750506127b691611b92565b6126f8565b835467ffffffff000000001916602083901b67ffffffff0000000016179093556127b692909150875f526002602052855f2060ff8c165f52602052855f2090865192612806846118ca565b83525f602084015286830152611bc2565b505050505050505f90565b60ff165f81815260016020526040902080549192915f1981019081116103df5761284b916117d6565b509080156128e45763ffffffff6128708354926001600160601b038460401c16612ff5565b93849243831692168203612889575050611d0c91611b92565b835467ffffffff000000001916602083901b67ffffffff000000001617909355611d0c929091505f52600160205260405f20604051916128c8836118ca565b82525f60208301526001600160601b0384166040830152611bc2565b506001600160601b0391505460401c1690565b919082018092116103df57565b815115612c415760ff8251911691825f52600360205260405f205492602061292c84866128f7565b11612bdb575f925b808410612942575050505050565b90919293945f5b61295386886128f7565b8110156129ef57835f52600360205261296f8160405f206117d6565b50546001600160a01b03908116906129878888611acd565b5151161461299757600101612949565b60405162461bcd60e51b815260206004820152603d60248201525f51602061305f5f395f51905f5260448201527f3a2063616e6e6f74206164642073616d652073747261746567792032780000006064820152608490fd5b509493929190926001600160601b036020612a0a8386611acd565b5101511615612b7457815f52600360205260405f20612a298285611acd565b51908054600160401b8110156118e557612a48916001820181556117d6565b61108b5781516020929092015160a01b6001600160a01b0319166001600160a01b03929092169190911790555f828152600460205260409020906001600160a01b03612a948286611acd565b515116825490600160401b8210156118e5576110208260019586612aba950181556117d6565b827f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f54046020848060a01b03612aee8589611acd565b515116604051908152a2827f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838060a01b03612b2a8488611acd565b5151166001600160601b036020612b41868a611acd565b510151604080516001600160a01b0394909416845291166001600160601b03166020830152819081010390a20192612934565b60405162461bcd60e51b815260206004820152604660248201525f51602061305f5f395f51905f5260448201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f206064820152651dd95a59da1d60d21b608482015260a490fd5b60405162461bcd60e51b815260206004820152604560248201525f51602061305f5f395f51905f5260448201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60648201526408a9c8ea8960db1b608482015260a490fd5b60405162461bcd60e51b815260206004820152603860248201525f51602061305f5f395f51905f5260448201527f3a206e6f20737472617465676965732070726f766964656400000000000000006064820152608490fd5b602060ff7f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf921692835f525f82526001600160601b0360405f20911690816001600160601b0319825416179055604051908152a2565b63ffffffff808251169216918210612dcc576020015163ffffffff168015918215612dc2575b505015612d1e57565b60405162461bcd60e51b815260206004820152606a60248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560648201527f72207374616b6555706461746520617661696c61626c65206265666f726520626084820152693637b1b5a73ab6b132b960b11b60a482015260c490fd5b1090505f80612d15565b60405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a207374616b6555706461746520697320606482015275333937b69030b33a32b910313637b1b5a73ab6b132b960511b608482015260a490fd5b60ff165f90815260066020908152604091829020805463ffffffff94851663ffffffff1982168117909255835194168452908301527f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c791a1565b929190835f52600260205260405f2060ff82165f5260205260405f2054805b612f945760405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e490fd5b845f52600260205260405f2060ff83165f5260205260405f205f198201908282116103df57612fc88263ffffffff926117d6565b50541663ffffffff85161015612fe8575080156103df575f190180612ecf565b63ffffffff169450505050565b905f81121561302a57600160ff1b81146103df576001600160601b0380915f03169116036001600160601b0381116103df5790565b906001600160601b03611d0c921690611e3d56fe2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d5374616b6552656769737472792e5f6164645374726174656779506172616d73a2646970667358221220bb2094734430917001b4297013c31c80ee3f66305351dce8ee0dea1b714f3b5764736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80\x80`@R`\x046\x10\x15a\0\x12W_\x80\xFD[_5`\xE0\x1C\x90\x81c\x04\x91\xB4\x1C\x14a\x17\x88WP\x80c\x08s$a\x14a\x17+W\x80c\x1F\x9Bt\xE0\x14a\x16\xD7W\x80c \xB6b\x98\x14a\x14\xA9W\x80c%PGw\x14a\x13BW\x80c,\xD9Y@\x14a\x12FW\x80c9\x98\xFD\xD3\x14a\x12\x02W\x80c<\xA5\xA5\xF5\x14a\x11\xD0W\x80cK\xD2n\t\x14a\x11\x91W\x80cT\x01\xED'\x14a\x11iW\x80c^Zgu\x14a\x11GW\x80c_\x1F-w\x14a\x0E8W\x80cf\xAC\xFE\xFE\x14a\r\xA4W\x80ci\x7F\xBD\x93\x14a\rfW\x80ck:\xA7.\x14a\r\"W\x80cm\x14\xA9\x87\x14a\x0C\xDEW\x80cu\xD4\x17:\x14a\x0C9W\x80c|\x17#G\x14a\x0C\x1FW\x80c\x81\xC0u\x02\x14a\t\xE0W\x80c\x86\xC0hV\x14a\tWW\x80c\x9A\xB4\xD6\xFF\x14a\t\x1FW\x80c\x9F<\xCFe\x14a\x08\xC6W\x80c\x9F\x8A\xFF&\x14a\x07\xB2W\x80c\xACk\xFB\x03\x14a\x07dW\x80c\xAD\xC8\x04\xDA\x14a\x06\xFCW\x80c\xB6\x90Kx\x14a\x06\xBCW\x80c\xBC\x9A@\xC3\x14a\x06\x81W\x80c\xBD)\xB8\xCD\x14a\x06\x03W\x80c\xC4gx\xA5\x14a\x05\xC9W\x80c\xC6\x01R}\x14a\x05vW\x80c\xC8)LV\x14a\x05+W\x80c\xCCZ| \x14a\x03\xF3W\x80c\xD5\xEC\xCC\x05\x14a\x03\x95W\x80c\xDD\x98F\xB9\x14a\x03nW\x80c\xDF\\\xF7#\x14a\x03*W\x80c\xE0\x86\xAD\xB3\x14a\x02\xEDW\x80c\xF2\xBE\x94\xAE\x14a\x02~W\x80c\xF8Q\xE1\x98\x14a\x02 Wc\xFA(\xC6'\x14a\x01\xC9W_\x80\xFD[4a\x02\x1CW` `\x01`\x01``\x1B\x03a\x02\x0Ea\x02\x08a\x01\xE76a\x1A-V[\x90\x82_\x94\x93\x94R`\x02\x87R`@_ `\xFF\x82\x16_R\x87R`@_ \x93a.\xB0V[\x90a\x17\xD6V[PT`@\x1C\x16`@Q\x90\x81R\xF3[_\x80\xFD[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW``a\x02Fa\x02>a\x17\xC6V[`\x045a\x1C\x9FV[a\x02|`@Q\x80\x92`\x01`\x01``\x1B\x03`@\x80\x92c\xFF\xFF\xFF\xFF\x81Q\x16\x85Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[\xF3[4a\x02\x1CW`\x806`\x03\x19\x01\x12a\x02\x1CW` `\x01`\x01``\x1B\x03`@a\x02\xA3a\x17\xB6V[a\x02\xABa\x1A\x1AV[\x90`D5_R`\x02\x85R`\xFF\x83_ \x91\x16_R\x84Ra\x02\xE2a\x02\xDBa\x02\xD5\x84_ `d5\x90a\x17\xD6V[Pa\x1A\xE1V[\x91\x82a,\xEFV[\x01Q\x16`@Q\x90\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x03(a\x03\ta\x17\xB6V[a\x03\x11a\x1A\x1AV[\x90a\x03\x1Aa#\x17V[a\x03#\x81a\x1D\x0FV[a.VV[\0[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x1CW` a\x03\x87a\x03\x816a\x1A-V[\x91a.\xB0V[c\xFF\xFF\xFF\xFF`@Q\x91\x16\x81R\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x03\xB0a\x17\xB6V[\x16_\x90\x81R`\x01` R`@\x90 \x80T_\x19\x81\x01\x90\x81\x11a\x03\xDFWa\x02\x0E`\x01`\x01``\x1B\x03\x91` \x93a\x17\xD6V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[4a\x02\x1CW`\x806`\x03\x19\x01\x12a\x02\x1CWa\x04\x0Ca\x17\xB6V[a\x04\x14a\x19_V[\x90`D5\x91c\xFF\xFF\xFF\xFF\x83\x16\x83\x03a\x02\x1CW`d5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CWa\x04{a\x04La\x04\x81\x936\x90`\x04\x01a\x19uV[a\x04Ta$\x1FV[a\x04ua\x04o\x86`\xFF\x16_R`\x01` R`@_ T\x15\x15\x90V[\x15a\x1B(V[\x84a)\x04V[\x82a,\x99V[`\xFF\x81\x16\x91\x82_R`\x05` R`@_ `\x01`\xFF\x19\x82T\x16\x17\x90U`@Q\x91` \x83\x01`\x02`\x01\x10\x15a\x05\x17W\x83\x80\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x92`\x01a\x04\xE2\x97R\x03\x90\xA1a.VV[_R`\x01` Ra\x03(`@_ `@Q\x90a\x04\xFD\x82a\x18\xCAV[c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01R_`@\x83\x01Ra\x1B\xC2V[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[4a\x02\x1CW``6`\x03\x19\x01\x12a\x02\x1CW` `\x01`\x01``\x1B\x03`@a\x05Pa\x17\xB6V[`\xFFa\x05Za\x1A\x1AV[\x91\x16_R`\x01\x84Ra\x02\xE2a\x02\xDBa\x02\xD5`D5\x85_ a\x17\xD6V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x05\x8Fa\x17\xB6V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CWa\x05\xB2a\x03(\x926\x90`\x04\x01a\x19uV[\x90a\x05\xBBa#\x17V[a\x05\xC4\x81a\x1D\x0FV[a)\x04V[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x05\xE4a\x17\xB6V[\x16_R_` R` `\x01`\x01``\x1B\x03`@_ T\x16`@Q\x90\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW`\x045`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\x066\x906\x90`\x04\x01a\x18\x1BV[a\x06A\x92\x91\x92a$\x1FV[_[\x81\x81\x10a\x06LW\0[\x80a\x06za\x06]`\x01\x93\x85\x88a\x1A\xB4V[5`\xF8\x1Ca\x06j\x81a\x1D\x0FV[a\x06t\x81\x87a$\xD1V[\x90a(\"V[P\x01a\x06CV[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x03(a\x06\x9Da\x17\xB6V[a\x06\xA5a\x19_V[\x90a\x06\xAEa#\x17V[a\x06\xB7\x81a\x1D\x0FV[a,\x99V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x06\xD7a\x17\xB6V[a\x06\xDFa\x1CDV[P\x16_R`\x01` R``a\x02Fa\x02\xD5`$5`@_ a\x17\xD6V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x07\x17a\x17\xB6V[a\x07\x1Fa\x1CbV[P\x16_R`\x03` R`@a\x07Aa\x07;`$5\x83_ a\x17\xD6V[Pa\x1CzV[`\x01`\x01``\x1B\x03` \x83Q\x92`\x01\x80`\xA0\x1B\x03\x81Q\x16\x84R\x01Q\x16` \x82\x01R\xF3[4a\x02\x1CW``6`\x03\x19\x01\x12a\x02\x1CWa\x07}a\x17\xB6V[a\x07\x85a\x1CDV[P`$5_R`\x02` R`\xFF`@_ \x91\x16_R` R``a\x02Fa\x02\xD5`@_ `D5\x90a\x17\xD6V[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CWa\x07\xCBa\x17\xB6V[`@Qc\xA4\xD7\x87\x1F`\xE0\x1B\x81R`\xFF\x91\x90\x91\x16`\x04\x82\x01R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16` \x82`$\x81\x84Z\xFA\x90\x81\x15a\x08\x99W`\x04\x92_\x92a\x08\xA4W[P` \x90`@Q\x93\x84\x80\x92c\xCA\xBB\xB1\x7F`\xE0\x1B\x82RZ\xFA\x90\x81\x15a\x08\x99W` \x92_\x92a\x08jW[P\x81a\x08aW[P`@Q\x90\x15\x15\x81R\xF3[\x90P\x15\x82a\x08VV[a\x08\x8B\x91\x92P\x83=\x85\x11a\x08\x92W[a\x08\x83\x81\x83a\x19\x14V[\x81\x01\x90a\x1C,V[\x90\x83a\x08OV[P=a\x08yV[`@Q=_\x82>=\x90\xFD[` \x91\x92Pa\x08\xBF\x90\x82=\x84\x11a\x08\x92Wa\x08\x83\x81\x83a\x19\x14V[\x91\x90a\x08'V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x08\xDFa\x17\xB6V[`\xFF`$5\x91\x16_R`\x04` R`@_ \x80T\x82\x10\x15a\x02\x1CW` \x91a\t\x06\x91a\x17\xD6V[\x90T`@Q`\x03\x92\x90\x92\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16\x81R\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\t:a\x17\xB6V[\x16_R`\x06` R` c\xFF\xFF\xFF\xFF`@_ T\x16`@Q\x90\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\tpa\x17\xB6V[`$5\x90`\x02\x82\x10\x15a\x02\x1CW\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x91`\xFFa\t\xDB\x92a\t\xADa#\x17V[a\t\xB6\x81a\x1D\x0FV[\x16_R`\x05` R`@_ `\xFF\x19\x81T\x16`\xFF\x83\x16\x17\x90U`@Q\x91\x82\x91\x82a\x19LV[\x03\x90\xA1\0[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW`\x045c\xFF\xFF\xFF\xFF\x81\x16\x80\x91\x03a\x02\x1CW`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\n!\x906\x90`\x04\x01a\x18\x1BV[\x90a\n+\x82a\x195V[\x92a\n9`@Q\x94\x85a\x19\x14V[\x82\x84Ra\nE\x83a\x195V[` \x85\x01\x93\x90`\x1F\x19\x016\x857_[\x81\x81\x10a\n\xA5W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\n\x86WPPP\x03\x90\xF3[\x82Qc\xFF\xFF\xFF\xFF\x16\x84R\x85\x94P` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\nxV[a\n\xB0\x81\x83\x86a\x1A\xB4V[5`\xF8\x1Ca\n\xBD\x81a\x1D\x0FV[\x80_R`\x01` R`@_ \x80T\x15a\x0C\x0BW_R\x83c\xFF\xFF\xFF\xFF` _ T\x16\x11a\x0BzW\x80_R`\x01` R`@_ T_[\x81\x81\x10a\x0B\x05W[PPP`\x01\x01a\nTV[\x82_R`\x01` R`@_ a\x0B\x1B\x82\x84a\x1B\x1BV[_\x19\x81\x01\x90\x81\x11a\x03\xDFWa\x0B6c\xFF\xFF\xFF\xFF\x91\x89\x93a\x17\xD6V[PT\x16\x11\x15a\x0BGW`\x01\x01a\n\xF2V[\x90a\x0BR\x92Pa\x1B\x1BV[_\x19\x81\x01\x91\x90\x82\x11a\x03\xDFWc\xFF\xFF\xFF\xFF`\x01\x92\x16a\x0Bq\x82\x89a\x1A\xCDV[R\x90\x87\x80a\n\xFAV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW` `@Q\x81\x81R\xF3[4a\x02\x1CW``6`\x03\x19\x01\x12a\x02\x1CWa\x0CRa\x17\xB6V[a\x0CZa\x19_V[\x90`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CW`\xFF\x92a\x04{a\x04La\x0C\x83\x936\x90`\x04\x01a\x19uV[\x16_\x81\x81R`\x05` \x90\x81R`@\x80\x83 \x80T`\xFF\x19\x16\x90UQ\x91\x82R\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x91\xA1_R`\x01` Ra\x03(`@_ `@Q\x90a\x04\xFD\x82a\x18\xCAV[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\r\x81a\x17\xB6V[\x16_R`\x05` Ra\r\xA0`\xFF`@_ T\x16`@Q\x91\x82\x91\x82a\x19LV[\x03\x90\xF3[4a\x02\x1CWa\r\xB26a\x18HV[\x92\x90\x91a\r\xBDa$\x1FV[_\x93_[\x81\x81\x10a\r\xDCW`@Q`\x01`\x01`\xC0\x1B\x03\x87\x16\x81R` \x90\xF3[\x80a\x0E\x15a\r\xED`\x01\x93\x85\x89a\x1A\xB4V[5`\xF8\x1Ca\r\xFA\x81a\x1D\x0FV[a\x0E\x04\x87\x82a\x1E]V[\x15a\x0E\x1CW[a\x06t\x90\x82\x88a&uV[P\x01a\r\xC1V[P\x83\x81\x1B`\xC0\x85\x90\x1B\x85\x90\x03\x90\x81\x16\x99\x16\x98\x90\x98\x17\x97_a\x0E\nV[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x0EQa\x17\xB6V[`$5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CW6`#\x83\x01\x12\x15a\x02\x1CW\x81`\x04\x015\x91a\x0E}\x83a\x195V[\x92a\x0E\x8B`@Q\x94\x85a\x19\x14V[\x80\x84R`$` \x85\x01\x91`\x05\x1B\x83\x01\x01\x916\x83\x11a\x02\x1CW`$\x01\x90[\x82\x82\x10a\x117WPPPa\x0E\xBAa#\x17V[a\x0E\xC3\x81a\x1D\x0FV[\x81Q\x90\x81\x15a\x10\xCCW`\xFF\x16\x91\x82_R`\x03` R`@_ \x83_R`\x04` R`@_ \x93_[\x84\x81\x10a\x0E\xF4W\0[\x81\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7` a\x0F,a\x0F%\x85\x89a\x1A\xCDV[Q\x87a\x17\xD6V[PT`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\xA2\x81\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au`@a\x0Fpa\x0F%\x85\x89a\x1A\xCDV[PT\x81Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R_` \x82\x01R\xA2\x82T_\x19\x81\x01\x90\x81\x11a\x03\xDFWa\x0F\x9F\x90\x84a\x17\xD6V[Pa\x0F\xB4a\x0F\xAD\x83\x87a\x1A\xCDV[Q\x85a\x17\xD6V[a\x10\x8BW\x81\x81\x03a\x10\x9EW[PP\x82T\x80\x15a\x10wW_\x19\x01a\x0F\xD7\x81\x85a\x17\xD6V[a\x10\x8BW_\x90U\x83U\x85T_\x19\x81\x01\x90\x81\x11a\x03\xDFWa\x0F\xFAa\x10D\x91\x88a\x17\xD6V[\x90T`\x03\x91\x90\x91\x1B\x1C`\x01`\x01`\xA0\x1B\x03\x16a\x10 a\x10\x19\x84\x88a\x1A\xCDV[Q\x89a\x17\xD6V[\x81T`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\x03\x92\x90\x92\x1B\x91\x82\x1B\x93\x90\x91\x1B\x19\x16\x91\x90\x91\x17\x90UV[\x85T\x90\x81\x15a\x10wW`\x01\x91_\x19\x01a\x10]\x81\x89a\x17\xD6V[\x81T\x90\x85\x80`\xA0\x1B\x03\x90`\x03\x1B\x1B\x19\x16\x90U\x87U\x01a\x0E\xEBV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[cNH{q`\xE0\x1B_R_`\x04R`$_\xFD[\x81T\x81T`\x01`\x01`\xA0\x1B\x03\x90\x91\x16`\x01`\x01`\xA0\x1B\x03\x19\x91\x82\x16\x81\x17\x83U\x92T\x16\x90\x91\x17\x90U\x86\x80a\x0F\xC0V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x90\xFD[\x815\x81R` \x91\x82\x01\x91\x01a\x0E\xA8V[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW` `@Qg\r\xE0\xB6\xB3\xA7d\0\0\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CW` `\x01`\x01``\x1B\x03`@a\x02\xE2a\x02>a\x17\xC6V[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x11\xAAa\x17\xC6V[`\x045_R`\x02` R`\xFF`@_ \x91\x16_R` R` `@_ T`@Q\x90\x81R\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW`\xFFa\x11\xEBa\x17\xB6V[\x16_R`\x03` R` `@_ T`@Q\x90\x81R\xF3[4a\x02\x1CW_6`\x03\x19\x01\x12a\x02\x1CW`@Q\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x90\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x12_a\x17\xC6V[`\x045_R`\x02` R`\xFF`@_ \x91\x16_R` R`@_ \x80T\x90a\x12\x86\x82a\x195V[\x91a\x12\x94`@Q\x93\x84a\x19\x14V[\x80\x83R` \x83\x01\x80\x92_R` _ _\x91[\x83\x83\x10a\x13%W\x84\x86`@Q\x91\x82\x91` \x83\x01\x90` \x84RQ\x80\x91R`@\x83\x01\x91\x90_[\x81\x81\x10a\x12\xD8WPPP\x03\x90\xF3[\x91\x93P\x91` ``\x82a\x13\x17`\x01\x94\x88Q`\x01`\x01``\x1B\x03`@\x80\x92c\xFF\xFF\xFF\xFF\x81Q\x16\x85Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x86\x01R\x01Q\x16\x91\x01RV[\x01\x94\x01\x91\x01\x91\x84\x93\x92a\x12\xCAV[`\x01` \x81\x92a\x134\x85a\x1A\xE1V[\x81R\x01\x92\x01\x92\x01\x91\x90a\x12\xA6V[4a\x02\x1CWa\x13P6a\x18HV[\x90a\x13\\\x93\x92\x93a$\x1FV[a\x13e\x82a\x1A\x82V[\x92a\x13o\x83a\x1A\x82V[\x92_[\x81\x81\x10a\x13\xA3Wa\x13\x95\x86a\r\xA0\x87`@Q\x93\x84\x93`@\x85R`@\x85\x01\x90a\x18\x8EV[\x90\x83\x82\x03` \x85\x01Ra\x18\x8EV[a\x13\xAE\x81\x83\x86a\x1A\xB4V[5`\xF8\x1C\x90a\x13\xBC\x82a\x1D\x0FV[a\x13\xC6\x84\x83a\x1E]V[\x92\x90\x92\x15a\x14\x18W\x82\x81a\x13\xE0`\x01\x95a\x13\xFC\x94\x8Da&uV[\x91`\x01`\x01``\x1B\x03a\x13\xF3\x86\x8Da\x1A\xCDV[\x91\x16\x90Ra(\"V[`\x01`\x01``\x1B\x03a\x14\x0E\x83\x89a\x1A\xCDV[\x91\x16\x90R\x01a\x13rV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x90\xFD[4a\x02\x1CW``6`\x03\x19\x01\x12a\x02\x1CWa\x14\xC2a\x17\xB6V[`$5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\x14\xE1\x906\x90`\x04\x01a\x17\xEBV[\x91`D5`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\x15\x01\x906\x90`\x04\x01a\x17\xEBV[\x90\x91a\x15\x0Ba#\x17V[a\x15\x14\x81a\x1D\x0FV[\x84\x15a\x16mW\x84\x82\x03a\x16\x02W`\xFF\x16\x91\x82_R`\x03` R`@_ \x93_[\x86\x81\x10a\x15=W\0[\x80a\x15\x8Fa\x15Va\x15Q`\x01\x94\x88\x88a\x1A^V[a\x1AnV[a\x15ka\x15d\x84\x8C\x88a\x1A^V[5\x8Aa\x17\xD6V[P\x80T`\x01`\x01`\xA0\x1B\x03\x16`\xA0\x92\x90\x92\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16\x91\x90\x91\x17\x90UV[\x85\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Aua\x15\xBFa\x15d\x84\x8C\x88a\x1A^V[P\x84\x80`\xA0\x1B\x03\x90T\x16a\x15\xD7a\x15Q\x85\x8A\x8Aa\x1A^V[`@\x80Q`\x01`\x01`\xA0\x1B\x03\x93\x90\x93\x16\x83R`\x01`\x01``\x1B\x03\x91\x90\x91\x16` \x83\x01R\x90\xA2\x01a\x154V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[`\x84`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`@`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R\xFD[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x16\xF0a\x17\xB6V[`$5\x90`\x01`\x01`\xA0\x1B\x03\x82\x16\x82\x03a\x02\x1CW` \x91\x81a\x17\x14a\x17\x19\x93a\x1D\x0FV[a\x1E]V[P`\x01`\x01``\x1B\x03`@Q\x91\x16\x81R\xF3[4a\x02\x1CW`@6`\x03\x19\x01\x12a\x02\x1CWa\x17Da\x17\xB6V[`\xFF`$5\x91\x16_R`\x03` R`@_ \x80T\x82\x10\x15a\x02\x1CW`@\x91a\x17k\x91a\x17\xD6V[PT\x81Q`\x01`\x01`\xA0\x1B\x03\x82\x16\x81R`\xA0\x91\x90\x91\x1C` \x82\x01R\xF3[4a\x02\x1CW` 6`\x03\x19\x01\x12a\x02\x1CW` \x90`\xFFa\x17\xA6a\x17\xB6V[\x16_R`\x01\x82R`@_ T\x81R\xF3[`\x045\x90`\xFF\x82\x16\x82\x03a\x02\x1CWV[`$5\x90`\xFF\x82\x16\x82\x03a\x02\x1CWV[\x80T\x82\x10\x15a\x0C\x0BW_R` _ \x01\x90_\x90V[\x91\x81`\x1F\x84\x01\x12\x15a\x02\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\x1CW` \x80\x85\x01\x94\x84`\x05\x1B\x01\x01\x11a\x02\x1CWV[\x91\x81`\x1F\x84\x01\x12\x15a\x02\x1CW\x825\x91`\x01`\x01`@\x1B\x03\x83\x11a\x02\x1CW` \x83\x81\x86\x01\x95\x01\x01\x11a\x02\x1CWV[```\x03\x19\x82\x01\x12a\x02\x1CW`\x045`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x1CW\x91`$5\x91`D5\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CWa\x18\x8A\x91`\x04\x01a\x18\x1BV[\x90\x91V[\x90` \x80\x83Q\x92\x83\x81R\x01\x92\x01\x90_[\x81\x81\x10a\x18\xABWPPP\x90V[\x82Q`\x01`\x01``\x1B\x03\x16\x84R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a\x18\x9EV[``\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\xE5W`@RV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\xE5W`@RV[\x90`\x1F\x80\x19\x91\x01\x16\x81\x01\x90\x81\x10`\x01`\x01`@\x1B\x03\x82\x11\x17a\x18\xE5W`@RV[`\x01`\x01`@\x1B\x03\x81\x11a\x18\xE5W`\x05\x1B` \x01\x90V[\x91\x90` \x83\x01\x92`\x02\x82\x10\x15a\x05\x17WRV[`$5\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\x02\x1CWV[\x81`\x1F\x82\x01\x12\x15a\x02\x1CW\x805\x90a\x19\x8C\x82a\x195V[\x92a\x19\x9A`@Q\x94\x85a\x19\x14V[\x82\x84R` \x80\x85\x01\x93`\x06\x1B\x83\x01\x01\x91\x81\x83\x11a\x02\x1CW` \x01\x92[\x82\x84\x10a\x19\xC4WPPPP\x90V[`@\x84\x83\x03\x12a\x02\x1CW`@Q\x90a\x19\xDB\x82a\x18\xF9V[\x845`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x1CW\x82R` \x85\x015\x90`\x01`\x01``\x1B\x03\x82\x16\x82\x03a\x02\x1CW\x82` \x92\x83`@\x95\x01R\x81R\x01\x93\x01\x92a\x19\xB6V[`$5\x90c\xFF\xFF\xFF\xFF\x82\x16\x82\x03a\x02\x1CWV[``\x90`\x03\x19\x01\x12a\x02\x1CW`\x045\x90`$5`\xFF\x81\x16\x81\x03a\x02\x1CW\x90`D5c\xFF\xFF\xFF\xFF\x81\x16\x81\x03a\x02\x1CW\x90V[\x91\x90\x81\x10\x15a\x0C\x0BW`\x05\x1B\x01\x90V[5`\x01`\x01``\x1B\x03\x81\x16\x81\x03a\x02\x1CW\x90V[\x90a\x1A\x8C\x82a\x195V[a\x1A\x99`@Q\x91\x82a\x19\x14V[\x82\x81R\x80\x92a\x1A\xAA`\x1F\x19\x91a\x195V[\x01\x90` 6\x91\x017V[\x90\x82\x10\x15a\x0C\x0BW\x01\x90V[\x80Q\x15a\x0C\x0BW` \x01\x90V[\x80Q\x82\x10\x15a\x0C\x0BW` \x91`\x05\x1B\x01\x01\x90V[\x90`@Qa\x1A\xEE\x81a\x18\xCAV[`@`\x01`\x01``\x1B\x03\x82\x94Tc\xFF\xFF\xFF\xFF\x81\x16\x84Rc\xFF\xFF\xFF\xFF\x81` \x1C\x16` \x85\x01R\x82\x1C\x16\x91\x01RV[\x91\x90\x82\x03\x91\x82\x11a\x03\xDFWV[\x15a\x1B/WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x90\xFD[\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x82T\x91`@\x1B\x16\x90k\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`@\x1B\x19\x16\x17\x90UV[\x80T`\x01`@\x1B\x81\x10\x15a\x18\xE5Wa\x1B\xDF\x91`\x01\x82\x01\x81Ua\x17\xD6V[a\x10\x8BW\x81Q\x81T` \x80\x85\x01Qg\xFF\xFF\xFF\xFF\0\0\0\0\x91\x1B\x16c\xFF\xFF\xFF\xFF\x90\x92\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x17\x17\x81Ua\x1C*\x91`\x01`\x01``\x1B\x03\x90`@\x01Q\x16\x90a\x1B\x92V[V[\x90\x81` \x91\x03\x12a\x02\x1CWQ\x80\x15\x15\x81\x03a\x02\x1CW\x90V[`@Q\x90a\x1CQ\x82a\x18\xCAV[_`@\x83\x82\x81R\x82` \x82\x01R\x01RV[`@Q\x90a\x1Co\x82a\x18\xF9V[_` \x83\x82\x81R\x01RV[\x90`@Qa\x1C\x87\x81a\x18\xF9V[\x91T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\xA0\x1C` \x83\x01RV[\x90a\x1C\xA8a\x1CDV[P\x81_R`\x02` R`@_ `\xFF\x82\x16_R` R`@_ T\x90a\x1C\xCCa\x1CDV[\x92\x82a\x1C\xD8WPPP\x90V[\x90\x91\x92P_R`\x02` R`\xFF`@_ \x91\x16_R` R`@_ _\x19\x82\x01\x91\x82\x11a\x03\xDFWa\x1D\x0C\x91a\x02\xD5\x91a\x17\xD6V[\x90V[a\x1D'\x90`\xFF\x16_R`\x01` R`@_ T\x15\x15\x90V[\x15a\x1D.WV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FStakeRegistry.quorumExists: quor`D\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B`d\x82\x01R`\x84\x90\xFD[\x90\x80`\x1F\x83\x01\x12\x15a\x02\x1CW\x81Qa\x1D\xA4\x81a\x195V[\x92a\x1D\xB2`@Q\x94\x85a\x19\x14V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x92\x83\x11a\x02\x1CW` \x01\x90[\x82\x82\x10a\x1D\xDAWPPP\x90V[\x81Q\x81R` \x91\x82\x01\x91\x01a\x1D\xCDV[\x90` \x82T\x91\x82\x81R\x01\x91_R` _ \x90_[\x81\x81\x10a\x1E\x0BWPPP\x90V[\x82T`\x01`\x01`\xA0\x1B\x03\x16\x84R` \x90\x93\x01\x92`\x01\x92\x83\x01\x92\x01a\x1D\xFEV[\x81\x81\x02\x92\x91\x81\x15\x91\x84\x04\x14\x17\x15a\x03\xDFWV[\x90`\x01`\x01``\x1B\x03\x80\x91\x16\x91\x16\x01\x90`\x01`\x01``\x1B\x03\x82\x11a\x03\xDFWV[\x91\x90`\xFF_\x93\x16\x90\x81_R`\x03` R`@_ T\x90`@Qa\x1E\x7F\x81a\x18\xF9V[_\x81R_` \x82\x01RP\x82_R`\x05` R`\xFF`@_ T\x16`\x02\x81\x10\x15a\x05\x17W`\x01\x03a!\xC7W`@\x90\x81Qa\x1E\xB8\x83\x82a\x19\x14V[`\x01\x81R` \x81\x01\x91`\x1F\x19\x84\x016\x847a\x1E\xD2\x82a\x1A\xC0V[\x90`\x01\x80`\xA0\x1B\x03\x16\x90R\x84_R`\x06` Rc\xFF\xFF\xFF\xFFa\x1E\xF9\x81\x85_ T\x16Ba(\xF7V[\x84Qc\xCA\x8A\xA7\xC7`\xE0\x1B\x81R\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16\x94\x90\x93\x92\x90\x91\x16\x91` \x84`\x04\x81\x88Z\xFA\x93\x84\x15a!\xBDW_\x94a!\x8CW[P\x90c\xFF\xFF\xFF\xFF\x94\x92\x91\x86Q\x93a\x1Fd\x85a\x18\xF9V[\x84R` \x84\x01\x93\x89\x85R\x89_R`\x04` R\x87_ \x91\x88Q\x97\x88\x96c\x15\xD5\x96%`\xE1\x1B\x88R`\xA4\x88\x01\x93`\x01\x80`\xA0\x1B\x03\x90Q\x16`\x04\x89\x01RQ\x16`$\x87\x01R`\xA0`D\x87\x01RQ\x80\x91R`\xC4\x85\x01\x92\x90_[\x81\x81\x10a!jWPPP_\x94\x92a\x1F\xDC\x85\x93\x84\x93`\x03\x19\x85\x83\x03\x01`d\x86\x01Ra\x1D\xEAV[`\x84\x83\x01\x91\x90\x91R\x03\x91`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x80\x15a!`W_\x90a \xAEW[a \x08\x91Pa\x1A\xC0V[Q\x90_[\x83\x81\x10a ;WPPPP[_R_` R`\x01`\x01``\x1B\x03`@_ T\x16`\x01`\x01``\x1B\x03\x83\x16\x10\x15\x90V[\x84_R`\x03` Ra Ra\x07;\x82\x84_ a\x17\xD6V[a \\\x82\x85a\x1A\xCDV[Qa kW[P`\x01\x01a \x0CV[\x81\x97`\x01`\x01``\x1B\x03g\r\xE0\xB6\xB3\xA7d\0\0a \x9Fa \xA7\x94\x83` a \x94`\x01\x99\x8Ca\x1A\xCDV[Q\x92\x01Q\x16\x90a\x1E*V[\x04\x16\x90a\x1E=V[\x96\x90a bV[P=\x80_\x83>a \xBE\x81\x83a\x19\x14V[\x81\x01\x90` \x81\x83\x03\x12a\x02\x1CW\x80Q\x90`\x01`\x01`@\x1B\x03\x82\x11a\x02\x1CW\x01\x90\x80`\x1F\x83\x01\x12\x15a\x02\x1CW\x81Qa \xF4\x81a\x195V[\x92a!\x01\x85Q\x94\x85a\x19\x14V[\x81\x84R` \x80\x85\x01\x92`\x05\x1B\x82\x01\x01\x91\x83\x83\x11a\x02\x1CW` \x82\x01\x90[\x83\x82\x10a!3WPPPPPa \x08\x90a\x1F\xFEV[\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CW` \x91a!U\x87\x84\x80\x94\x88\x01\x01a\x1D\x8DV[\x81R\x01\x91\x01\x90a!\x1EV[\x82Q=_\x82>=\x90\xFD[\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x85R\x88\x96P` \x94\x85\x01\x94\x90\x92\x01\x91`\x01\x01a\x1F\xB7V[a!\xAF\x91\x94P` =` \x11a!\xB6W[a!\xA7\x81\x83a\x19\x14V[\x81\x01\x90a\"\xF8V[\x92_a\x1FNV[P=a!\x9DV[\x86Q=_\x82>=\x90\xFD[_\x83\x81R`\x04` \x81\x90R`@\x80\x83 \x81Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x90\x95\x16\x92\x85\x01\x92\x90\x92R`$\x84\x01R\x82\x90\x81\x90a\"\x0C\x90`D\x83\x01\x90a\x1D\xEAV[\x03\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x08\x99W_\x91a\"\xBBW[P_[\x82\x81\x10a\"ZWPPPa \x18V[\x83_R`\x03` Ra\"ra\x07;\x82`@_ a\x17\xD6V[a\"|\x82\x84a\x1A\xCDV[Qa\"\x8BW[P`\x01\x01a\"KV[\x81\x96`\x01`\x01``\x1B\x03g\r\xE0\xB6\xB3\xA7d\0\0a \x9Fa\"\xB4\x94\x83` a \x94`\x01\x99\x8Ba\x1A\xCDV[\x95\x90a\"\x82V[\x90P=\x80_\x83>a\"\xCC\x81\x83a\x19\x14V[\x81\x01` \x82\x82\x03\x12a\x02\x1CW\x81Q`\x01`\x01`@\x1B\x03\x81\x11a\x02\x1CWa\"\xF2\x92\x01a\x1D\x8DV[_a\"HV[\x90\x81` \x91\x03\x12a\x02\x1CWQ`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x03a\x02\x1CW\x90V[`@Qc\x8D\xA5\xCB[`\xE0\x1B\x81R` \x81`\x04\x81\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16Z\xFA\x90\x81\x15a\x08\x99W_\x91a$\0W[P`\x01`\x01`\xA0\x1B\x03\x163\x03a#vWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`D\x82\x01R\x7Fer: caller is not the owner of t`d\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x84\x82\x01R`\xA4\x90\xFD[a$\x19\x91P` =` \x11a!\xB6Wa!\xA7\x81\x83a\x19\x14V[_a#dV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x163\x03a$QWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`D\x82\x01R\x7Fnator: caller is not the Registr`d\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x90\xFD[_\x90\x80_R`\x02` R`@_ `\xFF\x84\x16_R` R`@_ T\x80\x15_\x14a%xWP_Q` a0?_9_Q\x90_R`@`\x01`\x01``\x1B\x03\x94\x83_R`\x02` R\x81_ `\xFF\x82\x16_R` Ra%N\x82_ \x83Q\x90a%5\x82a\x18\xCAV[c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01R_\x85\x83\x01Ra\x1B\xC2V[`\xFF\x82Q\x91\x16\x81R_` \x82\x01R\xA2\x16_\x81\x81\x03\x91\x12_\x82\x12\x81\x16\x90_\x83\x13\x90\x15\x16\x17a\x03\xDFW\x90V[\x90\x80\x92P_R`\x02` R`@_ `\xFF\x84\x16_R` R`@_ \x90_\x19\x81\x01\x90\x81\x11a\x03\xDFWa%\xA9\x91a\x17\xD6V[P\x90\x81T\x91`\x01`\x01``\x1B\x03\x83`@\x1C\x16\x92\x83\x15a&lW`\x01`\x01``\x1B\x03\x94_Q` a0?_9_Q\x90_R\x92`@\x92c\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x81\x03a&\x0EWP\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\x19\x16\x90Ua%NV[\x81Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x82\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x91Ua&g\x90\x85_R`\x02` R\x83_ `\xFF\x84\x16_R` R\x83_ \x84Q\x91a&U\x83a\x18\xCAV[\x82R_` \x83\x01R_\x85\x83\x01Ra\x1B\xC2V[a%NV[PPPPP_\x90V[\x91\x90\x91_\x90\x80_R`\x02` R`@_ `\xFF\x85\x16_R` R`@_ T\x80\x15_\x14a'$WP_Q` a0?_9_Q\x90_R`@`\x01`\x01``\x1B\x03\x80\x95\x84_R`\x02` R\x82_ `\xFF\x89\x16_R` Ra&\xF8\x83_ \x84Q\x90a&\xDD\x82a\x18\xCAV[c\xFF\xFF\xFF\xFFC\x16\x82R_` \x83\x01R\x84\x84\x16\x86\x83\x01Ra\x1B\xC2V[`\xFF\x83Q\x98\x16\x88R\x16\x95\x86` \x82\x01R\xA2\x16\x90_\x82\x82\x03\x92\x12\x81\x83\x12\x81\x16\x91\x83\x13\x90\x15\x16\x17a\x03\xDFW\x90V[\x90\x80\x92P_R`\x02` R`@_ `\xFF\x85\x16_R` R`@_ \x90_\x19\x81\x01\x90\x81\x11a\x03\xDFWa'U\x91a\x17\xD6V[P\x90\x81T\x91`\x01`\x01``\x1B\x03\x83`@\x1C\x16\x92`\x01`\x01``\x1B\x03\x85\x16\x90\x81\x85\x14a(\x17W\x85_Q` a0?_9_Q\x90_R\x93`\x01`\x01``\x1B\x03\x97c\xFF\xFF\xFF\xFF`@\x95\x8A\x95\x82C\x16\x92\x83\x91\x16\x14_\x14a'\xBBWPPa'\xB6\x91a\x1B\x92V[a&\xF8V[\x83Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x83\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x93Ua'\xB6\x92\x90\x91P\x87_R`\x02` R\x85_ `\xFF\x8C\x16_R` R\x85_ \x90\x86Q\x92a(\x06\x84a\x18\xCAV[\x83R_` \x84\x01R\x86\x83\x01Ra\x1B\xC2V[PPPPPPP_\x90V[`\xFF\x16_\x81\x81R`\x01` R`@\x90 \x80T\x91\x92\x91_\x19\x81\x01\x90\x81\x11a\x03\xDFWa(K\x91a\x17\xD6V[P\x90\x80\x15a(\xE4Wc\xFF\xFF\xFF\xFFa(p\x83T\x92`\x01`\x01``\x1B\x03\x84`@\x1C\x16a/\xF5V[\x93\x84\x92C\x83\x16\x92\x16\x82\x03a(\x89WPPa\x1D\x0C\x91a\x1B\x92V[\x83Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16` \x83\x90\x1Bg\xFF\xFF\xFF\xFF\0\0\0\0\x16\x17\x90\x93Ua\x1D\x0C\x92\x90\x91P_R`\x01` R`@_ `@Q\x91a(\xC8\x83a\x18\xCAV[\x82R_` \x83\x01R`\x01`\x01``\x1B\x03\x84\x16`@\x83\x01Ra\x1B\xC2V[P`\x01`\x01``\x1B\x03\x91PT`@\x1C\x16\x90V[\x91\x90\x82\x01\x80\x92\x11a\x03\xDFWV[\x81Q\x15a,AW`\xFF\x82Q\x91\x16\x91\x82_R`\x03` R`@_ T\x92` a),\x84\x86a(\xF7V[\x11a+\xDBW_\x92[\x80\x84\x10a)BWPPPPPV[\x90\x91\x92\x93\x94_[a)S\x86\x88a(\xF7V[\x81\x10\x15a)\xEFW\x83_R`\x03` Ra)o\x81`@_ a\x17\xD6V[PT`\x01`\x01`\xA0\x1B\x03\x90\x81\x16\x90a)\x87\x88\x88a\x1A\xCDV[QQ\x16\x14a)\x97W`\x01\x01a)IV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R_Q` a0__9_Q\x90_R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x90\xFD[P\x94\x93\x92\x91\x90\x92`\x01`\x01``\x1B\x03` a*\n\x83\x86a\x1A\xCDV[Q\x01Q\x16\x15a+tW\x81_R`\x03` R`@_ a*)\x82\x85a\x1A\xCDV[Q\x90\x80T`\x01`@\x1B\x81\x10\x15a\x18\xE5Wa*H\x91`\x01\x82\x01\x81Ua\x17\xD6V[a\x10\x8BW\x81Q` \x92\x90\x92\x01Q`\xA0\x1B`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x91\x90\x91\x17\x90U_\x82\x81R`\x04` R`@\x90 \x90`\x01`\x01`\xA0\x1B\x03a*\x94\x82\x86a\x1A\xCDV[QQ\x16\x82T\x90`\x01`@\x1B\x82\x10\x15a\x18\xE5Wa\x10 \x82`\x01\x95\x86a*\xBA\x95\x01\x81Ua\x17\xD6V[\x82\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04` \x84\x80`\xA0\x1B\x03a*\xEE\x85\x89a\x1A\xCDV[QQ\x16`@Q\x90\x81R\xA2\x82\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x80`\xA0\x1B\x03a+*\x84\x88a\x1A\xCDV[QQ\x16`\x01`\x01``\x1B\x03` a+A\x86\x8Aa\x1A\xCDV[Q\x01Q`@\x80Q`\x01`\x01`\xA0\x1B\x03\x94\x90\x94\x16\x84R\x91\x16`\x01`\x01``\x1B\x03\x16` \x83\x01R\x81\x90\x81\x01\x03\x90\xA2\x01\x92a)4V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R_Q` a0__9_Q\x90_R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R_Q` a0__9_Q\x90_R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x90\xFD[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R_Q` a0__9_Q\x90_R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x90\xFD[` `\xFF\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x92\x16\x92\x83_R_\x82R`\x01`\x01``\x1B\x03`@_ \x91\x16\x90\x81`\x01`\x01``\x1B\x03\x19\x82T\x16\x17\x90U`@Q\x90\x81R\xA2V[c\xFF\xFF\xFF\xFF\x80\x82Q\x16\x92\x16\x91\x82\x10a-\xCCW` \x01Qc\xFF\xFF\xFF\xFF\x16\x80\x15\x91\x82\x15a-\xC2W[PP\x15a-\x1EWV[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x90\xFD[\x10\x90P_\x80a-\x15V[`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x90\xFD[`\xFF\x16_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x94\x85\x16c\xFF\xFF\xFF\xFF\x19\x82\x16\x81\x17\x90\x92U\x83Q\x94\x16\x84R\x90\x83\x01R\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x91\xA1V[\x92\x91\x90\x83_R`\x02` R`@_ `\xFF\x82\x16_R` R`@_ T\x80[a/\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x90\xFD[\x84_R`\x02` R`@_ `\xFF\x83\x16_R` R`@_ _\x19\x82\x01\x90\x82\x82\x11a\x03\xDFWa/\xC8\x82c\xFF\xFF\xFF\xFF\x92a\x17\xD6V[PT\x16c\xFF\xFF\xFF\xFF\x85\x16\x10\x15a/\xE8WP\x80\x15a\x03\xDFW_\x19\x01\x80a.\xCFV[c\xFF\xFF\xFF\xFF\x16\x94PPPPV[\x90_\x81\x12\x15a0*W`\x01`\xFF\x1B\x81\x14a\x03\xDFW`\x01`\x01``\x1B\x03\x80\x91_\x03\x16\x91\x16\x03`\x01`\x01``\x1B\x03\x81\x11a\x03\xDFW\x90V[\x90`\x01`\x01``\x1B\x03a\x1D\x0C\x92\x16\x90a\x1E=V\xFE/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}StakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \xBB \x94sD0\x91p\x01\xB4)p\x13\xC3\x1C\x80\xEE?f0SQ\xDC\xE8\xEE\r\xEA\x1BqO;WdsolcC\0\x08\x1B\x003",
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
            ) -> <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'_>
            {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Uint<8>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(self).0
            }
            #[inline]
            fn stv_abi_encode_packed_to(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                <alloy::sol_types::sol_data::Uint<
                    8,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
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
            type Token<'a> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> =
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Uint<
                8,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::detokenize(token)
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
            fn encode_topic(rust: &Self::RustType) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "LookAheadPeriodChanged(uint32,uint32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    40u8, 215u8, 53u8, 139u8, 121u8, 240u8, 45u8, 33u8, 184u8, 183u8, 225u8, 122u8,
                    239u8, 196u8, 24u8, 90u8, 100u8, 48u8, 138u8, 163u8, 116u8, 6u8, 250u8, 91u8,
                    239u8, 192u8, 91u8, 145u8, 147u8, 44u8, 57u8, 199u8,
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.oldLookAheadDays,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.newLookAheadDays,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "MinimumStakeForQuorumUpdated(uint8,uint96)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    38u8, 238u8, 207u8, 242u8, 183u8, 11u8, 10u8, 113u8, 16u8, 79u8, 244u8, 217u8,
                    64u8, 186u8, 113u8, 98u8, 210u8, 58u8, 149u8, 194u8, 72u8, 119u8, 31u8, 196u8,
                    135u8, 167u8, 190u8, 23u8, 165u8, 150u8, 179u8, 207u8,
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
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.minimumStake,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &MinimumStakeForQuorumUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            const SIGNATURE: &'static str = "OperatorStakeUpdate(bytes32,uint8,uint96)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    47u8, 82u8, 125u8, 82u8, 126u8, 149u8, 216u8, 254u8, 64u8, 174u8, 197u8, 83u8,
                    119u8, 116u8, 59u8, 183u8, 121u8, 8u8, 125u8, 163u8, 246u8, 208u8, 208u8,
                    143u8, 18u8, 227u8, 100u8, 68u8, 218u8, 98u8, 50u8, 125u8,
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
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.stake,
                    ),
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "QuorumCreated(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    131u8, 26u8, 156u8, 134u8, 196u8, 91u8, 179u8, 3u8, 202u8, 243u8, 240u8, 100u8,
                    190u8, 43u8, 194u8, 185u8, 253u8, 78u8, 207u8, 25u8, 228u8, 124u8, 74u8, 192u8,
                    42u8, 97u8, 231u8, 93u8, 171u8, 254u8, 85u8, 180u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>,);
            const SIGNATURE: &'static str = "StakeTypeSet(uint8)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    124u8, 17u8, 46u8, 134u8, 60u8, 207u8, 0u8, 120u8, 98u8, 226u8, 201u8, 226u8,
                    88u8, 25u8, 201u8, 51u8, 254u8, 219u8, 201u8, 53u8, 10u8, 100u8, 67u8, 66u8,
                    59u8, 74u8, 133u8, 153u8, 194u8, 232u8, 165u8, 45u8,
                ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    newStakeType: data.0,
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
                (<StakeType as alloy_sol_types::SolType>::tokenize(
                    &self.newStakeType,
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyAddedToQuorum(uint8,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    16u8, 86u8, 94u8, 86u8, 202u8, 203u8, 243u8, 46u8, 202u8, 38u8, 121u8, 69u8,
                    240u8, 84u8, 254u8, 192u8, 46u8, 89u8, 117u8, 0u8, 50u8, 209u8, 19u8, 211u8,
                    48u8, 33u8, 130u8, 173u8, 150u8, 127u8, 84u8, 4u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyMultiplierUpdated(uint8,address,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    17u8, 165u8, 100u8, 19u8, 34u8, 218u8, 29u8, 255u8, 86u8, 164u8, 182u8, 110u8,
                    170u8, 195u8, 31u8, 250u8, 70u8, 82u8, 149u8, 236u8, 233u8, 7u8, 205u8, 22u8,
                    52u8, 55u8, 121u8, 59u8, 77u8, 0u8, 154u8, 117u8,
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
                        &self.strategy,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self.multiplier,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &StrategyMultiplierUpdated) -> alloy_sol_types::private::LogData {
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
            type DataToken<'a> = <Self::DataTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<8>,
            );
            const SIGNATURE: &'static str = "StrategyRemovedFromQuorum(uint8,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 =
                alloy_sol_types::private::B256::new([
                    49u8, 250u8, 46u8, 44u8, 210u8, 128u8, 201u8, 55u8, 94u8, 19u8, 255u8, 207u8,
                    61u8, 129u8, 226u8, 55u8, 129u8, 0u8, 24u8, 110u8, 64u8, 88u8, 248u8, 211u8,
                    221u8, 182u8, 144u8, 184u8, 45u8, 205u8, 49u8, 247u8,
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
                out[0usize] = alloy_sol_types::abi::token::WordToken(Self::SIGNATURE_HASH);
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
            fn from(this: &StrategyRemovedFromQuorum) -> alloy_sol_types::private::LogData {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_WEIGHING_FUNCTION_LENGTHCall> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WEIGHING_FUNCTION_LENGTHCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_WEIGHING_FUNCTION_LENGTHCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<MAX_WEIGHING_FUNCTION_LENGTHReturn> for UnderlyingRustTuple<'_> {
                fn from(value: MAX_WEIGHING_FUNCTION_LENGTHReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for MAX_WEIGHING_FUNCTION_LENGTHReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for MAX_WEIGHING_FUNCTION_LENGTHCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = MAX_WEIGHING_FUNCTION_LENGTHReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<WEIGHTING_DIVISORCall> for UnderlyingRustTuple<'_> {
                fn from(value: WEIGHTING_DIVISORCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WEIGHTING_DIVISORCall {
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
            impl ::core::convert::From<WEIGHTING_DIVISORReturn> for UnderlyingRustTuple<'_> {
                fn from(value: WEIGHTING_DIVISORReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for WEIGHTING_DIVISORReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for WEIGHTING_DIVISORCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = WEIGHTING_DIVISORReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = addStrategiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = avsDirectoryReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorCall) -> Self {
                    (value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<deregisterOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: deregisterOperatorReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for deregisterOperatorReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = deregisterOperatorReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8);
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getCurrentStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentStakeReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getCurrentTotalStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalStakeCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentTotalStakeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getCurrentTotalStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getCurrentTotalStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getCurrentTotalStakeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getCurrentTotalStakeCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getCurrentTotalStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8);
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
            impl ::core::convert::From<getLatestStakeUpdateCall> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestStakeUpdateCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestStakeUpdateCall {
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
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getLatestStakeUpdateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getLatestStakeUpdateReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getLatestStakeUpdateReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getLatestStakeUpdateReturn;
            type ReturnTuple<'a> = (IStakeRegistry::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8, u32);
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
            impl ::core::convert::From<getStakeAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeAtBlockNumberCall {
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getStakeAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeAtBlockNumberReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeAtBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeAtBlockNumberAndIndexCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeAtBlockNumberAndIndexCall {
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getStakeAtBlockNumberAndIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeAtBlockNumberAndIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeAtBlockNumberAndIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeAtBlockNumberAndIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getStakeAtBlockNumberAndIndex(uint8,uint32,bytes32,uint256)";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8);
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<IStakeRegistry::StakeUpdate>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    <IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,
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
            impl ::core::convert::From<getStakeHistoryReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeHistoryReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<IStakeRegistry::StakeUpdate>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8);
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
            impl ::core::convert::From<getStakeHistoryLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryLengthCall) -> Self {
                    (value.operatorId, value.quorumNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryLengthCall {
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
            impl ::core::convert::From<getStakeHistoryLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeHistoryLengthReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeUpdateAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.operatorId, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeUpdateAtIndexCall {
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
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getStakeUpdateAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeUpdateAtIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistry::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>, u8, u32);
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
            impl ::core::convert::From<getStakeUpdateIndexAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateIndexAtBlockNumberCall) -> Self {
                    (value.operatorId, value.quorumNumber, value.blockNumber)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeUpdateIndexAtBlockNumberCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getStakeUpdateIndexAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getStakeUpdateIndexAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getStakeUpdateIndexAtBlockNumberReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getStakeUpdateIndexAtBlockNumberReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getStakeUpdateIndexAtBlockNumber(bytes32,uint8,uint32)";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeAtBlockNumberFromIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeAtBlockNumberFromIndexCall) -> Self {
                    (value.quorumNumber, value.blockNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeAtBlockNumberFromIndexCall {
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<getTotalStakeAtBlockNumberFromIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeAtBlockNumberFromIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeAtBlockNumberFromIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeAtBlockNumberFromIndexReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "getTotalStakeAtBlockNumberFromIndex(uint8,uint32,uint256)";
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeHistoryLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeHistoryLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeHistoryLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
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
            impl ::core::convert::From<getTotalStakeHistoryLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeHistoryLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeHistoryLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for getTotalStakeHistoryLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeHistoryLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<getTotalStakeIndicesAtBlockNumberCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeIndicesAtBlockNumberCall) -> Self {
                    (value.blockNumber, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeIndicesAtBlockNumberCall {
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
            type UnderlyingSolTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Vec<u32>,);
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
            impl ::core::convert::From<getTotalStakeIndicesAtBlockNumberReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeIndicesAtBlockNumberReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeIndicesAtBlockNumberReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeIndicesAtBlockNumberReturn;
            type ReturnTuple<'a> =
                (alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<32>>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self.blockNumber,
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
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
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
            impl ::core::convert::From<getTotalStakeUpdateAtIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeUpdateAtIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeUpdateAtIndexCall {
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
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistry::StakeUpdate as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<getTotalStakeUpdateAtIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: getTotalStakeUpdateAtIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for getTotalStakeUpdateAtIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = getTotalStakeUpdateAtIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistry::StakeUpdate,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeDelegatedStakeQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeDelegatedStakeQuorumCall) -> Self {
                    (
                        value.quorumNumber,
                        value.minimumStake,
                        value._strategyParams,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeDelegatedStakeQuorumCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeDelegatedStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeDelegatedStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeDelegatedStakeQuorumReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeDelegatedStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "initializeDelegatedStakeQuorum(uint8,uint96,(address,uint96)[])";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeSlashableStakeQuorumCall> for UnderlyingRustTuple<'_> {
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
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeSlashableStakeQuorumCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeSlashableStakeQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeSlashableStakeQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeSlashableStakeQuorumReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeSlashableStakeQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str =
                "initializeSlashableStakeQuorum(uint8,uint96,uint32,(address,uint96)[])";
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
                .map(Into::into)
            }
        }
    };
    /**Function with signature `isOperatorSetQuorum(uint8)` and selector `0x9f8aff26`.
    ```solidity
    function isOperatorSetQuorum(uint8 quorumNumber) external view returns (bool);
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetQuorumCall {
        pub quorumNumber: u8,
    }
    ///Container type for the return parameters of the [`isOperatorSetQuorum(uint8)`](isOperatorSetQuorumCall) function.
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct isOperatorSetQuorumReturn {
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
            impl ::core::convert::From<isOperatorSetQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetQuorumCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
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
            impl ::core::convert::From<isOperatorSetQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorSetQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorSetQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorSetQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorSetQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperatorSetQuorum(uint8)";
            const SELECTOR: [u8; 4] = [159u8, 138u8, 255u8, 38u8];
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
                        &self.quorumNumber,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<minimumStakeForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: minimumStakeForQuorumCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumStakeForQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<minimumStakeForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: minimumStakeForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for minimumStakeForQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for minimumStakeForQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = minimumStakeForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
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
    /**Function with signature `modifyStrategyParams(uint8,uint256[],uint96[])` and selector `0x20b66298`.
    ```solidity
    function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct modifyStrategyParamsCall {
        pub quorumNumber: u8,
        pub strategyIndices:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
        pub newMultipliers:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
            impl ::core::convert::From<modifyStrategyParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: modifyStrategyParamsCall) -> Self {
                    (
                        value.quorumNumber,
                        value.strategyIndices,
                        value.newMultipliers,
                    )
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyStrategyParamsCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyStrategyParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: modifyStrategyParamsReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyStrategyParamsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyStrategyParamsReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        pub _0: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
        pub _1: alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U96>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<registerOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorCall) -> Self {
                    (value.operator, value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorCall {
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
            impl ::core::convert::From<registerOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: registerOperatorReturn) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for registerOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = registerOperatorReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
                alloy::sol_types::sol_data::Array<alloy::sol_types::sol_data::Uint<96>>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
    /**Function with signature `removeStrategies(uint8,uint256[])` and selector `0x5f1f2d77`.
    ```solidity
    function removeStrategies(uint8 quorumNumber, uint256[] memory indicesToRemove) external;
    ```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
    #[derive(Clone)]
    pub struct removeStrategiesCall {
        pub quorumNumber: u8,
        pub indicesToRemove:
            alloy::sol_types::private::Vec<alloy::sol_types::private::primitives::aliases::U256>,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeStrategiesCall> for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesCall) -> Self {
                    (value.quorumNumber, value.indicesToRemove)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<removeStrategiesReturn> for UnderlyingRustTuple<'_> {
                fn from(value: removeStrategiesReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for removeStrategiesReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = removeStrategiesReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U96);
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
            impl ::core::convert::From<setMinimumStakeForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakeForQuorumCall) -> Self {
                    (value.quorumNumber, value.minimumStake)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMinimumStakeForQuorumCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setMinimumStakeForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setMinimumStakeForQuorumReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setMinimumStakeForQuorumReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setMinimumStakeForQuorumReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<96> as alloy_sol_types::SolType>::tokenize(
                        &self.minimumStake,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setSlashableStakeLookaheadCall> for UnderlyingRustTuple<'_> {
                fn from(value: setSlashableStakeLookaheadCall) -> Self {
                    (value.quorumNumber, value._lookAheadPeriod)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setSlashableStakeLookaheadCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setSlashableStakeLookaheadReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setSlashableStakeLookaheadReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setSlashableStakeLookaheadReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setSlashableStakeLookaheadReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<32> as alloy_sol_types::SolType>::tokenize(
                        &self._lookAheadPeriod,
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
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<8>, StakeType);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (u8, <StakeType as alloy::sol_types::SolType>::RustType);
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = setStakeTypeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <StakeType as alloy_sol_types::SolType>::tokenize(&self._stakeType),
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slashableStakeLookAheadPerQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: slashableStakeLookAheadPerQuorumCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashableStakeLookAheadPerQuorumCall {
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<slashableStakeLookAheadPerQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: slashableStakeLookAheadPerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for slashableStakeLookAheadPerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for slashableStakeLookAheadPerQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = slashableStakeLookAheadPerQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<stakeTypePerQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: stakeTypePerQuorumCall) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeTypePerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (StakeType,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (<StakeType as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<stakeTypePerQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: stakeTypePerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for stakeTypePerQuorumReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for stakeTypePerQuorumCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = stakeTypePerQuorumReturn;
            type ReturnTuple<'a> = (StakeType,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
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
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
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
            impl ::core::convert::From<strategiesPerQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategiesPerQuorumCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategiesPerQuorumCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategiesPerQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategiesPerQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategiesPerQuorumReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategiesPerQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
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
            impl ::core::convert::From<strategyParamsCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsCall) -> Self {
                    (value._0, value._1)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        _0: tuple.0,
                        _1: tuple.1,
                    }
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyParamsReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsReturn) -> Self {
                    (value.strategy, value.multiplier)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsReturn;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<96>,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self._0,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
                        &self._1,
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
            type UnderlyingRustTuple<'a> =
                (u8, alloy::sol_types::private::primitives::aliases::U256);
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
            impl ::core::convert::From<strategyParamsByIndexCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsByIndexCall) -> Self {
                    (value.quorumNumber, value.index)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsByIndexCall {
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
            type UnderlyingRustTuple<'a> =
                (<IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,);
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
            impl ::core::convert::From<strategyParamsByIndexReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsByIndexReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsByIndexReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsByIndexReturn;
            type ReturnTuple<'a> = (IStakeRegistry::StrategyParams,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
                    <alloy::sol_types::sol_data::Uint<256> as alloy_sol_types::SolType>::tokenize(
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<strategyParamsLengthCall> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsLengthCall) -> Self {
                    (value.quorumNumber,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsLengthCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        quorumNumber: tuple.0,
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
            impl ::core::convert::From<strategyParamsLengthReturn> for UnderlyingRustTuple<'_> {
                fn from(value: strategyParamsLengthReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for strategyParamsLengthReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for strategyParamsLengthCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Uint<8>,);
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = strategyParamsLengthReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateOperatorStakeCall> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorStakeCall) -> Self {
                    (value.operator, value.operatorId, value.quorumNumbers)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorStakeCall {
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U192,);
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
            impl ::core::convert::From<updateOperatorStakeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: updateOperatorStakeReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for updateOperatorStakeReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateOperatorStakeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<192>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
            fn _type_assertion(_t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<weightOfOperatorForQuorumCall> for UnderlyingRustTuple<'_> {
                fn from(value: weightOfOperatorForQuorumCall) -> Self {
                    (value.quorumNumber, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for weightOfOperatorForQuorumCall {
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
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::primitives::aliases::U96,);
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
            impl ::core::convert::From<weightOfOperatorForQuorumReturn> for UnderlyingRustTuple<'_> {
                fn from(value: weightOfOperatorForQuorumReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for weightOfOperatorForQuorumReturn {
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
            type Token<'a> = <Self::Parameters<'a> as alloy_sol_types::SolType>::Token<'a>;
            type Return = weightOfOperatorForQuorumReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<96>,);
            type ReturnToken<'a> = <Self::ReturnTuple<'a> as alloy_sol_types::SolType>::Token<'a>;
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
                    <alloy::sol_types::sol_data::Uint<8> as alloy_sol_types::SolType>::tokenize(
                        &self.quorumNumber,
                    ),
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
                <Self::ReturnTuple<'_> as alloy_sol_types::SolType>::abi_decode_sequence(
                    data, validate,
                )
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
        isOperatorSetQuorum(isOperatorSetQuorumCall),
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
            [159u8, 138u8, 255u8, 38u8],
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
        const COUNT: usize = 39usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::MAX_WEIGHING_FUNCTION_LENGTH(_) => {
                    <MAX_WEIGHING_FUNCTION_LENGTHCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::WEIGHTING_DIVISOR(_) => {
                    <WEIGHTING_DIVISORCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::addStrategies(_) => <addStrategiesCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::avsDirectory(_) => <avsDirectoryCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::delegation(_) => <delegationCall as alloy_sol_types::SolCall>::SELECTOR,
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
                Self::isOperatorSetQuorum(_) => {
                    <isOperatorSetQuorumCall as alloy_sol_types::SolCall>::SELECTOR
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
                Self::setStakeType(_) => <setStakeTypeCall as alloy_sol_types::SolCall>::SELECTOR,
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
            )
                -> alloy_sol_types::Result<StakeRegistryCalls>] = &[
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(StakeRegistryCalls::strategiesPerQuorum)
                    }
                    strategiesPerQuorum
                },
                {
                    fn isOperatorSetQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <isOperatorSetQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
                        )
                        .map(StakeRegistryCalls::isOperatorSetQuorum)
                    }
                    isOperatorSetQuorum
                },
                {
                    fn getStakeUpdateAtIndex(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryCalls> {
                        <getStakeUpdateAtIndexCall as alloy_sol_types::SolCall>::abi_decode_raw(
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                            data, validate,
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
                        <delegationCall as alloy_sol_types::SolCall>::abi_decode_raw(data, validate)
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
                            data, validate,
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
                            data, validate,
                        )
                        .map(StakeRegistryCalls::getStakeAtBlockNumber)
                    }
                    getStakeAtBlockNumber
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
                Self::isOperatorSetQuorum(inner) => {
                    <isOperatorSetQuorumCall as alloy_sol_types::SolCall>::abi_encoded_size(
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
                Self::isOperatorSetQuorum(inner) => {
                    <isOperatorSetQuorumCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
                16u8, 86u8, 94u8, 86u8, 202u8, 203u8, 243u8, 46u8, 202u8, 38u8, 121u8, 69u8, 240u8,
                84u8, 254u8, 192u8, 46u8, 89u8, 117u8, 0u8, 50u8, 209u8, 19u8, 211u8, 48u8, 33u8,
                130u8, 173u8, 150u8, 127u8, 84u8, 4u8,
            ],
            [
                17u8, 165u8, 100u8, 19u8, 34u8, 218u8, 29u8, 255u8, 86u8, 164u8, 182u8, 110u8,
                170u8, 195u8, 31u8, 250u8, 70u8, 82u8, 149u8, 236u8, 233u8, 7u8, 205u8, 22u8, 52u8,
                55u8, 121u8, 59u8, 77u8, 0u8, 154u8, 117u8,
            ],
            [
                38u8, 238u8, 207u8, 242u8, 183u8, 11u8, 10u8, 113u8, 16u8, 79u8, 244u8, 217u8,
                64u8, 186u8, 113u8, 98u8, 210u8, 58u8, 149u8, 194u8, 72u8, 119u8, 31u8, 196u8,
                135u8, 167u8, 190u8, 23u8, 165u8, 150u8, 179u8, 207u8,
            ],
            [
                40u8, 215u8, 53u8, 139u8, 121u8, 240u8, 45u8, 33u8, 184u8, 183u8, 225u8, 122u8,
                239u8, 196u8, 24u8, 90u8, 100u8, 48u8, 138u8, 163u8, 116u8, 6u8, 250u8, 91u8,
                239u8, 192u8, 91u8, 145u8, 147u8, 44u8, 57u8, 199u8,
            ],
            [
                47u8, 82u8, 125u8, 82u8, 126u8, 149u8, 216u8, 254u8, 64u8, 174u8, 197u8, 83u8,
                119u8, 116u8, 59u8, 183u8, 121u8, 8u8, 125u8, 163u8, 246u8, 208u8, 208u8, 143u8,
                18u8, 227u8, 100u8, 68u8, 218u8, 98u8, 50u8, 125u8,
            ],
            [
                49u8, 250u8, 46u8, 44u8, 210u8, 128u8, 201u8, 55u8, 94u8, 19u8, 255u8, 207u8, 61u8,
                129u8, 226u8, 55u8, 129u8, 0u8, 24u8, 110u8, 64u8, 88u8, 248u8, 211u8, 221u8,
                182u8, 144u8, 184u8, 45u8, 205u8, 49u8, 247u8,
            ],
            [
                124u8, 17u8, 46u8, 134u8, 60u8, 207u8, 0u8, 120u8, 98u8, 226u8, 201u8, 226u8, 88u8,
                25u8, 201u8, 51u8, 254u8, 219u8, 201u8, 53u8, 10u8, 100u8, 67u8, 66u8, 59u8, 74u8,
                133u8, 153u8, 194u8, 232u8, 165u8, 45u8,
            ],
            [
                131u8, 26u8, 156u8, 134u8, 196u8, 91u8, 179u8, 3u8, 202u8, 243u8, 240u8, 100u8,
                190u8, 43u8, 194u8, 185u8, 253u8, 78u8, 207u8, 25u8, 228u8, 124u8, 74u8, 192u8,
                42u8, 97u8, 231u8, 93u8, 171u8, 254u8, 85u8, 180u8,
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
                Some(<LookAheadPeriodChanged as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <LookAheadPeriodChanged as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::LookAheadPeriodChanged)
                }
                Some(
                    <MinimumStakeForQuorumUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => <MinimumStakeForQuorumUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                    topics, data, validate,
                )
                .map(Self::MinimumStakeForQuorumUpdated),
                Some(<OperatorStakeUpdate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorStakeUpdate as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::OperatorStakeUpdate)
                }
                Some(<QuorumCreated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <QuorumCreated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::QuorumCreated)
                }
                Some(<StakeTypeSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StakeTypeSet as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StakeTypeSet)
                }
                Some(<StrategyAddedToQuorum as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StrategyAddedToQuorum as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StrategyAddedToQuorum)
                }
                Some(<StrategyMultiplierUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StrategyMultiplierUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StrategyMultiplierUpdated)
                }
                Some(<StrategyRemovedFromQuorum as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <StrategyRemovedFromQuorum as alloy_sol_types::SolEvent>::decode_raw_log(
                        topics, data, validate,
                    )
                    .map(Self::StrategyRemovedFromQuorum)
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
    ) -> impl ::core::future::Future<Output = alloy_contract::Result<StakeRegistryInstance<T, P, N>>>
    {
        StakeRegistryInstance::<T, P, N>::deploy(
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
        StakeRegistryInstance::<T, P, N>::deploy_builder(
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
            f.debug_tuple("StakeRegistryInstance")
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
        > StakeRegistryInstance<T, P, N>
    {
        /**Creates a new wrapper around an on-chain [`StakeRegistry`](self) contract instance.

        See the [wrapper's documentation](`StakeRegistryInstance`) for more details.*/
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
                    &alloy_sol_types::SolConstructor::abi_encode(&constructorCall {
                        _registryCoordinator,
                        _delegationManager,
                        _avsDirectory,
                        _serviceManager,
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
        > StakeRegistryInstance<T, P, N>
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
        ///Creates a new call builder for the [`MAX_WEIGHING_FUNCTION_LENGTH`] function.
        pub fn MAX_WEIGHING_FUNCTION_LENGTH(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, MAX_WEIGHING_FUNCTION_LENGTHCall, N> {
            self.call_builder(&MAX_WEIGHING_FUNCTION_LENGTHCall {})
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
            self.call_builder(&addStrategiesCall {
                quorumNumber,
                _strategyParams,
            })
        }
        ///Creates a new call builder for the [`avsDirectory`] function.
        pub fn avsDirectory(&self) -> alloy_contract::SolCallBuilder<T, &P, avsDirectoryCall, N> {
            self.call_builder(&avsDirectoryCall {})
        }
        ///Creates a new call builder for the [`delegation`] function.
        pub fn delegation(&self) -> alloy_contract::SolCallBuilder<T, &P, delegationCall, N> {
            self.call_builder(&delegationCall {})
        }
        ///Creates a new call builder for the [`deregisterOperator`] function.
        pub fn deregisterOperator(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, deregisterOperatorCall, N> {
            self.call_builder(&deregisterOperatorCall {
                operatorId,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`getCurrentStake`] function.
        pub fn getCurrentStake(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentStakeCall, N> {
            self.call_builder(&getCurrentStakeCall {
                operatorId,
                quorumNumber,
            })
        }
        ///Creates a new call builder for the [`getCurrentTotalStake`] function.
        pub fn getCurrentTotalStake(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getCurrentTotalStakeCall, N> {
            self.call_builder(&getCurrentTotalStakeCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getLatestStakeUpdate`] function.
        pub fn getLatestStakeUpdate(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getLatestStakeUpdateCall, N> {
            self.call_builder(&getLatestStakeUpdateCall {
                operatorId,
                quorumNumber,
            })
        }
        ///Creates a new call builder for the [`getStakeAtBlockNumber`] function.
        pub fn getStakeAtBlockNumber(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeAtBlockNumberCall, N> {
            self.call_builder(&getStakeAtBlockNumberCall {
                operatorId,
                quorumNumber,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getStakeAtBlockNumberAndIndex`] function.
        pub fn getStakeAtBlockNumberAndIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeAtBlockNumberAndIndexCall, N> {
            self.call_builder(&getStakeAtBlockNumberAndIndexCall {
                quorumNumber,
                blockNumber,
                operatorId,
                index,
            })
        }
        ///Creates a new call builder for the [`getStakeHistory`] function.
        pub fn getStakeHistory(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeHistoryCall, N> {
            self.call_builder(&getStakeHistoryCall {
                operatorId,
                quorumNumber,
            })
        }
        ///Creates a new call builder for the [`getStakeHistoryLength`] function.
        pub fn getStakeHistoryLength(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeHistoryLengthCall, N> {
            self.call_builder(&getStakeHistoryLengthCall {
                operatorId,
                quorumNumber,
            })
        }
        ///Creates a new call builder for the [`getStakeUpdateAtIndex`] function.
        pub fn getStakeUpdateAtIndex(
            &self,
            quorumNumber: u8,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeUpdateAtIndexCall, N> {
            self.call_builder(&getStakeUpdateAtIndexCall {
                quorumNumber,
                operatorId,
                index,
            })
        }
        ///Creates a new call builder for the [`getStakeUpdateIndexAtBlockNumber`] function.
        pub fn getStakeUpdateIndexAtBlockNumber(
            &self,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumber: u8,
            blockNumber: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, getStakeUpdateIndexAtBlockNumberCall, N>
        {
            self.call_builder(&getStakeUpdateIndexAtBlockNumberCall {
                operatorId,
                quorumNumber,
                blockNumber,
            })
        }
        ///Creates a new call builder for the [`getTotalStakeAtBlockNumberFromIndex`] function.
        pub fn getTotalStakeAtBlockNumberFromIndex(
            &self,
            quorumNumber: u8,
            blockNumber: u32,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeAtBlockNumberFromIndexCall, N>
        {
            self.call_builder(&getTotalStakeAtBlockNumberFromIndexCall {
                quorumNumber,
                blockNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`getTotalStakeHistoryLength`] function.
        pub fn getTotalStakeHistoryLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeHistoryLengthCall, N> {
            self.call_builder(&getTotalStakeHistoryLengthCall { quorumNumber })
        }
        ///Creates a new call builder for the [`getTotalStakeIndicesAtBlockNumber`] function.
        pub fn getTotalStakeIndicesAtBlockNumber(
            &self,
            blockNumber: u32,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeIndicesAtBlockNumberCall, N>
        {
            self.call_builder(&getTotalStakeIndicesAtBlockNumberCall {
                blockNumber,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`getTotalStakeUpdateAtIndex`] function.
        pub fn getTotalStakeUpdateAtIndex(
            &self,
            quorumNumber: u8,
            index: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, getTotalStakeUpdateAtIndexCall, N> {
            self.call_builder(&getTotalStakeUpdateAtIndexCall {
                quorumNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`initializeDelegatedStakeQuorum`] function.
        pub fn initializeDelegatedStakeQuorum(
            &self,
            quorumNumber: u8,
            minimumStake: alloy::sol_types::private::primitives::aliases::U96,
            _strategyParams: alloy::sol_types::private::Vec<
                <IStakeRegistry::StrategyParams as alloy::sol_types::SolType>::RustType,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeDelegatedStakeQuorumCall, N> {
            self.call_builder(&initializeDelegatedStakeQuorumCall {
                quorumNumber,
                minimumStake,
                _strategyParams,
            })
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
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeSlashableStakeQuorumCall, N> {
            self.call_builder(&initializeSlashableStakeQuorumCall {
                quorumNumber,
                minimumStake,
                lookAheadPeriod,
                _strategyParams,
            })
        }
        ///Creates a new call builder for the [`isOperatorSetQuorum`] function.
        pub fn isOperatorSetQuorum(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorSetQuorumCall, N> {
            self.call_builder(&isOperatorSetQuorumCall { quorumNumber })
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
            self.call_builder(&modifyStrategyParamsCall {
                quorumNumber,
                strategyIndices,
                newMultipliers,
            })
        }
        ///Creates a new call builder for the [`registerOperator`] function.
        pub fn registerOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, registerOperatorCall, N> {
            self.call_builder(&registerOperatorCall {
                operator,
                operatorId,
                quorumNumbers,
            })
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
            self.call_builder(&removeStrategiesCall {
                quorumNumber,
                indicesToRemove,
            })
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
            self.call_builder(&setMinimumStakeForQuorumCall {
                quorumNumber,
                minimumStake,
            })
        }
        ///Creates a new call builder for the [`setSlashableStakeLookahead`] function.
        pub fn setSlashableStakeLookahead(
            &self,
            quorumNumber: u8,
            _lookAheadPeriod: u32,
        ) -> alloy_contract::SolCallBuilder<T, &P, setSlashableStakeLookaheadCall, N> {
            self.call_builder(&setSlashableStakeLookaheadCall {
                quorumNumber,
                _lookAheadPeriod,
            })
        }
        ///Creates a new call builder for the [`setStakeType`] function.
        pub fn setStakeType(
            &self,
            quorumNumber: u8,
            _stakeType: <StakeType as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, setStakeTypeCall, N> {
            self.call_builder(&setStakeTypeCall {
                quorumNumber,
                _stakeType,
            })
        }
        ///Creates a new call builder for the [`slashableStakeLookAheadPerQuorum`] function.
        pub fn slashableStakeLookAheadPerQuorum(
            &self,
            _0: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, slashableStakeLookAheadPerQuorumCall, N>
        {
            self.call_builder(&slashableStakeLookAheadPerQuorumCall { _0 })
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
            self.call_builder(&strategyParamsByIndexCall {
                quorumNumber,
                index,
            })
        }
        ///Creates a new call builder for the [`strategyParamsLength`] function.
        pub fn strategyParamsLength(
            &self,
            quorumNumber: u8,
        ) -> alloy_contract::SolCallBuilder<T, &P, strategyParamsLengthCall, N> {
            self.call_builder(&strategyParamsLengthCall { quorumNumber })
        }
        ///Creates a new call builder for the [`updateOperatorStake`] function.
        pub fn updateOperatorStake(
            &self,
            operator: alloy::sol_types::private::Address,
            operatorId: alloy::sol_types::private::FixedBytes<32>,
            quorumNumbers: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateOperatorStakeCall, N> {
            self.call_builder(&updateOperatorStakeCall {
                operator,
                operatorId,
                quorumNumbers,
            })
        }
        ///Creates a new call builder for the [`weightOfOperatorForQuorum`] function.
        pub fn weightOfOperatorForQuorum(
            &self,
            quorumNumber: u8,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, weightOfOperatorForQuorumCall, N> {
            self.call_builder(&weightOfOperatorForQuorumCall {
                quorumNumber,
                operator,
            })
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
            T: alloy_contract::private::Transport + ::core::clone::Clone,
            P: alloy_contract::private::Provider<T, N>,
            N: alloy_contract::private::Network,
        > StakeRegistryInstance<T, P, N>
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
        pub fn QuorumCreated_filter(&self) -> alloy_contract::Event<T, &P, QuorumCreated, N> {
            self.event_filter::<QuorumCreated>()
        }
        ///Creates a new event filter for the [`StakeTypeSet`] event.
        pub fn StakeTypeSet_filter(&self) -> alloy_contract::Event<T, &P, StakeTypeSet, N> {
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
