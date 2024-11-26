use crate::error::ElContractsError;
use alloy::providers::Provider;
use alloy_primitives::{ruint::aliases::U256, Address, FixedBytes};
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::Operator;
use eigen_utils::{
    allocationmanager::AllocationManager::{self, OperatorSet},
    avsdirectory::AVSDirectory,
    delegationmanager::DelegationManager,
    erc20::ERC20,
    get_provider,
    istrategy::IStrategy,
};

#[derive(Debug, Clone)]
pub struct ELChainReader {
    _logger: SharedLogger,
    allocation_manager: Address,
    delegation_manager: Address,
    avs_directory: Address,
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
        provider: String,
    ) -> Self {
        ELChainReader {
            _logger,
            allocation_manager,
            delegation_manager,
            avs_directory,
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

        Ok(Self {
            _logger,
            avs_directory,
            allocation_manager,
            delegation_manager,
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
            underlying_token_addr,
            *contract_ierc20.address(),
        ))
    }

    /// Get the operator's details
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator's address
    ///
    /// # Returns
    ///
    /// * `Operator` - The operator's details
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn get_operator_details(
        &self,
        operator: Address,
    ) -> Result<Operator, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_delegation_manager =
            DelegationManager::new(self.delegation_manager, &provider);

        let operator_det = contract_delegation_manager
            .operatorDetails(operator)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::operatorDetailsReturn {
            _0: operator_details,
        } = operator_det;

        let AllocationManager::getAllocationDelayReturn {
            _0: is_set,
            _1: delay,
        } = AllocationManager::new(self.allocation_manager, &provider)
            .getAllocationDelay(operator)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;
        let allocation_delay = if is_set { delay } else { 0 };

        Ok(Operator {
            address: operator,
            staker_opt_out_window_blocks: operator_details.__deprecated_stakerOptOutWindowBlocks,
            metadata_url: None,
            allocation_delay,
            delegation_approver_address: operator_details.delegationApprover,
        })
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
    /// * `Vec<(Address, U96)>` - An array of tuples containing the strategy address and the amount of shares the staker has in that strategy
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
            .map_err(ElContractsError::AlloyContractError)?;

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

    // Returns the number of operator sets that an operator is part of
    // Doesn't include M2 AVSs
    pub async fn get_num_operator_sets_for_operator(
        &self,
        operator_addr: Address,
    ) -> Result<U256, ElContractsError> {
        self.get_operator_sets_for_operator(operator_addr)
            .await
            .map(|operator_sets| U256::from(operator_sets.len() as u64))
    }

    // Return the list of operator sets that an operator is part of. Doesn't include M2 AVSs
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

    // Returns if an operator is registered with a specific operator set
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

    // Returns the list of operators in a specific operator set.
    // Not supported for M2 AVSs
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

    // Returns the number of operators in a specific operator set
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

    // Returns the list of strategies that an operator set takes into account
    // Not supported for M2 AVSs
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

    // Returns the strategies the operatorSets take into account, their
    // operators, and the minimum amount of shares that are slashable by the operatorSets.
    // Not supported for M2 AVSs
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

    // Returns the strategies the operatorSets take into account, their
    // operators, and the minimum amount of shares that multiple operators delegated to them and slashable by the
    // operatorSets before a given timestamp.
    // Timestamp must be in the future. Used to underestimate future slashable stake.
    // Not supported for M2 AVSs
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
}

// TODO: move to types.rs?
pub struct OperatorSetStakes {
    pub operator_set: OperatorSet,
    pub strategies: Vec<Address>,
    pub operators: Vec<Address>,
    pub slashable_stakes: Vec<Vec<U256>>,
}

pub struct AllocationInfo {
    pub current_magnitude: U256,
    pub pending_diff: U256,
    pub effect_block: u32,
    pub operator_set: OperatorSet,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::eips::eip1898::BlockNumberOrTag::Number;
    use alloy::providers::Provider;
    use alloy_primitives::{address, keccak256, Address, FixedBytes, U256};
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::start_anvil_container,
        anvil_constants::{get_delegation_manager_address, get_service_manager_address},
    };
    use eigen_utils::{
        avsdirectory::AVSDirectory,
        avsdirectory::AVSDirectory::calculateOperatorAVSRegistrationDigestHashReturn,
        delegationmanager::DelegationManager,
        delegationmanager::DelegationManager::calculateDelegationApprovalDigestHashReturn,
        mockavsservicemanager::MockAvsServiceManager,
    };
    use std::str::FromStr;

    const OPERATOR_ADDRESS: &str = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720";

    async fn build_el_chain_reader(http_endpoint: String) -> ELChainReader {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let delegation_manager_contract =
            DelegationManager::new(delegation_manager_address, get_provider(&http_endpoint));
        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let service_manager_contract =
            MockAvsServiceManager::new(service_manager_address, get_provider(&http_endpoint));
        let avs_directory_address_return = service_manager_contract
            .avsDirectory()
            .call()
            .await
            .unwrap();
        let MockAvsServiceManager::avsDirectoryReturn {
            _0: avs_directory_address,
        } = avs_directory_address_return;
        let allocation_manager_address = delegation_manager_contract
            .allocationManager()
            .call()
            .await
            .unwrap()
            ._0;

        ELChainReader::new(
            get_test_logger(),
            allocation_manager_address,
            delegation_manager_address,
            avs_directory_address,
            http_endpoint,
        )
    }

    #[tokio::test]
    #[ignore] // TODO: this test is ignored until the anvil state is updated to slashing
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
            .get_block_by_number(Number(current_block_number), true)
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
    #[ignore] // TODO: this test is ignored until the anvil state is updated to slashing
    async fn test_calculate_operator_avs_registration_digest_hash() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let operator: Address = address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2");
        let avs = Address::from_slice(&keccak256("avs ")[0..20]);
        let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let current_block_number = provider.get_block_number().await.unwrap();
        let block_info = provider
            .get_block_by_number(Number(current_block_number), true)
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
    #[ignore] // TODO: this test is ignored until the anvil state is updated to slashing
    async fn test_get_operator_details() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let operator_addr = Address::from_str(OPERATOR_ADDRESS).unwrap();
        let chain_reader = build_el_chain_reader(http_endpoint).await;

        let operator = chain_reader
            .get_operator_details(operator_addr)
            .await
            .unwrap();

        assert!(operator.metadata_url.is_none());
        println!("{:?}", operator.metadata_url);
    }
}
