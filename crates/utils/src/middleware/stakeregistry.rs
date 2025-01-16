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
    clippy::style,
    clippy::empty_structs_with_brackets
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
    clippy::style,
    clippy::empty_structs_with_brackets
)]
pub mod StakeRegistry {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x610100604052348015610010575f5ffd5b50604051613c40380380613c4083398101604081905261002f91610068565b6001600160a01b0393841660e052918316608052821660a0521660c0526100c4565b6001600160a01b0381168114610065575f5ffd5b50565b5f5f5f5f6080858703121561007b575f5ffd5b845161008681610051565b602086015190945061009781610051565b60408601519093506100a881610051565b60608601519092506100b981610051565b939692955090935050565b60805160a05160c05160e051613b186101285f395f818161045a015281816113980152818161140601528181611e150152611f4501525f818161031401528181612c870152612d1801525f61043301525f81816106710152611c860152613b185ff3fe608060405234801561000f575f5ffd5b5060043610610234575f3560e01c806386c0685611610135578063c601527d116100b4578063df5cf72311610079578063df5cf7231461066c578063e086adb314610693578063f2be94ae146106a6578063f851e198146106b9578063fa28c627146106cc575f5ffd5b8063c601527d1461060d578063c8294c5614610620578063cc5a7c2014610633578063d5eccc0514610646578063dd9846b914610659575f5ffd5b8063adc804da116100fa578063adc804da1461056c578063b6904b78146105ac578063bc9a40c3146105bf578063bd29b8cd146105d2578063c46778a5146105e5575f5ffd5b806386c06856146104c95780639ab4d6ff146104dc5780639f3ccf65146105165780639f8aff2614610529578063ac6bfb031461054c575f5ffd5b80635401ed27116101c15780636b3aa72e116101865780636b3aa72e1461042e5780636d14a9871461045557806375d4173a1461047c5780637c1723471461048f57806381c07502146104a9575f5ffd5b80635401ed271461039f5780635e5a6775146103b25780635f1f2d77146103c157806366acfefe146103d4578063697fbd93146103ff575f5ffd5b8063255047771161020757806325504777146102ce5780632cd95940146102ef5780633998fdd31461030f5780633ca5a5f51461034e5780634bd26e0914610370575f5ffd5b80630491b41c14610238578063087324611461026d5780631f9b74e01461028e57806320b66298146102b9575b5f5ffd5b61025a610246366004612e3c565b60ff165f9081526001602052604090205490565b6040519081526020015b60405180910390f35b61028061027b366004612e55565b6106df565b604051610264929190612e7d565b6102a161029c366004612eb3565b610724565b6040516001600160601b039091168152602001610264565b6102cc6102c7366004612f28565b610746565b005b6102e16102dc366004612fe3565b610995565b60405161026492919061307d565b6103026102fd3660046130a1565b610b98565b60405161026491906130cb565b6103367f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610264565b61025a61035c366004612e3c565b60ff165f9081526003602052604090205490565b61025a61037e3660046130a1565b5f91825260026020908152604080842060ff93909316845291905290205490565b6102a16103ad3660046130a1565b610c35565b61025a670de0b6b3a764000081565b6102cc6103cf3660046131d2565b610c4d565b6103e76103e2366004612fe3565b610f7b565b6040516001600160c01b039091168152602001610264565b61042161040d366004612e3c565b60056020525f908152604090205460ff1681565b604051610264919061328e565b6103367f000000000000000000000000000000000000000000000000000000000000000081565b6103367f000000000000000000000000000000000000000000000000000000000000000081565b6102cc61048a36600461335f565b611017565b610497602081565b60405160ff9091168152602001610264565b6104bc6104b73660046133cb565b6110fc565b6040516102649190613419565b6102cc6104d7366004613456565b61131f565b6105016104ea366004612e3c565b60066020525f908152604090205463ffffffff1681565b60405163ffffffff9091168152602001610264565b610336610524366004612e55565b611340565b61053c610537366004612e3c565b611374565b6040519015158152602001610264565b61055f61055a366004613483565b611499565b60405161026491906134b3565b61057f61057a366004612e55565b61152f565b6040805182516001600160a01b031681526020928301516001600160601b03169281019290925201610264565b61055f6105ba366004612e55565b6115a6565b6102cc6105cd3660046134ed565b611633565b6102cc6105e0366004613515565b61164f565b6102a16105f3366004612e3c565b5f602081905290815260409020546001600160601b031681565b6102cc61061b366004613543565b6116b2565b6102a161062e36600461358d565b6116ce565b6102cc6106413660046135c7565b61174a565b6102a1610654366004612e3c565b61183b565b610501610667366004613631565b61188c565b6103367f000000000000000000000000000000000000000000000000000000000000000081565b6102cc6106a136600461366a565b611898565b6102a16106b4366004613692565b6118b4565b61055f6106c73660046130a1565b611947565b6102a16106da366004613631565b611a2d565b6003602052815f5260405f2081815481106106f8575f80fd5b5f918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b5f8261072f81611a8c565b5f61073a8585611b07565b509250505b5092915050565b61074e611e13565b8461075881611a8c565b83806107d3576040805162461bcd60e51b81526020600482015260248101919091527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a206e6f20737472617465677920696e64696365732070726f766964656460648201526084015b60405180910390fd5b8281146108485760405162461bcd60e51b815260206004820152603960248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a20696e707574206c656e677468206d69736d617463680000000000000060648201526084016107ca565b60ff87165f908152600360205260408120905b8281101561098a57858582818110610875576108756136d1565b905060200201602081019061088a91906136e5565b8289898481811061089d5761089d6136d1565b90506020020135815481106108b4576108b46136d1565b905f5260205f20015f0160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a8581811061091a5761091a6136d1565b9050602002013581548110610931576109316136d1565b5f918252602090912001546001600160a01b0316888885818110610957576109576136d1565b905060200201602081019061096c91906136e5565b60405161097a929190612e7d565b60405180910390a260010161085b565b505050505050505050565b6060806109a0611f3a565b5f836001600160401b038111156109b9576109b9613144565b6040519080825280602002602001820160405280156109e2578160200160208202803683370190505b5090505f846001600160401b038111156109fe576109fe613144565b604051908082528060200260200182016040528015610a27578160200160208202803683370190505b5090505f5b85811015610b8a575f878783818110610a4757610a476136d1565b919091013560f81c9150610a5c905081611a8c565b5f5f610a68838d611b07565b9150915080610b055760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a2060448201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360648201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000608482015260a4016107ca565b5f610b118c8585611fed565b905082878681518110610b2657610b266136d1565b60200260200101906001600160601b031690816001600160601b031681525050610b508482612266565b868681518110610b6257610b626136d1565b6001600160601b0390921660209283029190910190910152505060019092019150610a2c9050565b509097909650945050505050565b5f82815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610c28575f848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610bd0565b5050505090505b92915050565b5f5f610c418484611947565b60400151949350505050565b610c55611e13565b81610c5f81611a8c565b815180610cd45760405162461bcd60e51b815260206004820152603d60248201527f5374616b6552656769737472792e72656d6f7665537472617465676965733a2060448201527f6e6f20696e646963657320746f2072656d6f76652070726f766964656400000060648201526084016107ca565b60ff84165f9081526003602090815260408083206004909252822090915b83811015610f72578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f784888481518110610d3257610d326136d1565b602002602001015181548110610d4a57610d4a6136d1565b5f91825260209182902001546040516001600160a01b0390911681520160405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7584888481518110610da757610da76136d1565b602002602001015181548110610dbf57610dbf6136d1565b5f91825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a282548390610dfe90600190613712565b81548110610e0e57610e0e6136d1565b905f5260205f200183878381518110610e2957610e296136d1565b602002602001015181548110610e4157610e416136d1565b5f91825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b9182900416021790558254839080610e9357610e93613725565b5f8281526020812082015f199081019190915501905581548290610eb990600190613712565b81548110610ec957610ec96136d1565b905f5260205f20015f9054906101000a90046001600160a01b031682878381518110610ef757610ef76136d1565b602002602001015181548110610f0f57610f0f6136d1565b905f5260205f20015f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555081805480610f4a57610f4a613725565b5f8281526020902081015f1990810180546001600160a01b0319169055019055600101610cf2565b50505050505050565b5f610f84611f3a565b5f805b8381101561100d575f858583818110610fa257610fa26136d1565b919091013560f81c9150610fb7905081611a8c565b5f5f610fc3838b611b07565b9150915080610fe4575f9150600160ff84161b6001600160c01b0386161794505b5f610ff08a8585611fed565b9050610ffc8482612266565b505060019093019250610f87915050565b5095945050505050565b61101f611f3a565b60ff83165f908152600160205260409020541561104e5760405162461bcd60e51b81526004016107ca90613739565b61105883826123d7565b61106283836127f4565b61106c835f61285c565b505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60605f826001600160401b0381111561111757611117613144565b604051908082528060200260200182016040528015611140578160200160208202803683370190505b5090505f5b83811015611314575f858583818110611160576111606136d1565b919091013560f81c9150611175905081611a8c565b60ff81165f908152600160205260408120805463ffffffff8a16929061119d5761119d6136d1565b5f9182526020909120015463ffffffff1611156112485760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360648201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000608482015260a4016107ca565b60ff81165f90815260016020526040812054905b818110156113095760ff83165f90815260016020819052604090912063ffffffff8b169161128a8486613712565b6112949190613712565b815481106112a4576112a46136d1565b5f9182526020909120015463ffffffff16116113015760016112c68284613712565b6112d09190613712565b8585815181106112e2576112e26136d1565b602002602001019063ffffffff16908163ffffffff1681525050611309565b60010161125c565b505050600101611145565b5090505b9392505050565b611327611e13565b8161133181611a8c565b61133b838361285c565b505050565b6004602052815f5260405f208181548110611359575f80fd5b5f918252602090912001546001600160a01b03169150829050565b60405163a4d7871f60e01b815260ff821660048201525f9081906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a4d7871f90602401602060405180830381865afa1580156113dd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611401919061378e565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663cabbb17f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611460573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611484919061378e565b9050808015611491575081155b949350505050565b604080516060810182525f80825260208083018290528284018290528582526002815283822060ff881683529052919091208054839081106114dd576114dd6136d1565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091525f808252602082015260ff83165f908152600360205260409020805483908110611565576115656136d1565b5f918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182525f808252602080830182905282840182905260ff8616825260019052919091208054839081106115e2576115e26136d1565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b61163b611e13565b8161164581611a8c565b61133b83836127f4565b611657611f3a565b5f5b818110156116ac575f838383818110611674576116746136d1565b919091013560f81c9150611689905081611a8c565b5f61169586835f611fed565b90506116a18282612266565b505050600101611659565b50505050565b6116ba611e13565b816116c481611a8c565b61133b83836123d7565b60ff83165f9081526001602052604081208054829190849081106116f4576116f46136d1565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610c4181856128c9565b611752611f3a565b60ff84165f90815260016020526040902054156117815760405162461bcd60e51b81526004016107ca90613739565b61178b84826123d7565b61179584846127f4565b6117a084600161285c565b6117aa8483612a42565b50505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60ff81165f90815260016020819052604082208054909161185b91613712565b8154811061186b5761186b6136d1565b5f91825260209091200154600160401b90046001600160601b031692915050565b5f611491848484612aae565b6118a0611e13565b816118aa81611a8c565b61133b8383612a42565b5f82815260026020908152604080832060ff8816845290915281208054829190849081106118e4576118e46136d1565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b9093049290921690820152905061193a81866128c9565b6040015195945050505050565b60408051606080820183525f80835260208084018290528385018290528682526002815284822060ff87168352815284822054855193840186528284529083018290529382018190529192918290036119a3579150610c2f9050565b5f85815260026020908152604080832060ff8816845290915290206119c9600184613712565b815481106119d9576119d96136d1565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610c2f915050565b5f83815260026020908152604080832060ff861684529091528120611a53858585612aae565b63ffffffff1681548110611a6957611a696136d1565b5f91825260209091200154600160401b90046001600160601b0316949350505050565b60ff81165f90815260016020526040902054611b045760405162461bcd60e51b815260206004820152603160248201527f5374616b6552656769737472792e71756f72756d4578697374733a2071756f726044820152701d5b48191bd95cc81b9bdd08195e1a5cdd607a1b60648201526084016107ca565b50565b5f5f5f5f611b238660ff165f9081526003602052604090205490565b604080518082019091525f80825260208201529091506060600160ff808a165f90815260056020526040902054166001811115611b6257611b6261327a565b03611c5b57611b718888612c11565b90505f5b83811015611c555760ff89165f908152600360205260409020805482908110611ba057611ba06136d1565b5f9182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611bed57611bed6136d1565b60200260200101511115611c4d57670de0b6b3a764000083602001516001600160601b0316838381518110611c2457611c246136d1565b6020026020010151611c3691906137ad565b611c4091906137c4565b611c4a90866137e3565b94505b600101611b75565b50611de1565b60ff88165f908152600460208190526040918290209151639004134760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001692639004134792611cba928c92910161383e565b5f60405180830381865afa158015611cd4573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611cfb91908101906138bc565b90505f5b83811015611ddf5760ff89165f908152600360205260409020805482908110611d2a57611d2a6136d1565b5f9182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611d7757611d776136d1565b60200260200101511115611dd757670de0b6b3a764000083602001516001600160601b0316838381518110611dae57611dae6136d1565b6020026020010151611dc091906137ad565b611dca91906137c4565b611dd490866137e3565b94505b600101611cff565b505b50505060ff85165f908152602081905260409020549092506001600160601b03908116908316101590505b9250929050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611e6f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e9391906138ed565b6001600160a01b0316336001600160a01b031614611f385760405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e60448201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f6620746064820152753432903932b3b4b9ba393ca1b7b7b93234b730ba37b960511b608482015260a4016107ca565b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611f385760405162461bcd60e51b815260206004820152604c60248201527f5374616b6552656769737472792e6f6e6c795265676973747279436f6f72646960448201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260648201526b3ca1b7b7b93234b730ba37b960a11b608482015260a4016107ca565b5f83815260026020908152604080832060ff8616845290915281205481908082036120b1575f86815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff199096169190921617939093171691909117905561220c565b5f86815260026020908152604080832060ff8916845290915281206120d7600184613712565b815481106120e7576120e76136d1565b5f91825260209091200180546001600160601b03600160401b909104811694509091508516830361211d575f9350505050611318565b805463ffffffff438116911603612155578054600160401b600160a01b031916600160401b6001600160601b0387160217815561220a565b805467ffffffff000000001916600160201b4363ffffffff9081168281029390931784555f8a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a261225c8285612de3565b9695505050505050565b60ff82165f90815260016020819052604082208054918391906122899084613712565b81548110612299576122996136d1565b905f5260205f20019050835f036122c45754600160401b90046001600160601b03169150610c2f9050565b80545f906122e290600160401b90046001600160601b031686612dfa565b825490915063ffffffff43811691160361231d578154600160401b600160a01b031916600160401b6001600160601b038316021782556123ce565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff89165f90815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b5f81511161243a5760405162461bcd60e51b815260206004820152603860248201525f516020613ac35f395f51905f5260448201527f3a206e6f20737472617465676965732070726f7669646564000000000000000060648201526084016107ca565b805160ff83165f908152600360209081526040909120549061245c8383613908565b11156124cb5760405162461bcd60e51b815260206004820152604560248201525f516020613ac35f395f51905f5260448201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60648201526408a9c8ea8960db1b608482015260a4016107ca565b5f5b828110156127ed575f5b6124e18284613908565b8110156125b2578482815181106124fa576124fa6136d1565b60200260200101515f01516001600160a01b031660035f8860ff1660ff1681526020019081526020015f208281548110612536576125366136d1565b5f918252602090912001546001600160a01b0316036125aa5760405162461bcd60e51b815260206004820152603d60248201525f516020613ac35f395f51905f5260448201527f3a2063616e6e6f74206164642073616d6520737472617465677920327800000060648201526084016107ca565b6001016124d7565b505f8482815181106125c6576125c66136d1565b6020026020010151602001516001600160601b03161161264a5760405162461bcd60e51b815260206004820152604660248201525f516020613ac35f395f51905f5260448201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f206064820152651dd95a59da1d60d21b608482015260a4016107ca565b60ff85165f908152600360205260409020845185908390811061266f5761266f6136d1565b60209081029190910181015182546001810184555f9384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff87168252600490526040902084518590839081106126d3576126d36136d1565b6020908102919091018101515182546001810184555f938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540490869084908110612749576127496136d1565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a758583815181106127a6576127a66136d1565b60200260200101515f01518684815181106127c3576127c36136d1565b6020026020010151602001516040516127dd929190612e7d565b60405180910390a26001016124cd565b5050505050565b60ff82165f818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b60ff82165f908152600560205260409020805482919060ff1916600183818111156128895761288961327a565b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d816040516128bd919061328e565b60405180910390a15050565b815f015163ffffffff168163ffffffff16101561296d5760405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a207374616b6555706461746520697320606482015275333937b69030b33a32b910313637b1b5a73ab6b132b960511b608482015260a4016107ca565b602082015163ffffffff1615806129935750816020015163ffffffff168163ffffffff16105b612a3e5760405162461bcd60e51b815260206004820152606a60248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560648201527f72207374616b6555706461746520617661696c61626c65206265666f726520626084820152693637b1b5a73ab6b132b960b11b60a482015260c4016107ca565b5050565b60ff82165f90815260066020908152604091829020805463ffffffff19811663ffffffff8681169182179093558451929091168083529282015290917f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7910160405180910390a1505050565b5f83815260026020908152604080832060ff86168452909152812054805b8015612b4c575f86815260026020908152604080832060ff89168452909152902063ffffffff851690612b00600184613712565b81548110612b1057612b106136d1565b5f9182526020909120015463ffffffff1611612b3a57612b31600182613712565b92505050611318565b80612b448161391b565b915050612acc565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e4016107ca565b6040805160018082528183019092526060915f91906020808301908036833701905050905082815f81518110612c4957612c496136d1565b6001600160a01b0390921660209283029190910182015260ff85165f90815260069091526040812054612c829063ffffffff1642613908565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663ca8aa7c76040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ce1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d0591906138ed565b6040805180820182526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000008116825260ff8a1660208084018290525f9182526004908190529084902093516315d5962560e11b81529490911693632bab2c4a93612d7b93928992899101613930565b5f60405180830381865afa158015612d95573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612dbc91908101906139bb565b9050805f81518110612dd057612dd06136d1565b6020026020010151935050505092915050565b5f6113186001600160601b03808516908416613a6a565b5f5f821215612e1d57612e0c82613a89565b612e169084613aa3565b9050610c2f565b612e1682846137e3565b803560ff81168114612e37575f5ffd5b919050565b5f60208284031215612e4c575f5ffd5b61131882612e27565b5f5f60408385031215612e66575f5ffd5b612e6f83612e27565b946020939093013593505050565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114611b04575f5ffd5b5f5f60408385031215612ec4575f5ffd5b612ecd83612e27565b91506020830135612edd81612e9f565b809150509250929050565b5f5f83601f840112612ef8575f5ffd5b5081356001600160401b03811115612f0e575f5ffd5b6020830191508360208260051b8501011115611e0c575f5ffd5b5f5f5f5f5f60608688031215612f3c575f5ffd5b612f4586612e27565b945060208601356001600160401b03811115612f5f575f5ffd5b612f6b88828901612ee8565b90955093505060408601356001600160401b03811115612f89575f5ffd5b612f9588828901612ee8565b969995985093965092949392505050565b5f5f83601f840112612fb6575f5ffd5b5081356001600160401b03811115612fcc575f5ffd5b602083019150836020828501011115611e0c575f5ffd5b5f5f5f5f60608587031215612ff6575f5ffd5b843561300181612e9f565b93506020850135925060408501356001600160401b03811115613022575f5ffd5b61302e87828801612fa6565b95989497509550505050565b5f8151808452602084019350602083015f5b828110156130735781516001600160601b031686526020958601959091019060010161304c565b5093949350505050565b604081525f61308f604083018561303a565b82810360208401526123ce818561303a565b5f5f604083850312156130b2575f5ffd5b823591506130c260208401612e27565b90509250929050565b602080825282518282018190525f918401906040840190835b818110156131395761312383855163ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b60209390930192606092909201916001016130e4565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b038111828210171561317a5761317a613144565b60405290565b604051601f8201601f191681016001600160401b03811182821017156131a8576131a8613144565b604052919050565b5f6001600160401b038211156131c8576131c8613144565b5060051b60200190565b5f5f604083850312156131e3575f5ffd5b6131ec83612e27565b915060208301356001600160401b03811115613206575f5ffd5b8301601f81018513613216575f5ffd5b8035613229613224826131b0565b613180565b8082825260208201915060208360051b85010192508783111561324a575f5ffd5b6020840193505b8284101561326c578335825260209384019390910190613251565b809450505050509250929050565b634e487b7160e01b5f52602160045260245ffd5b60208101600283106132ae57634e487b7160e01b5f52602160045260245ffd5b91905290565b80356001600160601b0381168114612e37575f5ffd5b5f82601f8301126132d9575f5ffd5b81356132e7613224826131b0565b8082825260208201915060208360061b860101925085831115613308575f5ffd5b602085015b8381101561100d5760408188031215613324575f5ffd5b61332c613158565b813561333781612e9f565b8152613345602083016132b4565b60208201528084525060208301925060408101905061330d565b5f5f5f60608486031215613371575f5ffd5b61337a84612e27565b9250613388602085016132b4565b915060408401356001600160401b038111156133a2575f5ffd5b6133ae868287016132ca565b9150509250925092565b803563ffffffff81168114612e37575f5ffd5b5f5f5f604084860312156133dd575f5ffd5b6133e6846133b8565b925060208401356001600160401b03811115613400575f5ffd5b61340c86828701612fa6565b9497909650939450505050565b602080825282518282018190525f918401906040840190835b8181101561313957835163ffffffff16835260209384019390920191600101613432565b5f5f60408385031215613467575f5ffd5b61347083612e27565b9150602083013560028110612edd575f5ffd5b5f5f5f60608486031215613495575f5ffd5b61349e84612e27565b95602085013595506040909401359392505050565b60608101610c2f828463ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b5f5f604083850312156134fe575f5ffd5b61350783612e27565b91506130c2602084016132b4565b5f5f5f60408486031215613527575f5ffd5b8335925060208401356001600160401b03811115613400575f5ffd5b5f5f60408385031215613554575f5ffd5b61355d83612e27565b915060208301356001600160401b03811115613577575f5ffd5b613583858286016132ca565b9150509250929050565b5f5f5f6060848603121561359f575f5ffd5b6135a884612e27565b92506135b6602085016133b8565b929592945050506040919091013590565b5f5f5f5f608085870312156135da575f5ffd5b6135e385612e27565b93506135f1602086016132b4565b92506135ff604086016133b8565b915060608501356001600160401b03811115613619575f5ffd5b613625878288016132ca565b91505092959194509250565b5f5f5f60608486031215613643575f5ffd5b8335925061365360208501612e27565b9150613661604085016133b8565b90509250925092565b5f5f6040838503121561367b575f5ffd5b61368483612e27565b91506130c2602084016133b8565b5f5f5f5f608085870312156136a5575f5ffd5b6136ae85612e27565b93506136bc602086016133b8565b93969395505050506040820135916060013590565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156136f5575f5ffd5b611318826132b4565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610c2f57610c2f6136fe565b634e487b7160e01b5f52603160045260245ffd5b60208082526035908201527f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a2060408201527471756f72756d20616c72656164792065786973747360581b606082015260800190565b5f6020828403121561379e575f5ffd5b81518015158114611318575f5ffd5b8082028115828204841417610c2f57610c2f6136fe565b5f826137de57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160601b038181168382160190811115610c2f57610c2f6136fe565b5f8154808452602084019350825f5260205f205f5b828110156130735781546001600160a01b0316865260209095019460019182019101613817565b6001600160a01b03831681526040602082018190525f9061149190830184613802565b5f82601f830112613870575f5ffd5b815161387e613224826131b0565b8082825260208201915060208360051b86010192508583111561389f575f5ffd5b602085015b8381101561100d5780518352602092830192016138a4565b5f602082840312156138cc575f5ffd5b81516001600160401b038111156138e1575f5ffd5b61149184828501613861565b5f602082840312156138fd575f5ffd5b815161131881612e9f565b80820180821115610c2f57610c2f6136fe565b5f81613929576139296136fe565b505f190190565b84516001600160a01b0316815260208086015163ffffffff168183015260a06040830181905285519083018190525f9186019060c0840190835b818110156139915783516001600160a01b031683526020938401939092019160010161396a565b505083810360608501526139a58187613802565b925050506123ce608083018463ffffffff169052565b5f602082840312156139cb575f5ffd5b81516001600160401b038111156139e0575f5ffd5b8201601f810184136139f0575f5ffd5b80516139fe613224826131b0565b8082825260208201915060208360051b850101925086831115613a1f575f5ffd5b602084015b83811015613a5f5780516001600160401b03811115613a41575f5ffd5b613a5089602083890101613861565b84525060209283019201613a24565b509695505050505050565b8181035f83128015838313168383128216171561073f5761073f6136fe565b5f600160ff1b8201613a9d57613a9d6136fe565b505f0390565b6001600160601b038281168282160390811115610c2f57610c2f6136fe56fe5374616b6552656769737472792e5f6164645374726174656779506172616d73a26469706673582212208e87719f9a1798605a8a38743948d4fa0c43a41d75e838d40d6af0fcdf19d45564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"a\x01\0`@R4\x80\x15a\0\x10W__\xFD[P`@Qa<@8\x03\x80a<@\x839\x81\x01`@\x81\x90Ra\0/\x91a\0hV[`\x01`\x01`\xA0\x1B\x03\x93\x84\x16`\xE0R\x91\x83\x16`\x80R\x82\x16`\xA0R\x16`\xC0Ra\0\xC4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\0eW__\xFD[PV[____`\x80\x85\x87\x03\x12\x15a\0{W__\xFD[\x84Qa\0\x86\x81a\0QV[` \x86\x01Q\x90\x94Pa\0\x97\x81a\0QV[`@\x86\x01Q\x90\x93Pa\0\xA8\x81a\0QV[``\x86\x01Q\x90\x92Pa\0\xB9\x81a\0QV[\x93\x96\x92\x95P\x90\x93PPV[`\x80Q`\xA0Q`\xC0Q`\xE0Qa;\x18a\x01(_9_\x81\x81a\x04Z\x01R\x81\x81a\x13\x98\x01R\x81\x81a\x14\x06\x01R\x81\x81a\x1E\x15\x01Ra\x1FE\x01R_\x81\x81a\x03\x14\x01R\x81\x81a,\x87\x01Ra-\x18\x01R_a\x043\x01R_\x81\x81a\x06q\x01Ra\x1C\x86\x01Ra;\x18_\xF3\xFE`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x024W_5`\xE0\x1C\x80c\x86\xC0hV\x11a\x015W\x80c\xC6\x01R}\x11a\0\xB4W\x80c\xDF\\\xF7#\x11a\0yW\x80c\xDF\\\xF7#\x14a\x06lW\x80c\xE0\x86\xAD\xB3\x14a\x06\x93W\x80c\xF2\xBE\x94\xAE\x14a\x06\xA6W\x80c\xF8Q\xE1\x98\x14a\x06\xB9W\x80c\xFA(\xC6'\x14a\x06\xCCW__\xFD[\x80c\xC6\x01R}\x14a\x06\rW\x80c\xC8)LV\x14a\x06 W\x80c\xCCZ| \x14a\x063W\x80c\xD5\xEC\xCC\x05\x14a\x06FW\x80c\xDD\x98F\xB9\x14a\x06YW__\xFD[\x80c\xAD\xC8\x04\xDA\x11a\0\xFAW\x80c\xAD\xC8\x04\xDA\x14a\x05lW\x80c\xB6\x90Kx\x14a\x05\xACW\x80c\xBC\x9A@\xC3\x14a\x05\xBFW\x80c\xBD)\xB8\xCD\x14a\x05\xD2W\x80c\xC4gx\xA5\x14a\x05\xE5W__\xFD[\x80c\x86\xC0hV\x14a\x04\xC9W\x80c\x9A\xB4\xD6\xFF\x14a\x04\xDCW\x80c\x9F<\xCFe\x14a\x05\x16W\x80c\x9F\x8A\xFF&\x14a\x05)W\x80c\xACk\xFB\x03\x14a\x05LW__\xFD[\x80cT\x01\xED'\x11a\x01\xC1W\x80ck:\xA7.\x11a\x01\x86W\x80ck:\xA7.\x14a\x04.W\x80cm\x14\xA9\x87\x14a\x04UW\x80cu\xD4\x17:\x14a\x04|W\x80c|\x17#G\x14a\x04\x8FW\x80c\x81\xC0u\x02\x14a\x04\xA9W__\xFD[\x80cT\x01\xED'\x14a\x03\x9FW\x80c^Zgu\x14a\x03\xB2W\x80c_\x1F-w\x14a\x03\xC1W\x80cf\xAC\xFE\xFE\x14a\x03\xD4W\x80ci\x7F\xBD\x93\x14a\x03\xFFW__\xFD[\x80c%PGw\x11a\x02\x07W\x80c%PGw\x14a\x02\xCEW\x80c,\xD9Y@\x14a\x02\xEFW\x80c9\x98\xFD\xD3\x14a\x03\x0FW\x80c<\xA5\xA5\xF5\x14a\x03NW\x80cK\xD2n\t\x14a\x03pW__\xFD[\x80c\x04\x91\xB4\x1C\x14a\x028W\x80c\x08s$a\x14a\x02mW\x80c\x1F\x9Bt\xE0\x14a\x02\x8EW\x80c \xB6b\x98\x14a\x02\xB9W[__\xFD[a\x02Za\x02F6`\x04a.<V[`\xFF\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x80a\x02{6`\x04a.UV[a\x06\xDFV[`@Qa\x02d\x92\x91\x90a.}V[a\x02\xA1a\x02\x9C6`\x04a.\xB3V[a\x07$V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02dV[a\x02\xCCa\x02\xC76`\x04a/(V[a\x07FV[\0[a\x02\xE1a\x02\xDC6`\x04a/\xE3V[a\t\x95V[`@Qa\x02d\x92\x91\x90a0}V[a\x03\x02a\x02\xFD6`\x04a0\xA1V[a\x0B\x98V[`@Qa\x02d\x91\x90a0\xCBV[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02dV[a\x02Za\x03\\6`\x04a.<V[`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Za\x03~6`\x04a0\xA1V[_\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\xA1a\x03\xAD6`\x04a0\xA1V[a\x0C5V[a\x02Zg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xCCa\x03\xCF6`\x04a1\xD2V[a\x0CMV[a\x03\xE7a\x03\xE26`\x04a/\xE3V[a\x0F{V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02dV[a\x04!a\x04\r6`\x04a.<V[`\x05` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x02d\x91\x90a2\x8EV[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xCCa\x04\x8A6`\x04a3_V[a\x10\x17V[a\x04\x97` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02dV[a\x04\xBCa\x04\xB76`\x04a3\xCBV[a\x10\xFCV[`@Qa\x02d\x91\x90a4\x19V[a\x02\xCCa\x04\xD76`\x04a4VV[a\x13\x1FV[a\x05\x01a\x04\xEA6`\x04a.<V[`\x06` R_\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02dV[a\x036a\x05$6`\x04a.UV[a\x13@V[a\x05<a\x0576`\x04a.<V[a\x13tV[`@Q\x90\x15\x15\x81R` \x01a\x02dV[a\x05_a\x05Z6`\x04a4\x83V[a\x14\x99V[`@Qa\x02d\x91\x90a4\xB3V[a\x05\x7Fa\x05z6`\x04a.UV[a\x15/V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02dV[a\x05_a\x05\xBA6`\x04a.UV[a\x15\xA6V[a\x02\xCCa\x05\xCD6`\x04a4\xEDV[a\x163V[a\x02\xCCa\x05\xE06`\x04a5\x15V[a\x16OV[a\x02\xA1a\x05\xF36`\x04a.<V[_` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\xCCa\x06\x1B6`\x04a5CV[a\x16\xB2V[a\x02\xA1a\x06.6`\x04a5\x8DV[a\x16\xCEV[a\x02\xCCa\x06A6`\x04a5\xC7V[a\x17JV[a\x02\xA1a\x06T6`\x04a.<V[a\x18;V[a\x05\x01a\x06g6`\x04a61V[a\x18\x8CV[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xCCa\x06\xA16`\x04a6jV[a\x18\x98V[a\x02\xA1a\x06\xB46`\x04a6\x92V[a\x18\xB4V[a\x05_a\x06\xC76`\x04a0\xA1V[a\x19GV[a\x02\xA1a\x06\xDA6`\x04a61V[a\x1A-V[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\x06\xF8W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[_\x82a\x07/\x81a\x1A\x8CV[_a\x07:\x85\x85a\x1B\x07V[P\x92PP[P\x92\x91PPV[a\x07Na\x1E\x13V[\x84a\x07X\x81a\x1A\x8CV[\x83\x80a\x07\xD3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x08HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xCAV[`\xFF\x87\x16_\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\t\x8AW\x85\x85\x82\x81\x81\x10a\x08uWa\x08ua6\xD1V[\x90P` \x02\x01` \x81\x01\x90a\x08\x8A\x91\x90a6\xE5V[\x82\x89\x89\x84\x81\x81\x10a\x08\x9DWa\x08\x9Da6\xD1V[\x90P` \x02\x015\x81T\x81\x10a\x08\xB4Wa\x08\xB4a6\xD1V[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\t\x1AWa\t\x1Aa6\xD1V[\x90P` \x02\x015\x81T\x81\x10a\t1Wa\t1a6\xD1V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\tWWa\tWa6\xD1V[\x90P` \x02\x01` \x81\x01\x90a\tl\x91\x90a6\xE5V[`@Qa\tz\x92\x91\x90a.}V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x08[V[PPPPPPPPPV[``\x80a\t\xA0a\x1F:V[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xB9Wa\t\xB9a1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\xE2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xFEWa\t\xFEa1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n'W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x85\x81\x10\x15a\x0B\x8AW_\x87\x87\x83\x81\x81\x10a\nGWa\nGa6\xD1V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\n\\\x90P\x81a\x1A\x8CV[__a\nh\x83\x8Da\x1B\x07V[\x91P\x91P\x80a\x0B\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\xCAV[_a\x0B\x11\x8C\x85\x85a\x1F\xEDV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0B&Wa\x0B&a6\xD1V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0BP\x84\x82a\"fV[\x86\x86\x81Q\x81\x10a\x0BbWa\x0Bba6\xD1V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x90\x92\x01\x91Pa\n,\x90PV[P\x90\x97\x90\x96P\x94PPPPPV[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x0C(W_\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0B\xD0V[PPPP\x90P[\x92\x91PPV[__a\x0CA\x84\x84a\x19GV[`@\x01Q\x94\x93PPPPV[a\x0CUa\x1E\x13V[\x81a\x0C_\x81a\x1A\x8CV[\x81Q\x80a\x0C\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x07\xCAV[`\xFF\x84\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91[\x83\x81\x10\x15a\x0FrW\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\r2Wa\r2a6\xD1V[` \x02` \x01\x01Q\x81T\x81\x10a\rJWa\rJa6\xD1V[_\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\r\xA7Wa\r\xA7a6\xD1V[` \x02` \x01\x01Q\x81T\x81\x10a\r\xBFWa\r\xBFa6\xD1V[_\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x82T\x83\x90a\r\xFE\x90`\x01\x90a7\x12V[\x81T\x81\x10a\x0E\x0EWa\x0E\x0Ea6\xD1V[\x90_R` _ \x01\x83\x87\x83\x81Q\x81\x10a\x0E)Wa\x0E)a6\xD1V[` \x02` \x01\x01Q\x81T\x81\x10a\x0EAWa\x0EAa6\xD1V[_\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x82T\x83\x90\x80a\x0E\x93Wa\x0E\x93a7%V[_\x82\x81R` \x81 \x82\x01_\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x81T\x82\x90a\x0E\xB9\x90`\x01\x90a7\x12V[\x81T\x81\x10a\x0E\xC9Wa\x0E\xC9a6\xD1V[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x83\x81Q\x81\x10a\x0E\xF7Wa\x0E\xF7a6\xD1V[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\x0FWa\x0F\x0Fa6\xD1V[\x90_R` _ \x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x80T\x80a\x0FJWa\x0FJa7%V[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\x0C\xF2V[PPPPPPPV[_a\x0F\x84a\x1F:V[_\x80[\x83\x81\x10\x15a\x10\rW_\x85\x85\x83\x81\x81\x10a\x0F\xA2Wa\x0F\xA2a6\xD1V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x0F\xB7\x90P\x81a\x1A\x8CV[__a\x0F\xC3\x83\x8Ba\x1B\x07V[\x91P\x91P\x80a\x0F\xE4W_\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[_a\x0F\xF0\x8A\x85\x85a\x1F\xEDV[\x90Pa\x0F\xFC\x84\x82a\"fV[PP`\x01\x90\x93\x01\x92Pa\x0F\x87\x91PPV[P\x95\x94PPPPPV[a\x10\x1Fa\x1F:V[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x10NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xCA\x90a79V[a\x10X\x83\x82a#\xD7V[a\x10b\x83\x83a'\xF4V[a\x10l\x83_a(\\V[PP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x17Wa\x11\x17a1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11@W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x13\x14W_\x85\x85\x83\x81\x81\x10a\x11`Wa\x11`a6\xD1V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x11u\x90P\x81a\x1A\x8CV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x11\x9DWa\x11\x9Da6\xD1V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x12HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\xCAV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x13\tW`\xFF\x83\x16_\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x12\x8A\x84\x86a7\x12V[a\x12\x94\x91\x90a7\x12V[\x81T\x81\x10a\x12\xA4Wa\x12\xA4a6\xD1V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x13\x01W`\x01a\x12\xC6\x82\x84a7\x12V[a\x12\xD0\x91\x90a7\x12V[\x85\x85\x81Q\x81\x10a\x12\xE2Wa\x12\xE2a6\xD1V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x13\tV[`\x01\x01a\x12\\V[PPP`\x01\x01a\x11EV[P\x90P[\x93\x92PPPV[a\x13'a\x1E\x13V[\x81a\x131\x81a\x1A\x8CV[a\x13;\x83\x83a(\\V[PPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x13YW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@Qc\xA4\xD7\x87\x1F`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA4\xD7\x87\x1F\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x01\x91\x90a7\x8EV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCA\xBB\xB1\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x84\x91\x90a7\x8EV[\x90P\x80\x80\x15a\x14\x91WP\x81\x15[\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x14\xDDWa\x14\xDDa6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x83\x16_\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x15eWa\x15ea6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15\xE2Wa\x15\xE2a6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x16;a\x1E\x13V[\x81a\x16E\x81a\x1A\x8CV[a\x13;\x83\x83a'\xF4V[a\x16Wa\x1F:V[_[\x81\x81\x10\x15a\x16\xACW_\x83\x83\x83\x81\x81\x10a\x16tWa\x16ta6\xD1V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x16\x89\x90P\x81a\x1A\x8CV[_a\x16\x95\x86\x83_a\x1F\xEDV[\x90Pa\x16\xA1\x82\x82a\"fV[PPP`\x01\x01a\x16YV[PPPPV[a\x16\xBAa\x1E\x13V[\x81a\x16\xC4\x81a\x1A\x8CV[a\x13;\x83\x83a#\xD7V[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x16\xF4Wa\x16\xF4a6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x0CA\x81\x85a(\xC9V[a\x17Ra\x1F:V[`\xFF\x84\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x17\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xCA\x90a79V[a\x17\x8B\x84\x82a#\xD7V[a\x17\x95\x84\x84a'\xF4V[a\x17\xA0\x84`\x01a(\\V[a\x17\xAA\x84\x83a*BV[PPP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x81\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x18[\x91a7\x12V[\x81T\x81\x10a\x18kWa\x18ka6\xD1V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[_a\x14\x91\x84\x84\x84a*\xAEV[a\x18\xA0a\x1E\x13V[\x81a\x18\xAA\x81a\x1A\x8CV[a\x13;\x83\x83a*BV[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x18\xE4Wa\x18\xE4a6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x19:\x81\x86a(\xC9V[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x91\x92\x91\x82\x90\x03a\x19\xA3W\x91Pa\x0C/\x90PV[_\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x19\xC9`\x01\x84a7\x12V[\x81T\x81\x10a\x19\xD9Wa\x19\xD9a6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x0C/\x91PPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1AS\x85\x85\x85a*\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1AiWa\x1Aia6\xD1V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x90 Ta\x1B\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FStakeRegistry.quorumExists: quor`D\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B`d\x82\x01R`\x84\x01a\x07\xCAV[PV[____a\x1B#\x86`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90\x91P```\x01`\xFF\x80\x8A\x16_\x90\x81R`\x05` R`@\x90 T\x16`\x01\x81\x11\x15a\x1BbWa\x1Bba2zV[\x03a\x1C[Wa\x1Bq\x88\x88a,\x11V[\x90P_[\x83\x81\x10\x15a\x1CUW`\xFF\x89\x16_\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1B\xA0Wa\x1B\xA0a6\xD1V[_\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1B\xEDWa\x1B\xEDa6\xD1V[` \x02` \x01\x01Q\x11\x15a\x1CMWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1C$Wa\x1C$a6\xD1V[` \x02` \x01\x01Qa\x1C6\x91\x90a7\xADV[a\x1C@\x91\x90a7\xC4V[a\x1CJ\x90\x86a7\xE3V[\x94P[`\x01\x01a\x1BuV[Pa\x1D\xE1V[`\xFF\x88\x16_\x90\x81R`\x04` \x81\x90R`@\x91\x82\x90 \x91Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a\x1C\xBA\x92\x8C\x92\x91\x01a8>V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xD4W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1C\xFB\x91\x90\x81\x01\x90a8\xBCV[\x90P_[\x83\x81\x10\x15a\x1D\xDFW`\xFF\x89\x16_\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1D*Wa\x1D*a6\xD1V[_\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1DwWa\x1Dwa6\xD1V[` \x02` \x01\x01Q\x11\x15a\x1D\xD7Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1D\xAEWa\x1D\xAEa6\xD1V[` \x02` \x01\x01Qa\x1D\xC0\x91\x90a7\xADV[a\x1D\xCA\x91\x90a7\xC4V[a\x1D\xD4\x90\x86a7\xE3V[\x94P[`\x01\x01a\x1C\xFFV[P[PPP`\xFF\x85\x16_\x90\x81R` \x81\x90R`@\x90 T\x90\x92P`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EoW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x93\x91\x90a8\xEDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1F8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`D\x82\x01R\x7Fer: caller is not the owner of t`d\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1F8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`D\x82\x01R\x7Fnator: caller is not the Registr`d\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x82\x03a \xB1W_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"\x0CV[_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a \xD7`\x01\x84a7\x12V[\x81T\x81\x10a \xE7Wa \xE7a6\xD1V[_\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x03a!\x1DW_\x93PPPPa\x13\x18V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a!UW\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"\nV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U_\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"\\\x82\x85a-\xE3V[\x96\x95PPPPPPV[`\xFF\x82\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\"\x89\x90\x84a7\x12V[\x81T\x81\x10a\"\x99Wa\"\x99a6\xD1V[\x90_R` _ \x01\x90P\x83_\x03a\"\xC4WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\x0C/\x90PV[\x80T_\x90a\"\xE2\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a-\xFAV[\x82T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a#\x1DW\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua#\xCEV[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[_\x81Q\x11a$:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R_Q` a:\xC3_9_Q\x90_R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xCAV[\x80Q`\xFF\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a$\\\x83\x83a9\x08V[\x11\x15a$\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R_Q` a:\xC3_9_Q\x90_R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[_[\x82\x81\x10\x15a'\xEDW_[a$\xE1\x82\x84a9\x08V[\x81\x10\x15a%\xB2W\x84\x82\x81Q\x81\x10a$\xFAWa$\xFAa6\xD1V[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a%6Wa%6a6\xD1V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a%\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R_Q` a:\xC3_9_Q\x90_R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x07\xCAV[`\x01\x01a$\xD7V[P_\x84\x82\x81Q\x81\x10a%\xC6Wa%\xC6a6\xD1V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a&JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R_Q` a:\xC3_9_Q\x90_R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[`\xFF\x85\x16_\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a&oWa&oa6\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U_\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a&\xD3Wa&\xD3a6\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U_\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a'IWa'Ia6\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a'\xA6Wa'\xA6a6\xD1V[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a'\xC3Wa'\xC3a6\xD1V[` \x02` \x01\x01Q` \x01Q`@Qa'\xDD\x92\x91\x90a.}V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a$\xCDV[PPPPPV[`\xFF\x82\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\xFF\x82\x16_\x90\x81R`\x05` R`@\x90 \x80T\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a(\x89Wa(\x89a2zV[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa(\xBD\x91\x90a2\x8EV[`@Q\x80\x91\x03\x90\xA1PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a)\x93WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a*>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x07\xCAV[PPV[`\xFF\x82\x16_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x81\x16c\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x17\x90\x93U\x84Q\x92\x90\x91\x16\x80\x83R\x92\x82\x01R\x90\x91\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a+LW_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a+\0`\x01\x84a7\x12V[\x81T\x81\x10a+\x10Wa+\x10a6\xD1V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a+:Wa+1`\x01\x82a7\x12V[\x92PPPa\x13\x18V[\x80a+D\x81a9\x1BV[\x91PPa*\xCCV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x07\xCAV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91_\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x82\x81_\x81Q\x81\x10a,IWa,Ia6\xD1V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`\xFF\x85\x16_\x90\x81R`\x06\x90\x91R`@\x81 Ta,\x82\x90c\xFF\xFF\xFF\xFF\x16Ba9\x08V[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCA\x8A\xA7\xC7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\x05\x91\x90a8\xEDV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x82R`\xFF\x8A\x16` \x80\x84\x01\x82\x90R_\x91\x82R`\x04\x90\x81\x90R\x90\x84\x90 \x93Qc\x15\xD5\x96%`\xE1\x1B\x81R\x94\x90\x91\x16\x93c+\xAB,J\x93a-{\x93\x92\x89\x92\x89\x91\x01a90V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x95W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\xBC\x91\x90\x81\x01\x90a9\xBBV[\x90P\x80_\x81Q\x81\x10a-\xD0Wa-\xD0a6\xD1V[` \x02` \x01\x01Q\x93PPPP\x92\x91PPV[_a\x13\x18`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a:jV[__\x82\x12\x15a.\x1DWa.\x0C\x82a:\x89V[a.\x16\x90\x84a:\xA3V[\x90Pa\x0C/V[a.\x16\x82\x84a7\xE3V[\x805`\xFF\x81\x16\x81\x14a.7W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a.LW__\xFD[a\x13\x18\x82a.'V[__`@\x83\x85\x03\x12\x15a.fW__\xFD[a.o\x83a.'V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\x04W__\xFD[__`@\x83\x85\x03\x12\x15a.\xC4W__\xFD[a.\xCD\x83a.'V[\x91P` \x83\x015a.\xDD\x81a.\x9FV[\x80\x91PP\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a.\xF8W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x0EW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\x0CW__\xFD[_____``\x86\x88\x03\x12\x15a/<W__\xFD[a/E\x86a.'V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/_W__\xFD[a/k\x88\x82\x89\x01a.\xE8V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x89W__\xFD[a/\x95\x88\x82\x89\x01a.\xE8V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__\x83`\x1F\x84\x01\x12a/\xB6W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xCCW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E\x0CW__\xFD[____``\x85\x87\x03\x12\x15a/\xF6W__\xFD[\x845a0\x01\x81a.\x9FV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\"W__\xFD[a0.\x87\x82\x88\x01a/\xA6V[\x95\x98\x94\x97P\x95PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a0sW\x81Q`\x01`\x01``\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a0LV[P\x93\x94\x93PPPPV[`@\x81R_a0\x8F`@\x83\x01\x85a0:V[\x82\x81\x03` \x84\x01Ra#\xCE\x81\x85a0:V[__`@\x83\x85\x03\x12\x15a0\xB2W__\xFD[\x825\x91Pa0\xC2` \x84\x01a.'V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a19Wa1#\x83\x85Qc\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[` \x93\x90\x93\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a0\xE4V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1zWa1za1DV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1\xA8Wa1\xA8a1DV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a1\xC8Wa1\xC8a1DV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15a1\xE3W__\xFD[a1\xEC\x83a.'V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x06W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a2\x16W__\xFD[\x805a2)a2$\x82a1\xB0V[a1\x80V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a2JW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a2lW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a2QV[\x80\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01`\x02\x83\x10a2\xAEWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a.7W__\xFD[_\x82`\x1F\x83\x01\x12a2\xD9W__\xFD[\x815a2\xE7a2$\x82a1\xB0V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a3\x08W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x10\rW`@\x81\x88\x03\x12\x15a3$W__\xFD[a3,a1XV[\x815a37\x81a.\x9FV[\x81Ra3E` \x83\x01a2\xB4V[` \x82\x01R\x80\x84RP` \x83\x01\x92P`@\x81\x01\x90Pa3\rV[___``\x84\x86\x03\x12\x15a3qW__\xFD[a3z\x84a.'V[\x92Pa3\x88` \x85\x01a2\xB4V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xA2W__\xFD[a3\xAE\x86\x82\x87\x01a2\xCAV[\x91PP\x92P\x92P\x92V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a.7W__\xFD[___`@\x84\x86\x03\x12\x15a3\xDDW__\xFD[a3\xE6\x84a3\xB8V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\0W__\xFD[a4\x0C\x86\x82\x87\x01a/\xA6V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a19W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a42V[__`@\x83\x85\x03\x12\x15a4gW__\xFD[a4p\x83a.'V[\x91P` \x83\x015`\x02\x81\x10a.\xDDW__\xFD[___``\x84\x86\x03\x12\x15a4\x95W__\xFD[a4\x9E\x84a.'V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[``\x81\x01a\x0C/\x82\x84c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[__`@\x83\x85\x03\x12\x15a4\xFEW__\xFD[a5\x07\x83a.'V[\x91Pa0\xC2` \x84\x01a2\xB4V[___`@\x84\x86\x03\x12\x15a5'W__\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\0W__\xFD[__`@\x83\x85\x03\x12\x15a5TW__\xFD[a5]\x83a.'V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5wW__\xFD[a5\x83\x85\x82\x86\x01a2\xCAV[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a5\x9FW__\xFD[a5\xA8\x84a.'V[\x92Pa5\xB6` \x85\x01a3\xB8V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[____`\x80\x85\x87\x03\x12\x15a5\xDAW__\xFD[a5\xE3\x85a.'V[\x93Pa5\xF1` \x86\x01a2\xB4V[\x92Pa5\xFF`@\x86\x01a3\xB8V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a6\x19W__\xFD[a6%\x87\x82\x88\x01a2\xCAV[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a6CW__\xFD[\x835\x92Pa6S` \x85\x01a.'V[\x91Pa6a`@\x85\x01a3\xB8V[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a6{W__\xFD[a6\x84\x83a.'V[\x91Pa0\xC2` \x84\x01a3\xB8V[____`\x80\x85\x87\x03\x12\x15a6\xA5W__\xFD[a6\xAE\x85a.'V[\x93Pa6\xBC` \x86\x01a3\xB8V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a6\xF5W__\xFD[a\x13\x18\x82a2\xB4V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C/Wa\x0C/a6\xFEV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[` \x80\x82R`5\x90\x82\x01R\x7FStakeRegistry.initializeQuorum: `@\x82\x01Rtquorum already exists`X\x1B``\x82\x01R`\x80\x01\x90V[_` \x82\x84\x03\x12\x15a7\x9EW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x18W__\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C/Wa\x0C/a6\xFEV[_\x82a7\xDEWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C/Wa\x0C/a6\xFEV[_\x81T\x80\x84R` \x84\x01\x93P\x82_R` _ _[\x82\x81\x10\x15a0sW\x81T`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x95\x01\x94`\x01\x91\x82\x01\x91\x01a8\x17V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x14\x91\x90\x83\x01\x84a8\x02V[_\x82`\x1F\x83\x01\x12a8pW__\xFD[\x81Qa8~a2$\x82a1\xB0V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a8\x9FW__\xFD[` \x85\x01[\x83\x81\x10\x15a\x10\rW\x80Q\x83R` \x92\x83\x01\x92\x01a8\xA4V[_` \x82\x84\x03\x12\x15a8\xCCW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xE1W__\xFD[a\x14\x91\x84\x82\x85\x01a8aV[_` \x82\x84\x03\x12\x15a8\xFDW__\xFD[\x81Qa\x13\x18\x81a.\x9FV[\x80\x82\x01\x80\x82\x11\x15a\x0C/Wa\x0C/a6\xFEV[_\x81a9)Wa9)a6\xFEV[P_\x19\x01\x90V[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x81\x83\x01R`\xA0`@\x83\x01\x81\x90R\x85Q\x90\x83\x01\x81\x90R_\x91\x86\x01\x90`\xC0\x84\x01\x90\x83[\x81\x81\x10\x15a9\x91W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a9jV[PP\x83\x81\x03``\x85\x01Ra9\xA5\x81\x87a8\x02V[\x92PPPa#\xCE`\x80\x83\x01\x84c\xFF\xFF\xFF\xFF\x16\x90RV[_` \x82\x84\x03\x12\x15a9\xCBW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a9\xE0W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a9\xF0W__\xFD[\x80Qa9\xFEa2$\x82a1\xB0V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a:\x1FW__\xFD[` \x84\x01[\x83\x81\x10\x15a:_W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a:AW__\xFD[a:P\x89` \x83\x89\x01\x01a8aV[\x84RP` \x92\x83\x01\x92\x01a:$V[P\x96\x95PPPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x07?Wa\x07?a6\xFEV[_`\x01`\xFF\x1B\x82\x01a:\x9DWa:\x9Da6\xFEV[P_\x03\x90V[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C/Wa\x0C/a6\xFEV\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \x8E\x87q\x9F\x9A\x17\x98`Z\x8A8t9H\xD4\xFA\x0CC\xA4\x1Du\xE88\xD4\rj\xF0\xFC\xDF\x19\xD4UdsolcC\0\x08\x1B\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561000f575f5ffd5b5060043610610234575f3560e01c806386c0685611610135578063c601527d116100b4578063df5cf72311610079578063df5cf7231461066c578063e086adb314610693578063f2be94ae146106a6578063f851e198146106b9578063fa28c627146106cc575f5ffd5b8063c601527d1461060d578063c8294c5614610620578063cc5a7c2014610633578063d5eccc0514610646578063dd9846b914610659575f5ffd5b8063adc804da116100fa578063adc804da1461056c578063b6904b78146105ac578063bc9a40c3146105bf578063bd29b8cd146105d2578063c46778a5146105e5575f5ffd5b806386c06856146104c95780639ab4d6ff146104dc5780639f3ccf65146105165780639f8aff2614610529578063ac6bfb031461054c575f5ffd5b80635401ed27116101c15780636b3aa72e116101865780636b3aa72e1461042e5780636d14a9871461045557806375d4173a1461047c5780637c1723471461048f57806381c07502146104a9575f5ffd5b80635401ed271461039f5780635e5a6775146103b25780635f1f2d77146103c157806366acfefe146103d4578063697fbd93146103ff575f5ffd5b8063255047771161020757806325504777146102ce5780632cd95940146102ef5780633998fdd31461030f5780633ca5a5f51461034e5780634bd26e0914610370575f5ffd5b80630491b41c14610238578063087324611461026d5780631f9b74e01461028e57806320b66298146102b9575b5f5ffd5b61025a610246366004612e3c565b60ff165f9081526001602052604090205490565b6040519081526020015b60405180910390f35b61028061027b366004612e55565b6106df565b604051610264929190612e7d565b6102a161029c366004612eb3565b610724565b6040516001600160601b039091168152602001610264565b6102cc6102c7366004612f28565b610746565b005b6102e16102dc366004612fe3565b610995565b60405161026492919061307d565b6103026102fd3660046130a1565b610b98565b60405161026491906130cb565b6103367f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610264565b61025a61035c366004612e3c565b60ff165f9081526003602052604090205490565b61025a61037e3660046130a1565b5f91825260026020908152604080842060ff93909316845291905290205490565b6102a16103ad3660046130a1565b610c35565b61025a670de0b6b3a764000081565b6102cc6103cf3660046131d2565b610c4d565b6103e76103e2366004612fe3565b610f7b565b6040516001600160c01b039091168152602001610264565b61042161040d366004612e3c565b60056020525f908152604090205460ff1681565b604051610264919061328e565b6103367f000000000000000000000000000000000000000000000000000000000000000081565b6103367f000000000000000000000000000000000000000000000000000000000000000081565b6102cc61048a36600461335f565b611017565b610497602081565b60405160ff9091168152602001610264565b6104bc6104b73660046133cb565b6110fc565b6040516102649190613419565b6102cc6104d7366004613456565b61131f565b6105016104ea366004612e3c565b60066020525f908152604090205463ffffffff1681565b60405163ffffffff9091168152602001610264565b610336610524366004612e55565b611340565b61053c610537366004612e3c565b611374565b6040519015158152602001610264565b61055f61055a366004613483565b611499565b60405161026491906134b3565b61057f61057a366004612e55565b61152f565b6040805182516001600160a01b031681526020928301516001600160601b03169281019290925201610264565b61055f6105ba366004612e55565b6115a6565b6102cc6105cd3660046134ed565b611633565b6102cc6105e0366004613515565b61164f565b6102a16105f3366004612e3c565b5f602081905290815260409020546001600160601b031681565b6102cc61061b366004613543565b6116b2565b6102a161062e36600461358d565b6116ce565b6102cc6106413660046135c7565b61174a565b6102a1610654366004612e3c565b61183b565b610501610667366004613631565b61188c565b6103367f000000000000000000000000000000000000000000000000000000000000000081565b6102cc6106a136600461366a565b611898565b6102a16106b4366004613692565b6118b4565b61055f6106c73660046130a1565b611947565b6102a16106da366004613631565b611a2d565b6003602052815f5260405f2081815481106106f8575f80fd5b5f918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b5f8261072f81611a8c565b5f61073a8585611b07565b509250505b5092915050565b61074e611e13565b8461075881611a8c565b83806107d3576040805162461bcd60e51b81526020600482015260248101919091527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a206e6f20737472617465677920696e64696365732070726f766964656460648201526084015b60405180910390fd5b8281146108485760405162461bcd60e51b815260206004820152603960248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a20696e707574206c656e677468206d69736d617463680000000000000060648201526084016107ca565b60ff87165f908152600360205260408120905b8281101561098a57858582818110610875576108756136d1565b905060200201602081019061088a91906136e5565b8289898481811061089d5761089d6136d1565b90506020020135815481106108b4576108b46136d1565b905f5260205f20015f0160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a8581811061091a5761091a6136d1565b9050602002013581548110610931576109316136d1565b5f918252602090912001546001600160a01b0316888885818110610957576109576136d1565b905060200201602081019061096c91906136e5565b60405161097a929190612e7d565b60405180910390a260010161085b565b505050505050505050565b6060806109a0611f3a565b5f836001600160401b038111156109b9576109b9613144565b6040519080825280602002602001820160405280156109e2578160200160208202803683370190505b5090505f846001600160401b038111156109fe576109fe613144565b604051908082528060200260200182016040528015610a27578160200160208202803683370190505b5090505f5b85811015610b8a575f878783818110610a4757610a476136d1565b919091013560f81c9150610a5c905081611a8c565b5f5f610a68838d611b07565b9150915080610b055760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a2060448201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360648201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000608482015260a4016107ca565b5f610b118c8585611fed565b905082878681518110610b2657610b266136d1565b60200260200101906001600160601b031690816001600160601b031681525050610b508482612266565b868681518110610b6257610b626136d1565b6001600160601b0390921660209283029190910190910152505060019092019150610a2c9050565b509097909650945050505050565b5f82815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610c28575f848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610bd0565b5050505090505b92915050565b5f5f610c418484611947565b60400151949350505050565b610c55611e13565b81610c5f81611a8c565b815180610cd45760405162461bcd60e51b815260206004820152603d60248201527f5374616b6552656769737472792e72656d6f7665537472617465676965733a2060448201527f6e6f20696e646963657320746f2072656d6f76652070726f766964656400000060648201526084016107ca565b60ff84165f9081526003602090815260408083206004909252822090915b83811015610f72578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f784888481518110610d3257610d326136d1565b602002602001015181548110610d4a57610d4a6136d1565b5f91825260209182902001546040516001600160a01b0390911681520160405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7584888481518110610da757610da76136d1565b602002602001015181548110610dbf57610dbf6136d1565b5f91825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a282548390610dfe90600190613712565b81548110610e0e57610e0e6136d1565b905f5260205f200183878381518110610e2957610e296136d1565b602002602001015181548110610e4157610e416136d1565b5f91825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b9182900416021790558254839080610e9357610e93613725565b5f8281526020812082015f199081019190915501905581548290610eb990600190613712565b81548110610ec957610ec96136d1565b905f5260205f20015f9054906101000a90046001600160a01b031682878381518110610ef757610ef76136d1565b602002602001015181548110610f0f57610f0f6136d1565b905f5260205f20015f6101000a8154816001600160a01b0302191690836001600160a01b0316021790555081805480610f4a57610f4a613725565b5f8281526020902081015f1990810180546001600160a01b0319169055019055600101610cf2565b50505050505050565b5f610f84611f3a565b5f805b8381101561100d575f858583818110610fa257610fa26136d1565b919091013560f81c9150610fb7905081611a8c565b5f5f610fc3838b611b07565b9150915080610fe4575f9150600160ff84161b6001600160c01b0386161794505b5f610ff08a8585611fed565b9050610ffc8482612266565b505060019093019250610f87915050565b5095945050505050565b61101f611f3a565b60ff83165f908152600160205260409020541561104e5760405162461bcd60e51b81526004016107ca90613739565b61105883826123d7565b61106283836127f4565b61106c835f61285c565b505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60605f826001600160401b0381111561111757611117613144565b604051908082528060200260200182016040528015611140578160200160208202803683370190505b5090505f5b83811015611314575f858583818110611160576111606136d1565b919091013560f81c9150611175905081611a8c565b60ff81165f908152600160205260408120805463ffffffff8a16929061119d5761119d6136d1565b5f9182526020909120015463ffffffff1611156112485760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360648201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000608482015260a4016107ca565b60ff81165f90815260016020526040812054905b818110156113095760ff83165f90815260016020819052604090912063ffffffff8b169161128a8486613712565b6112949190613712565b815481106112a4576112a46136d1565b5f9182526020909120015463ffffffff16116113015760016112c68284613712565b6112d09190613712565b8585815181106112e2576112e26136d1565b602002602001019063ffffffff16908163ffffffff1681525050611309565b60010161125c565b505050600101611145565b5090505b9392505050565b611327611e13565b8161133181611a8c565b61133b838361285c565b505050565b6004602052815f5260405f208181548110611359575f80fd5b5f918252602090912001546001600160a01b03169150829050565b60405163a4d7871f60e01b815260ff821660048201525f9081906001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169063a4d7871f90602401602060405180830381865afa1580156113dd573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611401919061378e565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663cabbb17f6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611460573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611484919061378e565b9050808015611491575081155b949350505050565b604080516060810182525f80825260208083018290528284018290528582526002815283822060ff881683529052919091208054839081106114dd576114dd6136d1565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091525f808252602082015260ff83165f908152600360205260409020805483908110611565576115656136d1565b5f918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182525f808252602080830182905282840182905260ff8616825260019052919091208054839081106115e2576115e26136d1565b5f91825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b61163b611e13565b8161164581611a8c565b61133b83836127f4565b611657611f3a565b5f5b818110156116ac575f838383818110611674576116746136d1565b919091013560f81c9150611689905081611a8c565b5f61169586835f611fed565b90506116a18282612266565b505050600101611659565b50505050565b6116ba611e13565b816116c481611a8c565b61133b83836123d7565b60ff83165f9081526001602052604081208054829190849081106116f4576116f46136d1565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610c4181856128c9565b611752611f3a565b60ff84165f90815260016020526040902054156117815760405162461bcd60e51b81526004016107ca90613739565b61178b84826123d7565b61179584846127f4565b6117a084600161285c565b6117aa8483612a42565b50505060ff165f908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60ff81165f90815260016020819052604082208054909161185b91613712565b8154811061186b5761186b6136d1565b5f91825260209091200154600160401b90046001600160601b031692915050565b5f611491848484612aae565b6118a0611e13565b816118aa81611a8c565b61133b8383612a42565b5f82815260026020908152604080832060ff8816845290915281208054829190849081106118e4576118e46136d1565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b9093049290921690820152905061193a81866128c9565b6040015195945050505050565b60408051606080820183525f80835260208084018290528385018290528682526002815284822060ff87168352815284822054855193840186528284529083018290529382018190529192918290036119a3579150610c2f9050565b5f85815260026020908152604080832060ff8816845290915290206119c9600184613712565b815481106119d9576119d96136d1565b5f91825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610c2f915050565b5f83815260026020908152604080832060ff861684529091528120611a53858585612aae565b63ffffffff1681548110611a6957611a696136d1565b5f91825260209091200154600160401b90046001600160601b0316949350505050565b60ff81165f90815260016020526040902054611b045760405162461bcd60e51b815260206004820152603160248201527f5374616b6552656769737472792e71756f72756d4578697374733a2071756f726044820152701d5b48191bd95cc81b9bdd08195e1a5cdd607a1b60648201526084016107ca565b50565b5f5f5f5f611b238660ff165f9081526003602052604090205490565b604080518082019091525f80825260208201529091506060600160ff808a165f90815260056020526040902054166001811115611b6257611b6261327a565b03611c5b57611b718888612c11565b90505f5b83811015611c555760ff89165f908152600360205260409020805482908110611ba057611ba06136d1565b5f9182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611bed57611bed6136d1565b60200260200101511115611c4d57670de0b6b3a764000083602001516001600160601b0316838381518110611c2457611c246136d1565b6020026020010151611c3691906137ad565b611c4091906137c4565b611c4a90866137e3565b94505b600101611b75565b50611de1565b60ff88165f908152600460208190526040918290209151639004134760e01b81526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001692639004134792611cba928c92910161383e565b5f60405180830381865afa158015611cd4573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052611cfb91908101906138bc565b90505f5b83811015611ddf5760ff89165f908152600360205260409020805482908110611d2a57611d2a6136d1565b5f9182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b0316908201528351909450839083908110611d7757611d776136d1565b60200260200101511115611dd757670de0b6b3a764000083602001516001600160601b0316838381518110611dae57611dae6136d1565b6020026020010151611dc091906137ad565b611dca91906137c4565b611dd490866137e3565b94505b600101611cff565b505b50505060ff85165f908152602081905260409020549092506001600160601b03908116908316101590505b9250929050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611e6f573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190611e9391906138ed565b6001600160a01b0316336001600160a01b031614611f385760405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e60448201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f6620746064820152753432903932b3b4b9ba393ca1b7b7b93234b730ba37b960511b608482015260a4016107ca565b565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611f385760405162461bcd60e51b815260206004820152604c60248201527f5374616b6552656769737472792e6f6e6c795265676973747279436f6f72646960448201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260648201526b3ca1b7b7b93234b730ba37b960a11b608482015260a4016107ca565b5f83815260026020908152604080832060ff8616845290915281205481908082036120b1575f86815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff199096169190921617939093171691909117905561220c565b5f86815260026020908152604080832060ff8916845290915281206120d7600184613712565b815481106120e7576120e76136d1565b5f91825260209091200180546001600160601b03600160401b909104811694509091508516830361211d575f9350505050611318565b805463ffffffff438116911603612155578054600160401b600160a01b031916600160401b6001600160601b0387160217815561220a565b805467ffffffff000000001916600160201b4363ffffffff9081168281029390931784555f8a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a261225c8285612de3565b9695505050505050565b60ff82165f90815260016020819052604082208054918391906122899084613712565b81548110612299576122996136d1565b905f5260205f20019050835f036122c45754600160401b90046001600160601b03169150610c2f9050565b80545f906122e290600160401b90046001600160601b031686612dfa565b825490915063ffffffff43811691160361231d578154600160401b600160a01b031916600160401b6001600160601b038316021782556123ce565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff89165f90815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b5f81511161243a5760405162461bcd60e51b815260206004820152603860248201525f516020613ac35f395f51905f5260448201527f3a206e6f20737472617465676965732070726f7669646564000000000000000060648201526084016107ca565b805160ff83165f908152600360209081526040909120549061245c8383613908565b11156124cb5760405162461bcd60e51b815260206004820152604560248201525f516020613ac35f395f51905f5260448201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60648201526408a9c8ea8960db1b608482015260a4016107ca565b5f5b828110156127ed575f5b6124e18284613908565b8110156125b2578482815181106124fa576124fa6136d1565b60200260200101515f01516001600160a01b031660035f8860ff1660ff1681526020019081526020015f208281548110612536576125366136d1565b5f918252602090912001546001600160a01b0316036125aa5760405162461bcd60e51b815260206004820152603d60248201525f516020613ac35f395f51905f5260448201527f3a2063616e6e6f74206164642073616d6520737472617465677920327800000060648201526084016107ca565b6001016124d7565b505f8482815181106125c6576125c66136d1565b6020026020010151602001516001600160601b03161161264a5760405162461bcd60e51b815260206004820152604660248201525f516020613ac35f395f51905f5260448201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f206064820152651dd95a59da1d60d21b608482015260a4016107ca565b60ff85165f908152600360205260409020845185908390811061266f5761266f6136d1565b60209081029190910181015182546001810184555f9384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff87168252600490526040902084518590839081106126d3576126d36136d1565b6020908102919091018101515182546001810184555f938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f540490869084908110612749576127496136d1565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a758583815181106127a6576127a66136d1565b60200260200101515f01518684815181106127c3576127c36136d1565b6020026020010151602001516040516127dd929190612e7d565b60405180910390a26001016124cd565b5050505050565b60ff82165f818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b60ff82165f908152600560205260409020805482919060ff1916600183818111156128895761288961327a565b02179055507f7c112e863ccf007862e2c9e25819c933fedbc9350a6443423b4a8599c2e8a52d816040516128bd919061328e565b60405180910390a15050565b815f015163ffffffff168163ffffffff16101561296d5760405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a207374616b6555706461746520697320606482015275333937b69030b33a32b910313637b1b5a73ab6b132b960511b608482015260a4016107ca565b602082015163ffffffff1615806129935750816020015163ffffffff168163ffffffff16105b612a3e5760405162461bcd60e51b815260206004820152606a60248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560648201527f72207374616b6555706461746520617661696c61626c65206265666f726520626084820152693637b1b5a73ab6b132b960b11b60a482015260c4016107ca565b5050565b60ff82165f90815260066020908152604091829020805463ffffffff19811663ffffffff8681169182179093558451929091168083529282015290917f28d7358b79f02d21b8b7e17aefc4185a64308aa37406fa5befc05b91932c39c7910160405180910390a1505050565b5f83815260026020908152604080832060ff86168452909152812054805b8015612b4c575f86815260026020908152604080832060ff89168452909152902063ffffffff851690612b00600184613712565b81548110612b1057612b106136d1565b5f9182526020909120015463ffffffff1611612b3a57612b31600182613712565b92505050611318565b80612b448161391b565b915050612acc565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e4016107ca565b6040805160018082528183019092526060915f91906020808301908036833701905050905082815f81518110612c4957612c496136d1565b6001600160a01b0390921660209283029190910182015260ff85165f90815260069091526040812054612c829063ffffffff1642613908565b90505f7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b031663ca8aa7c76040518163ffffffff1660e01b8152600401602060405180830381865afa158015612ce1573d5f5f3e3d5ffd5b505050506040513d601f19601f82011682018060405250810190612d0591906138ed565b6040805180820182526001600160a01b037f00000000000000000000000000000000000000000000000000000000000000008116825260ff8a1660208084018290525f9182526004908190529084902093516315d5962560e11b81529490911693632bab2c4a93612d7b93928992899101613930565b5f60405180830381865afa158015612d95573d5f5f3e3d5ffd5b505050506040513d5f823e601f3d908101601f19168201604052612dbc91908101906139bb565b9050805f81518110612dd057612dd06136d1565b6020026020010151935050505092915050565b5f6113186001600160601b03808516908416613a6a565b5f5f821215612e1d57612e0c82613a89565b612e169084613aa3565b9050610c2f565b612e1682846137e3565b803560ff81168114612e37575f5ffd5b919050565b5f60208284031215612e4c575f5ffd5b61131882612e27565b5f5f60408385031215612e66575f5ffd5b612e6f83612e27565b946020939093013593505050565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114611b04575f5ffd5b5f5f60408385031215612ec4575f5ffd5b612ecd83612e27565b91506020830135612edd81612e9f565b809150509250929050565b5f5f83601f840112612ef8575f5ffd5b5081356001600160401b03811115612f0e575f5ffd5b6020830191508360208260051b8501011115611e0c575f5ffd5b5f5f5f5f5f60608688031215612f3c575f5ffd5b612f4586612e27565b945060208601356001600160401b03811115612f5f575f5ffd5b612f6b88828901612ee8565b90955093505060408601356001600160401b03811115612f89575f5ffd5b612f9588828901612ee8565b969995985093965092949392505050565b5f5f83601f840112612fb6575f5ffd5b5081356001600160401b03811115612fcc575f5ffd5b602083019150836020828501011115611e0c575f5ffd5b5f5f5f5f60608587031215612ff6575f5ffd5b843561300181612e9f565b93506020850135925060408501356001600160401b03811115613022575f5ffd5b61302e87828801612fa6565b95989497509550505050565b5f8151808452602084019350602083015f5b828110156130735781516001600160601b031686526020958601959091019060010161304c565b5093949350505050565b604081525f61308f604083018561303a565b82810360208401526123ce818561303a565b5f5f604083850312156130b2575f5ffd5b823591506130c260208401612e27565b90509250929050565b602080825282518282018190525f918401906040840190835b818110156131395761312383855163ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b60209390930192606092909201916001016130e4565b509095945050505050565b634e487b7160e01b5f52604160045260245ffd5b604080519081016001600160401b038111828210171561317a5761317a613144565b60405290565b604051601f8201601f191681016001600160401b03811182821017156131a8576131a8613144565b604052919050565b5f6001600160401b038211156131c8576131c8613144565b5060051b60200190565b5f5f604083850312156131e3575f5ffd5b6131ec83612e27565b915060208301356001600160401b03811115613206575f5ffd5b8301601f81018513613216575f5ffd5b8035613229613224826131b0565b613180565b8082825260208201915060208360051b85010192508783111561324a575f5ffd5b6020840193505b8284101561326c578335825260209384019390910190613251565b809450505050509250929050565b634e487b7160e01b5f52602160045260245ffd5b60208101600283106132ae57634e487b7160e01b5f52602160045260245ffd5b91905290565b80356001600160601b0381168114612e37575f5ffd5b5f82601f8301126132d9575f5ffd5b81356132e7613224826131b0565b8082825260208201915060208360061b860101925085831115613308575f5ffd5b602085015b8381101561100d5760408188031215613324575f5ffd5b61332c613158565b813561333781612e9f565b8152613345602083016132b4565b60208201528084525060208301925060408101905061330d565b5f5f5f60608486031215613371575f5ffd5b61337a84612e27565b9250613388602085016132b4565b915060408401356001600160401b038111156133a2575f5ffd5b6133ae868287016132ca565b9150509250925092565b803563ffffffff81168114612e37575f5ffd5b5f5f5f604084860312156133dd575f5ffd5b6133e6846133b8565b925060208401356001600160401b03811115613400575f5ffd5b61340c86828701612fa6565b9497909650939450505050565b602080825282518282018190525f918401906040840190835b8181101561313957835163ffffffff16835260209384019390920191600101613432565b5f5f60408385031215613467575f5ffd5b61347083612e27565b9150602083013560028110612edd575f5ffd5b5f5f5f60608486031215613495575f5ffd5b61349e84612e27565b95602085013595506040909401359392505050565b60608101610c2f828463ffffffff815116825263ffffffff60208201511660208301526001600160601b0360408201511660408301525050565b5f5f604083850312156134fe575f5ffd5b61350783612e27565b91506130c2602084016132b4565b5f5f5f60408486031215613527575f5ffd5b8335925060208401356001600160401b03811115613400575f5ffd5b5f5f60408385031215613554575f5ffd5b61355d83612e27565b915060208301356001600160401b03811115613577575f5ffd5b613583858286016132ca565b9150509250929050565b5f5f5f6060848603121561359f575f5ffd5b6135a884612e27565b92506135b6602085016133b8565b929592945050506040919091013590565b5f5f5f5f608085870312156135da575f5ffd5b6135e385612e27565b93506135f1602086016132b4565b92506135ff604086016133b8565b915060608501356001600160401b03811115613619575f5ffd5b613625878288016132ca565b91505092959194509250565b5f5f5f60608486031215613643575f5ffd5b8335925061365360208501612e27565b9150613661604085016133b8565b90509250925092565b5f5f6040838503121561367b575f5ffd5b61368483612e27565b91506130c2602084016133b8565b5f5f5f5f608085870312156136a5575f5ffd5b6136ae85612e27565b93506136bc602086016133b8565b93969395505050506040820135916060013590565b634e487b7160e01b5f52603260045260245ffd5b5f602082840312156136f5575f5ffd5b611318826132b4565b634e487b7160e01b5f52601160045260245ffd5b81810381811115610c2f57610c2f6136fe565b634e487b7160e01b5f52603160045260245ffd5b60208082526035908201527f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a2060408201527471756f72756d20616c72656164792065786973747360581b606082015260800190565b5f6020828403121561379e575f5ffd5b81518015158114611318575f5ffd5b8082028115828204841417610c2f57610c2f6136fe565b5f826137de57634e487b7160e01b5f52601260045260245ffd5b500490565b6001600160601b038181168382160190811115610c2f57610c2f6136fe565b5f8154808452602084019350825f5260205f205f5b828110156130735781546001600160a01b0316865260209095019460019182019101613817565b6001600160a01b03831681526040602082018190525f9061149190830184613802565b5f82601f830112613870575f5ffd5b815161387e613224826131b0565b8082825260208201915060208360051b86010192508583111561389f575f5ffd5b602085015b8381101561100d5780518352602092830192016138a4565b5f602082840312156138cc575f5ffd5b81516001600160401b038111156138e1575f5ffd5b61149184828501613861565b5f602082840312156138fd575f5ffd5b815161131881612e9f565b80820180821115610c2f57610c2f6136fe565b5f81613929576139296136fe565b505f190190565b84516001600160a01b0316815260208086015163ffffffff168183015260a06040830181905285519083018190525f9186019060c0840190835b818110156139915783516001600160a01b031683526020938401939092019160010161396a565b505083810360608501526139a58187613802565b925050506123ce608083018463ffffffff169052565b5f602082840312156139cb575f5ffd5b81516001600160401b038111156139e0575f5ffd5b8201601f810184136139f0575f5ffd5b80516139fe613224826131b0565b8082825260208201915060208360051b850101925086831115613a1f575f5ffd5b602084015b83811015613a5f5780516001600160401b03811115613a41575f5ffd5b613a5089602083890101613861565b84525060209283019201613a24565b509695505050505050565b8181035f83128015838313168383128216171561073f5761073f6136fe565b5f600160ff1b8201613a9d57613a9d6136fe565b505f0390565b6001600160601b038281168282160390811115610c2f57610c2f6136fe56fe5374616b6552656769737472792e5f6164645374726174656779506172616d73a26469706673582212208e87719f9a1798605a8a38743948d4fa0c43a41d75e838d40d6af0fcdf19d45564736f6c634300081b0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x0FW__\xFD[P`\x046\x10a\x024W_5`\xE0\x1C\x80c\x86\xC0hV\x11a\x015W\x80c\xC6\x01R}\x11a\0\xB4W\x80c\xDF\\\xF7#\x11a\0yW\x80c\xDF\\\xF7#\x14a\x06lW\x80c\xE0\x86\xAD\xB3\x14a\x06\x93W\x80c\xF2\xBE\x94\xAE\x14a\x06\xA6W\x80c\xF8Q\xE1\x98\x14a\x06\xB9W\x80c\xFA(\xC6'\x14a\x06\xCCW__\xFD[\x80c\xC6\x01R}\x14a\x06\rW\x80c\xC8)LV\x14a\x06 W\x80c\xCCZ| \x14a\x063W\x80c\xD5\xEC\xCC\x05\x14a\x06FW\x80c\xDD\x98F\xB9\x14a\x06YW__\xFD[\x80c\xAD\xC8\x04\xDA\x11a\0\xFAW\x80c\xAD\xC8\x04\xDA\x14a\x05lW\x80c\xB6\x90Kx\x14a\x05\xACW\x80c\xBC\x9A@\xC3\x14a\x05\xBFW\x80c\xBD)\xB8\xCD\x14a\x05\xD2W\x80c\xC4gx\xA5\x14a\x05\xE5W__\xFD[\x80c\x86\xC0hV\x14a\x04\xC9W\x80c\x9A\xB4\xD6\xFF\x14a\x04\xDCW\x80c\x9F<\xCFe\x14a\x05\x16W\x80c\x9F\x8A\xFF&\x14a\x05)W\x80c\xACk\xFB\x03\x14a\x05LW__\xFD[\x80cT\x01\xED'\x11a\x01\xC1W\x80ck:\xA7.\x11a\x01\x86W\x80ck:\xA7.\x14a\x04.W\x80cm\x14\xA9\x87\x14a\x04UW\x80cu\xD4\x17:\x14a\x04|W\x80c|\x17#G\x14a\x04\x8FW\x80c\x81\xC0u\x02\x14a\x04\xA9W__\xFD[\x80cT\x01\xED'\x14a\x03\x9FW\x80c^Zgu\x14a\x03\xB2W\x80c_\x1F-w\x14a\x03\xC1W\x80cf\xAC\xFE\xFE\x14a\x03\xD4W\x80ci\x7F\xBD\x93\x14a\x03\xFFW__\xFD[\x80c%PGw\x11a\x02\x07W\x80c%PGw\x14a\x02\xCEW\x80c,\xD9Y@\x14a\x02\xEFW\x80c9\x98\xFD\xD3\x14a\x03\x0FW\x80c<\xA5\xA5\xF5\x14a\x03NW\x80cK\xD2n\t\x14a\x03pW__\xFD[\x80c\x04\x91\xB4\x1C\x14a\x028W\x80c\x08s$a\x14a\x02mW\x80c\x1F\x9Bt\xE0\x14a\x02\x8EW\x80c \xB6b\x98\x14a\x02\xB9W[__\xFD[a\x02Za\x02F6`\x04a.<V[`\xFF\x16_\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02\x80a\x02{6`\x04a.UV[a\x06\xDFV[`@Qa\x02d\x92\x91\x90a.}V[a\x02\xA1a\x02\x9C6`\x04a.\xB3V[a\x07$V[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02dV[a\x02\xCCa\x02\xC76`\x04a/(V[a\x07FV[\0[a\x02\xE1a\x02\xDC6`\x04a/\xE3V[a\t\x95V[`@Qa\x02d\x92\x91\x90a0}V[a\x03\x02a\x02\xFD6`\x04a0\xA1V[a\x0B\x98V[`@Qa\x02d\x91\x90a0\xCBV[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02dV[a\x02Za\x03\\6`\x04a.<V[`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Za\x03~6`\x04a0\xA1V[_\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\xA1a\x03\xAD6`\x04a0\xA1V[a\x0C5V[a\x02Zg\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02\xCCa\x03\xCF6`\x04a1\xD2V[a\x0CMV[a\x03\xE7a\x03\xE26`\x04a/\xE3V[a\x0F{V[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02dV[a\x04!a\x04\r6`\x04a.<V[`\x05` R_\x90\x81R`@\x90 T`\xFF\x16\x81V[`@Qa\x02d\x91\x90a2\x8EV[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xCCa\x04\x8A6`\x04a3_V[a\x10\x17V[a\x04\x97` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02dV[a\x04\xBCa\x04\xB76`\x04a3\xCBV[a\x10\xFCV[`@Qa\x02d\x91\x90a4\x19V[a\x02\xCCa\x04\xD76`\x04a4VV[a\x13\x1FV[a\x05\x01a\x04\xEA6`\x04a.<V[`\x06` R_\x90\x81R`@\x90 Tc\xFF\xFF\xFF\xFF\x16\x81V[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02dV[a\x036a\x05$6`\x04a.UV[a\x13@V[a\x05<a\x0576`\x04a.<V[a\x13tV[`@Q\x90\x15\x15\x81R` \x01a\x02dV[a\x05_a\x05Z6`\x04a4\x83V[a\x14\x99V[`@Qa\x02d\x91\x90a4\xB3V[a\x05\x7Fa\x05z6`\x04a.UV[a\x15/V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02dV[a\x05_a\x05\xBA6`\x04a.UV[a\x15\xA6V[a\x02\xCCa\x05\xCD6`\x04a4\xEDV[a\x163V[a\x02\xCCa\x05\xE06`\x04a5\x15V[a\x16OV[a\x02\xA1a\x05\xF36`\x04a.<V[_` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02\xCCa\x06\x1B6`\x04a5CV[a\x16\xB2V[a\x02\xA1a\x06.6`\x04a5\x8DV[a\x16\xCEV[a\x02\xCCa\x06A6`\x04a5\xC7V[a\x17JV[a\x02\xA1a\x06T6`\x04a.<V[a\x18;V[a\x05\x01a\x06g6`\x04a61V[a\x18\x8CV[a\x036\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\xCCa\x06\xA16`\x04a6jV[a\x18\x98V[a\x02\xA1a\x06\xB46`\x04a6\x92V[a\x18\xB4V[a\x05_a\x06\xC76`\x04a0\xA1V[a\x19GV[a\x02\xA1a\x06\xDA6`\x04a61V[a\x1A-V[`\x03` R\x81_R`@_ \x81\x81T\x81\x10a\x06\xF8W_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[_\x82a\x07/\x81a\x1A\x8CV[_a\x07:\x85\x85a\x1B\x07V[P\x92PP[P\x92\x91PPV[a\x07Na\x1E\x13V[\x84a\x07X\x81a\x1A\x8CV[\x83\x80a\x07\xD3W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01[`@Q\x80\x91\x03\x90\xFD[\x82\x81\x14a\x08HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xCAV[`\xFF\x87\x16_\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\t\x8AW\x85\x85\x82\x81\x81\x10a\x08uWa\x08ua6\xD1V[\x90P` \x02\x01` \x81\x01\x90a\x08\x8A\x91\x90a6\xE5V[\x82\x89\x89\x84\x81\x81\x10a\x08\x9DWa\x08\x9Da6\xD1V[\x90P` \x02\x015\x81T\x81\x10a\x08\xB4Wa\x08\xB4a6\xD1V[\x90_R` _ \x01_\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\t\x1AWa\t\x1Aa6\xD1V[\x90P` \x02\x015\x81T\x81\x10a\t1Wa\t1a6\xD1V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\tWWa\tWa6\xD1V[\x90P` \x02\x01` \x81\x01\x90a\tl\x91\x90a6\xE5V[`@Qa\tz\x92\x91\x90a.}V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a\x08[V[PPPPPPPPPV[``\x80a\t\xA0a\x1F:V[_\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xB9Wa\t\xB9a1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\t\xE2W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\t\xFEWa\t\xFEa1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n'W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x85\x81\x10\x15a\x0B\x8AW_\x87\x87\x83\x81\x81\x10a\nGWa\nGa6\xD1V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\n\\\x90P\x81a\x1A\x8CV[__a\nh\x83\x8Da\x1B\x07V[\x91P\x91P\x80a\x0B\x05W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\xCAV[_a\x0B\x11\x8C\x85\x85a\x1F\xEDV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0B&Wa\x0B&a6\xD1V[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0BP\x84\x82a\"fV[\x86\x86\x81Q\x81\x10a\x0BbWa\x0Bba6\xD1V[`\x01`\x01``\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x90\x91\x01RPP`\x01\x90\x92\x01\x91Pa\n,\x90PV[P\x90\x97\x90\x96P\x94PPPPPV[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\x0C(W_\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0B\xD0V[PPPP\x90P[\x92\x91PPV[__a\x0CA\x84\x84a\x19GV[`@\x01Q\x94\x93PPPPV[a\x0CUa\x1E\x13V[\x81a\x0C_\x81a\x1A\x8CV[\x81Q\x80a\x0C\xD4W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x07\xCAV[`\xFF\x84\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91[\x83\x81\x10\x15a\x0FrW\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\r2Wa\r2a6\xD1V[` \x02` \x01\x01Q\x81T\x81\x10a\rJWa\rJa6\xD1V[_\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\r\xA7Wa\r\xA7a6\xD1V[` \x02` \x01\x01Q\x81T\x81\x10a\r\xBFWa\r\xBFa6\xD1V[_\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x82T\x83\x90a\r\xFE\x90`\x01\x90a7\x12V[\x81T\x81\x10a\x0E\x0EWa\x0E\x0Ea6\xD1V[\x90_R` _ \x01\x83\x87\x83\x81Q\x81\x10a\x0E)Wa\x0E)a6\xD1V[` \x02` \x01\x01Q\x81T\x81\x10a\x0EAWa\x0EAa6\xD1V[_\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x82T\x83\x90\x80a\x0E\x93Wa\x0E\x93a7%V[_\x82\x81R` \x81 \x82\x01_\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x81T\x82\x90a\x0E\xB9\x90`\x01\x90a7\x12V[\x81T\x81\x10a\x0E\xC9Wa\x0E\xC9a6\xD1V[\x90_R` _ \x01_\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x83\x81Q\x81\x10a\x0E\xF7Wa\x0E\xF7a6\xD1V[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\x0FWa\x0F\x0Fa6\xD1V[\x90_R` _ \x01_a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x80T\x80a\x0FJWa\x0FJa7%V[_\x82\x81R` \x90 \x81\x01_\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U`\x01\x01a\x0C\xF2V[PPPPPPPV[_a\x0F\x84a\x1F:V[_\x80[\x83\x81\x10\x15a\x10\rW_\x85\x85\x83\x81\x81\x10a\x0F\xA2Wa\x0F\xA2a6\xD1V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x0F\xB7\x90P\x81a\x1A\x8CV[__a\x0F\xC3\x83\x8Ba\x1B\x07V[\x91P\x91P\x80a\x0F\xE4W_\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[_a\x0F\xF0\x8A\x85\x85a\x1F\xEDV[\x90Pa\x0F\xFC\x84\x82a\"fV[PP`\x01\x90\x93\x01\x92Pa\x0F\x87\x91PPV[P\x95\x94PPPPPV[a\x10\x1Fa\x1F:V[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x10NW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xCA\x90a79V[a\x10X\x83\x82a#\xD7V[a\x10b\x83\x83a'\xF4V[a\x10l\x83_a(\\V[PP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[``_\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x11\x17Wa\x11\x17a1DV[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x11@W\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P_[\x83\x81\x10\x15a\x13\x14W_\x85\x85\x83\x81\x81\x10a\x11`Wa\x11`a6\xD1V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x11u\x90P\x81a\x1A\x8CV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x11\x9DWa\x11\x9Da6\xD1V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x12HW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x07\xCAV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x13\tW`\xFF\x83\x16_\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x12\x8A\x84\x86a7\x12V[a\x12\x94\x91\x90a7\x12V[\x81T\x81\x10a\x12\xA4Wa\x12\xA4a6\xD1V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x13\x01W`\x01a\x12\xC6\x82\x84a7\x12V[a\x12\xD0\x91\x90a7\x12V[\x85\x85\x81Q\x81\x10a\x12\xE2Wa\x12\xE2a6\xD1V[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x13\tV[`\x01\x01a\x12\\V[PPP`\x01\x01a\x11EV[P\x90P[\x93\x92PPPV[a\x13'a\x1E\x13V[\x81a\x131\x81a\x1A\x8CV[a\x13;\x83\x83a(\\V[PPPV[`\x04` R\x81_R`@_ \x81\x81T\x81\x10a\x13YW_\x80\xFD[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@Qc\xA4\xD7\x87\x1F`\xE0\x1B\x81R`\xFF\x82\x16`\x04\x82\x01R_\x90\x81\x90`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x90c\xA4\xD7\x87\x1F\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x13\xDDW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x01\x91\x90a7\x8EV[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCA\xBB\xB1\x7F`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x14`W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x14\x84\x91\x90a7\x8EV[\x90P\x80\x80\x15a\x14\x91WP\x81\x15[\x94\x93PPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x14\xDDWa\x14\xDDa6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R`\xFF\x83\x16_\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x15eWa\x15ea6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x15\xE2Wa\x15\xE2a6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[a\x16;a\x1E\x13V[\x81a\x16E\x81a\x1A\x8CV[a\x13;\x83\x83a'\xF4V[a\x16Wa\x1F:V[_[\x81\x81\x10\x15a\x16\xACW_\x83\x83\x83\x81\x81\x10a\x16tWa\x16ta6\xD1V[\x91\x90\x91\x015`\xF8\x1C\x91Pa\x16\x89\x90P\x81a\x1A\x8CV[_a\x16\x95\x86\x83_a\x1F\xEDV[\x90Pa\x16\xA1\x82\x82a\"fV[PPP`\x01\x01a\x16YV[PPPPV[a\x16\xBAa\x1E\x13V[\x81a\x16\xC4\x81a\x1A\x8CV[a\x13;\x83\x83a#\xD7V[`\xFF\x83\x16_\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x16\xF4Wa\x16\xF4a6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x0CA\x81\x85a(\xC9V[a\x17Ra\x1F:V[`\xFF\x84\x16_\x90\x81R`\x01` R`@\x90 T\x15a\x17\x81W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x07\xCA\x90a79V[a\x17\x8B\x84\x82a#\xD7V[a\x17\x95\x84\x84a'\xF4V[a\x17\xA0\x84`\x01a(\\V[a\x17\xAA\x84\x83a*BV[PPP`\xFF\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x81\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x18[\x91a7\x12V[\x81T\x81\x10a\x18kWa\x18ka6\xD1V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[_a\x14\x91\x84\x84\x84a*\xAEV[a\x18\xA0a\x1E\x13V[\x81a\x18\xAA\x81a\x1A\x8CV[a\x13;\x83\x83a*BV[_\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x18\xE4Wa\x18\xE4a6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x19:\x81\x86a(\xC9V[`@\x01Q\x95\x94PPPPPV[`@\x80Q``\x80\x82\x01\x83R_\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01\x81\x90R\x91\x92\x91\x82\x90\x03a\x19\xA3W\x91Pa\x0C/\x90PV[_\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x19\xC9`\x01\x84a7\x12V[\x81T\x81\x10a\x19\xD9Wa\x19\xD9a6\xD1V[_\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\x0C/\x91PPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1AS\x85\x85\x85a*\xAEV[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1AiWa\x1Aia6\xD1V[_\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[`\xFF\x81\x16_\x90\x81R`\x01` R`@\x90 Ta\x1B\x04W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`1`$\x82\x01R\x7FStakeRegistry.quorumExists: quor`D\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B`d\x82\x01R`\x84\x01a\x07\xCAV[PV[____a\x1B#\x86`\xFF\x16_\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R_\x80\x82R` \x82\x01R\x90\x91P```\x01`\xFF\x80\x8A\x16_\x90\x81R`\x05` R`@\x90 T\x16`\x01\x81\x11\x15a\x1BbWa\x1Bba2zV[\x03a\x1C[Wa\x1Bq\x88\x88a,\x11V[\x90P_[\x83\x81\x10\x15a\x1CUW`\xFF\x89\x16_\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1B\xA0Wa\x1B\xA0a6\xD1V[_\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1B\xEDWa\x1B\xEDa6\xD1V[` \x02` \x01\x01Q\x11\x15a\x1CMWg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1C$Wa\x1C$a6\xD1V[` \x02` \x01\x01Qa\x1C6\x91\x90a7\xADV[a\x1C@\x91\x90a7\xC4V[a\x1CJ\x90\x86a7\xE3V[\x94P[`\x01\x01a\x1BuV[Pa\x1D\xE1V[`\xFF\x88\x16_\x90\x81R`\x04` \x81\x90R`@\x91\x82\x90 \x91Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a\x1C\xBA\x92\x8C\x92\x91\x01a8>V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1C\xD4W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra\x1C\xFB\x91\x90\x81\x01\x90a8\xBCV[\x90P_[\x83\x81\x10\x15a\x1D\xDFW`\xFF\x89\x16_\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a\x1D*Wa\x1D*a6\xD1V[_\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a\x1DwWa\x1Dwa6\xD1V[` \x02` \x01\x01Q\x11\x15a\x1D\xD7Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a\x1D\xAEWa\x1D\xAEa6\xD1V[` \x02` \x01\x01Qa\x1D\xC0\x91\x90a7\xADV[a\x1D\xCA\x91\x90a7\xC4V[a\x1D\xD4\x90\x86a7\xE3V[\x94P[`\x01\x01a\x1C\xFFV[P[PPP`\xFF\x85\x16_\x90\x81R` \x81\x90R`@\x90 T\x90\x92P`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1EoW=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1E\x93\x91\x90a8\xEDV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1F8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`D\x82\x01R\x7Fer: caller is not the owner of t`d\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1F8W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`L`$\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`D\x82\x01R\x7Fnator: caller is not the Registr`d\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80\x82\x03a \xB1W_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua\"\x0CV[_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a \xD7`\x01\x84a7\x12V[\x81T\x81\x10a \xE7Wa \xE7a6\xD1V[_\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x03a!\x1DW_\x93PPPPa\x13\x18V[\x80Tc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a!UW\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua\"\nV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U_\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a\"\\\x82\x85a-\xE3V[\x96\x95PPPPPPV[`\xFF\x82\x16_\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\"\x89\x90\x84a7\x12V[\x81T\x81\x10a\"\x99Wa\"\x99a6\xD1V[\x90_R` _ \x01\x90P\x83_\x03a\"\xC4WT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\x0C/\x90PV[\x80T_\x90a\"\xE2\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a-\xFAV[\x82T\x90\x91Pc\xFF\xFF\xFF\xFFC\x81\x16\x91\x16\x03a#\x1DW\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua#\xCEV[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16_\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[_\x81Q\x11a$:W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R_Q` a:\xC3_9_Q\x90_R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x07\xCAV[\x80Q`\xFF\x83\x16_\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a$\\\x83\x83a9\x08V[\x11\x15a$\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R_Q` a:\xC3_9_Q\x90_R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[_[\x82\x81\x10\x15a'\xEDW_[a$\xE1\x82\x84a9\x08V[\x81\x10\x15a%\xB2W\x84\x82\x81Q\x81\x10a$\xFAWa$\xFAa6\xD1V[` \x02` \x01\x01Q_\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03_\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01_ \x82\x81T\x81\x10a%6Wa%6a6\xD1V[_\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x03a%\xAAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R_Q` a:\xC3_9_Q\x90_R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x07\xCAV[`\x01\x01a$\xD7V[P_\x84\x82\x81Q\x81\x10a%\xC6Wa%\xC6a6\xD1V[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a&JW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R_Q` a:\xC3_9_Q\x90_R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[`\xFF\x85\x16_\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a&oWa&oa6\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U_\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a&\xD3Wa&\xD3a6\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U_\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a'IWa'Ia6\xD1V[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a'\xA6Wa'\xA6a6\xD1V[` \x02` \x01\x01Q_\x01Q\x86\x84\x81Q\x81\x10a'\xC3Wa'\xC3a6\xD1V[` \x02` \x01\x01Q` \x01Q`@Qa'\xDD\x92\x91\x90a.}V[`@Q\x80\x91\x03\x90\xA2`\x01\x01a$\xCDV[PPPPPV[`\xFF\x82\x16_\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\xFF\x82\x16_\x90\x81R`\x05` R`@\x90 \x80T\x82\x91\x90`\xFF\x19\x16`\x01\x83\x81\x81\x11\x15a(\x89Wa(\x89a2zV[\x02\x17\x90UP\x7F|\x11.\x86<\xCF\0xb\xE2\xC9\xE2X\x19\xC93\xFE\xDB\xC95\ndCB;J\x85\x99\xC2\xE8\xA5-\x81`@Qa(\xBD\x91\x90a2\x8EV[`@Q\x80\x91\x03\x90\xA1PPV[\x81_\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a)mW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x07\xCAV[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a)\x93WP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a*>W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x07\xCAV[PPV[`\xFF\x82\x16_\x90\x81R`\x06` \x90\x81R`@\x91\x82\x90 \x80Tc\xFF\xFF\xFF\xFF\x19\x81\x16c\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x82\x17\x90\x93U\x84Q\x92\x90\x91\x16\x80\x83R\x92\x82\x01R\x90\x91\x7F(\xD75\x8By\xF0-!\xB8\xB7\xE1z\xEF\xC4\x18Zd0\x8A\xA3t\x06\xFA[\xEF\xC0[\x91\x93,9\xC7\x91\x01`@Q\x80\x91\x03\x90\xA1PPPV[_\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a+LW_\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a+\0`\x01\x84a7\x12V[\x81T\x81\x10a+\x10Wa+\x10a6\xD1V[_\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a+:Wa+1`\x01\x82a7\x12V[\x92PPPa\x13\x18V[\x80a+D\x81a9\x1BV[\x91PPa*\xCCV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x07\xCAV[`@\x80Q`\x01\x80\x82R\x81\x83\x01\x90\x92R``\x91_\x91\x90` \x80\x83\x01\x90\x806\x837\x01\x90PP\x90P\x82\x81_\x81Q\x81\x10a,IWa,Ia6\xD1V[`\x01`\x01`\xA0\x1B\x03\x90\x92\x16` \x92\x83\x02\x91\x90\x91\x01\x82\x01R`\xFF\x85\x16_\x90\x81R`\x06\x90\x91R`@\x81 Ta,\x82\x90c\xFF\xFF\xFF\xFF\x16Ba9\x08V[\x90P_\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\xCA\x8A\xA7\xC7`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a,\xE1W=__>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a-\x05\x91\x90a8\xEDV[`@\x80Q\x80\x82\x01\x82R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x82R`\xFF\x8A\x16` \x80\x84\x01\x82\x90R_\x91\x82R`\x04\x90\x81\x90R\x90\x84\x90 \x93Qc\x15\xD5\x96%`\xE1\x1B\x81R\x94\x90\x91\x16\x93c+\xAB,J\x93a-{\x93\x92\x89\x92\x89\x91\x01a90V[_`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a-\x95W=__>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra-\xBC\x91\x90\x81\x01\x90a9\xBBV[\x90P\x80_\x81Q\x81\x10a-\xD0Wa-\xD0a6\xD1V[` \x02` \x01\x01Q\x93PPPP\x92\x91PPV[_a\x13\x18`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a:jV[__\x82\x12\x15a.\x1DWa.\x0C\x82a:\x89V[a.\x16\x90\x84a:\xA3V[\x90Pa\x0C/V[a.\x16\x82\x84a7\xE3V[\x805`\xFF\x81\x16\x81\x14a.7W__\xFD[\x91\x90PV[_` \x82\x84\x03\x12\x15a.LW__\xFD[a\x13\x18\x82a.'V[__`@\x83\x85\x03\x12\x15a.fW__\xFD[a.o\x83a.'V[\x94` \x93\x90\x93\x015\x93PPPV[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a\x1B\x04W__\xFD[__`@\x83\x85\x03\x12\x15a.\xC4W__\xFD[a.\xCD\x83a.'V[\x91P` \x83\x015a.\xDD\x81a.\x9FV[\x80\x91PP\x92P\x92\x90PV[__\x83`\x1F\x84\x01\x12a.\xF8W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x0EW__\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\x1E\x0CW__\xFD[_____``\x86\x88\x03\x12\x15a/<W__\xFD[a/E\x86a.'V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/_W__\xFD[a/k\x88\x82\x89\x01a.\xE8V[\x90\x95P\x93PP`@\x86\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a/\x89W__\xFD[a/\x95\x88\x82\x89\x01a.\xE8V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[__\x83`\x1F\x84\x01\x12a/\xB6W__\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a/\xCCW__\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\x1E\x0CW__\xFD[____``\x85\x87\x03\x12\x15a/\xF6W__\xFD[\x845a0\x01\x81a.\x9FV[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\"W__\xFD[a0.\x87\x82\x88\x01a/\xA6V[\x95\x98\x94\x97P\x95PPPPV[_\x81Q\x80\x84R` \x84\x01\x93P` \x83\x01_[\x82\x81\x10\x15a0sW\x81Q`\x01`\x01``\x1B\x03\x16\x86R` \x95\x86\x01\x95\x90\x91\x01\x90`\x01\x01a0LV[P\x93\x94\x93PPPPV[`@\x81R_a0\x8F`@\x83\x01\x85a0:V[\x82\x81\x03` \x84\x01Ra#\xCE\x81\x85a0:V[__`@\x83\x85\x03\x12\x15a0\xB2W__\xFD[\x825\x91Pa0\xC2` \x84\x01a.'V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a19Wa1#\x83\x85Qc\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[` \x93\x90\x93\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a0\xE4V[P\x90\x95\x94PPPPPV[cNH{q`\xE0\x1B_R`A`\x04R`$_\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1zWa1za1DV[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a1\xA8Wa1\xA8a1DV[`@R\x91\x90PV[_`\x01`\x01`@\x1B\x03\x82\x11\x15a1\xC8Wa1\xC8a1DV[P`\x05\x1B` \x01\x90V[__`@\x83\x85\x03\x12\x15a1\xE3W__\xFD[a1\xEC\x83a.'V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a2\x06W__\xFD[\x83\x01`\x1F\x81\x01\x85\x13a2\x16W__\xFD[\x805a2)a2$\x82a1\xB0V[a1\x80V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x87\x83\x11\x15a2JW__\xFD[` \x84\x01\x93P[\x82\x84\x10\x15a2lW\x835\x82R` \x93\x84\x01\x93\x90\x91\x01\x90a2QV[\x80\x94PPPPP\x92P\x92\x90PV[cNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[` \x81\x01`\x02\x83\x10a2\xAEWcNH{q`\xE0\x1B_R`!`\x04R`$_\xFD[\x91\x90R\x90V[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a.7W__\xFD[_\x82`\x1F\x83\x01\x12a2\xD9W__\xFD[\x815a2\xE7a2$\x82a1\xB0V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x06\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a3\x08W__\xFD[` \x85\x01[\x83\x81\x10\x15a\x10\rW`@\x81\x88\x03\x12\x15a3$W__\xFD[a3,a1XV[\x815a37\x81a.\x9FV[\x81Ra3E` \x83\x01a2\xB4V[` \x82\x01R\x80\x84RP` \x83\x01\x92P`@\x81\x01\x90Pa3\rV[___``\x84\x86\x03\x12\x15a3qW__\xFD[a3z\x84a.'V[\x92Pa3\x88` \x85\x01a2\xB4V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xA2W__\xFD[a3\xAE\x86\x82\x87\x01a2\xCAV[\x91PP\x92P\x92P\x92V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a.7W__\xFD[___`@\x84\x86\x03\x12\x15a3\xDDW__\xFD[a3\xE6\x84a3\xB8V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\0W__\xFD[a4\x0C\x86\x82\x87\x01a/\xA6V[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a19W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a42V[__`@\x83\x85\x03\x12\x15a4gW__\xFD[a4p\x83a.'V[\x91P` \x83\x015`\x02\x81\x10a.\xDDW__\xFD[___``\x84\x86\x03\x12\x15a4\x95W__\xFD[a4\x9E\x84a.'V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[``\x81\x01a\x0C/\x82\x84c\xFF\xFF\xFF\xFF\x81Q\x16\x82Rc\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01R`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[__`@\x83\x85\x03\x12\x15a4\xFEW__\xFD[a5\x07\x83a.'V[\x91Pa0\xC2` \x84\x01a2\xB4V[___`@\x84\x86\x03\x12\x15a5'W__\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\0W__\xFD[__`@\x83\x85\x03\x12\x15a5TW__\xFD[a5]\x83a.'V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a5wW__\xFD[a5\x83\x85\x82\x86\x01a2\xCAV[\x91PP\x92P\x92\x90PV[___``\x84\x86\x03\x12\x15a5\x9FW__\xFD[a5\xA8\x84a.'V[\x92Pa5\xB6` \x85\x01a3\xB8V[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[____`\x80\x85\x87\x03\x12\x15a5\xDAW__\xFD[a5\xE3\x85a.'V[\x93Pa5\xF1` \x86\x01a2\xB4V[\x92Pa5\xFF`@\x86\x01a3\xB8V[\x91P``\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a6\x19W__\xFD[a6%\x87\x82\x88\x01a2\xCAV[\x91PP\x92\x95\x91\x94P\x92PV[___``\x84\x86\x03\x12\x15a6CW__\xFD[\x835\x92Pa6S` \x85\x01a.'V[\x91Pa6a`@\x85\x01a3\xB8V[\x90P\x92P\x92P\x92V[__`@\x83\x85\x03\x12\x15a6{W__\xFD[a6\x84\x83a.'V[\x91Pa0\xC2` \x84\x01a3\xB8V[____`\x80\x85\x87\x03\x12\x15a6\xA5W__\xFD[a6\xAE\x85a.'V[\x93Pa6\xBC` \x86\x01a3\xB8V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[cNH{q`\xE0\x1B_R`2`\x04R`$_\xFD[_` \x82\x84\x03\x12\x15a6\xF5W__\xFD[a\x13\x18\x82a2\xB4V[cNH{q`\xE0\x1B_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x0C/Wa\x0C/a6\xFEV[cNH{q`\xE0\x1B_R`1`\x04R`$_\xFD[` \x80\x82R`5\x90\x82\x01R\x7FStakeRegistry.initializeQuorum: `@\x82\x01Rtquorum already exists`X\x1B``\x82\x01R`\x80\x01\x90V[_` \x82\x84\x03\x12\x15a7\x9EW__\xFD[\x81Q\x80\x15\x15\x81\x14a\x13\x18W__\xFD[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x0C/Wa\x0C/a6\xFEV[_\x82a7\xDEWcNH{q`\xE0\x1B_R`\x12`\x04R`$_\xFD[P\x04\x90V[`\x01`\x01``\x1B\x03\x81\x81\x16\x83\x82\x16\x01\x90\x81\x11\x15a\x0C/Wa\x0C/a6\xFEV[_\x81T\x80\x84R` \x84\x01\x93P\x82_R` _ _[\x82\x81\x10\x15a0sW\x81T`\x01`\x01`\xA0\x1B\x03\x16\x86R` \x90\x95\x01\x94`\x01\x91\x82\x01\x91\x01a8\x17V[`\x01`\x01`\xA0\x1B\x03\x83\x16\x81R`@` \x82\x01\x81\x90R_\x90a\x14\x91\x90\x83\x01\x84a8\x02V[_\x82`\x1F\x83\x01\x12a8pW__\xFD[\x81Qa8~a2$\x82a1\xB0V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x86\x01\x01\x92P\x85\x83\x11\x15a8\x9FW__\xFD[` \x85\x01[\x83\x81\x10\x15a\x10\rW\x80Q\x83R` \x92\x83\x01\x92\x01a8\xA4V[_` \x82\x84\x03\x12\x15a8\xCCW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a8\xE1W__\xFD[a\x14\x91\x84\x82\x85\x01a8aV[_` \x82\x84\x03\x12\x15a8\xFDW__\xFD[\x81Qa\x13\x18\x81a.\x9FV[\x80\x82\x01\x80\x82\x11\x15a\x0C/Wa\x0C/a6\xFEV[_\x81a9)Wa9)a6\xFEV[P_\x19\x01\x90V[\x84Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x80\x86\x01Qc\xFF\xFF\xFF\xFF\x16\x81\x83\x01R`\xA0`@\x83\x01\x81\x90R\x85Q\x90\x83\x01\x81\x90R_\x91\x86\x01\x90`\xC0\x84\x01\x90\x83[\x81\x81\x10\x15a9\x91W\x83Q`\x01`\x01`\xA0\x1B\x03\x16\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a9jV[PP\x83\x81\x03``\x85\x01Ra9\xA5\x81\x87a8\x02V[\x92PPPa#\xCE`\x80\x83\x01\x84c\xFF\xFF\xFF\xFF\x16\x90RV[_` \x82\x84\x03\x12\x15a9\xCBW__\xFD[\x81Q`\x01`\x01`@\x1B\x03\x81\x11\x15a9\xE0W__\xFD[\x82\x01`\x1F\x81\x01\x84\x13a9\xF0W__\xFD[\x80Qa9\xFEa2$\x82a1\xB0V[\x80\x82\x82R` \x82\x01\x91P` \x83`\x05\x1B\x85\x01\x01\x92P\x86\x83\x11\x15a:\x1FW__\xFD[` \x84\x01[\x83\x81\x10\x15a:_W\x80Q`\x01`\x01`@\x1B\x03\x81\x11\x15a:AW__\xFD[a:P\x89` \x83\x89\x01\x01a8aV[\x84RP` \x92\x83\x01\x92\x01a:$V[P\x96\x95PPPPPPV[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a\x07?Wa\x07?a6\xFEV[_`\x01`\xFF\x1B\x82\x01a:\x9DWa:\x9Da6\xFEV[P_\x03\x90V[`\x01`\x01``\x1B\x03\x82\x81\x16\x82\x82\x16\x03\x90\x81\x11\x15a\x0C/Wa\x0C/a6\xFEV\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \x8E\x87q\x9F\x9A\x17\x98`Z\x8A8t9H\xD4\xFA\x0CC\xA4\x1Du\xE88\xD4\rj\xF0\xFC\xDF\x19\xD4UdsolcC\0\x08\x1B\x003",
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
            DECODE_SHIMS[idx](data, validate)
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
