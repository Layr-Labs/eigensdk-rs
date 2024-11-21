use crate::error::ElContractsError;
use alloy_primitives::{Address, FixedBytes, U256};
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::Operator;
use eigen_utils::{
    get_provider,
    {
        avsdirectory::AVSDirectory, delegationmanager::DelegationManager, erc20::ERC20,
        istrategy::IStrategy,
    },
};

#[derive(Debug, Clone)]
pub struct ELChainReader {
    logger: SharedLogger,
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
        logger: SharedLogger,
        allocation_manager: Address,
        delegation_manager: Address,
        avs_directory: Address,
        provider: String,
    ) -> Self {
        ELChainReader {
            logger,
            allocation_manager,
            delegation_manager,
            avs_directory,
            provider,
        }
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

        let DelegationManager::operatorSharesReturn { _0: shares } = operator_shares_in_strategy;
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

        Ok(Operator {
            address: operator,
            staker_opt_out_window_blocks: operator_details.__deprecated_stakerOptOutWindowBlocks,
            metadata_url: None,
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

        let DelegationManager::delegatedToReturn {
            _0: operator_address,
        } = delegated;

        Ok(operator_address)
    }
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
