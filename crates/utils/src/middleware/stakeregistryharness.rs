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
    function applyDelta(uint96 value, int256 delta) external pure returns (uint96);
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
    function initializeQuorum(uint8 quorumNumber, uint96 minimumStake, IStakeRegistry.StrategyParams[] memory _strategyParams) external;
    function minimumStakeForQuorum(uint8) external view returns (uint96);
    function modifyStrategyParams(uint8 quorumNumber, uint256[] memory strategyIndices, uint96[] memory newMultipliers) external;
    function recordOperatorStakeUpdate(bytes32 operatorId, uint8 quorumNumber, uint96 newStake) external returns (int256);
    function recordTotalStakeUpdate(uint8 quorumNumber, int256 stakeDelta) external;
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
pub mod StakeRegistryHarness {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60c06040523480156200001157600080fd5b5060405162003a6a38038062003a6a833981016040819052620000349162000065565b6001600160a01b0391821660a05216608052620000a4565b6001600160a01b03811681146200006257600080fd5b50565b600080604083850312156200007957600080fd5b825162000086816200004c565b602084015190925062000099816200004c565b809150509250929050565b60805160a05161396162000109600039600081816103b9015281816106b9015281816109f601528181610d6d01528181611198015281816117ab015281816118a8015281816119cc0152611d8e01526000818161058e01526120c901526139616000f3fe608060405234801561001057600080fd5b50600436106102115760003560e01c806381c0750211610125578063c8294c56116100ad578063f2be94ae1161007c578063f2be94ae146105b0578063f509551a146105c3578063f851e198146105d6578063fa28c627146105e9578063ff694a77146105fc57600080fd5b8063c8294c561461053b578063d5eccc051461054e578063dd9846b914610561578063df5cf7231461058957600080fd5b8063b6904b78116100f4578063b6904b78146104c6578063bc9a40c3146104d9578063bd29b8cd146104ec578063c46778a5146104ff578063c601527d1461052857600080fd5b806381c07502146104335780639f3ccf6514610453578063ac6bfb0314610466578063adc804da1461048657600080fd5b80634bd26e09116101a857806366acfefe1161017757806366acfefe146103895780636d14a987146103b457806374454c6d146103f35780637c172347146104065780637f4298221461042057600080fd5b80634bd26e09146103245780635401ed27146103545780635e5a6775146103675780635f1f2d771461037657600080fd5b806320b66298116101e457806320b66298146102ad57806325504777146102c05780632cd95940146102e15780633ca5a5f51461030157600080fd5b80630390a4d5146102165780630491b41c1461022b57806308732461146102615780631f9b74e014610282575b600080fd5b610229610224366004612cce565b61060f565b005b61024e610239366004612cf8565b60ff1660009081526001602052604090205490565b6040519081526020015b60405180910390f35b61027461026f366004612cce565b61061e565b604051610258929190612d13565b610295610290366004612d4d565b610667565b6040516001600160601b039091168152602001610258565b6102296102bb366004612dc8565b6106b7565b6102d36102ce366004612e89565b6109e8565b604051610258929190612f28565b6102f46102ef366004612f4d565b610cb3565b6040516102589190612f79565b61024e61030f366004612cf8565b60ff1660009081526003602052604090205490565b61024e610332366004612f4d565b600091825260026020908152604080842060ff93909316845291905290205490565b610295610362366004612f4d565b610d52565b61024e670de0b6b3a764000081565b610229610384366004613082565b610d6b565b61039c610397366004612e89565b61118b565b6040516001600160c01b039091168152602001610258565b6103db7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610258565b61024e610401366004613141565b6112e5565b61040e602081565b60405160ff9091168152602001610258565b61029561042e36600461317d565b6112fc565b6104466104413660046131ad565b611308565b60405161025891906131ff565b6103db610461366004612cce565b6115d0565b61047961047436600461323d565b611608565b6040516102589190613270565b610499610494366004612cce565b6116a0565b6040805182516001600160a01b031681526020928301516001600160601b03169281019290925201610258565b6104796104d4366004612cce565b61171a565b6102296104e73660046132a5565b6117a9565b6102296104fa3660046132cf565b61189d565b61029561050d366004612cf8565b6000602081905290815260409020546001600160601b031681565b61022961053636600461339b565b6119ca565b6102956105493660046133e8565b611abe565b61029561055c366004612cf8565b611b3c565b61057461056f366004613424565b611b8f565b60405163ffffffff9091168152602001610258565b6103db7f000000000000000000000000000000000000000000000000000000000000000081565b6102956105be366004613457565b611b9c565b61024e6105d1366004613499565b611c31565b6104796105e4366004612f4d565b611c3d565b6102956105f7366004613424565b611d22565b61022961060a3660046134b5565b611d83565b6106198282611eee565b505050565b6003602052816000526040600020818154811061063a57600080fd5b6000918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b60ff821660009081526001602052604081205483906106a15760405162461bcd60e51b815260040161069890613512565b60405180910390fd5b60006106ad8585612068565b5095945050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610715573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107399190613563565b6001600160a01b0316336001600160a01b0316146107695760405162461bcd60e51b815260040161069890613580565b846107858160ff16600090815260016020526040902054151590565b6107a15760405162461bcd60e51b815260040161069890613512565b8380610817576040805162461bcd60e51b81526020600482015260248101919091527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a206e6f20737472617465677920696e64696365732070726f76696465646064820152608401610698565b82811461088c5760405162461bcd60e51b815260206004820152603960248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a20696e707574206c656e677468206d69736d61746368000000000000006064820152608401610698565b60ff87166000908152600360205260408120905b828110156109dd578585828181106108ba576108ba6135fc565b90506020020160208101906108cf9190613612565b828989848181106108e2576108e26135fc565b90506020020135815481106108f9576108f96135fc565b9060005260206000200160000160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a85818110610962576109626135fc565b9050602002013581548110610979576109796135fc565b6000918252602090912001546001600160a01b03168888858181106109a0576109a06135fc565b90506020020160208101906109b59190613612565b6040516109c3929190612d13565b60405180910390a2806109d581613643565b9150506108a0565b505050505050505050565b606080336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a335760405162461bcd60e51b81526004016106989061365e565b6000836001600160401b03811115610a4d57610a4d612ff1565b604051908082528060200260200182016040528015610a76578160200160208202803683370190505b5090506000846001600160401b03811115610a9357610a93612ff1565b604051908082528060200260200182016040528015610abc578160200160208202803683370190505b50905060005b85811015610ca5576000878783818110610ade57610ade6135fc565b919091013560f81c60008181526001602052604090205490925015159050610b665760405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a206044820152741c5d5bdc9d5b48191bd95cc81b9bdd08195e1a5cdd605a1b6064820152608401610698565b600080610b73838d612068565b9150915080610c105760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a2060448201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360648201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000608482015260a401610698565b6000610c1d8c8585612266565b905082878681518110610c3257610c326135fc565b60200260200101906001600160601b031690816001600160601b031681525050610c5c8482611eee565b868681518110610c6e57610c6e6135fc565b60200260200101906001600160601b031690816001600160601b031681525050505050508080610c9d90613643565b915050610ac2565b509097909650945050505050565b600082815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610d45576000848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610cec565b5050505090505b92915050565b600080610d5f8484611c3d565b60400151949350505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610dc9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ded9190613563565b6001600160a01b0316336001600160a01b031614610e1d5760405162461bcd60e51b815260040161069890613580565b81610e398160ff16600090815260016020526040902054151590565b610e555760405162461bcd60e51b815260040161069890613512565b815180610eca5760405162461bcd60e51b815260206004820152603d60248201527f5374616b6552656769737472792e72656d6f7665537472617465676965733a2060448201527f6e6f20696e646963657320746f2072656d6f76652070726f76696465640000006064820152608401610698565b60ff841660009081526003602090815260408083206004909252822090915b83811015611182578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f784888481518110610f2957610f296135fc565b602002602001015181548110610f4157610f416135fc565b600091825260209182902001546040516001600160a01b0390911681520160405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7584888481518110610f9f57610f9f6135fc565b602002602001015181548110610fb757610fb76135fc565b600091825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a282548390610ff7906001906136d0565b81548110611007576110076135fc565b9060005260206000200183878381518110611024576110246135fc565b60200260200101518154811061103c5761103c6135fc565b600091825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b918290041602179055825483908061108f5761108f6136e7565b600082815260208120820160001990810191909155019055815482906110b7906001906136d0565b815481106110c7576110c76135fc565b9060005260206000200160009054906101000a90046001600160a01b0316828783815181106110f8576110f86135fc565b602002602001015181548110611110576111106135fc565b9060005260206000200160006101000a8154816001600160a01b0302191690836001600160a01b031602179055508180548061114e5761114e6136e7565b600082815260209020810160001990810180546001600160a01b03191690550190558061117a81613643565b915050610ee9565b50505050505050565b6000336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146111d55760405162461bcd60e51b81526004016106989061365e565b6000805b838110156106ad5760008585838181106111f5576111f56135fc565b919091013560f81c600081815260016020526040902054909250151590506112855760405162461bcd60e51b815260206004820152603860248201527f5374616b6552656769737472792e7570646174654f70657261746f725374616b60448201527f653a2071756f72756d20646f6573206e6f7420657869737400000000000000006064820152608401610698565b600080611292838b612068565b91509150806112b45760009150600160ff84161b6001600160c01b0386161794505b60006112c18a8585612266565b90506112cd8482611eee565b505050505080806112dd90613643565b9150506111d9565b60006112f2848484612266565b90505b9392505050565b60006112f583836124e6565b60606000826001600160401b0381111561132457611324612ff1565b60405190808252806020026020018201604052801561134d578160200160208202803683370190505b50905060005b838110156115c757600085858381811061136f5761136f6135fc565b919091013560f81c6000818152600160205260409020549092501515905061140e5760405162461bcd60e51b815260206004820152604660248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20646f6573206e6f7460648201526508195e1a5cdd60d21b608482015260a401610698565b60ff81166000908152600160205260408120805463ffffffff8a169290611437576114376135fc565b60009182526020909120015463ffffffff1611156114e35760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360648201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000608482015260a401610698565b60ff8116600090815260016020526040812054905b818110156115b15760ff8316600090815260016020819052604090912063ffffffff8b169161152784866136d0565b61153191906136d0565b81548110611541576115416135fc565b60009182526020909120015463ffffffff161161159f57600161156482846136d0565b61156e91906136d0565b858581518110611580576115806135fc565b602002602001019063ffffffff16908163ffffffff16815250506115b1565b806115a981613643565b9150506114f8565b50505080806115bf90613643565b915050611353565b50949350505050565b600460205281600052604060002081815481106115ec57600080fd5b6000918252602090912001546001600160a01b03169150829050565b60408051606081018252600080825260208083018290528284018290528582526002815283822060ff8816835290529190912080548390811061164d5761164d6135fc565b600091825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091526000808252602082015260ff831660009081526003602052604090208054839081106116d8576116d86135fc565b6000918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182526000808252602080830182905282840182905260ff861682526001905291909120805483908110611757576117576135fc565b600091825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611807573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061182b9190613563565b6001600160a01b0316336001600160a01b03161461185b5760405162461bcd60e51b815260040161069890613580565b816118778160ff16600090815260016020526040902054151590565b6118935760405162461bcd60e51b815260040161069890613512565b6106198383612514565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146118e55760405162461bcd60e51b81526004016106989061365e565b60005b818110156119c4576000838383818110611904576119046135fc565b919091013560f81c600081815260016020526040902054909250151590506119945760405162461bcd60e51b815260206004820152603760248201527f5374616b6552656769737472792e646572656769737465724f70657261746f7260448201527f3a2071756f72756d20646f6573206e6f742065786973740000000000000000006064820152608401610698565b60006119a286836000612266565b90506119ae8282611eee565b50505080806119bc90613643565b9150506118e8565b50505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a28573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a4c9190613563565b6001600160a01b0316336001600160a01b031614611a7c5760405162461bcd60e51b815260040161069890613580565b81611a988160ff16600090815260016020526040902054151590565b611ab45760405162461bcd60e51b815260040161069890613512565b610619838361257d565b60ff83166000908152600160205260408120805482919084908110611ae557611ae56135fc565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610d5f81856129c0565b60ff81166000908152600160208190526040822080549091611b5d916136d0565b81548110611b6d57611b6d6135fc565b600091825260209091200154600160401b90046001600160601b031692915050565b60006112f2848484612b3a565b600082815260026020908152604080832060ff881684529091528120805482919084908110611bcd57611bcd6135fc565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050611c2481866129c0565b6040015195945050505050565b60006112f58383612ca0565b6040805160608082018352600080835260208084018290528385018290528682526002815284822060ff87168352815284822054855193840186528284529083018290529382015290919081611c96579150610d4c9050565b600085815260026020908152604080832060ff881684529091529020611cbd6001846136d0565b81548110611ccd57611ccd6135fc565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610d4c915050565b600083815260026020908152604080832060ff861684529091528120611d49858585612b3a565b63ffffffff1681548110611d5f57611d5f6135fc565b600091825260209091200154600160401b90046001600160601b0316949350505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611dcb5760405162461bcd60e51b81526004016106989061365e565b60ff831660009081526001602052604090205415611e495760405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a2060448201527471756f72756d20616c72656164792065786973747360581b6064820152608401610698565b611e53838261257d565b611e5d8383612514565b505060ff166000908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60ff821660009081526001602081905260408220805491839190611f1290846136d0565b81548110611f2257611f226135fc565b9060005260206000200190508360001415611f515754600160401b90046001600160601b03169150610d4c9050565b8054600090611f7090600160401b90046001600160601b0316866124e6565b82549091504363ffffffff90811691161415611fad578154600160401b600160a01b031916600160401b6001600160601b0383160217825561205f565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff8916600090815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b6000806000806120878660ff1660009081526003602052604090205490565b604080518082019091526000808252602082015290915060ff871660009081526004602081905260408083209051639004134760e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016926390041347926120fc928c92016136fd565b600060405180830381865afa158015612119573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052612141919081019061375c565b905060005b838110156122325760ff89166000908152600360205260409020805482908110612172576121726135fc565b60009182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b03169082015283519094508390839081106121c0576121c06135fc565b6020026020010151111561222057670de0b6b3a764000083602001516001600160601b03168383815181106121f7576121f76135fc565b602002602001015161220991906137ec565b612213919061380b565b61221d908661382d565b94505b8061222a81613643565b915050612146565b50505060ff8616600090815260208190526040902054919350506001600160601b03908116908316101590505b9250929050565b600083815260026020908152604080832060ff8616845290915281205481908061232a57600086815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff199096169190921617939093171691909117905561248c565b600086815260026020908152604080832060ff8916845290915281206123516001846136d0565b81548110612361576123616135fc565b600091825260209091200180546001600160601b03600160401b90910481169450909150851683141561239a57600093505050506112f5565b80544363ffffffff908116911614156123d4578054600160401b600160a01b031916600160401b6001600160601b0387160217815561248a565b805467ffffffff000000001916600160201b4363ffffffff90811682810293909317845560008a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a26124dc8285612ca0565b9695505050505050565b60008082121561250a576124f982613858565b6125039084613875565b9050610d4c565b612503828461382d565b60ff82166000818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b60008151116125e25760405162461bcd60e51b8152602060048201526038602482015260008051602061390c83398151915260448201527f3a206e6f20737472617465676965732070726f766964656400000000000000006064820152608401610698565b805160ff831660009081526003602090815260409091205490612605838361389d565b11156126755760405162461bcd60e51b8152602060048201526045602482015260008051602061390c83398151915260448201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60648201526408a9c8ea8960db1b608482015260a401610698565b60005b828110156129b95760005b61268d828461389d565b81101561276e578482815181106126a6576126a66135fc565b6020026020010151600001516001600160a01b0316600360008860ff1660ff16815260200190815260200160002082815481106126e5576126e56135fc565b6000918252602090912001546001600160a01b0316141561275c5760405162461bcd60e51b815260206004820152603d602482015260008051602061390c83398151915260448201527f3a2063616e6e6f74206164642073616d652073747261746567792032780000006064820152608401610698565b8061276681613643565b915050612683565b506000848281518110612783576127836135fc565b6020026020010151602001516001600160601b0316116128085760405162461bcd60e51b8152602060048201526046602482015260008051602061390c83398151915260448201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f206064820152651dd95a59da1d60d21b608482015260a401610698565b60ff85166000908152600360205260409020845185908390811061282e5761282e6135fc565b602090810291909101810151825460018101845560009384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff8716825260049052604090208451859083908110612893576128936135fc565b6020908102919091018101515182546001810184556000938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f54049086908490811061290a5761290a6135fc565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75858381518110612967576129676135fc565b602002602001015160000151868481518110612985576129856135fc565b60200260200101516020015160405161299f929190612d13565b60405180910390a2806129b181613643565b915050612678565b5050505050565b816000015163ffffffff168163ffffffff161015612a655760405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a207374616b6555706461746520697320606482015275333937b69030b33a32b910313637b1b5a73ab6b132b960511b608482015260a401610698565b602082015163ffffffff161580612a8b5750816020015163ffffffff168163ffffffff16105b612b365760405162461bcd60e51b815260206004820152606a60248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560648201527f72207374616b6555706461746520617661696c61626c65206265666f726520626084820152693637b1b5a73ab6b132b960b11b60a482015260c401610698565b5050565b600083815260026020908152604080832060ff86168452909152812054805b8015612bdb57600086815260026020908152604080832060ff89168452909152902063ffffffff851690612b8e6001846136d0565b81548110612b9e57612b9e6135fc565b60009182526020909120015463ffffffff1611612bc957612bc06001826136d0565b925050506112f5565b80612bd3816138b5565b915050612b59565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e401610698565b60006112f56001600160601b038085169084166138cc565b803560ff81168114612cc957600080fd5b919050565b60008060408385031215612ce157600080fd5b612cea83612cb8565b946020939093013593505050565b600060208284031215612d0a57600080fd5b6112f582612cb8565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114612d4a57600080fd5b50565b60008060408385031215612d6057600080fd5b612d6983612cb8565b91506020830135612d7981612d35565b809150509250929050565b60008083601f840112612d9657600080fd5b5081356001600160401b03811115612dad57600080fd5b6020830191508360208260051b850101111561225f57600080fd5b600080600080600060608688031215612de057600080fd5b612de986612cb8565b945060208601356001600160401b0380821115612e0557600080fd5b612e1189838a01612d84565b90965094506040880135915080821115612e2a57600080fd5b50612e3788828901612d84565b969995985093965092949392505050565b60008083601f840112612e5a57600080fd5b5081356001600160401b03811115612e7157600080fd5b60208301915083602082850101111561225f57600080fd5b60008060008060608587031215612e9f57600080fd5b8435612eaa81612d35565b93506020850135925060408501356001600160401b03811115612ecc57600080fd5b612ed887828801612e48565b95989497509550505050565b600081518084526020808501945080840160005b83811015612f1d5781516001600160601b031687529582019590820190600101612ef8565b509495945050505050565b604081526000612f3b6040830185612ee4565b828103602084015261205f8185612ee4565b60008060408385031215612f6057600080fd5b82359150612f7060208401612cb8565b90509250929050565b6020808252825182820181905260009190848201906040850190845b81811015612fe557612fd283855163ffffffff808251168352806020830151166020840152506001600160601b0360408201511660408301525050565b9284019260609290920191600101612f95565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b038111828210171561302957613029612ff1565b60405290565b604051601f8201601f191681016001600160401b038111828210171561305757613057612ff1565b604052919050565b60006001600160401b0382111561307857613078612ff1565b5060051b60200190565b6000806040838503121561309557600080fd5b61309e83612cb8565b91506020808401356001600160401b038111156130ba57600080fd5b8401601f810186136130cb57600080fd5b80356130de6130d98261305f565b61302f565b81815260059190911b820183019083810190888311156130fd57600080fd5b928401925b8284101561311b57833582529284019290840190613102565b80955050505050509250929050565b80356001600160601b0381168114612cc957600080fd5b60008060006060848603121561315657600080fd5b8335925061316660208501612cb8565b91506131746040850161312a565b90509250925092565b6000806040838503121561319057600080fd5b612cea8361312a565b803563ffffffff81168114612cc957600080fd5b6000806000604084860312156131c257600080fd5b6131cb84613199565b925060208401356001600160401b038111156131e657600080fd5b6131f286828701612e48565b9497909650939450505050565b6020808252825182820181905260009190848201906040850190845b81811015612fe557835163ffffffff168352928401929184019160010161321b565b60008060006060848603121561325257600080fd5b61325b84612cb8565b95602085013595506040909401359392505050565b815163ffffffff9081168252602080840151909116908201526040808301516001600160601b03169082015260608101610d4c565b600080604083850312156132b857600080fd5b6132c183612cb8565b9150612f706020840161312a565b6000806000604084860312156132e457600080fd5b8335925060208401356001600160401b038111156131e657600080fd5b600082601f83011261331257600080fd5b813560206133226130d98361305f565b82815260069290921b8401810191818101908684111561334157600080fd5b8286015b84811015613390576040818903121561335e5760008081fd5b613366613007565b813561337181612d35565b815261337e82860161312a565b81860152835291830191604001613345565b509695505050505050565b600080604083850312156133ae57600080fd5b6133b783612cb8565b915060208301356001600160401b038111156133d257600080fd5b6133de85828601613301565b9150509250929050565b6000806000606084860312156133fd57600080fd5b61340684612cb8565b925061341460208501613199565b9150604084013590509250925092565b60008060006060848603121561343957600080fd5b8335925061344960208501612cb8565b915061317460408501613199565b6000806000806080858703121561346d57600080fd5b61347685612cb8565b935061348460208601613199565b93969395505050506040820135916060013590565b600080604083850312156134ac57600080fd5b6132c18361312a565b6000806000606084860312156134ca57600080fd5b6134d384612cb8565b92506134e16020850161312a565b915060408401356001600160401b038111156134fc57600080fd5b61350886828701613301565b9150509250925092565b60208082526031908201527f5374616b6552656769737472792e71756f72756d4578697374733a2071756f726040820152701d5b48191bd95cc81b9bdd08195e1a5cdd607a1b606082015260800190565b60006020828403121561357557600080fd5b81516112f581612d35565b60208082526056908201527f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e60408201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f6620746060820152753432903932b3b4b9ba393ca1b7b7b93234b730ba37b960511b608082015260a00190565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561362457600080fd5b6112f58261312a565b634e487b7160e01b600052601160045260246000fd5b60006000198214156136575761365761362d565b5060010190565b6020808252604c908201527f5374616b6552656769737472792e6f6e6c795265676973747279436f6f72646960408201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260608201526b3ca1b7b7b93234b730ba37b960a11b608082015260a00190565b6000828210156136e2576136e261362d565b500390565b634e487b7160e01b600052603160045260246000fd5b60006040820160018060a01b03808616845260206040818601528286548085526060870191508760005282600020945060005b8181101561374e578554851683526001958601959284019201613730565b509098975050505050505050565b6000602080838503121561376f57600080fd5b82516001600160401b0381111561378557600080fd5b8301601f8101851361379657600080fd5b80516137a46130d98261305f565b81815260059190911b820183019083810190878311156137c357600080fd5b928401925b828410156137e1578351825292840192908401906137c8565b979650505050505050565b60008160001904831182151516156138065761380661362d565b500290565b60008261382857634e487b7160e01b600052601260045260246000fd5b500490565b60006001600160601b0380831681851680830382111561384f5761384f61362d565b01949350505050565b6000600160ff1b82141561386e5761386e61362d565b5060000390565b60006001600160601b03838116908316818110156138955761389561362d565b039392505050565b600082198211156138b0576138b061362d565b500190565b6000816138c4576138c461362d565b506000190190565b60008083128015600160ff1b8501841216156138ea576138ea61362d565b6001600160ff1b03840183138116156139055761390561362d565b5050039056fe5374616b6552656769737472792e5f6164645374726174656779506172616d73a2646970667358221220d1c160b66d71a1a431473146b286303b69b5167369c9475b6da7380e18c6b56f64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xC0`@R4\x80\x15b\0\0\x11W`\0\x80\xFD[P`@Qb\0:j8\x03\x80b\0:j\x839\x81\x01`@\x81\x90Rb\0\x004\x91b\0\0eV[`\x01`\x01`\xA0\x1B\x03\x91\x82\x16`\xA0R\x16`\x80Rb\0\0\xA4V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14b\0\0bW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15b\0\0yW`\0\x80\xFD[\x82Qb\0\0\x86\x81b\0\0LV[` \x84\x01Q\x90\x92Pb\0\0\x99\x81b\0\0LV[\x80\x91PP\x92P\x92\x90PV[`\x80Q`\xA0Qa9ab\0\x01\t`\09`\0\x81\x81a\x03\xB9\x01R\x81\x81a\x06\xB9\x01R\x81\x81a\t\xF6\x01R\x81\x81a\rm\x01R\x81\x81a\x11\x98\x01R\x81\x81a\x17\xAB\x01R\x81\x81a\x18\xA8\x01R\x81\x81a\x19\xCC\x01Ra\x1D\x8E\x01R`\0\x81\x81a\x05\x8E\x01Ra \xC9\x01Ra9a`\0\xF3\xFE`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80c\x81\xC0u\x02\x11a\x01%W\x80c\xC8)LV\x11a\0\xADW\x80c\xF2\xBE\x94\xAE\x11a\0|W\x80c\xF2\xBE\x94\xAE\x14a\x05\xB0W\x80c\xF5\tU\x1A\x14a\x05\xC3W\x80c\xF8Q\xE1\x98\x14a\x05\xD6W\x80c\xFA(\xC6'\x14a\x05\xE9W\x80c\xFFiJw\x14a\x05\xFCW`\0\x80\xFD[\x80c\xC8)LV\x14a\x05;W\x80c\xD5\xEC\xCC\x05\x14a\x05NW\x80c\xDD\x98F\xB9\x14a\x05aW\x80c\xDF\\\xF7#\x14a\x05\x89W`\0\x80\xFD[\x80c\xB6\x90Kx\x11a\0\xF4W\x80c\xB6\x90Kx\x14a\x04\xC6W\x80c\xBC\x9A@\xC3\x14a\x04\xD9W\x80c\xBD)\xB8\xCD\x14a\x04\xECW\x80c\xC4gx\xA5\x14a\x04\xFFW\x80c\xC6\x01R}\x14a\x05(W`\0\x80\xFD[\x80c\x81\xC0u\x02\x14a\x043W\x80c\x9F<\xCFe\x14a\x04SW\x80c\xACk\xFB\x03\x14a\x04fW\x80c\xAD\xC8\x04\xDA\x14a\x04\x86W`\0\x80\xFD[\x80cK\xD2n\t\x11a\x01\xA8W\x80cf\xAC\xFE\xFE\x11a\x01wW\x80cf\xAC\xFE\xFE\x14a\x03\x89W\x80cm\x14\xA9\x87\x14a\x03\xB4W\x80ctELm\x14a\x03\xF3W\x80c|\x17#G\x14a\x04\x06W\x80c\x7FB\x98\"\x14a\x04 W`\0\x80\xFD[\x80cK\xD2n\t\x14a\x03$W\x80cT\x01\xED'\x14a\x03TW\x80c^Zgu\x14a\x03gW\x80c_\x1F-w\x14a\x03vW`\0\x80\xFD[\x80c \xB6b\x98\x11a\x01\xE4W\x80c \xB6b\x98\x14a\x02\xADW\x80c%PGw\x14a\x02\xC0W\x80c,\xD9Y@\x14a\x02\xE1W\x80c<\xA5\xA5\xF5\x14a\x03\x01W`\0\x80\xFD[\x80c\x03\x90\xA4\xD5\x14a\x02\x16W\x80c\x04\x91\xB4\x1C\x14a\x02+W\x80c\x08s$a\x14a\x02aW\x80c\x1F\x9Bt\xE0\x14a\x02\x82W[`\0\x80\xFD[a\x02)a\x02$6`\x04a,\xCEV[a\x06\x0FV[\0[a\x02Na\x0296`\x04a,\xF8V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ta\x02o6`\x04a,\xCEV[a\x06\x1EV[`@Qa\x02X\x92\x91\x90a-\x13V[a\x02\x95a\x02\x906`\x04a-MV[a\x06gV[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02XV[a\x02)a\x02\xBB6`\x04a-\xC8V[a\x06\xB7V[a\x02\xD3a\x02\xCE6`\x04a.\x89V[a\t\xE8V[`@Qa\x02X\x92\x91\x90a/(V[a\x02\xF4a\x02\xEF6`\x04a/MV[a\x0C\xB3V[`@Qa\x02X\x91\x90a/yV[a\x02Na\x03\x0F6`\x04a,\xF8V[`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Na\x0326`\x04a/MV[`\0\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\x95a\x03b6`\x04a/MV[a\rRV[a\x02Ng\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02)a\x03\x846`\x04a0\x82V[a\rkV[a\x03\x9Ca\x03\x976`\x04a.\x89V[a\x11\x8BV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02XV[a\x03\xDB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02XV[a\x02Na\x04\x016`\x04a1AV[a\x12\xE5V[a\x04\x0E` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02XV[a\x02\x95a\x04.6`\x04a1}V[a\x12\xFCV[a\x04Fa\x04A6`\x04a1\xADV[a\x13\x08V[`@Qa\x02X\x91\x90a1\xFFV[a\x03\xDBa\x04a6`\x04a,\xCEV[a\x15\xD0V[a\x04ya\x04t6`\x04a2=V[a\x16\x08V[`@Qa\x02X\x91\x90a2pV[a\x04\x99a\x04\x946`\x04a,\xCEV[a\x16\xA0V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02XV[a\x04ya\x04\xD46`\x04a,\xCEV[a\x17\x1AV[a\x02)a\x04\xE76`\x04a2\xA5V[a\x17\xA9V[a\x02)a\x04\xFA6`\x04a2\xCFV[a\x18\x9DV[a\x02\x95a\x05\r6`\x04a,\xF8V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02)a\x0566`\x04a3\x9BV[a\x19\xCAV[a\x02\x95a\x05I6`\x04a3\xE8V[a\x1A\xBEV[a\x02\x95a\x05\\6`\x04a,\xF8V[a\x1B<V[a\x05ta\x05o6`\x04a4$V[a\x1B\x8FV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02XV[a\x03\xDB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x95a\x05\xBE6`\x04a4WV[a\x1B\x9CV[a\x02Na\x05\xD16`\x04a4\x99V[a\x1C1V[a\x04ya\x05\xE46`\x04a/MV[a\x1C=V[a\x02\x95a\x05\xF76`\x04a4$V[a\x1D\"V[a\x02)a\x06\n6`\x04a4\xB5V[a\x1D\x83V[a\x06\x19\x82\x82a\x1E\xEEV[PPPV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x06:W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x83\x90a\x06\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xAD\x85\x85a hV[P\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x079\x91\x90a5cV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07iW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x80V[\x84a\x07\x85\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x07\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[\x83\x80a\x08\x17W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01a\x06\x98V[\x82\x81\x14a\x08\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[`\xFF\x87\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\t\xDDW\x85\x85\x82\x81\x81\x10a\x08\xBAWa\x08\xBAa5\xFCV[\x90P` \x02\x01` \x81\x01\x90a\x08\xCF\x91\x90a6\x12V[\x82\x89\x89\x84\x81\x81\x10a\x08\xE2Wa\x08\xE2a5\xFCV[\x90P` \x02\x015\x81T\x81\x10a\x08\xF9Wa\x08\xF9a5\xFCV[\x90`\0R` `\0 \x01`\0\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\tbWa\tba5\xFCV[\x90P` \x02\x015\x81T\x81\x10a\tyWa\tya5\xFCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\t\xA0Wa\t\xA0a5\xFCV[\x90P` \x02\x01` \x81\x01\x90a\t\xB5\x91\x90a6\x12V[`@Qa\t\xC3\x92\x91\x90a-\x13V[`@Q\x80\x91\x03\x90\xA2\x80a\t\xD5\x81a6CV[\x91PPa\x08\xA0V[PPPPPPPPPV[``\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a6^V[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\nMWa\nMa/\xF1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\nvW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x93Wa\n\x93a/\xF1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xBCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85\x81\x10\x15a\x0C\xA5W`\0\x87\x87\x83\x81\x81\x10a\n\xDEWa\n\xDEa5\xFCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x0BfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x01a\x06\x98V[`\0\x80a\x0Bs\x83\x8Da hV[\x91P\x91P\x80a\x0C\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\0a\x0C\x1D\x8C\x85\x85a\"fV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0C2Wa\x0C2a5\xFCV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0C\\\x84\x82a\x1E\xEEV[\x86\x86\x81Q\x81\x10a\x0CnWa\x0Cna5\xFCV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPPPPP\x80\x80a\x0C\x9D\x90a6CV[\x91PPa\n\xC2V[P\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\rEW`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0C\xECV[PPPP\x90P[\x92\x91PPV[`\0\x80a\r_\x84\x84a\x1C=V[`@\x01Q\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xED\x91\x90a5cV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x80V[\x81a\x0E9\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x0EUW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[\x81Q\x80a\x0E\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[`\xFF\x84\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91[\x83\x81\x10\x15a\x11\x82W\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x0F)Wa\x0F)a5\xFCV[` \x02` \x01\x01Q\x81T\x81\x10a\x0FAWa\x0FAa5\xFCV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x0F\x9FWa\x0F\x9Fa5\xFCV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\xB7Wa\x0F\xB7a5\xFCV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x82T\x83\x90a\x0F\xF7\x90`\x01\x90a6\xD0V[\x81T\x81\x10a\x10\x07Wa\x10\x07a5\xFCV[\x90`\0R` `\0 \x01\x83\x87\x83\x81Q\x81\x10a\x10$Wa\x10$a5\xFCV[` \x02` \x01\x01Q\x81T\x81\x10a\x10<Wa\x10<a5\xFCV[`\0\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x82T\x83\x90\x80a\x10\x8FWa\x10\x8Fa6\xE7V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x81T\x82\x90a\x10\xB7\x90`\x01\x90a6\xD0V[\x81T\x81\x10a\x10\xC7Wa\x10\xC7a5\xFCV[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x83\x81Q\x81\x10a\x10\xF8Wa\x10\xF8a5\xFCV[` \x02` \x01\x01Q\x81T\x81\x10a\x11\x10Wa\x11\x10a5\xFCV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x80T\x80a\x11NWa\x11Na6\xE7V[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U\x80a\x11z\x81a6CV[\x91PPa\x0E\xE9V[PPPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a6^V[`\0\x80[\x83\x81\x10\x15a\x06\xADW`\0\x85\x85\x83\x81\x81\x10a\x11\xF5Wa\x11\xF5a5\xFCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x12\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FStakeRegistry.updateOperatorStak`D\x82\x01R\x7Fe: quorum does not exist\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[`\0\x80a\x12\x92\x83\x8Ba hV[\x91P\x91P\x80a\x12\xB4W`\0\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[`\0a\x12\xC1\x8A\x85\x85a\"fV[\x90Pa\x12\xCD\x84\x82a\x1E\xEEV[PPPPP\x80\x80a\x12\xDD\x90a6CV[\x91PPa\x11\xD9V[`\0a\x12\xF2\x84\x84\x84a\"fV[\x90P[\x93\x92PPPV[`\0a\x12\xF5\x83\x83a$\xE6V[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13$Wa\x13$a/\xF1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13MW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x15\xC7W`\0\x85\x85\x83\x81\x81\x10a\x13oWa\x13oa5\xFCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x14\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum does not`d\x82\x01Re\x08\x19^\x1A\\\xDD`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x147Wa\x147a5\xFCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x14\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x15\xB1W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x15'\x84\x86a6\xD0V[a\x151\x91\x90a6\xD0V[\x81T\x81\x10a\x15AWa\x15Aa5\xFCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x15\x9FW`\x01a\x15d\x82\x84a6\xD0V[a\x15n\x91\x90a6\xD0V[\x85\x85\x81Q\x81\x10a\x15\x80Wa\x15\x80a5\xFCV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x15\xB1V[\x80a\x15\xA9\x81a6CV[\x91PPa\x14\xF8V[PPP\x80\x80a\x15\xBF\x90a6CV[\x91PPa\x13SV[P\x94\x93PPPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x15\xECW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x16MWa\x16Ma5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x16\xD8Wa\x16\xD8a5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x17WWa\x17Wa5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18+\x91\x90a5cV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18[W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x80V[\x81a\x18w\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x18\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[a\x06\x19\x83\x83a%\x14V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a6^V[`\0[\x81\x81\x10\x15a\x19\xC4W`\0\x83\x83\x83\x81\x81\x10a\x19\x04Wa\x19\x04a5\xFCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x19\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStakeRegistry.deregisterOperator`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[`\0a\x19\xA2\x86\x83`\0a\"fV[\x90Pa\x19\xAE\x82\x82a\x1E\xEEV[PPP\x80\x80a\x19\xBC\x90a6CV[\x91PPa\x18\xE8V[PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AL\x91\x90a5cV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1A|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x80V[\x81a\x1A\x98\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x1A\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[a\x06\x19\x83\x83a%}V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1A\xE5Wa\x1A\xE5a5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\r_\x81\x85a)\xC0V[`\xFF\x81\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x1B]\x91a6\xD0V[\x81T\x81\x10a\x1BmWa\x1Bma5\xFCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`\0a\x12\xF2\x84\x84\x84a+:V[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B\xCDWa\x1B\xCDa5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1C$\x81\x86a)\xC0V[`@\x01Q\x95\x94PPPPPV[`\0a\x12\xF5\x83\x83a,\xA0V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1C\x96W\x91Pa\rL\x90PV[`\0\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1C\xBD`\x01\x84a6\xD0V[\x81T\x81\x10a\x1C\xCDWa\x1C\xCDa5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\rL\x91PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1DI\x85\x85\x85a+:V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1D_Wa\x1D_a5\xFCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1D\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a6^V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x1EIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x01a\x06\x98V[a\x1ES\x83\x82a%}V[a\x1E]\x83\x83a%\x14V[PP`\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\x1F\x12\x90\x84a6\xD0V[\x81T\x81\x10a\x1F\"Wa\x1F\"a5\xFCV[\x90`\0R` `\0 \x01\x90P\x83`\0\x14\x15a\x1FQWT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\rL\x90PV[\x80T`\0\x90a\x1Fp\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a$\xE6V[\x82T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\x1F\xADW\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua _V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[`\0\x80`\0\x80a \x87\x86`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\xFF\x87\x16`\0\x90\x81R`\x04` \x81\x90R`@\x80\x83 \x90Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a \xFC\x92\x8C\x92\x01a6\xFDV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!A\x91\x90\x81\x01\x90a7\\V[\x90P`\0[\x83\x81\x10\x15a\"2W`\xFF\x89\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a!rWa!ra5\xFCV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a!\xC0Wa!\xC0a5\xFCV[` \x02` \x01\x01Q\x11\x15a\" Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a!\xF7Wa!\xF7a5\xFCV[` \x02` \x01\x01Qa\"\t\x91\x90a7\xECV[a\"\x13\x91\x90a8\x0BV[a\"\x1D\x90\x86a8-V[\x94P[\x80a\"*\x81a6CV[\x91PPa!FV[PPP`\xFF\x86\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x93PP`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80a#*W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua$\x8CV[`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a#Q`\x01\x84a6\xD0V[\x81T\x81\x10a#aWa#aa5\xFCV[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x14\x15a#\x9AW`\0\x93PPPPa\x12\xF5V[\x80TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a#\xD4W\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua$\x8AV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U`\0\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a$\xDC\x82\x85a,\xA0V[\x96\x95PPPPPPV[`\0\x80\x82\x12\x15a%\nWa$\xF9\x82a8XV[a%\x03\x90\x84a8uV[\x90Pa\rLV[a%\x03\x82\x84a8-V[`\xFF\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81Q\x11a%\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R`\0\x80Q` a9\x0C\x839\x81Q\x91R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a&\x05\x83\x83a8\x9DV[\x11\x15a&uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` a9\x0C\x839\x81Q\x91R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\0[\x82\x81\x10\x15a)\xB9W`\0[a&\x8D\x82\x84a8\x9DV[\x81\x10\x15a'nW\x84\x82\x81Q\x81\x10a&\xA6Wa&\xA6a5\xFCV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a&\xE5Wa&\xE5a5\xFCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a'\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a9\x0C\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[\x80a'f\x81a6CV[\x91PPa&\x83V[P`\0\x84\x82\x81Q\x81\x10a'\x83Wa'\x83a5\xFCV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a(\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` a9\x0C\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a(.Wa(.a5\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a(\x93Wa(\x93a5\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a)\nWa)\na5\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a)gWa)ga5\xFCV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a)\x85Wa)\x85a5\xFCV[` \x02` \x01\x01Q` \x01Q`@Qa)\x9F\x92\x91\x90a-\x13V[`@Q\x80\x91\x03\x90\xA2\x80a)\xB1\x81a6CV[\x91PPa&xV[PPPPPV[\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a*eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x06\x98V[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a*\x8BWP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a+6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x06\x98V[PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a+\xDBW`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a+\x8E`\x01\x84a6\xD0V[\x81T\x81\x10a+\x9EWa+\x9Ea5\xFCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a+\xC9Wa+\xC0`\x01\x82a6\xD0V[\x92PPPa\x12\xF5V[\x80a+\xD3\x81a8\xB5V[\x91PPa+YV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x06\x98V[`\0a\x12\xF5`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a8\xCCV[\x805`\xFF\x81\x16\x81\x14a,\xC9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xE1W`\0\x80\xFD[a,\xEA\x83a,\xB8V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a-\nW`\0\x80\xFD[a\x12\xF5\x82a,\xB8V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-JW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a-`W`\0\x80\xFD[a-i\x83a,\xB8V[\x91P` \x83\x015a-y\x81a-5V[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a-\x96W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xADW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\"_W`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a-\xE0W`\0\x80\xFD[a-\xE9\x86a,\xB8V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a.\x05W`\0\x80\xFD[a.\x11\x89\x83\x8A\x01a-\x84V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a.*W`\0\x80\xFD[Pa.7\x88\x82\x89\x01a-\x84V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a.ZW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a.qW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\"_W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a.\x9FW`\0\x80\xFD[\x845a.\xAA\x81a-5V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xCCW`\0\x80\xFD[a.\xD8\x87\x82\x88\x01a.HV[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a/\x1DW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a.\xF8V[P\x94\x95\x94PPPPPV[`@\x81R`\0a/;`@\x83\x01\x85a.\xE4V[\x82\x81\x03` \x84\x01Ra _\x81\x85a.\xE4V[`\0\x80`@\x83\x85\x03\x12\x15a/`W`\0\x80\xFD[\x825\x91Pa/p` \x84\x01a,\xB8V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/\xE5Wa/\xD2\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a/\x95V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a0)Wa0)a/\xF1V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a0WWa0Wa/\xF1V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a0xWa0xa/\xF1V[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a0\x95W`\0\x80\xFD[a0\x9E\x83a,\xB8V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xBAW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a0\xCBW`\0\x80\xFD[\x805a0\xDEa0\xD9\x82a0_V[a0/V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15a0\xFDW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a1\x1BW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a1\x02V[\x80\x95PPPPPP\x92P\x92\x90PV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a,\xC9W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a1VW`\0\x80\xFD[\x835\x92Pa1f` \x85\x01a,\xB8V[\x91Pa1t`@\x85\x01a1*V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a1\x90W`\0\x80\xFD[a,\xEA\x83a1*V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\xC9W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xC2W`\0\x80\xFD[a1\xCB\x84a1\x99V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xE6W`\0\x80\xFD[a1\xF2\x86\x82\x87\x01a.HV[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/\xE5W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a2\x1BV[`\0\x80`\0``\x84\x86\x03\x12\x15a2RW`\0\x80\xFD[a2[\x84a,\xB8V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\rLV[`\0\x80`@\x83\x85\x03\x12\x15a2\xB8W`\0\x80\xFD[a2\xC1\x83a,\xB8V[\x91Pa/p` \x84\x01a1*V[`\0\x80`\0`@\x84\x86\x03\x12\x15a2\xE4W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xE6W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a3\x12W`\0\x80\xFD[\x815` a3\"a0\xD9\x83a0_V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a3AW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a3\x90W`@\x81\x89\x03\x12\x15a3^W`\0\x80\x81\xFD[a3fa0\x07V[\x815a3q\x81a-5V[\x81Ra3~\x82\x86\x01a1*V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a3EV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a3\xAEW`\0\x80\xFD[a3\xB7\x83a,\xB8V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xD2W`\0\x80\xFD[a3\xDE\x85\x82\x86\x01a3\x01V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xFDW`\0\x80\xFD[a4\x06\x84a,\xB8V[\x92Pa4\x14` \x85\x01a1\x99V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a49W`\0\x80\xFD[\x835\x92Pa4I` \x85\x01a,\xB8V[\x91Pa1t`@\x85\x01a1\x99V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a4mW`\0\x80\xFD[a4v\x85a,\xB8V[\x93Pa4\x84` \x86\x01a1\x99V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a4\xACW`\0\x80\xFD[a2\xC1\x83a1*V[`\0\x80`\0``\x84\x86\x03\x12\x15a4\xCAW`\0\x80\xFD[a4\xD3\x84a,\xB8V[\x92Pa4\xE1` \x85\x01a1*V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\xFCW`\0\x80\xFD[a5\x08\x86\x82\x87\x01a3\x01V[\x91PP\x92P\x92P\x92V[` \x80\x82R`1\x90\x82\x01R\x7FStakeRegistry.quorumExists: quor`@\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a5uW`\0\x80\xFD[\x81Qa\x12\xF5\x81a-5V[` \x80\x82R`V\x90\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`@\x82\x01R\x7Fer: caller is not the owner of t``\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a6$W`\0\x80\xFD[a\x12\xF5\x82a1*V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a6WWa6Wa6-V[P`\x01\x01\x90V[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a6\xE2Wa6\xE2a6-V[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0`@\x82\x01`\x01\x80`\xA0\x1B\x03\x80\x86\x16\x84R` `@\x81\x86\x01R\x82\x86T\x80\x85R``\x87\x01\x91P\x87`\0R\x82`\0 \x94P`\0[\x81\x81\x10\x15a7NW\x85T\x85\x16\x83R`\x01\x95\x86\x01\x95\x92\x84\x01\x92\x01a70V[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a7oW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x85W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a7\x96W`\0\x80\xFD[\x80Qa7\xA4a0\xD9\x82a0_V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a7\xC3W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a7\xE1W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a7\xC8V[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a8\x06Wa8\x06a6-V[P\x02\x90V[`\0\x82a8(WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a8OWa8Oa6-V[\x01\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x14\x15a8nWa8na6-V[P`\0\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a8\x95Wa8\x95a6-V[\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a8\xB0Wa8\xB0a6-V[P\x01\x90V[`\0\x81a8\xC4Wa8\xC4a6-V[P`\0\x19\x01\x90V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a8\xEAWa8\xEAa6-V[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a9\x05Wa9\x05a6-V[PP\x03\x90V\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \xD1\xC1`\xB6mq\xA1\xA41G1F\xB2\x860;i\xB5\x16si\xC9G[m\xA78\x0E\x18\xC6\xB5odsolcC\0\x08\x0C\x003",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x608060405234801561001057600080fd5b50600436106102115760003560e01c806381c0750211610125578063c8294c56116100ad578063f2be94ae1161007c578063f2be94ae146105b0578063f509551a146105c3578063f851e198146105d6578063fa28c627146105e9578063ff694a77146105fc57600080fd5b8063c8294c561461053b578063d5eccc051461054e578063dd9846b914610561578063df5cf7231461058957600080fd5b8063b6904b78116100f4578063b6904b78146104c6578063bc9a40c3146104d9578063bd29b8cd146104ec578063c46778a5146104ff578063c601527d1461052857600080fd5b806381c07502146104335780639f3ccf6514610453578063ac6bfb0314610466578063adc804da1461048657600080fd5b80634bd26e09116101a857806366acfefe1161017757806366acfefe146103895780636d14a987146103b457806374454c6d146103f35780637c172347146104065780637f4298221461042057600080fd5b80634bd26e09146103245780635401ed27146103545780635e5a6775146103675780635f1f2d771461037657600080fd5b806320b66298116101e457806320b66298146102ad57806325504777146102c05780632cd95940146102e15780633ca5a5f51461030157600080fd5b80630390a4d5146102165780630491b41c1461022b57806308732461146102615780631f9b74e014610282575b600080fd5b610229610224366004612cce565b61060f565b005b61024e610239366004612cf8565b60ff1660009081526001602052604090205490565b6040519081526020015b60405180910390f35b61027461026f366004612cce565b61061e565b604051610258929190612d13565b610295610290366004612d4d565b610667565b6040516001600160601b039091168152602001610258565b6102296102bb366004612dc8565b6106b7565b6102d36102ce366004612e89565b6109e8565b604051610258929190612f28565b6102f46102ef366004612f4d565b610cb3565b6040516102589190612f79565b61024e61030f366004612cf8565b60ff1660009081526003602052604090205490565b61024e610332366004612f4d565b600091825260026020908152604080842060ff93909316845291905290205490565b610295610362366004612f4d565b610d52565b61024e670de0b6b3a764000081565b610229610384366004613082565b610d6b565b61039c610397366004612e89565b61118b565b6040516001600160c01b039091168152602001610258565b6103db7f000000000000000000000000000000000000000000000000000000000000000081565b6040516001600160a01b039091168152602001610258565b61024e610401366004613141565b6112e5565b61040e602081565b60405160ff9091168152602001610258565b61029561042e36600461317d565b6112fc565b6104466104413660046131ad565b611308565b60405161025891906131ff565b6103db610461366004612cce565b6115d0565b61047961047436600461323d565b611608565b6040516102589190613270565b610499610494366004612cce565b6116a0565b6040805182516001600160a01b031681526020928301516001600160601b03169281019290925201610258565b6104796104d4366004612cce565b61171a565b6102296104e73660046132a5565b6117a9565b6102296104fa3660046132cf565b61189d565b61029561050d366004612cf8565b6000602081905290815260409020546001600160601b031681565b61022961053636600461339b565b6119ca565b6102956105493660046133e8565b611abe565b61029561055c366004612cf8565b611b3c565b61057461056f366004613424565b611b8f565b60405163ffffffff9091168152602001610258565b6103db7f000000000000000000000000000000000000000000000000000000000000000081565b6102956105be366004613457565b611b9c565b61024e6105d1366004613499565b611c31565b6104796105e4366004612f4d565b611c3d565b6102956105f7366004613424565b611d22565b61022961060a3660046134b5565b611d83565b6106198282611eee565b505050565b6003602052816000526040600020818154811061063a57600080fd5b6000918252602090912001546001600160a01b0381169250600160a01b90046001600160601b0316905082565b60ff821660009081526001602052604081205483906106a15760405162461bcd60e51b815260040161069890613512565b60405180910390fd5b60006106ad8585612068565b5095945050505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610715573d6000803e3d6000fd5b505050506040513d601f19601f820116820180604052508101906107399190613563565b6001600160a01b0316336001600160a01b0316146107695760405162461bcd60e51b815260040161069890613580565b846107858160ff16600090815260016020526040902054151590565b6107a15760405162461bcd60e51b815260040161069890613512565b8380610817576040805162461bcd60e51b81526020600482015260248101919091527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a206e6f20737472617465677920696e64696365732070726f76696465646064820152608401610698565b82811461088c5760405162461bcd60e51b815260206004820152603960248201527f5374616b6552656769737472792e6d6f6469667953747261746567795061726160448201527f6d733a20696e707574206c656e677468206d69736d61746368000000000000006064820152608401610698565b60ff87166000908152600360205260408120905b828110156109dd578585828181106108ba576108ba6135fc565b90506020020160208101906108cf9190613612565b828989848181106108e2576108e26135fc565b90506020020135815481106108f9576108f96135fc565b9060005260206000200160000160146101000a8154816001600160601b0302191690836001600160601b031602179055508860ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75838a8a85818110610962576109626135fc565b9050602002013581548110610979576109796135fc565b6000918252602090912001546001600160a01b03168888858181106109a0576109a06135fc565b90506020020160208101906109b59190613612565b6040516109c3929190612d13565b60405180910390a2806109d581613643565b9150506108a0565b505050505050505050565b606080336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614610a335760405162461bcd60e51b81526004016106989061365e565b6000836001600160401b03811115610a4d57610a4d612ff1565b604051908082528060200260200182016040528015610a76578160200160208202803683370190505b5090506000846001600160401b03811115610a9357610a93612ff1565b604051908082528060200260200182016040528015610abc578160200160208202803683370190505b50905060005b85811015610ca5576000878783818110610ade57610ade6135fc565b919091013560f81c60008181526001602052604090205490925015159050610b665760405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a206044820152741c5d5bdc9d5b48191bd95cc81b9bdd08195e1a5cdd605a1b6064820152608401610698565b600080610b73838d612068565b9150915080610c105760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e72656769737465724f70657261746f723a2060448201527f4f70657261746f7220646f6573206e6f74206d656574206d696e696d756d207360648201527f74616b6520726571756972656d656e7420666f722071756f72756d0000000000608482015260a401610698565b6000610c1d8c8585612266565b905082878681518110610c3257610c326135fc565b60200260200101906001600160601b031690816001600160601b031681525050610c5c8482611eee565b868681518110610c6e57610c6e6135fc565b60200260200101906001600160601b031690816001600160601b031681525050505050508080610c9d90613643565b915050610ac2565b509097909650945050505050565b600082815260026020908152604080832060ff851684528252808320805482518185028101850190935280835260609492939192909184015b82821015610d45576000848152602090819020604080516060810182529185015463ffffffff8082168452600160201b82041683850152600160401b90046001600160601b031690820152825260019092019101610cec565b5050505090505b92915050565b600080610d5f8484611c3d565b60400151949350505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015610dc9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610ded9190613563565b6001600160a01b0316336001600160a01b031614610e1d5760405162461bcd60e51b815260040161069890613580565b81610e398160ff16600090815260016020526040902054151590565b610e555760405162461bcd60e51b815260040161069890613512565b815180610eca5760405162461bcd60e51b815260206004820152603d60248201527f5374616b6552656769737472792e72656d6f7665537472617465676965733a2060448201527f6e6f20696e646963657320746f2072656d6f76652070726f76696465640000006064820152608401610698565b60ff841660009081526003602090815260408083206004909252822090915b83811015611182578660ff167f31fa2e2cd280c9375e13ffcf3d81e2378100186e4058f8d3ddb690b82dcd31f784888481518110610f2957610f296135fc565b602002602001015181548110610f4157610f416135fc565b600091825260209182902001546040516001600160a01b0390911681520160405180910390a28660ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a7584888481518110610f9f57610f9f6135fc565b602002602001015181548110610fb757610fb76135fc565b600091825260208083209190910154604080516001600160a01b039092168252918101929092520160405180910390a282548390610ff7906001906136d0565b81548110611007576110076135fc565b9060005260206000200183878381518110611024576110246135fc565b60200260200101518154811061103c5761103c6135fc565b600091825260209091208254910180546001600160a01b0319166001600160a01b03909216918217815591546001600160601b03600160a01b918290041602179055825483908061108f5761108f6136e7565b600082815260208120820160001990810191909155019055815482906110b7906001906136d0565b815481106110c7576110c76135fc565b9060005260206000200160009054906101000a90046001600160a01b0316828783815181106110f8576110f86135fc565b602002602001015181548110611110576111106135fc565b9060005260206000200160006101000a8154816001600160a01b0302191690836001600160a01b031602179055508180548061114e5761114e6136e7565b600082815260209020810160001990810180546001600160a01b03191690550190558061117a81613643565b915050610ee9565b50505050505050565b6000336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146111d55760405162461bcd60e51b81526004016106989061365e565b6000805b838110156106ad5760008585838181106111f5576111f56135fc565b919091013560f81c600081815260016020526040902054909250151590506112855760405162461bcd60e51b815260206004820152603860248201527f5374616b6552656769737472792e7570646174654f70657261746f725374616b60448201527f653a2071756f72756d20646f6573206e6f7420657869737400000000000000006064820152608401610698565b600080611292838b612068565b91509150806112b45760009150600160ff84161b6001600160c01b0386161794505b60006112c18a8585612266565b90506112cd8482611eee565b505050505080806112dd90613643565b9150506111d9565b60006112f2848484612266565b90505b9392505050565b60006112f583836124e6565b60606000826001600160401b0381111561132457611324612ff1565b60405190808252806020026020018201604052801561134d578160200160208202803683370190505b50905060005b838110156115c757600085858381811061136f5761136f6135fc565b919091013560f81c6000818152600160205260409020549092501515905061140e5760405162461bcd60e51b815260206004820152604660248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20646f6573206e6f7460648201526508195e1a5cdd60d21b608482015260a401610698565b60ff81166000908152600160205260408120805463ffffffff8a169290611437576114376135fc565b60009182526020909120015463ffffffff1611156114e35760405162461bcd60e51b815260206004820152605b60248201527f5374616b6552656769737472792e676574546f74616c5374616b65496e64696360448201527f65734174426c6f636b4e756d6265723a2071756f72756d20686173206e6f207360648201527f74616b6520686973746f727920617420626c6f636b4e756d6265720000000000608482015260a401610698565b60ff8116600090815260016020526040812054905b818110156115b15760ff8316600090815260016020819052604090912063ffffffff8b169161152784866136d0565b61153191906136d0565b81548110611541576115416135fc565b60009182526020909120015463ffffffff161161159f57600161156482846136d0565b61156e91906136d0565b858581518110611580576115806135fc565b602002602001019063ffffffff16908163ffffffff16815250506115b1565b806115a981613643565b9150506114f8565b50505080806115bf90613643565b915050611353565b50949350505050565b600460205281600052604060002081815481106115ec57600080fd5b6000918252602090912001546001600160a01b03169150829050565b60408051606081018252600080825260208083018290528284018290528582526002815283822060ff8816835290529190912080548390811061164d5761164d6135fc565b600091825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b031691810191909152949350505050565b604080518082019091526000808252602082015260ff831660009081526003602052604090208054839081106116d8576116d86135fc565b6000918252602091829020604080518082019091529101546001600160a01b0381168252600160a01b90046001600160601b0316918101919091529392505050565b604080516060810182526000808252602080830182905282840182905260ff861682526001905291909120805483908110611757576117576135fc565b600091825260209182902060408051606081018252929091015463ffffffff8082168452600160201b82041693830193909352600160401b9092046001600160601b0316918101919091529392505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611807573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061182b9190613563565b6001600160a01b0316336001600160a01b03161461185b5760405162461bcd60e51b815260040161069890613580565b816118778160ff16600090815260016020526040902054151590565b6118935760405162461bcd60e51b815260040161069890613512565b6106198383612514565b336001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016146118e55760405162461bcd60e51b81526004016106989061365e565b60005b818110156119c4576000838383818110611904576119046135fc565b919091013560f81c600081815260016020526040902054909250151590506119945760405162461bcd60e51b815260206004820152603760248201527f5374616b6552656769737472792e646572656769737465724f70657261746f7260448201527f3a2071756f72756d20646f6573206e6f742065786973740000000000000000006064820152608401610698565b60006119a286836000612266565b90506119ae8282611eee565b50505080806119bc90613643565b9150506118e8565b50505050565b7f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316638da5cb5b6040518163ffffffff1660e01b8152600401602060405180830381865afa158015611a28573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190611a4c9190613563565b6001600160a01b0316336001600160a01b031614611a7c5760405162461bcd60e51b815260040161069890613580565b81611a988160ff16600090815260016020526040902054151590565b611ab45760405162461bcd60e51b815260040161069890613512565b610619838361257d565b60ff83166000908152600160205260408120805482919084908110611ae557611ae56135fc565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050610d5f81856129c0565b60ff81166000908152600160208190526040822080549091611b5d916136d0565b81548110611b6d57611b6d6135fc565b600091825260209091200154600160401b90046001600160601b031692915050565b60006112f2848484612b3a565b600082815260026020908152604080832060ff881684529091528120805482919084908110611bcd57611bcd6135fc565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529050611c2481866129c0565b6040015195945050505050565b60006112f58383612ca0565b6040805160608082018352600080835260208084018290528385018290528682526002815284822060ff87168352815284822054855193840186528284529083018290529382015290919081611c96579150610d4c9050565b600085815260026020908152604080832060ff881684529091529020611cbd6001846136d0565b81548110611ccd57611ccd6135fc565b600091825260209182902060408051606081018252919092015463ffffffff8082168352600160201b820416938201939093526001600160601b03600160401b90930492909216908201529250610d4c915050565b600083815260026020908152604080832060ff861684529091528120611d49858585612b3a565b63ffffffff1681548110611d5f57611d5f6135fc565b600091825260209091200154600160401b90046001600160601b0316949350505050565b336001600160a01b037f00000000000000000000000000000000000000000000000000000000000000001614611dcb5760405162461bcd60e51b81526004016106989061365e565b60ff831660009081526001602052604090205415611e495760405162461bcd60e51b815260206004820152603560248201527f5374616b6552656769737472792e696e697469616c697a6551756f72756d3a2060448201527471756f72756d20616c72656164792065786973747360581b6064820152608401610698565b611e53838261257d565b611e5d8383612514565b505060ff166000908152600160208181526040808420815160608101835263ffffffff438116825281850187815293820187815283549687018455928752939095209451949093018054915193516001600160601b0316600160401b02600160401b600160a01b0319948416600160201b0267ffffffffffffffff1990931695909316949094171791909116179055565b60ff821660009081526001602081905260408220805491839190611f1290846136d0565b81548110611f2257611f226135fc565b9060005260206000200190508360001415611f515754600160401b90046001600160601b03169150610d4c9050565b8054600090611f7090600160401b90046001600160601b0316866124e6565b82549091504363ffffffff90811691161415611fad578154600160401b600160a01b031916600160401b6001600160601b0383160217825561205f565b815463ffffffff438116600160201b81810267ffffffff000000001990941693909317855560ff8916600090815260016020818152604080842081516060810183529586528583018581526001600160601b03808b169388019384528254958601835591865292909420945194909201805491519251909316600160401b02600160401b600160a01b031992861690960267ffffffffffffffff19909116939094169290921792909217169190911790555b95945050505050565b6000806000806120878660ff1660009081526003602052604090205490565b604080518082019091526000808252602082015290915060ff871660009081526004602081905260408083209051639004134760e01b81526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016926390041347926120fc928c92016136fd565b600060405180830381865afa158015612119573d6000803e3d6000fd5b505050506040513d6000823e601f3d908101601f19168201604052612141919081019061375c565b905060005b838110156122325760ff89166000908152600360205260409020805482908110612172576121726135fc565b60009182526020808320604080518082019091529201546001600160a01b0381168352600160a01b90046001600160601b03169082015283519094508390839081106121c0576121c06135fc565b6020026020010151111561222057670de0b6b3a764000083602001516001600160601b03168383815181106121f7576121f76135fc565b602002602001015161220991906137ec565b612213919061380b565b61221d908661382d565b94505b8061222a81613643565b915050612146565b50505060ff8616600090815260208190526040902054919350506001600160601b03908116908316101590505b9250929050565b600083815260026020908152604080832060ff8616845290915281205481908061232a57600086815260026020908152604080832060ff891684528252808320815160608101835263ffffffff43811682528185018681526001600160601b03808c16958401958652845460018101865594885295909620915191909201805495519351909416600160401b02600160401b600160a01b0319938316600160201b0267ffffffffffffffff199096169190921617939093171691909117905561248c565b600086815260026020908152604080832060ff8916845290915281206123516001846136d0565b81548110612361576123616135fc565b600091825260209091200180546001600160601b03600160401b90910481169450909150851683141561239a57600093505050506112f5565b80544363ffffffff908116911614156123d4578054600160401b600160a01b031916600160401b6001600160601b0387160217815561248a565b805467ffffffff000000001916600160201b4363ffffffff90811682810293909317845560008a815260026020908152604080832060ff8d168452825280832081516060810183529687528683018481526001600160601b038d81169389019384528254600181018455928652939094209651960180549351915196851667ffffffffffffffff1990941693909317931690930291909117600160401b600160a01b031916600160401b93909216929092021790555b505b6040805160ff871681526001600160601b038616602082015287917f2f527d527e95d8fe40aec55377743bb779087da3f6d0d08f12e36444da62327d910160405180910390a26124dc8285612ca0565b9695505050505050565b60008082121561250a576124f982613858565b6125039084613875565b9050610d4c565b612503828461382d565b60ff82166000818152602081815260409182902080546bffffffffffffffffffffffff19166001600160601b03861690811790915591519182527f26eecff2b70b0a71104ff4d940ba7162d23a95c248771fc487a7be17a596b3cf910160405180910390a25050565b60008151116125e25760405162461bcd60e51b8152602060048201526038602482015260008051602061390c83398151915260448201527f3a206e6f20737472617465676965732070726f766964656400000000000000006064820152608401610698565b805160ff831660009081526003602090815260409091205490612605838361389d565b11156126755760405162461bcd60e51b8152602060048201526045602482015260008051602061390c83398151915260448201527f3a20657863656564204d41585f5745494748494e475f46554e4354494f4e5f4c60648201526408a9c8ea8960db1b608482015260a401610698565b60005b828110156129b95760005b61268d828461389d565b81101561276e578482815181106126a6576126a66135fc565b6020026020010151600001516001600160a01b0316600360008860ff1660ff16815260200190815260200160002082815481106126e5576126e56135fc565b6000918252602090912001546001600160a01b0316141561275c5760405162461bcd60e51b815260206004820152603d602482015260008051602061390c83398151915260448201527f3a2063616e6e6f74206164642073616d652073747261746567792032780000006064820152608401610698565b8061276681613643565b915050612683565b506000848281518110612783576127836135fc565b6020026020010151602001516001600160601b0316116128085760405162461bcd60e51b8152602060048201526046602482015260008051602061390c83398151915260448201527f3a2063616e6e6f74206164642073747261746567792077697468207a65726f206064820152651dd95a59da1d60d21b608482015260a401610698565b60ff85166000908152600360205260409020845185908390811061282e5761282e6135fc565b602090810291909101810151825460018101845560009384528284208251928401516001600160601b0316600160a01b026001600160a01b039093169290921791015560ff8716825260049052604090208451859083908110612893576128936135fc565b6020908102919091018101515182546001810184556000938452919092200180546001600160a01b0319166001600160a01b03909216919091179055835160ff8616907f10565e56cacbf32eca267945f054fec02e59750032d113d3302182ad967f54049086908490811061290a5761290a6135fc565b602090810291909101810151516040516001600160a01b0390911681520160405180910390a28460ff167f11a5641322da1dff56a4b66eaac31ffa465295ece907cd163437793b4d009a75858381518110612967576129676135fc565b602002602001015160000151868481518110612985576129856135fc565b60200260200101516020015160405161299f929190612d13565b60405180910390a2806129b181613643565b915050612678565b5050505050565b816000015163ffffffff168163ffffffff161015612a655760405162461bcd60e51b815260206004820152605660248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a207374616b6555706461746520697320606482015275333937b69030b33a32b910313637b1b5a73ab6b132b960511b608482015260a401610698565b602082015163ffffffff161580612a8b5750816020015163ffffffff168163ffffffff16105b612b365760405162461bcd60e51b815260206004820152606a60248201527f5374616b6552656769737472792e5f76616c69646174655374616b655570646160448201527f74654174426c6f636b4e756d6265723a2074686572652069732061206e65776560648201527f72207374616b6555706461746520617661696c61626c65206265666f726520626084820152693637b1b5a73ab6b132b960b11b60a482015260c401610698565b5050565b600083815260026020908152604080832060ff86168452909152812054805b8015612bdb57600086815260026020908152604080832060ff89168452909152902063ffffffff851690612b8e6001846136d0565b81548110612b9e57612b9e6135fc565b60009182526020909120015463ffffffff1611612bc957612bc06001826136d0565b925050506112f5565b80612bd3816138b5565b915050612b59565b5060405162461bcd60e51b815260206004820152608160248201527f5374616b6552656769737472792e5f6765745374616b65557064617465496e6460448201527f6578466f724f70657261746f724174426c6f636b4e756d6265723a206e6f207360648201527f74616b652075706461746520666f756e6420666f72206f70657261746f72496460848201527f20616e642071756f72756d4e756d62657220617420626c6f636b206e756d626560a4820152603960f91b60c482015260e401610698565b60006112f56001600160601b038085169084166138cc565b803560ff81168114612cc957600080fd5b919050565b60008060408385031215612ce157600080fd5b612cea83612cb8565b946020939093013593505050565b600060208284031215612d0a57600080fd5b6112f582612cb8565b6001600160a01b039290921682526001600160601b0316602082015260400190565b6001600160a01b0381168114612d4a57600080fd5b50565b60008060408385031215612d6057600080fd5b612d6983612cb8565b91506020830135612d7981612d35565b809150509250929050565b60008083601f840112612d9657600080fd5b5081356001600160401b03811115612dad57600080fd5b6020830191508360208260051b850101111561225f57600080fd5b600080600080600060608688031215612de057600080fd5b612de986612cb8565b945060208601356001600160401b0380821115612e0557600080fd5b612e1189838a01612d84565b90965094506040880135915080821115612e2a57600080fd5b50612e3788828901612d84565b969995985093965092949392505050565b60008083601f840112612e5a57600080fd5b5081356001600160401b03811115612e7157600080fd5b60208301915083602082850101111561225f57600080fd5b60008060008060608587031215612e9f57600080fd5b8435612eaa81612d35565b93506020850135925060408501356001600160401b03811115612ecc57600080fd5b612ed887828801612e48565b95989497509550505050565b600081518084526020808501945080840160005b83811015612f1d5781516001600160601b031687529582019590820190600101612ef8565b509495945050505050565b604081526000612f3b6040830185612ee4565b828103602084015261205f8185612ee4565b60008060408385031215612f6057600080fd5b82359150612f7060208401612cb8565b90509250929050565b6020808252825182820181905260009190848201906040850190845b81811015612fe557612fd283855163ffffffff808251168352806020830151166020840152506001600160601b0360408201511660408301525050565b9284019260609290920191600101612f95565b50909695505050505050565b634e487b7160e01b600052604160045260246000fd5b604080519081016001600160401b038111828210171561302957613029612ff1565b60405290565b604051601f8201601f191681016001600160401b038111828210171561305757613057612ff1565b604052919050565b60006001600160401b0382111561307857613078612ff1565b5060051b60200190565b6000806040838503121561309557600080fd5b61309e83612cb8565b91506020808401356001600160401b038111156130ba57600080fd5b8401601f810186136130cb57600080fd5b80356130de6130d98261305f565b61302f565b81815260059190911b820183019083810190888311156130fd57600080fd5b928401925b8284101561311b57833582529284019290840190613102565b80955050505050509250929050565b80356001600160601b0381168114612cc957600080fd5b60008060006060848603121561315657600080fd5b8335925061316660208501612cb8565b91506131746040850161312a565b90509250925092565b6000806040838503121561319057600080fd5b612cea8361312a565b803563ffffffff81168114612cc957600080fd5b6000806000604084860312156131c257600080fd5b6131cb84613199565b925060208401356001600160401b038111156131e657600080fd5b6131f286828701612e48565b9497909650939450505050565b6020808252825182820181905260009190848201906040850190845b81811015612fe557835163ffffffff168352928401929184019160010161321b565b60008060006060848603121561325257600080fd5b61325b84612cb8565b95602085013595506040909401359392505050565b815163ffffffff9081168252602080840151909116908201526040808301516001600160601b03169082015260608101610d4c565b600080604083850312156132b857600080fd5b6132c183612cb8565b9150612f706020840161312a565b6000806000604084860312156132e457600080fd5b8335925060208401356001600160401b038111156131e657600080fd5b600082601f83011261331257600080fd5b813560206133226130d98361305f565b82815260069290921b8401810191818101908684111561334157600080fd5b8286015b84811015613390576040818903121561335e5760008081fd5b613366613007565b813561337181612d35565b815261337e82860161312a565b81860152835291830191604001613345565b509695505050505050565b600080604083850312156133ae57600080fd5b6133b783612cb8565b915060208301356001600160401b038111156133d257600080fd5b6133de85828601613301565b9150509250929050565b6000806000606084860312156133fd57600080fd5b61340684612cb8565b925061341460208501613199565b9150604084013590509250925092565b60008060006060848603121561343957600080fd5b8335925061344960208501612cb8565b915061317460408501613199565b6000806000806080858703121561346d57600080fd5b61347685612cb8565b935061348460208601613199565b93969395505050506040820135916060013590565b600080604083850312156134ac57600080fd5b6132c18361312a565b6000806000606084860312156134ca57600080fd5b6134d384612cb8565b92506134e16020850161312a565b915060408401356001600160401b038111156134fc57600080fd5b61350886828701613301565b9150509250925092565b60208082526031908201527f5374616b6552656769737472792e71756f72756d4578697374733a2071756f726040820152701d5b48191bd95cc81b9bdd08195e1a5cdd607a1b606082015260800190565b60006020828403121561357557600080fd5b81516112f581612d35565b60208082526056908201527f5374616b6552656769737472792e6f6e6c79436f6f7264696e61746f724f776e60408201527f65723a2063616c6c6572206973206e6f7420746865206f776e6572206f6620746060820152753432903932b3b4b9ba393ca1b7b7b93234b730ba37b960511b608082015260a00190565b634e487b7160e01b600052603260045260246000fd5b60006020828403121561362457600080fd5b6112f58261312a565b634e487b7160e01b600052601160045260246000fd5b60006000198214156136575761365761362d565b5060010190565b6020808252604c908201527f5374616b6552656769737472792e6f6e6c795265676973747279436f6f72646960408201527f6e61746f723a2063616c6c6572206973206e6f7420746865205265676973747260608201526b3ca1b7b7b93234b730ba37b960a11b608082015260a00190565b6000828210156136e2576136e261362d565b500390565b634e487b7160e01b600052603160045260246000fd5b60006040820160018060a01b03808616845260206040818601528286548085526060870191508760005282600020945060005b8181101561374e578554851683526001958601959284019201613730565b509098975050505050505050565b6000602080838503121561376f57600080fd5b82516001600160401b0381111561378557600080fd5b8301601f8101851361379657600080fd5b80516137a46130d98261305f565b81815260059190911b820183019083810190878311156137c357600080fd5b928401925b828410156137e1578351825292840192908401906137c8565b979650505050505050565b60008160001904831182151516156138065761380661362d565b500290565b60008261382857634e487b7160e01b600052601260045260246000fd5b500490565b60006001600160601b0380831681851680830382111561384f5761384f61362d565b01949350505050565b6000600160ff1b82141561386e5761386e61362d565b5060000390565b60006001600160601b03838116908316818110156138955761389561362d565b039392505050565b600082198211156138b0576138b061362d565b500190565b6000816138c4576138c461362d565b506000190190565b60008083128015600160ff1b8501841216156138ea576138ea61362d565b6001600160ff1b03840183138116156139055761390561362d565b5050039056fe5374616b6552656769737472792e5f6164645374726174656779506172616d73a2646970667358221220d1c160b66d71a1a431473146b286303b69b5167369c9475b6da7380e18c6b56f64736f6c634300080c0033
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R4\x80\x15a\0\x10W`\0\x80\xFD[P`\x046\x10a\x02\x11W`\x005`\xE0\x1C\x80c\x81\xC0u\x02\x11a\x01%W\x80c\xC8)LV\x11a\0\xADW\x80c\xF2\xBE\x94\xAE\x11a\0|W\x80c\xF2\xBE\x94\xAE\x14a\x05\xB0W\x80c\xF5\tU\x1A\x14a\x05\xC3W\x80c\xF8Q\xE1\x98\x14a\x05\xD6W\x80c\xFA(\xC6'\x14a\x05\xE9W\x80c\xFFiJw\x14a\x05\xFCW`\0\x80\xFD[\x80c\xC8)LV\x14a\x05;W\x80c\xD5\xEC\xCC\x05\x14a\x05NW\x80c\xDD\x98F\xB9\x14a\x05aW\x80c\xDF\\\xF7#\x14a\x05\x89W`\0\x80\xFD[\x80c\xB6\x90Kx\x11a\0\xF4W\x80c\xB6\x90Kx\x14a\x04\xC6W\x80c\xBC\x9A@\xC3\x14a\x04\xD9W\x80c\xBD)\xB8\xCD\x14a\x04\xECW\x80c\xC4gx\xA5\x14a\x04\xFFW\x80c\xC6\x01R}\x14a\x05(W`\0\x80\xFD[\x80c\x81\xC0u\x02\x14a\x043W\x80c\x9F<\xCFe\x14a\x04SW\x80c\xACk\xFB\x03\x14a\x04fW\x80c\xAD\xC8\x04\xDA\x14a\x04\x86W`\0\x80\xFD[\x80cK\xD2n\t\x11a\x01\xA8W\x80cf\xAC\xFE\xFE\x11a\x01wW\x80cf\xAC\xFE\xFE\x14a\x03\x89W\x80cm\x14\xA9\x87\x14a\x03\xB4W\x80ctELm\x14a\x03\xF3W\x80c|\x17#G\x14a\x04\x06W\x80c\x7FB\x98\"\x14a\x04 W`\0\x80\xFD[\x80cK\xD2n\t\x14a\x03$W\x80cT\x01\xED'\x14a\x03TW\x80c^Zgu\x14a\x03gW\x80c_\x1F-w\x14a\x03vW`\0\x80\xFD[\x80c \xB6b\x98\x11a\x01\xE4W\x80c \xB6b\x98\x14a\x02\xADW\x80c%PGw\x14a\x02\xC0W\x80c,\xD9Y@\x14a\x02\xE1W\x80c<\xA5\xA5\xF5\x14a\x03\x01W`\0\x80\xFD[\x80c\x03\x90\xA4\xD5\x14a\x02\x16W\x80c\x04\x91\xB4\x1C\x14a\x02+W\x80c\x08s$a\x14a\x02aW\x80c\x1F\x9Bt\xE0\x14a\x02\x82W[`\0\x80\xFD[a\x02)a\x02$6`\x04a,\xCEV[a\x06\x0FV[\0[a\x02Na\x0296`\x04a,\xF8V[`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x90V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[a\x02ta\x02o6`\x04a,\xCEV[a\x06\x1EV[`@Qa\x02X\x92\x91\x90a-\x13V[a\x02\x95a\x02\x906`\x04a-MV[a\x06gV[`@Q`\x01`\x01``\x1B\x03\x90\x91\x16\x81R` \x01a\x02XV[a\x02)a\x02\xBB6`\x04a-\xC8V[a\x06\xB7V[a\x02\xD3a\x02\xCE6`\x04a.\x89V[a\t\xE8V[`@Qa\x02X\x92\x91\x90a/(V[a\x02\xF4a\x02\xEF6`\x04a/MV[a\x0C\xB3V[`@Qa\x02X\x91\x90a/yV[a\x02Na\x03\x0F6`\x04a,\xF8V[`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[a\x02Na\x0326`\x04a/MV[`\0\x91\x82R`\x02` \x90\x81R`@\x80\x84 `\xFF\x93\x90\x93\x16\x84R\x91\x90R\x90 T\x90V[a\x02\x95a\x03b6`\x04a/MV[a\rRV[a\x02Ng\r\xE0\xB6\xB3\xA7d\0\0\x81V[a\x02)a\x03\x846`\x04a0\x82V[a\rkV[a\x03\x9Ca\x03\x976`\x04a.\x89V[a\x11\x8BV[`@Q`\x01`\x01`\xC0\x1B\x03\x90\x91\x16\x81R` \x01a\x02XV[a\x03\xDB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R` \x01a\x02XV[a\x02Na\x04\x016`\x04a1AV[a\x12\xE5V[a\x04\x0E` \x81V[`@Q`\xFF\x90\x91\x16\x81R` \x01a\x02XV[a\x02\x95a\x04.6`\x04a1}V[a\x12\xFCV[a\x04Fa\x04A6`\x04a1\xADV[a\x13\x08V[`@Qa\x02X\x91\x90a1\xFFV[a\x03\xDBa\x04a6`\x04a,\xCEV[a\x15\xD0V[a\x04ya\x04t6`\x04a2=V[a\x16\x08V[`@Qa\x02X\x91\x90a2pV[a\x04\x99a\x04\x946`\x04a,\xCEV[a\x16\xA0V[`@\x80Q\x82Q`\x01`\x01`\xA0\x1B\x03\x16\x81R` \x92\x83\x01Q`\x01`\x01``\x1B\x03\x16\x92\x81\x01\x92\x90\x92R\x01a\x02XV[a\x04ya\x04\xD46`\x04a,\xCEV[a\x17\x1AV[a\x02)a\x04\xE76`\x04a2\xA5V[a\x17\xA9V[a\x02)a\x04\xFA6`\x04a2\xCFV[a\x18\x9DV[a\x02\x95a\x05\r6`\x04a,\xF8V[`\0` \x81\x90R\x90\x81R`@\x90 T`\x01`\x01``\x1B\x03\x16\x81V[a\x02)a\x0566`\x04a3\x9BV[a\x19\xCAV[a\x02\x95a\x05I6`\x04a3\xE8V[a\x1A\xBEV[a\x02\x95a\x05\\6`\x04a,\xF8V[a\x1B<V[a\x05ta\x05o6`\x04a4$V[a\x1B\x8FV[`@Qc\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x02XV[a\x03\xDB\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81V[a\x02\x95a\x05\xBE6`\x04a4WV[a\x1B\x9CV[a\x02Na\x05\xD16`\x04a4\x99V[a\x1C1V[a\x04ya\x05\xE46`\x04a/MV[a\x1C=V[a\x02\x95a\x05\xF76`\x04a4$V[a\x1D\"V[a\x02)a\x06\n6`\x04a4\xB5V[a\x1D\x83V[a\x06\x19\x82\x82a\x1E\xEEV[PPPV[`\x03` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x06:W`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x92P`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90P\x82V[`\xFF\x82\x16`\0\x90\x81R`\x01` R`@\x81 T\x83\x90a\x06\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[`@Q\x80\x91\x03\x90\xFD[`\0a\x06\xAD\x85\x85a hV[P\x95\x94PPPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x07\x15W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x079\x91\x90a5cV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x07iW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x80V[\x84a\x07\x85\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x07\xA1W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[\x83\x80a\x08\x17W`@\x80QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`$\x81\x01\x91\x90\x91R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: no strategy indices provided`d\x82\x01R`\x84\x01a\x06\x98V[\x82\x81\x14a\x08\x8CW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`9`$\x82\x01R\x7FStakeRegistry.modifyStrategyPara`D\x82\x01R\x7Fms: input length mismatch\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[`\xFF\x87\x16`\0\x90\x81R`\x03` R`@\x81 \x90[\x82\x81\x10\x15a\t\xDDW\x85\x85\x82\x81\x81\x10a\x08\xBAWa\x08\xBAa5\xFCV[\x90P` \x02\x01` \x81\x01\x90a\x08\xCF\x91\x90a6\x12V[\x82\x89\x89\x84\x81\x81\x10a\x08\xE2Wa\x08\xE2a5\xFCV[\x90P` \x02\x015\x81T\x81\x10a\x08\xF9Wa\x08\xF9a5\xFCV[\x90`\0R` `\0 \x01`\0\x01`\x14a\x01\0\n\x81T\x81`\x01`\x01``\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01``\x1B\x03\x16\x02\x17\x90UP\x88`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x83\x8A\x8A\x85\x81\x81\x10a\tbWa\tba5\xFCV[\x90P` \x02\x015\x81T\x81\x10a\tyWa\tya5\xFCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x88\x88\x85\x81\x81\x10a\t\xA0Wa\t\xA0a5\xFCV[\x90P` \x02\x01` \x81\x01\x90a\t\xB5\x91\x90a6\x12V[`@Qa\t\xC3\x92\x91\x90a-\x13V[`@Q\x80\x91\x03\x90\xA2\x80a\t\xD5\x81a6CV[\x91PPa\x08\xA0V[PPPPPPPPPV[``\x803`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\n3W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a6^V[`\0\x83`\x01`\x01`@\x1B\x03\x81\x11\x15a\nMWa\nMa/\xF1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\nvW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0\x84`\x01`\x01`@\x1B\x03\x81\x11\x15a\n\x93Wa\n\x93a/\xF1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\n\xBCW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x85\x81\x10\x15a\x0C\xA5W`\0\x87\x87\x83\x81\x81\x10a\n\xDEWa\n\xDEa5\xFCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x0BfW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01Rt\x1C][\xDC\x9D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`Z\x1B`d\x82\x01R`\x84\x01a\x06\x98V[`\0\x80a\x0Bs\x83\x8Da hV[\x91P\x91P\x80a\x0C\x10W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.registerOperator: `D\x82\x01R\x7FOperator does not meet minimum s`d\x82\x01R\x7Ftake requirement for quorum\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\0a\x0C\x1D\x8C\x85\x85a\"fV[\x90P\x82\x87\x86\x81Q\x81\x10a\x0C2Wa\x0C2a5\xFCV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPa\x0C\\\x84\x82a\x1E\xEEV[\x86\x86\x81Q\x81\x10a\x0CnWa\x0Cna5\xFCV[` \x02` \x01\x01\x90`\x01`\x01``\x1B\x03\x16\x90\x81`\x01`\x01``\x1B\x03\x16\x81RPPPPPP\x80\x80a\x0C\x9D\x90a6CV[\x91PPa\n\xC2V[P\x90\x97\x90\x96P\x94PPPPPV[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x85\x16\x84R\x82R\x80\x83 \x80T\x82Q\x81\x85\x02\x81\x01\x85\x01\x90\x93R\x80\x83R``\x94\x92\x93\x91\x92\x90\x91\x84\x01[\x82\x82\x10\x15a\rEW`\0\x84\x81R` \x90\x81\x90 `@\x80Q``\x81\x01\x82R\x91\x85\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x83\x85\x01R`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x82R`\x01\x90\x92\x01\x91\x01a\x0C\xECV[PPPP\x90P[\x92\x91PPV[`\0\x80a\r_\x84\x84a\x1C=V[`@\x01Q\x94\x93PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\r\xC9W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\r\xED\x91\x90a5cV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x0E\x1DW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x80V[\x81a\x0E9\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x0EUW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[\x81Q\x80a\x0E\xCAW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R\x7FStakeRegistry.removeStrategies: `D\x82\x01R\x7Fno indices to remove provided\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[`\xFF\x84\x16`\0\x90\x81R`\x03` \x90\x81R`@\x80\x83 `\x04\x90\x92R\x82 \x90\x91[\x83\x81\x10\x15a\x11\x82W\x86`\xFF\x16\x7F1\xFA.,\xD2\x80\xC97^\x13\xFF\xCF=\x81\xE27\x81\0\x18n@X\xF8\xD3\xDD\xB6\x90\xB8-\xCD1\xF7\x84\x88\x84\x81Q\x81\x10a\x0F)Wa\x0F)a5\xFCV[` \x02` \x01\x01Q\x81T\x81\x10a\x0FAWa\x0FAa5\xFCV[`\0\x91\x82R` \x91\x82\x90 \x01T`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x86`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x84\x88\x84\x81Q\x81\x10a\x0F\x9FWa\x0F\x9Fa5\xFCV[` \x02` \x01\x01Q\x81T\x81\x10a\x0F\xB7Wa\x0F\xB7a5\xFCV[`\0\x91\x82R` \x80\x83 \x91\x90\x91\x01T`@\x80Q`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x82R\x91\x81\x01\x92\x90\x92R\x01`@Q\x80\x91\x03\x90\xA2\x82T\x83\x90a\x0F\xF7\x90`\x01\x90a6\xD0V[\x81T\x81\x10a\x10\x07Wa\x10\x07a5\xFCV[\x90`\0R` `\0 \x01\x83\x87\x83\x81Q\x81\x10a\x10$Wa\x10$a5\xFCV[` \x02` \x01\x01Q\x81T\x81\x10a\x10<Wa\x10<a5\xFCV[`\0\x91\x82R` \x90\x91 \x82T\x91\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x82\x17\x81U\x91T`\x01`\x01``\x1B\x03`\x01`\xA0\x1B\x91\x82\x90\x04\x16\x02\x17\x90U\x82T\x83\x90\x80a\x10\x8FWa\x10\x8Fa6\xE7V[`\0\x82\x81R` \x81 \x82\x01`\0\x19\x90\x81\x01\x91\x90\x91U\x01\x90U\x81T\x82\x90a\x10\xB7\x90`\x01\x90a6\xD0V[\x81T\x81\x10a\x10\xC7Wa\x10\xC7a5\xFCV[\x90`\0R` `\0 \x01`\0\x90T\x90a\x01\0\n\x90\x04`\x01`\x01`\xA0\x1B\x03\x16\x82\x87\x83\x81Q\x81\x10a\x10\xF8Wa\x10\xF8a5\xFCV[` \x02` \x01\x01Q\x81T\x81\x10a\x11\x10Wa\x11\x10a5\xFCV[\x90`\0R` `\0 \x01`\0a\x01\0\n\x81T\x81`\x01`\x01`\xA0\x1B\x03\x02\x19\x16\x90\x83`\x01`\x01`\xA0\x1B\x03\x16\x02\x17\x90UP\x81\x80T\x80a\x11NWa\x11Na6\xE7V[`\0\x82\x81R` \x90 \x81\x01`\0\x19\x90\x81\x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16\x90U\x01\x90U\x80a\x11z\x81a6CV[\x91PPa\x0E\xE9V[PPPPPPPV[`\x003`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x11\xD5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a6^V[`\0\x80[\x83\x81\x10\x15a\x06\xADW`\0\x85\x85\x83\x81\x81\x10a\x11\xF5Wa\x11\xF5a5\xFCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x12\x85W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R\x7FStakeRegistry.updateOperatorStak`D\x82\x01R\x7Fe: quorum does not exist\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[`\0\x80a\x12\x92\x83\x8Ba hV[\x91P\x91P\x80a\x12\xB4W`\0\x91P`\x01`\xFF\x84\x16\x1B`\x01`\x01`\xC0\x1B\x03\x86\x16\x17\x94P[`\0a\x12\xC1\x8A\x85\x85a\"fV[\x90Pa\x12\xCD\x84\x82a\x1E\xEEV[PPPPP\x80\x80a\x12\xDD\x90a6CV[\x91PPa\x11\xD9V[`\0a\x12\xF2\x84\x84\x84a\"fV[\x90P[\x93\x92PPPV[`\0a\x12\xF5\x83\x83a$\xE6V[```\0\x82`\x01`\x01`@\x1B\x03\x81\x11\x15a\x13$Wa\x13$a/\xF1V[`@Q\x90\x80\x82R\x80` \x02` \x01\x82\x01`@R\x80\x15a\x13MW\x81` \x01` \x82\x02\x806\x837\x01\x90P[P\x90P`\0[\x83\x81\x10\x15a\x15\xC7W`\0\x85\x85\x83\x81\x81\x10a\x13oWa\x13oa5\xFCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x14\x0EW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum does not`d\x82\x01Re\x08\x19^\x1A\\\xDD`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 \x80Tc\xFF\xFF\xFF\xFF\x8A\x16\x92\x90a\x147Wa\x147a5\xFCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11\x15a\x14\xE3W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`[`$\x82\x01R\x7FStakeRegistry.getTotalStakeIndic`D\x82\x01R\x7FesAtBlockNumber: quorum has no s`d\x82\x01R\x7Ftake history at blockNumber\0\0\0\0\0`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\xFF\x81\x16`\0\x90\x81R`\x01` R`@\x81 T\x90[\x81\x81\x10\x15a\x15\xB1W`\xFF\x83\x16`\0\x90\x81R`\x01` \x81\x90R`@\x90\x91 c\xFF\xFF\xFF\xFF\x8B\x16\x91a\x15'\x84\x86a6\xD0V[a\x151\x91\x90a6\xD0V[\x81T\x81\x10a\x15AWa\x15Aa5\xFCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a\x15\x9FW`\x01a\x15d\x82\x84a6\xD0V[a\x15n\x91\x90a6\xD0V[\x85\x85\x81Q\x81\x10a\x15\x80Wa\x15\x80a5\xFCV[` \x02` \x01\x01\x90c\xFF\xFF\xFF\xFF\x16\x90\x81c\xFF\xFF\xFF\xFF\x16\x81RPPa\x15\xB1V[\x80a\x15\xA9\x81a6CV[\x91PPa\x14\xF8V[PPP\x80\x80a\x15\xBF\x90a6CV[\x91PPa\x13SV[P\x94\x93PPPPV[`\x04` R\x81`\0R`@`\0 \x81\x81T\x81\x10a\x15\xECW`\0\x80\xFD[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x91P\x82\x90PV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R\x85\x82R`\x02\x81R\x83\x82 `\xFF\x88\x16\x83R\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x16MWa\x16Ma5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x94\x93PPPPV[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R`\xFF\x83\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x83\x90\x81\x10a\x16\xD8Wa\x16\xD8a5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q\x80\x82\x01\x90\x91R\x91\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x82R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[`@\x80Q``\x81\x01\x82R`\0\x80\x82R` \x80\x83\x01\x82\x90R\x82\x84\x01\x82\x90R`\xFF\x86\x16\x82R`\x01\x90R\x91\x90\x91 \x80T\x83\x90\x81\x10a\x17WWa\x17Wa5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x92\x90\x91\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x84R`\x01` \x1B\x82\x04\x16\x93\x83\x01\x93\x90\x93R`\x01`@\x1B\x90\x92\x04`\x01`\x01``\x1B\x03\x16\x91\x81\x01\x91\x90\x91R\x93\x92PPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x18\x07W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x18+\x91\x90a5cV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x18[W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x80V[\x81a\x18w\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x18\x93W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[a\x06\x19\x83\x83a%\x14V[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x18\xE5W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a6^V[`\0[\x81\x81\x10\x15a\x19\xC4W`\0\x83\x83\x83\x81\x81\x10a\x19\x04Wa\x19\x04a5\xFCV[\x91\x90\x91\x015`\xF8\x1C`\0\x81\x81R`\x01` R`@\x90 T\x90\x92P\x15\x15\x90Pa\x19\x94W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`7`$\x82\x01R\x7FStakeRegistry.deregisterOperator`D\x82\x01R\x7F: quorum does not exist\0\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[`\0a\x19\xA2\x86\x83`\0a\"fV[\x90Pa\x19\xAE\x82\x82a\x1E\xEEV[PPP\x80\x80a\x19\xBC\x90a6CV[\x91PPa\x18\xE8V[PPPPV[\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\x01`\x01`\xA0\x1B\x03\x16c\x8D\xA5\xCB[`@Q\x81c\xFF\xFF\xFF\xFF\x16`\xE0\x1B\x81R`\x04\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a\x1A(W=`\0\x80>=`\0\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x1AL\x91\x90a5cV[`\x01`\x01`\xA0\x1B\x03\x163`\x01`\x01`\xA0\x1B\x03\x16\x14a\x1A|W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x80V[\x81a\x1A\x98\x81`\xFF\x16`\0\x90\x81R`\x01` R`@\x90 T\x15\x15\x90V[a\x1A\xB4W`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a5\x12V[a\x06\x19\x83\x83a%}V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1A\xE5Wa\x1A\xE5a5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\r_\x81\x85a)\xC0V[`\xFF\x81\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x90\x91a\x1B]\x91a6\xD0V[\x81T\x81\x10a\x1BmWa\x1Bma5\xFCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x92\x91PPV[`\0a\x12\xF2\x84\x84\x84a+:V[`\0\x82\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x81 \x80T\x82\x91\x90\x84\x90\x81\x10a\x1B\xCDWa\x1B\xCDa5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x90Pa\x1C$\x81\x86a)\xC0V[`@\x01Q\x95\x94PPPPPV[`\0a\x12\xF5\x83\x83a,\xA0V[`@\x80Q``\x80\x82\x01\x83R`\0\x80\x83R` \x80\x84\x01\x82\x90R\x83\x85\x01\x82\x90R\x86\x82R`\x02\x81R\x84\x82 `\xFF\x87\x16\x83R\x81R\x84\x82 T\x85Q\x93\x84\x01\x86R\x82\x84R\x90\x83\x01\x82\x90R\x93\x82\x01R\x90\x91\x90\x81a\x1C\x96W\x91Pa\rL\x90PV[`\0\x85\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x88\x16\x84R\x90\x91R\x90 a\x1C\xBD`\x01\x84a6\xD0V[\x81T\x81\x10a\x1C\xCDWa\x1C\xCDa5\xFCV[`\0\x91\x82R` \x91\x82\x90 `@\x80Q``\x81\x01\x82R\x91\x90\x92\x01Tc\xFF\xFF\xFF\xFF\x80\x82\x16\x83R`\x01` \x1B\x82\x04\x16\x93\x82\x01\x93\x90\x93R`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x93\x04\x92\x90\x92\x16\x90\x82\x01R\x92Pa\rL\x91PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 a\x1DI\x85\x85\x85a+:V[c\xFF\xFF\xFF\xFF\x16\x81T\x81\x10a\x1D_Wa\x1D_a5\xFCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x94\x93PPPPV[3`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a\x1D\xCBW`@QbF\x1B\xCD`\xE5\x1B\x81R`\x04\x01a\x06\x98\x90a6^V[`\xFF\x83\x16`\0\x90\x81R`\x01` R`@\x90 T\x15a\x1EIW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`5`$\x82\x01R\x7FStakeRegistry.initializeQuorum: `D\x82\x01Rtquorum already exists`X\x1B`d\x82\x01R`\x84\x01a\x06\x98V[a\x1ES\x83\x82a%}V[a\x1E]\x83\x83a%\x14V[PP`\xFF\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x87\x81R\x93\x82\x01\x87\x81R\x83T\x96\x87\x01\x84U\x92\x87R\x93\x90\x95 \x94Q\x94\x90\x93\x01\x80T\x91Q\x93Q`\x01`\x01``\x1B\x03\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x94\x84\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x93\x16\x95\x90\x93\x16\x94\x90\x94\x17\x17\x91\x90\x91\x16\x17\x90UV[`\xFF\x82\x16`\0\x90\x81R`\x01` \x81\x90R`@\x82 \x80T\x91\x83\x91\x90a\x1F\x12\x90\x84a6\xD0V[\x81T\x81\x10a\x1F\"Wa\x1F\"a5\xFCV[\x90`\0R` `\0 \x01\x90P\x83`\0\x14\x15a\x1FQWT`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x91Pa\rL\x90PV[\x80T`\0\x90a\x1Fp\x90`\x01`@\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x86a$\xE6V[\x82T\x90\x91PCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a\x1F\xADW\x81T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x83\x16\x02\x17\x82Ua _V[\x81Tc\xFF\xFF\xFF\xFFC\x81\x16`\x01` \x1B\x81\x81\x02g\xFF\xFF\xFF\xFF\0\0\0\0\x19\x90\x94\x16\x93\x90\x93\x17\x85U`\xFF\x89\x16`\0\x90\x81R`\x01` \x81\x81R`@\x80\x84 \x81Q``\x81\x01\x83R\x95\x86R\x85\x83\x01\x85\x81R`\x01`\x01``\x1B\x03\x80\x8B\x16\x93\x88\x01\x93\x84R\x82T\x95\x86\x01\x83U\x91\x86R\x92\x90\x94 \x94Q\x94\x90\x92\x01\x80T\x91Q\x92Q\x90\x93\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x92\x86\x16\x90\x96\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x91\x16\x93\x90\x94\x16\x92\x90\x92\x17\x92\x90\x92\x17\x16\x91\x90\x91\x17\x90U[\x95\x94PPPPPV[`\0\x80`\0\x80a \x87\x86`\xFF\x16`\0\x90\x81R`\x03` R`@\x90 T\x90V[`@\x80Q\x80\x82\x01\x90\x91R`\0\x80\x82R` \x82\x01R\x90\x91P`\xFF\x87\x16`\0\x90\x81R`\x04` \x81\x90R`@\x80\x83 \x90Qc\x90\x04\x13G`\xE0\x1B\x81R`\x01`\x01`\xA0\x1B\x03\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x92c\x90\x04\x13G\x92a \xFC\x92\x8C\x92\x01a6\xFDV[`\0`@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a!\x19W=`\0\x80>=`\0\xFD[PPPP`@Q=`\0\x82>`\x1F=\x90\x81\x01`\x1F\x19\x16\x82\x01`@Ra!A\x91\x90\x81\x01\x90a7\\V[\x90P`\0[\x83\x81\x10\x15a\"2W`\xFF\x89\x16`\0\x90\x81R`\x03` R`@\x90 \x80T\x82\x90\x81\x10a!rWa!ra5\xFCV[`\0\x91\x82R` \x80\x83 `@\x80Q\x80\x82\x01\x90\x91R\x92\x01T`\x01`\x01`\xA0\x1B\x03\x81\x16\x83R`\x01`\xA0\x1B\x90\x04`\x01`\x01``\x1B\x03\x16\x90\x82\x01R\x83Q\x90\x94P\x83\x90\x83\x90\x81\x10a!\xC0Wa!\xC0a5\xFCV[` \x02` \x01\x01Q\x11\x15a\" Wg\r\xE0\xB6\xB3\xA7d\0\0\x83` \x01Q`\x01`\x01``\x1B\x03\x16\x83\x83\x81Q\x81\x10a!\xF7Wa!\xF7a5\xFCV[` \x02` \x01\x01Qa\"\t\x91\x90a7\xECV[a\"\x13\x91\x90a8\x0BV[a\"\x1D\x90\x86a8-V[\x94P[\x80a\"*\x81a6CV[\x91PPa!FV[PPP`\xFF\x86\x16`\0\x90\x81R` \x81\x90R`@\x90 T\x91\x93PP`\x01`\x01``\x1B\x03\x90\x81\x16\x90\x83\x16\x10\x15\x90P[\x92P\x92\x90PV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x81\x90\x80a#*W`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83Rc\xFF\xFF\xFF\xFFC\x81\x16\x82R\x81\x85\x01\x86\x81R`\x01`\x01``\x1B\x03\x80\x8C\x16\x95\x84\x01\x95\x86R\x84T`\x01\x81\x01\x86U\x94\x88R\x95\x90\x96 \x91Q\x91\x90\x92\x01\x80T\x95Q\x93Q\x90\x94\x16`\x01`@\x1B\x02`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x93\x83\x16`\x01` \x1B\x02g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x96\x16\x91\x90\x92\x16\x17\x93\x90\x93\x17\x16\x91\x90\x91\x17\x90Ua$\x8CV[`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x81 a#Q`\x01\x84a6\xD0V[\x81T\x81\x10a#aWa#aa5\xFCV[`\0\x91\x82R` \x90\x91 \x01\x80T`\x01`\x01``\x1B\x03`\x01`@\x1B\x90\x91\x04\x81\x16\x94P\x90\x91P\x85\x16\x83\x14\x15a#\x9AW`\0\x93PPPPa\x12\xF5V[\x80TCc\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14\x15a#\xD4W\x80T`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B`\x01`\x01``\x1B\x03\x87\x16\x02\x17\x81Ua$\x8AV[\x80Tg\xFF\xFF\xFF\xFF\0\0\0\0\x19\x16`\x01` \x1BCc\xFF\xFF\xFF\xFF\x90\x81\x16\x82\x81\x02\x93\x90\x93\x17\x84U`\0\x8A\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x8D\x16\x84R\x82R\x80\x83 \x81Q``\x81\x01\x83R\x96\x87R\x86\x83\x01\x84\x81R`\x01`\x01``\x1B\x03\x8D\x81\x16\x93\x89\x01\x93\x84R\x82T`\x01\x81\x01\x84U\x92\x86R\x93\x90\x94 \x96Q\x96\x01\x80T\x93Q\x91Q\x96\x85\x16g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x90\x94\x16\x93\x90\x93\x17\x93\x16\x90\x93\x02\x91\x90\x91\x17`\x01`@\x1B`\x01`\xA0\x1B\x03\x19\x16`\x01`@\x1B\x93\x90\x92\x16\x92\x90\x92\x02\x17\x90U[P[`@\x80Q`\xFF\x87\x16\x81R`\x01`\x01``\x1B\x03\x86\x16` \x82\x01R\x87\x91\x7F/R}R~\x95\xD8\xFE@\xAE\xC5Swt;\xB7y\x08}\xA3\xF6\xD0\xD0\x8F\x12\xE3dD\xDAb2}\x91\x01`@Q\x80\x91\x03\x90\xA2a$\xDC\x82\x85a,\xA0V[\x96\x95PPPPPPV[`\0\x80\x82\x12\x15a%\nWa$\xF9\x82a8XV[a%\x03\x90\x84a8uV[\x90Pa\rLV[a%\x03\x82\x84a8-V[`\xFF\x82\x16`\0\x81\x81R` \x81\x81R`@\x91\x82\x90 \x80Tk\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x19\x16`\x01`\x01``\x1B\x03\x86\x16\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x7F&\xEE\xCF\xF2\xB7\x0B\nq\x10O\xF4\xD9@\xBAqb\xD2:\x95\xC2Hw\x1F\xC4\x87\xA7\xBE\x17\xA5\x96\xB3\xCF\x91\x01`@Q\x80\x91\x03\x90\xA2PPV[`\0\x81Q\x11a%\xE2W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`8`$\x82\x01R`\0\x80Q` a9\x0C\x839\x81Q\x91R`D\x82\x01R\x7F: no strategies provided\0\0\0\0\0\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[\x80Q`\xFF\x83\x16`\0\x90\x81R`\x03` \x90\x81R`@\x90\x91 T\x90a&\x05\x83\x83a8\x9DV[\x11\x15a&uW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`E`$\x82\x01R`\0\x80Q` a9\x0C\x839\x81Q\x91R`D\x82\x01R\x7F: exceed MAX_WEIGHING_FUNCTION_L`d\x82\x01Rd\x08\xA9\xC8\xEA\x89`\xDB\x1B`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\0[\x82\x81\x10\x15a)\xB9W`\0[a&\x8D\x82\x84a8\x9DV[\x81\x10\x15a'nW\x84\x82\x81Q\x81\x10a&\xA6Wa&\xA6a5\xFCV[` \x02` \x01\x01Q`\0\x01Q`\x01`\x01`\xA0\x1B\x03\x16`\x03`\0\x88`\xFF\x16`\xFF\x16\x81R` \x01\x90\x81R` \x01`\0 \x82\x81T\x81\x10a&\xE5Wa&\xE5a5\xFCV[`\0\x91\x82R` \x90\x91 \x01T`\x01`\x01`\xA0\x1B\x03\x16\x14\x15a'\\W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`=`$\x82\x01R`\0\x80Q` a9\x0C\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add same strategy 2x\0\0\0`d\x82\x01R`\x84\x01a\x06\x98V[\x80a'f\x81a6CV[\x91PPa&\x83V[P`\0\x84\x82\x81Q\x81\x10a'\x83Wa'\x83a5\xFCV[` \x02` \x01\x01Q` \x01Q`\x01`\x01``\x1B\x03\x16\x11a(\x08W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`F`$\x82\x01R`\0\x80Q` a9\x0C\x839\x81Q\x91R`D\x82\x01R\x7F: cannot add strategy with zero `d\x82\x01Re\x1D\xD9ZY\xDA\x1D`\xD2\x1B`\x84\x82\x01R`\xA4\x01a\x06\x98V[`\xFF\x85\x16`\0\x90\x81R`\x03` R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a(.Wa(.a5\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01Q\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x82\x84 \x82Q\x92\x84\x01Q`\x01`\x01``\x1B\x03\x16`\x01`\xA0\x1B\x02`\x01`\x01`\xA0\x1B\x03\x90\x93\x16\x92\x90\x92\x17\x91\x01U`\xFF\x87\x16\x82R`\x04\x90R`@\x90 \x84Q\x85\x90\x83\x90\x81\x10a(\x93Wa(\x93a5\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ\x82T`\x01\x81\x01\x84U`\0\x93\x84R\x91\x90\x92 \x01\x80T`\x01`\x01`\xA0\x1B\x03\x19\x16`\x01`\x01`\xA0\x1B\x03\x90\x92\x16\x91\x90\x91\x17\x90U\x83Q`\xFF\x86\x16\x90\x7F\x10V^V\xCA\xCB\xF3.\xCA&yE\xF0T\xFE\xC0.Yu\x002\xD1\x13\xD30!\x82\xAD\x96\x7FT\x04\x90\x86\x90\x84\x90\x81\x10a)\nWa)\na5\xFCV[` \x90\x81\x02\x91\x90\x91\x01\x81\x01QQ`@Q`\x01`\x01`\xA0\x1B\x03\x90\x91\x16\x81R\x01`@Q\x80\x91\x03\x90\xA2\x84`\xFF\x16\x7F\x11\xA5d\x13\"\xDA\x1D\xFFV\xA4\xB6n\xAA\xC3\x1F\xFAFR\x95\xEC\xE9\x07\xCD\x1647y;M\0\x9Au\x85\x83\x81Q\x81\x10a)gWa)ga5\xFCV[` \x02` \x01\x01Q`\0\x01Q\x86\x84\x81Q\x81\x10a)\x85Wa)\x85a5\xFCV[` \x02` \x01\x01Q` \x01Q`@Qa)\x9F\x92\x91\x90a-\x13V[`@Q\x80\x91\x03\x90\xA2\x80a)\xB1\x81a6CV[\x91PPa&xV[PPPPPV[\x81`\0\x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10\x15a*eW`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`V`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: stakeUpdate is `d\x82\x01Ru397\xB6\x900\xB3:2\xB9\x10167\xB1\xB5\xA7:\xB6\xB12\xB9`Q\x1B`\x84\x82\x01R`\xA4\x01a\x06\x98V[` \x82\x01Qc\xFF\xFF\xFF\xFF\x16\x15\x80a*\x8BWP\x81` \x01Qc\xFF\xFF\xFF\xFF\x16\x81c\xFF\xFF\xFF\xFF\x16\x10[a+6W`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`j`$\x82\x01R\x7FStakeRegistry._validateStakeUpda`D\x82\x01R\x7FteAtBlockNumber: there is a newe`d\x82\x01R\x7Fr stakeUpdate available before b`\x84\x82\x01Ri67\xB1\xB5\xA7:\xB6\xB12\xB9`\xB1\x1B`\xA4\x82\x01R`\xC4\x01a\x06\x98V[PPV[`\0\x83\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x86\x16\x84R\x90\x91R\x81 T\x80[\x80\x15a+\xDBW`\0\x86\x81R`\x02` \x90\x81R`@\x80\x83 `\xFF\x89\x16\x84R\x90\x91R\x90 c\xFF\xFF\xFF\xFF\x85\x16\x90a+\x8E`\x01\x84a6\xD0V[\x81T\x81\x10a+\x9EWa+\x9Ea5\xFCV[`\0\x91\x82R` \x90\x91 \x01Tc\xFF\xFF\xFF\xFF\x16\x11a+\xC9Wa+\xC0`\x01\x82a6\xD0V[\x92PPPa\x12\xF5V[\x80a+\xD3\x81a8\xB5V[\x91PPa+YV[P`@QbF\x1B\xCD`\xE5\x1B\x81R` `\x04\x82\x01R`\x81`$\x82\x01R\x7FStakeRegistry._getStakeUpdateInd`D\x82\x01R\x7FexForOperatorAtBlockNumber: no s`d\x82\x01R\x7Ftake update found for operatorId`\x84\x82\x01R\x7F and quorumNumber at block numbe`\xA4\x82\x01R`9`\xF9\x1B`\xC4\x82\x01R`\xE4\x01a\x06\x98V[`\0a\x12\xF5`\x01`\x01``\x1B\x03\x80\x85\x16\x90\x84\x16a8\xCCV[\x805`\xFF\x81\x16\x81\x14a,\xC9W`\0\x80\xFD[\x91\x90PV[`\0\x80`@\x83\x85\x03\x12\x15a,\xE1W`\0\x80\xFD[a,\xEA\x83a,\xB8V[\x94` \x93\x90\x93\x015\x93PPPV[`\0` \x82\x84\x03\x12\x15a-\nW`\0\x80\xFD[a\x12\xF5\x82a,\xB8V[`\x01`\x01`\xA0\x1B\x03\x92\x90\x92\x16\x82R`\x01`\x01``\x1B\x03\x16` \x82\x01R`@\x01\x90V[`\x01`\x01`\xA0\x1B\x03\x81\x16\x81\x14a-JW`\0\x80\xFD[PV[`\0\x80`@\x83\x85\x03\x12\x15a-`W`\0\x80\xFD[a-i\x83a,\xB8V[\x91P` \x83\x015a-y\x81a-5V[\x80\x91PP\x92P\x92\x90PV[`\0\x80\x83`\x1F\x84\x01\x12a-\x96W`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a-\xADW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82`\x05\x1B\x85\x01\x01\x11\x15a\"_W`\0\x80\xFD[`\0\x80`\0\x80`\0``\x86\x88\x03\x12\x15a-\xE0W`\0\x80\xFD[a-\xE9\x86a,\xB8V[\x94P` \x86\x015`\x01`\x01`@\x1B\x03\x80\x82\x11\x15a.\x05W`\0\x80\xFD[a.\x11\x89\x83\x8A\x01a-\x84V[\x90\x96P\x94P`@\x88\x015\x91P\x80\x82\x11\x15a.*W`\0\x80\xFD[Pa.7\x88\x82\x89\x01a-\x84V[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[`\0\x80\x83`\x1F\x84\x01\x12a.ZW`\0\x80\xFD[P\x815`\x01`\x01`@\x1B\x03\x81\x11\x15a.qW`\0\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\"_W`\0\x80\xFD[`\0\x80`\0\x80``\x85\x87\x03\x12\x15a.\x9FW`\0\x80\xFD[\x845a.\xAA\x81a-5V[\x93P` \x85\x015\x92P`@\x85\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a.\xCCW`\0\x80\xFD[a.\xD8\x87\x82\x88\x01a.HV[\x95\x98\x94\x97P\x95PPPPV[`\0\x81Q\x80\x84R` \x80\x85\x01\x94P\x80\x84\x01`\0[\x83\x81\x10\x15a/\x1DW\x81Q`\x01`\x01``\x1B\x03\x16\x87R\x95\x82\x01\x95\x90\x82\x01\x90`\x01\x01a.\xF8V[P\x94\x95\x94PPPPPV[`@\x81R`\0a/;`@\x83\x01\x85a.\xE4V[\x82\x81\x03` \x84\x01Ra _\x81\x85a.\xE4V[`\0\x80`@\x83\x85\x03\x12\x15a/`W`\0\x80\xFD[\x825\x91Pa/p` \x84\x01a,\xB8V[\x90P\x92P\x92\x90PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/\xE5Wa/\xD2\x83\x85Qc\xFF\xFF\xFF\xFF\x80\x82Q\x16\x83R\x80` \x83\x01Q\x16` \x84\x01RP`\x01`\x01``\x1B\x03`@\x82\x01Q\x16`@\x83\x01RPPV[\x92\x84\x01\x92``\x92\x90\x92\x01\x91`\x01\x01a/\x95V[P\x90\x96\x95PPPPPPV[cNH{q`\xE0\x1B`\0R`A`\x04R`$`\0\xFD[`@\x80Q\x90\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a0)Wa0)a/\xF1V[`@R\x90V[`@Q`\x1F\x82\x01`\x1F\x19\x16\x81\x01`\x01`\x01`@\x1B\x03\x81\x11\x82\x82\x10\x17\x15a0WWa0Wa/\xF1V[`@R\x91\x90PV[`\0`\x01`\x01`@\x1B\x03\x82\x11\x15a0xWa0xa/\xF1V[P`\x05\x1B` \x01\x90V[`\0\x80`@\x83\x85\x03\x12\x15a0\x95W`\0\x80\xFD[a0\x9E\x83a,\xB8V[\x91P` \x80\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a0\xBAW`\0\x80\xFD[\x84\x01`\x1F\x81\x01\x86\x13a0\xCBW`\0\x80\xFD[\x805a0\xDEa0\xD9\x82a0_V[a0/V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x88\x83\x11\x15a0\xFDW`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a1\x1BW\x835\x82R\x92\x84\x01\x92\x90\x84\x01\x90a1\x02V[\x80\x95PPPPPP\x92P\x92\x90PV[\x805`\x01`\x01``\x1B\x03\x81\x16\x81\x14a,\xC9W`\0\x80\xFD[`\0\x80`\0``\x84\x86\x03\x12\x15a1VW`\0\x80\xFD[\x835\x92Pa1f` \x85\x01a,\xB8V[\x91Pa1t`@\x85\x01a1*V[\x90P\x92P\x92P\x92V[`\0\x80`@\x83\x85\x03\x12\x15a1\x90W`\0\x80\xFD[a,\xEA\x83a1*V[\x805c\xFF\xFF\xFF\xFF\x81\x16\x81\x14a,\xC9W`\0\x80\xFD[`\0\x80`\0`@\x84\x86\x03\x12\x15a1\xC2W`\0\x80\xFD[a1\xCB\x84a1\x99V[\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xE6W`\0\x80\xFD[a1\xF2\x86\x82\x87\x01a.HV[\x94\x97\x90\x96P\x93\x94PPPPV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R`\0\x91\x90\x84\x82\x01\x90`@\x85\x01\x90\x84[\x81\x81\x10\x15a/\xE5W\x83Qc\xFF\xFF\xFF\xFF\x16\x83R\x92\x84\x01\x92\x91\x84\x01\x91`\x01\x01a2\x1BV[`\0\x80`\0``\x84\x86\x03\x12\x15a2RW`\0\x80\xFD[a2[\x84a,\xB8V[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[\x81Qc\xFF\xFF\xFF\xFF\x90\x81\x16\x82R` \x80\x84\x01Q\x90\x91\x16\x90\x82\x01R`@\x80\x83\x01Q`\x01`\x01``\x1B\x03\x16\x90\x82\x01R``\x81\x01a\rLV[`\0\x80`@\x83\x85\x03\x12\x15a2\xB8W`\0\x80\xFD[a2\xC1\x83a,\xB8V[\x91Pa/p` \x84\x01a1*V[`\0\x80`\0`@\x84\x86\x03\x12\x15a2\xE4W`\0\x80\xFD[\x835\x92P` \x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a1\xE6W`\0\x80\xFD[`\0\x82`\x1F\x83\x01\x12a3\x12W`\0\x80\xFD[\x815` a3\"a0\xD9\x83a0_V[\x82\x81R`\x06\x92\x90\x92\x1B\x84\x01\x81\x01\x91\x81\x81\x01\x90\x86\x84\x11\x15a3AW`\0\x80\xFD[\x82\x86\x01[\x84\x81\x10\x15a3\x90W`@\x81\x89\x03\x12\x15a3^W`\0\x80\x81\xFD[a3fa0\x07V[\x815a3q\x81a-5V[\x81Ra3~\x82\x86\x01a1*V[\x81\x86\x01R\x83R\x91\x83\x01\x91`@\x01a3EV[P\x96\x95PPPPPPV[`\0\x80`@\x83\x85\x03\x12\x15a3\xAEW`\0\x80\xFD[a3\xB7\x83a,\xB8V[\x91P` \x83\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a3\xD2W`\0\x80\xFD[a3\xDE\x85\x82\x86\x01a3\x01V[\x91PP\x92P\x92\x90PV[`\0\x80`\0``\x84\x86\x03\x12\x15a3\xFDW`\0\x80\xFD[a4\x06\x84a,\xB8V[\x92Pa4\x14` \x85\x01a1\x99V[\x91P`@\x84\x015\x90P\x92P\x92P\x92V[`\0\x80`\0``\x84\x86\x03\x12\x15a49W`\0\x80\xFD[\x835\x92Pa4I` \x85\x01a,\xB8V[\x91Pa1t`@\x85\x01a1\x99V[`\0\x80`\0\x80`\x80\x85\x87\x03\x12\x15a4mW`\0\x80\xFD[a4v\x85a,\xB8V[\x93Pa4\x84` \x86\x01a1\x99V[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[`\0\x80`@\x83\x85\x03\x12\x15a4\xACW`\0\x80\xFD[a2\xC1\x83a1*V[`\0\x80`\0``\x84\x86\x03\x12\x15a4\xCAW`\0\x80\xFD[a4\xD3\x84a,\xB8V[\x92Pa4\xE1` \x85\x01a1*V[\x91P`@\x84\x015`\x01`\x01`@\x1B\x03\x81\x11\x15a4\xFCW`\0\x80\xFD[a5\x08\x86\x82\x87\x01a3\x01V[\x91PP\x92P\x92P\x92V[` \x80\x82R`1\x90\x82\x01R\x7FStakeRegistry.quorumExists: quor`@\x82\x01Rp\x1D[H\x19\x1B\xD9\\\xC8\x1B\x9B\xDD\x08\x19^\x1A\\\xDD`z\x1B``\x82\x01R`\x80\x01\x90V[`\0` \x82\x84\x03\x12\x15a5uW`\0\x80\xFD[\x81Qa\x12\xF5\x81a-5V[` \x80\x82R`V\x90\x82\x01R\x7FStakeRegistry.onlyCoordinatorOwn`@\x82\x01R\x7Fer: caller is not the owner of t``\x82\x01Ru42\x9092\xB3\xB4\xB9\xBA9<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`Q\x1B`\x80\x82\x01R`\xA0\x01\x90V[cNH{q`\xE0\x1B`\0R`2`\x04R`$`\0\xFD[`\0` \x82\x84\x03\x12\x15a6$W`\0\x80\xFD[a\x12\xF5\x82a1*V[cNH{q`\xE0\x1B`\0R`\x11`\x04R`$`\0\xFD[`\0`\0\x19\x82\x14\x15a6WWa6Wa6-V[P`\x01\x01\x90V[` \x80\x82R`L\x90\x82\x01R\x7FStakeRegistry.onlyRegistryCoordi`@\x82\x01R\x7Fnator: caller is not the Registr``\x82\x01Rk<\xA1\xB7\xB7\xB924\xB70\xBA7\xB9`\xA1\x1B`\x80\x82\x01R`\xA0\x01\x90V[`\0\x82\x82\x10\x15a6\xE2Wa6\xE2a6-V[P\x03\x90V[cNH{q`\xE0\x1B`\0R`1`\x04R`$`\0\xFD[`\0`@\x82\x01`\x01\x80`\xA0\x1B\x03\x80\x86\x16\x84R` `@\x81\x86\x01R\x82\x86T\x80\x85R``\x87\x01\x91P\x87`\0R\x82`\0 \x94P`\0[\x81\x81\x10\x15a7NW\x85T\x85\x16\x83R`\x01\x95\x86\x01\x95\x92\x84\x01\x92\x01a70V[P\x90\x98\x97PPPPPPPPV[`\0` \x80\x83\x85\x03\x12\x15a7oW`\0\x80\xFD[\x82Q`\x01`\x01`@\x1B\x03\x81\x11\x15a7\x85W`\0\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a7\x96W`\0\x80\xFD[\x80Qa7\xA4a0\xD9\x82a0_V[\x81\x81R`\x05\x91\x90\x91\x1B\x82\x01\x83\x01\x90\x83\x81\x01\x90\x87\x83\x11\x15a7\xC3W`\0\x80\xFD[\x92\x84\x01\x92[\x82\x84\x10\x15a7\xE1W\x83Q\x82R\x92\x84\x01\x92\x90\x84\x01\x90a7\xC8V[\x97\x96PPPPPPPV[`\0\x81`\0\x19\x04\x83\x11\x82\x15\x15\x16\x15a8\x06Wa8\x06a6-V[P\x02\x90V[`\0\x82a8(WcNH{q`\xE0\x1B`\0R`\x12`\x04R`$`\0\xFD[P\x04\x90V[`\0`\x01`\x01``\x1B\x03\x80\x83\x16\x81\x85\x16\x80\x83\x03\x82\x11\x15a8OWa8Oa6-V[\x01\x94\x93PPPPV[`\0`\x01`\xFF\x1B\x82\x14\x15a8nWa8na6-V[P`\0\x03\x90V[`\0`\x01`\x01``\x1B\x03\x83\x81\x16\x90\x83\x16\x81\x81\x10\x15a8\x95Wa8\x95a6-V[\x03\x93\x92PPPV[`\0\x82\x19\x82\x11\x15a8\xB0Wa8\xB0a6-V[P\x01\x90V[`\0\x81a8\xC4Wa8\xC4a6-V[P`\0\x19\x01\x90V[`\0\x80\x83\x12\x80\x15`\x01`\xFF\x1B\x85\x01\x84\x12\x16\x15a8\xEAWa8\xEAa6-V[`\x01`\x01`\xFF\x1B\x03\x84\x01\x83\x13\x81\x16\x15a9\x05Wa9\x05a6-V[PP\x03\x90V\xFEStakeRegistry._addStrategyParams\xA2dipfsX\"\x12 \xD1\xC1`\xB6mq\xA1\xA41G1F\xB2\x860;i\xB5\x16si\xC9G[m\xA78\x0E\x18\xC6\xB5odsolcC\0\x08\x0C\x003",
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
    ///Container for all the [`StakeRegistryHarness`](self) function calls.
    pub enum StakeRegistryHarnessCalls {
        MAX_WEIGHING_FUNCTION_LENGTH(MAX_WEIGHING_FUNCTION_LENGTHCall),
        WEIGHTING_DIVISOR(WEIGHTING_DIVISORCall),
        addStrategies(addStrategiesCall),
        applyDelta(applyDeltaCall),
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
        initializeQuorum(initializeQuorumCall),
        minimumStakeForQuorum(minimumStakeForQuorumCall),
        modifyStrategyParams(modifyStrategyParamsCall),
        recordOperatorStakeUpdate(recordOperatorStakeUpdateCall),
        recordTotalStakeUpdate(recordTotalStakeUpdateCall),
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
            [60u8, 165u8, 165u8, 245u8],
            [75u8, 210u8, 110u8, 9u8],
            [84u8, 1u8, 237u8, 39u8],
            [94u8, 90u8, 103u8, 117u8],
            [95u8, 31u8, 45u8, 119u8],
            [102u8, 172u8, 254u8, 254u8],
            [109u8, 20u8, 169u8, 135u8],
            [116u8, 69u8, 76u8, 109u8],
            [124u8, 23u8, 35u8, 71u8],
            [127u8, 66u8, 152u8, 34u8],
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
            [245u8, 9u8, 85u8, 26u8],
            [248u8, 81u8, 225u8, 152u8],
            [250u8, 40u8, 198u8, 39u8],
            [255u8, 105u8, 74u8, 119u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for StakeRegistryHarnessCalls {
        const NAME: &'static str = "StakeRegistryHarnessCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 35usize;
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
                Self::initializeQuorum(_) => {
                    <initializeQuorumCall as alloy_sol_types::SolCall>::SELECTOR
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
                {
                    fn initializeQuorum(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<StakeRegistryHarnessCalls> {
                        <initializeQuorumCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(StakeRegistryHarnessCalls::initializeQuorum)
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
                Self::applyDelta(inner) => {
                    <applyDeltaCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
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
                Self::applyDelta(inner) => {
                    <applyDeltaCall as alloy_sol_types::SolCall>::abi_encode_raw(
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
    ///Container for all the [`StakeRegistryHarness`](self) events.
    pub enum StakeRegistryHarnessEvents {
        MinimumStakeForQuorumUpdated(MinimumStakeForQuorumUpdated),
        OperatorStakeUpdate(OperatorStakeUpdate),
        QuorumCreated(QuorumCreated),
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
    impl alloy_sol_types::SolEventInterface for StakeRegistryHarnessEvents {
        const NAME: &'static str = "StakeRegistryHarnessEvents";
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
    impl alloy_sol_types::private::IntoLogData for StakeRegistryHarnessEvents {
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
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<StakeRegistryHarnessInstance<T, P, N>>,
    > {
        StakeRegistryHarnessInstance::<
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
        StakeRegistryHarnessInstance::<
            T,
            P,
            N,
        >::deploy_builder(provider, _registryCoordinator, _delegationManager)
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
        ) -> alloy_contract::Result<StakeRegistryHarnessInstance<T, P, N>> {
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
