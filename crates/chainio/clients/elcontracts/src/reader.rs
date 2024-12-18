use crate::error::ElContractsError;
use alloy::providers::Provider;
use alloy_primitives::{ruint::aliases::U256, Address, FixedBytes};
use eigen_logging::logger::SharedLogger;
use eigen_utils::{
    allocationmanager::AllocationManager::{self, OperatorSet},
    avsdirectory::AVSDirectory,
    delegationmanager::DelegationManager,
    erc20::ERC20,
    get_provider,
    istrategy::IStrategy,
    permissioncontroller::PermissionController,
};

#[derive(Debug, Clone)]
pub struct ELChainReader {
    _logger: SharedLogger,
    allocation_manager: Address,
    delegation_manager: Address,
    avs_directory: Address,
    permission_controller: Address,
    pub provider: String,
}

impl ELChainReader {
    /// Creates a new `ELChainReader` instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `_logger` - The logger to use for logging.
    /// * `allocation_manager` - The address of the allocation manager contract.
    /// * `delegation_manager` - The address of the delegation manager contract.
    /// * `avs_directory` - The address of the avs directory contract.
    /// * `provider` - The provider to use for the RPC client.
    ///
    /// # Returns
    ///
    /// A new `ELChainReader` instance.
    pub fn new(
        _logger: SharedLogger,
        allocation_manager: Address,
        delegation_manager: Address,
        avs_directory: Address,
        permission_controller: Address,
        provider: String,
    ) -> Self {
        ELChainReader {
            _logger,
            allocation_manager,
            delegation_manager,
            avs_directory,
            permission_controller,
            provider,
        }
    }

    /// Builds a new `ELChainReader` instance, getting the slasher address from the delegation manager
    /// by calling the `slasher()` function and the corresponding Contract function.
    ///
    /// # Arguments
    ///
    /// * `_logger` - The logger to use for logging.
    /// * `delegation_manager` - The address of the delegation manager contract.
    /// * `avs_directory` - The address of the avs directory contract.
    /// * `client` - The provider to use for the RPC client to call the contracts.
    ///
    /// # Returns
    ///
    /// A new `ELChainReader` instance.
    ///
    /// # Errors
    pub async fn build(
        _logger: SharedLogger,
        delegation_manager: Address,
        avs_directory: Address,
        client: &String,
    ) -> Result<Self, ElContractsError> {
        let provider = get_provider(client);

        let contract_delegation_manager = DelegationManager::new(delegation_manager, provider);

        let DelegationManager::allocationManagerReturn {
            _0: allocation_manager,
        } = contract_delegation_manager
            .allocationManager()
            .call()
            .await
            .unwrap();

        let DelegationManager::permissionControllerReturn {
            _0: permission_controller,
        } = contract_delegation_manager
            .permissionController()
            .call()
            .await
            .unwrap();

        Ok(Self {
            _logger,
            avs_directory,
            allocation_manager,
            delegation_manager,
            permission_controller,
            provider: client.to_string(),
        })
    }

    /// Calculate the delegation approval digest hash
    ///
    /// # Arguments
    ///
    /// * `staker` - The staker's address
    /// * `operator` - The operator's address
    /// * `delegation_approver` - The delegation approver's address
    /// * `approve_salt` - The approve salt
    /// * `expiry` - The expiry
    ///
    /// # Returns
    ///
    /// * `FixedBytes<32>` - The delegation approval digest hash
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn calculate_delegation_approval_digest_hash(
        &self,
        staker: Address,
        operator: Address,
        delegation_approver: Address,
        approve_salt: FixedBytes<32>,
        expiry: U256,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_provider(&self.provider);
        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);
        let delegation_approval_digest_hash = contract_delegation_manager
            .calculateDelegationApprovalDigestHash(
                staker,
                operator,
                delegation_approver,
                approve_salt,
                expiry,
            )
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::calculateDelegationApprovalDigestHashReturn { _0: digest_hash } =
            delegation_approval_digest_hash;

        Ok(digest_hash)
    }

    /// Calculate the operator avs registration digest hash
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator's address
    /// * `avs` - The avs's address
    /// * `salt` - The salt
    /// * `expiry` - The expiry
    ///
    /// # Returns
    ///
    /// * `FixedBytes<32>` - The operator avs registration digest hash
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn calculate_operator_avs_registration_digest_hash(
        &self,
        operator: Address,
        avs: Address,
        salt: FixedBytes<32>,
        expiry: U256,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_avs_directory = AVSDirectory::new(self.avs_directory, provider);

        let operator_avs_registration_digest_hash = contract_avs_directory
            .calculateOperatorAVSRegistrationDigestHash(operator, avs, salt, expiry)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AVSDirectory::calculateOperatorAVSRegistrationDigestHashReturn { _0: avs_hash } =
            operator_avs_registration_digest_hash;

        Ok(avs_hash)
    }

    /// Get the operator's shares in a strategy
    ///
    /// # Arguments
    ///
    /// * `operator_addr` - The operator's address
    /// * `strategy_addr` - The strategy's address
    ///
    /// # Returns
    ///
    /// * `U256` - The operator's shares in the strategy
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_operator_shares_in_strategy(
        &self,
        operator_addr: Address,
        strategy_addr: Address,
    ) -> Result<U256, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let operator_shares_in_strategy = contract_delegation_manager
            .operatorShares(operator_addr, strategy_addr)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::operatorSharesReturn { shares } = operator_shares_in_strategy;
        Ok(shares)
    }

    /// Get strategy and underlying ERC-20 token
    ///
    /// # Arguments
    ///
    /// * `strategy_addr` - The strategy's address
    ///
    /// # Returns
    ///
    /// - the strategy contract,
    /// - the erc20 bindings for the underlying token
    /// - and the underlying token address
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_strategy_and_underlying_erc20_token(
        &self,
        strategy_addr: Address,
    ) -> Result<(Address, Address, Address), ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_strategy = IStrategy::new(strategy_addr, &provider);

        let underlying_token = contract_strategy
            .underlyingToken()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IStrategy::underlyingTokenReturn {
            _0: underlying_token_addr,
        } = underlying_token;

        let contract_ierc20 = ERC20::new(underlying_token_addr, &provider);

        Ok((
            strategy_addr,
            *contract_ierc20.address(),
            underlying_token_addr,
        ))
    }

    /// Check if the operator is registered
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator's address
    ///
    /// # Returns
    ///
    /// * `bool` - true if the operator is registered, false otherwise
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn is_operator_registered(
        &self,
        operator: Address,
    ) -> Result<bool, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let is_operator = contract_delegation_manager
            .isOperator(operator)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::isOperatorReturn { _0: is_operator_is } = is_operator;
        Ok(is_operator_is)
    }

    /// Get the staker's shares in all of the strategies in which they have nonzero shares
    /// # Arguments
    /// * `staker_address` - The staker's address
    /// * `block_number` - The block number
    ///
    /// # Returns
    /// * `Vec<Address>` - An array of strategy addresses
    /// * `Vec<U256>` - An array with the amount of shares the staker has in each strategy
    ///
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_staker_shares(
        &self,
        staker_address: Address,
    ) -> Result<(Vec<Address>, Vec<U256>), ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let deposited_shares = contract_delegation_manager
            .getDepositedShares(staker_address)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::getDepositedSharesReturn {
            _0: addresses,
            _1: shares,
        } = deposited_shares;

        Ok((addresses, shares))
    }

    /// Get the delegated operator
    /// # Arguments
    /// * `staker_address` - The staker's address
    /// * `block_number` - The block number
    ///
    /// # Returns
    /// * `Address` - The address of the operator to whom the staker has delegated their shares
    ///
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_delegated_operator(
        &self,
        staker_address: Address,
    ) -> Result<Address, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let delegated = contract_delegation_manager
            .delegatedTo(staker_address)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::delegatedToReturn { operator } = delegated;

        Ok(operator)
    }

    /// # Returns the strategy contract and the underlying token address.
    ///
    /// # Arguments
    ///
    /// * `strategy_addr` - The strategy's address
    ///
    /// # Returns
    ///
    /// - the strategy contract address,
    /// - and the underlying token address
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_strategy_and_underlying_token(
        &self,
        strategy_addr: Address,
    ) -> Result<(Address, Address), ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_strategy = IStrategy::new(strategy_addr, &provider);
        let contract_strategy_address = contract_strategy.address();

        let underlying_token = contract_strategy
            .underlyingToken()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IStrategy::underlyingTokenReturn {
            _0: underlying_token_addr,
        } = underlying_token;

        Ok((*contract_strategy_address, underlying_token_addr))
    }

    /// For a strategy, get the amount of magnitude not currently allocated to any operator set
    /// # Arguments
    /// * `operator_address` - The operator's address to query
    /// * `strategy_address` - The strategy's address to get allocatable magnitude for
    /// # Returns
    /// * `u64` - The magnitude available to be allocated to an operator set
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_allocatable_magnitude(
        &self,
        operator_address: Address,
        strategy_address: Address,
    ) -> Result<u64, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let allocatable_magnitude = contract_allocation_manager
            .getAllocatableMagnitude(operator_address, strategy_address)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AllocationManager::getAllocatableMagnitudeReturn {
            _0: allocatable_magnitude,
        } = allocatable_magnitude;

        Ok(allocatable_magnitude)
    }

    /// Get the maximum magnitude an operator can allocate for the given strategies
    /// # Arguments
    /// * `operator_address` - The operator's address to query
    /// * `strategy_addresses` - The strategy's addresses to get max magnitudes for
    /// # Returns
    /// * `Vec<u64>` - The maximum magnitudes for the strategies
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_max_magnitudes(
        &self,
        operator_address: Address,
        strategy_addresses: Vec<Address>,
    ) -> Result<Vec<u64>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let max_magnitudes = contract_allocation_manager
            .getMaxMagnitudes_1(operator_address, strategy_addresses)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AllocationManager::getMaxMagnitudes_1Return { _0: max_magnitudes } = max_magnitudes;

        Ok(max_magnitudes)
    }

    /// Get the allocation info given a strategy and an operator. Returns the info for each operator set where an operator has allocation.
    /// # Arguments
    /// * `operator_address` - The operator's address to query
    /// * `strategy_address` - The strategy's address to get allocation info for
    /// # Returns
    /// * `Vec<AllocationInfo>` - The allocation info for each operator set
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_allocation_info(
        &self,
        operator_address: Address,
        strategy_address: Address,
    ) -> Result<Vec<AllocationInfo>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let allocations = contract_allocation_manager
            .getStrategyAllocations(operator_address, strategy_address)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)
            .unwrap();

        let AllocationManager::getStrategyAllocationsReturn {
            _0: operator_sets,
            _1: allocation_info,
        } = allocations;

        let mut allocations_info = vec![];
        for (i, operator_set) in operator_sets.iter().enumerate() {
            allocations_info.push(AllocationInfo {
                operator_set: operator_set.clone(),
                current_magnitude: U256::from(allocation_info[i].currentMagnitude),
                pending_diff: U256::from(allocation_info[i].pendingDiff),
                effect_block: allocation_info[i].effectBlock,
            });
        }

        Ok(allocations_info)
    }

    /// Get the shares that an operator owns in a set of strategies
    /// # Arguments
    /// * `operator_address` - The operator's address to get shares for
    /// * `strategy_addresses` - The strategy's addresses to get shares for
    /// # Returns
    /// * `Vec<U256>` - The list of shares for each strategy
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_operator_shares(
        &self,
        operator_address: Address,
        strategy_addresses: Vec<Address>,
    ) -> Result<Vec<U256>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let operator_shares = contract_delegation_manager
            .getOperatorShares(operator_address, strategy_addresses)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::getOperatorSharesReturn {
            _0: operator_shares,
        } = operator_shares;

        Ok(operator_shares)
    }

    /// Get the shares that a list of operators own in a set of strategies
    /// # Arguments
    /// * `operator_addresses` - The list of operators' addresses to get shares for
    /// * `strategy_addresses` - The strategy's addresses to get shares for
    /// # Returns
    /// * `Vec<Vec<U256>>` - The list of shares for each operator
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_operators_shares(
        &self,
        operator_addresses: Vec<Address>,
        strategy_addresses: Vec<Address>,
    ) -> Result<Vec<Vec<U256>>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let operators_shares = contract_delegation_manager
            .getOperatorsShares(operator_addresses, strategy_addresses)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::getOperatorsSharesReturn {
            _0: operators_shares,
        } = operators_shares;

        Ok(operators_shares)
    }

    /// Get the number of operator sets that an operator is part of. Doesn't include M2 AVSs
    /// # Arguments
    /// * `operator_addr` - The operator's address to query
    /// # Returns
    /// * `U256` - The number of operator sets the operator is part of
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_num_operator_sets_for_operator(
        &self,
        operator_addr: Address,
    ) -> Result<U256, ElContractsError> {
        self.get_operator_sets_for_operator(operator_addr)
            .await
            .map(|operator_sets| U256::from(operator_sets.len() as u64))
    }

    /// Get the operator sets that an operator is part of. Doesn't include M2 AVSs
    /// # Arguments
    /// * `operator_addr` - The operator's address to query
    /// # Returns
    /// * `Vec<OperatorSet>` - The operator sets the operator is part of
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_operator_sets_for_operator(
        &self,
        operator_addr: Address,
    ) -> Result<Vec<OperatorSet>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let allocated_sets = contract_allocation_manager
            .getAllocatedSets(operator_addr)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AllocationManager::getAllocatedSetsReturn { _0: operator_sets } = allocated_sets;

        Ok(operator_sets)
    }

    /// Check if an operator is registered with a specific operator set
    /// # Arguments
    /// * `operator_address` - The operator's address to query
    /// * `operator_set` - The operator set to check if the operator is registered with
    /// # Returns
    /// * `bool` - true if the operator is registered with the operator set, false otherwise
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn is_operator_registered_with_operator_set(
        &self,
        operator_address: Address,
        operator_set: OperatorSet,
    ) -> Result<bool, ElContractsError> {
        let provider = get_provider(&self.provider);

        if operator_set.id == 0 {
            // this is an M2 AVS
            let contract_avs_directory = AVSDirectory::new(self.avs_directory, provider);

            let operator_avs_registration_status = contract_avs_directory
                .avsOperatorStatus(operator_set.avs, operator_address)
                .call()
                .await
                .map_err(ElContractsError::AlloyContractError)?;

            let AVSDirectory::avsOperatorStatusReturn { _0: status } =
                operator_avs_registration_status;

            Ok(status == 1)
        } else {
            let contract_allocation_manager =
                AllocationManager::new(self.allocation_manager, provider);
            let registered_operator_sets = contract_allocation_manager
                .getRegisteredSets(operator_address)
                .call()
                .await
                .map_err(ElContractsError::AlloyContractError)?;
            let AllocationManager::getRegisteredSetsReturn { _0: operator_sets } =
                registered_operator_sets;

            let is_registered = operator_sets.iter().any(|registered_operator_set| {
                registered_operator_set.id == operator_set.id
                    && registered_operator_set.avs == operator_set.avs
            });
            Ok(is_registered)
        }
    }

    /// Get the operators in a specific operator set. Not supported for M2 AVSs
    /// # Arguments
    /// * `operator_set` - The operator set to query
    /// # Returns
    /// * `Vec<Address>` - The list of operator's addresses in the operator set
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_operators_for_operator_set(
        &self,
        operator_set: OperatorSet,
    ) -> Result<Vec<Address>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let operators = contract_allocation_manager
            .getMembers(operator_set)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AllocationManager::getMembersReturn { _0: addresses } = operators;
        Ok(addresses)
    }

    /// Get the number of operators in a specific operator set. Not supported for M2 AVSs
    /// # Arguments
    /// * `operator_set` - The operator set to query
    /// # Returns
    /// * `U256` - The number of operators in the operator set
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_num_operators_for_operator_set(
        &self,
        operator_set: OperatorSet,
    ) -> Result<U256, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let num_operators = contract_allocation_manager
            .getMemberCount(operator_set)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AllocationManager::getMemberCountReturn { _0: num_operators } = num_operators;

        Ok(num_operators)
    }

    /// Get the strategies in a specific operator set. Not supported for M2 AVSs
    /// # Arguments
    /// * `operator_set` - The operator set to query
    /// # Returns
    /// * `Vec<Address>` - The list of strategy's addresses in the operator set
    /// # Errors
    pub async fn get_strategies_for_operator_set(
        &self,
        operator_set: OperatorSet,
    ) -> Result<Vec<Address>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let strategies = contract_allocation_manager
            .getStrategiesInOperatorSet(operator_set)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AllocationManager::getStrategiesInOperatorSetReturn { _0: strategies } = strategies;

        Ok(strategies)
    }

    /// Get the slashable shares for an operator.
    /// # Arguments
    /// * `operator_address` - The operator's address to query
    /// * `operator_set` - The operator set to query
    /// * `strategies` - The strategies to query
    /// # Returns
    /// * `Vec<U256>` - The amount of slashable shares for each strategy
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_slashable_shares(
        &self,
        operator_address: Address,
        operator_set: OperatorSet,
        strategies: Vec<Address>,
    ) -> Result<Vec<U256>, ElContractsError> {
        let provider = get_provider(&self.provider);
        let current_block_number = provider.get_block_number().await.map_err(|e| {
            ElContractsError::AlloyContractError(alloy::contract::Error::TransportError(e))
        })?;

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let slashable_stake = contract_allocation_manager
            .getMinimumSlashableStake(
                operator_set,
                vec![operator_address],
                strategies.clone(),
                current_block_number as u32,
            )
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?
            .slashableStake;

        let Some(slashable_operator_stake) = slashable_stake.first() else {
            return Err(ElContractsError::NoSlashableSharesFound);
        };

        Ok(slashable_operator_stake.clone().into())
    }

    /// Get the minimum amount of shares that are slashable by the operator sets. Not supported for M2 AVSs.
    /// # Arguments
    /// * `operator_sets` - The operator sets to query
    /// # Returns
    /// * `Vec<OperatorSetStakes>` - The operator sets, their strategies, operators, and slashable stakes
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_slashable_shares_for_operator_sets(
        &self,
        operator_sets: Vec<OperatorSet>,
    ) -> Result<Vec<OperatorSetStakes>, ElContractsError> {
        let provider = get_provider(&self.provider);
        let current_block_number = provider.get_block_number().await.map_err(|e| {
            ElContractsError::AlloyContractError(alloy::contract::Error::TransportError(e))
        })?;
        self.get_delegated_and_slashable_shares_for_operator_sets_before(
            operator_sets,
            current_block_number as u32,
        )
        .await
    }

    /// Given a list of operator sets, for each one get:
    /// - the operators,
    /// - the strategies,
    /// - the minimum amount of shares that are slashable before a given block.
    /// Not supported for M2 AVSs.
    /// # Arguments
    /// * `operator_sets` - The operator sets to query
    /// * `future_block` - The block at which to get allocation information. It must be greater that the current block number.
    /// # Returns
    /// * `Vec<OperatorSetStakes>` - The operator sets, their strategies, operators, and slashable stakes
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_delegated_and_slashable_shares_for_operator_sets_before(
        &self,
        operator_sets: Vec<OperatorSet>,
        future_block: u32,
    ) -> Result<Vec<OperatorSetStakes>, ElContractsError> {
        let provider = get_provider(&self.provider);
        let mut operator_set_stakes = vec![];
        let allocation_manager_contract = AllocationManager::new(self.allocation_manager, provider);

        for operator_set in operator_sets {
            let operators = self
                .get_operators_for_operator_set(operator_set.clone())
                .await?;
            let strategies = self
                .get_strategies_for_operator_set(operator_set.clone())
                .await?;

            let slashable_stakes = allocation_manager_contract
                .getMinimumSlashableStake(
                    operator_set.clone(),
                    operators.clone(),
                    strategies.clone(),
                    future_block,
                )
                .call()
                .await?
                .slashableStake;
            operator_set_stakes.push(OperatorSetStakes {
                operator_set,
                strategies,
                operators,
                slashable_stakes,
            });
        }
        Ok(operator_set_stakes)
    }

    /// Get the allocation delay for an operator. Is the number of blocks between an operator allocating slashable magnitude and the magnitude becoming slashable.
    /// # Arguments
    /// * `operator_address` - The operator's address to query
    /// # Returns
    /// * `u32` - The allocation delay
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    /// * `AllocationDelayNotSet` - if the allocation delay is not set
    pub async fn get_allocation_delay(
        &self,
        operator_address: Address,
    ) -> Result<u32, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let allocation_delay = contract_allocation_manager
            .getAllocationDelay(operator_address)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AllocationManager::getAllocationDelayReturn {
            _0: is_set,
            _1: delay,
        } = allocation_delay;

        if !is_set {
            return Err(ElContractsError::AllocationDelayNotSet);
        }

        Ok(delay)
    }

    /// Get the operator sets that the operator is registered for
    /// # Arguments
    /// * `operator_address` - The operator's address to query
    /// # Returns
    /// * `Vec<OperatorSet>` - The operator sets the operator is registered for
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_registered_sets(
        &self,
        operator_address: Address,
    ) -> Result<Vec<OperatorSet>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_allocation_manager = AllocationManager::new(self.allocation_manager, provider);

        let registered_sets = contract_allocation_manager
            .getRegisteredSets(operator_address)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let AllocationManager::getRegisteredSetsReturn {
            _0: registered_sets,
        } = registered_sets;

        Ok(registered_sets)
    }

    /// Check if the given caller has permissions to call the function
    /// # Arguments
    /// * `account_address` - The account address to check
    /// * `appointee_address` - The caller address to check permissions for
    /// * `target` - The target address to check permissions for
    /// * `selector` - The selector of the function to check permissions for
    /// # Returns
    /// * `bool` - true if the account has permissions to call the function, false otherwise
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn can_call(
        &self,
        account_address: Address,
        appointee_address: Address,
        target: Address,
        selector: FixedBytes<4>,
    ) -> Result<bool, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_permission_controller =
            PermissionController::new(self.permission_controller, provider);

        let can_call = contract_permission_controller
            .canCall(account_address, appointee_address, target, selector)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?
            ._0;

        Ok(can_call)
    }

    /// Get the list of appointees for a given account and function
    /// # Arguments
    /// * `account_address` - The account address to get appointees for
    /// * `target` - The target address to get appointees for
    /// * `selector` - The selector of the function to get appointees for
    /// # Returns
    /// * `Vec<Address>` - The list of appointees
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn list_appointees(
        &self,
        account_address: Address,
        target: Address,
        selector: FixedBytes<4>,
    ) -> Result<Vec<Address>, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_permission_controller =
            PermissionController::new(self.permission_controller, provider);

        let appointees = contract_permission_controller
            .getAppointees(account_address, target, selector)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?
            ._0;

        Ok(appointees)
    }

    /// Get the list of permissions of an appointee for a given account
    /// # Arguments
    /// * `account_address` - The account address to get appointee permissions for
    /// * `appointee_address` - The appointee address to get permissions
    /// # Returns
    /// * `Vec<Address>` - The list of targets
    /// * `Vec<FixedBytes<4>>` - The list of selectors
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn list_appointee_permissions(
        &self,
        account_address: Address,
        appointee_address: Address,
    ) -> Result<(Vec<Address>, Vec<FixedBytes<4>>), ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_permission_controller =
            PermissionController::new(self.permission_controller, provider);

        let appointee_permissions = contract_permission_controller
            .getAppointeePermissions(account_address, appointee_address)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let PermissionController::getAppointeePermissionsReturn {
            _0: targets,
            _1: selectors,
        } = appointee_permissions;

        Ok((targets, selectors))
    }
}

// TODO: move to types.rs?
pub struct OperatorSetStakes {
    pub operator_set: OperatorSet,
    pub strategies: Vec<Address>,
    pub operators: Vec<Address>,
    pub slashable_stakes: Vec<Vec<U256>>,
}

#[derive(Debug, Clone)]
pub struct AllocationInfo {
    pub current_magnitude: U256,
    pub pending_diff: U256,
    pub effect_block: u32,
    pub operator_set: OperatorSet,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::providers::Provider;
    use alloy::{eips::eip1898::BlockNumberOrTag::Number, rpc::types::BlockTransactionsKind};
    use alloy_primitives::{address, keccak256, Address, FixedBytes, U256};
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants::{
        get_allocation_manager_address, get_avs_directory_address, get_erc20_mock_strategy,
        register_operator_to_el_if_not_registered,
    };
    use eigen_testing_utils::{
        anvil::start_anvil_container, anvil_constants::get_delegation_manager_address,
    };
    use eigen_utils::{
        avsdirectory::AVSDirectory,
        avsdirectory::AVSDirectory::calculateOperatorAVSRegistrationDigestHashReturn,
        delegationmanager::DelegationManager,
        delegationmanager::DelegationManager::calculateDelegationApprovalDigestHashReturn,
    };

    const OPERATOR_ADDRESS: Address = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
    const OPERATOR_PRIVATE_KEY: &str =
        "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

    async fn build_el_chain_reader(http_endpoint: String) -> ELChainReader {
        register_operator_to_el_if_not_registered(
            OPERATOR_PRIVATE_KEY,
            &http_endpoint,
            OPERATOR_ADDRESS,
            "metadata_uri",
        )
        .await
        .unwrap();

        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;

        let delegation_manager_contract =
            DelegationManager::new(delegation_manager_address, get_provider(&http_endpoint));
        let permission_controller_address = delegation_manager_contract
            .permissionController()
            .call()
            .await
            .unwrap()
            ._0;

        ELChainReader::new(
            get_test_logger(),
            allocation_manager_address,
            delegation_manager_address,
            avs_directory_address,
            permission_controller_address,
            http_endpoint,
        )
    }

    #[tokio::test]
    async fn test_calculate_delegation_approval_digest_hash() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let operator: Address = address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2");

        let staker = operator;

        let delegation_approver = Address::ZERO;

        let approve_salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let current_block_number = provider.get_block_number().await.unwrap();
        let block_info = provider
            .get_block_by_number(Number(current_block_number), BlockTransactionsKind::Hashes)
            .await
            .unwrap();

        let block = block_info.unwrap();
        let timestamp = block.header.timestamp;
        let expiry = U256::from::<u64>(timestamp + 100);
        let calculate_digest_hash = el_chain_reader
            .calculate_delegation_approval_digest_hash(
                staker,
                operator,
                delegation_approver,
                approve_salt,
                expiry,
            )
            .await
            .unwrap();

        // Directly calling the function through bindings to compare with the sdk .
        let delegation_manager_address = get_delegation_manager_address(http_endpoint).await;
        let delegation_manager_contract =
            DelegationManager::new(delegation_manager_address, provider);

        let hash = delegation_manager_contract
            .calculateDelegationApprovalDigestHash(
                staker,
                operator,
                delegation_approver,
                approve_salt,
                expiry,
            )
            .call()
            .await
            .unwrap();

        let calculateDelegationApprovalDigestHashReturn { _0: digest_hash } = hash;

        assert_eq!(digest_hash, calculate_digest_hash);
    }

    #[tokio::test]
    async fn test_calculate_operator_avs_registration_digest_hash() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let operator: Address = address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2");
        let avs = Address::from_slice(&keccak256("avs ")[0..20]);
        let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let current_block_number = provider.get_block_number().await.unwrap();
        let block_info = provider
            .get_block_by_number(Number(current_block_number), BlockTransactionsKind::Hashes)
            .await
            .unwrap();
        let block = block_info.unwrap();

        let timestamp = block.header.timestamp;
        let expiry = U256::from::<u64>(timestamp + 100);
        let operator_hash = el_chain_reader
            .calculate_operator_avs_registration_digest_hash(operator, avs, salt, expiry)
            .await
            .unwrap();

        // Using bindings directly to compare with sdk's output
        let avs_registry_contract =
            AVSDirectory::new(el_chain_reader.avs_directory, provider.clone());
        let operator_hash_from_bindings = avs_registry_contract
            .calculateOperatorAVSRegistrationDigestHash(operator, avs, salt, expiry)
            .call()
            .await
            .unwrap();

        let calculateOperatorAVSRegistrationDigestHashReturn { _0: hash } =
            operator_hash_from_bindings;

        assert_eq!(hash, operator_hash);
    }

    #[tokio::test]
    async fn test_is_operator_registered() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let is_registered = chain_reader
            .is_operator_registered(OPERATOR_ADDRESS)
            .await
            .unwrap();

        assert!(is_registered);
    }

    #[tokio::test]
    async fn test_get_staker_shares() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint.clone()).await;

        let (strategies, shares) = chain_reader
            .get_staker_shares(OPERATOR_ADDRESS)
            .await
            .unwrap();

        let expected_strategies: Vec<Address> = vec![];

        assert!(strategies.len() == shares.len());
        assert_eq!(strategies, expected_strategies);
    }

    #[tokio::test]
    async fn test_get_delegated_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint.clone()).await;

        let operator_addr = chain_reader
            .get_delegated_operator(OPERATOR_ADDRESS)
            .await
            .unwrap();

        assert_eq!(operator_addr, OPERATOR_ADDRESS); // operator is delegated to himself
    }

    #[tokio::test]
    async fn test_get_strategy_and_underlying_token() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let (strategy_contract_addr, underlying_token_addr) = chain_reader
            .get_strategy_and_underlying_token(strategy_addr)
            .await
            .unwrap();

        let underlying_token_addr_str = underlying_token_addr.to_string();
        assert_eq!(
            underlying_token_addr_str,
            "0x36C02dA8a0983159322a80FFE9F24b1acfF8B570"
        );

        let strategy_contract_addr_str = strategy_contract_addr.to_string();
        assert_eq!(
            strategy_contract_addr_str,
            "0xeC4cFde48EAdca2bC63E94BB437BbeAcE1371bF3"
        );
    }

    #[tokio::test]
    async fn test_get_allocatable_magnitude_and_max_magnitude() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let allocatable_magnitude = chain_reader
            .get_allocatable_magnitude(OPERATOR_ADDRESS, strategy_addr)
            .await
            .unwrap();

        assert!(allocatable_magnitude > 0);

        // Since the operator has no encumbered magnitude, the max magnitude should be the same as the allocatable magnitude
        let max_magnitude = chain_reader
            .get_max_magnitudes(OPERATOR_ADDRESS, vec![strategy_addr])
            .await
            .unwrap()[0];

        assert_eq!(allocatable_magnitude, max_magnitude);
    }

    #[tokio::test]
    async fn test_get_allocation_info() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        // TODO: fix this, `getStrategyAllocations` call reverts
        let allocation_info = chain_reader
            .get_allocation_info(OPERATOR_ADDRESS, strategy_addr)
            .await
            .unwrap();
        dbg!(allocation_info);
    }

    #[tokio::test]
    async fn test_get_operator_shares() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let operator_shares = chain_reader
            .get_operator_shares(OPERATOR_ADDRESS, vec![strategy_addr])
            .await
            .unwrap();

        assert_eq!(operator_shares.len(), 1);
        assert_eq!(operator_shares[0], U256::from(0));
    }

    #[tokio::test]
    async fn test_get_operators_shares() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let operator_shares = chain_reader
            .get_operators_shares(vec![OPERATOR_ADDRESS], vec![strategy_addr])
            .await
            .unwrap();

        assert_eq!(operator_shares.len(), 1);
        assert_eq!(operator_shares[0].len(), 1);
        assert_eq!(operator_shares[0][0], U256::from(0));
    }

    #[tokio::test]
    async fn test_get_num_operator_sets_for_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let num_operator_sets = chain_reader
            .get_num_operator_sets_for_operator(OPERATOR_ADDRESS)
            .await
            .unwrap();

        assert_eq!(num_operator_sets, U256::from(0));
    }

    #[tokio::test]
    async fn test_get_operator_sets_for_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let operator_sets = chain_reader
            .get_operator_sets_for_operator(OPERATOR_ADDRESS)
            .await
            .unwrap();

        assert_eq!(operator_sets.len(), 0);
    }

    #[tokio::test]
    async fn test_is_operator_registered_with_operator_set() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let operator_set = OperatorSet {
            id: 1,
            avs: Address::ZERO,
        };

        let is_registered = chain_reader
            .is_operator_registered_with_operator_set(OPERATOR_ADDRESS, operator_set)
            .await
            .unwrap();

        assert!(!is_registered);
    }

    #[tokio::test]
    async fn test_get_operators_for_operator_set() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let operator_set = OperatorSet {
            id: 1,
            avs: Address::ZERO,
        };

        let operators = chain_reader
            .get_operators_for_operator_set(operator_set)
            .await
            .unwrap();

        assert_eq!(operators.len(), 0);
    }

    #[tokio::test]
    async fn test_get_num_operators_for_operator_set() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let operator_set = OperatorSet {
            id: 1,
            avs: Address::ZERO,
        };

        let num_operators = chain_reader
            .get_num_operators_for_operator_set(operator_set)
            .await
            .unwrap();

        assert_eq!(num_operators, U256::from(0));
    }

    #[tokio::test]
    async fn test_get_strategies_for_operator_set() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint.clone()).await;

        let operator_set = OperatorSet {
            id: 1,
            avs: Address::ZERO,
        };

        let strategies = chain_reader
            .get_strategies_for_operator_set(operator_set)
            .await
            .unwrap();

        assert_eq!(strategies.len(), 0);
    }

    #[tokio::test]
    async fn test_get_slashable_shares_for_operator_sets() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let operator_set = OperatorSet {
            id: 1,
            avs: Address::ZERO,
        };

        let slashable_shares = chain_reader
            .get_slashable_shares_for_operator_sets(vec![operator_set])
            .await
            .unwrap();

        assert_eq!(slashable_shares.len(), 1);
        assert_eq!(slashable_shares[0].operator_set.id, 1);
        assert_eq!(slashable_shares[0].operators.len(), 0);
        assert_eq!(slashable_shares[0].slashable_stakes.len(), 0);
    }

    #[tokio::test]
    async fn test_get_delegated_and_slashable_shares_for_operator_sets_before() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint.clone()).await;

        let operator_set = OperatorSet {
            id: 1,
            avs: Address::ZERO,
        };

        let current_block_number = get_provider(&http_endpoint)
            .get_block_number()
            .await
            .unwrap() as u32;
        let slashable_shares = chain_reader
            .get_delegated_and_slashable_shares_for_operator_sets_before(
                vec![operator_set],
                current_block_number + 1,
            )
            .await
            .unwrap();

        assert_eq!(slashable_shares.len(), 1);
        assert_eq!(slashable_shares[0].operator_set.id, 1);
        assert_eq!(slashable_shares[0].operators.len(), 0);
        assert_eq!(slashable_shares[0].slashable_stakes.len(), 0);
    }

    #[tokio::test]
    async fn test_get_allocation_delay() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let allocation_delay = chain_reader
            .get_allocation_delay(OPERATOR_ADDRESS)
            .await
            .unwrap();

        assert_eq!(allocation_delay, 1);
    }

    #[tokio::test]
    async fn test_get_registered_sets() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let chain_reader = build_el_chain_reader(http_endpoint.clone()).await;

        let ret = chain_reader
            .get_registered_sets(OPERATOR_ADDRESS)
            .await
            .unwrap();

        assert_eq!(ret.len(), 0);
    }
}
