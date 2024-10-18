use crate::error::ElContractsError;
use alloy_primitives::{Address, FixedBytes, U256};
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::Operator;
use eigen_utils::{
    get_provider,
    {
        avsdirectory::AVSDirectory, delegationmanager::DelegationManager, erc20::ERC20,
        islasher::ISlasher, istrategy::IStrategy,
    },
};

#[derive(Debug, Clone)]
pub struct ELChainReader {
    _logger: SharedLogger,
    slasher: Address,
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
    /// * `slasher` - The address of the slasher contract.
    /// * `delegation_manager` - The address of the delegation manager contract.
    /// * `avs_directory` - The address of the avs directory contract.
    /// * `provider` - The provider to use for the RPC client.
    ///
    /// # Returns
    ///
    /// A new `ELChainReader` instance.
    pub fn new(
        _logger: SharedLogger,
        slasher: Address,
        delegation_manager: Address,
        avs_directory: Address,
        provider: String,
    ) -> Self {
        ELChainReader {
            _logger,
            slasher,
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

        let slasher = contract_delegation_manager
            .slasher()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let DelegationManager::slasherReturn { _0: slasher_addr } = slasher;

        Ok(Self {
            _logger,
            avs_directory,
            slasher: slasher_addr,
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

        let DelegationManager::operatorSharesReturn { _0: shares } = operator_shares_in_strategy;
        Ok(shares)
    }

    /// Check if the operator is frozen
    ///
    /// # Arguments
    ///
    /// * `operator_addr` - The operator's address
    ///
    /// # Returns
    ///
    /// * `bool` - True if the operator is frozen, false otherwise
    ///
    /// # Errors
    pub async fn operator_is_frozen(
        &self,
        operator_addr: Address,
    ) -> Result<bool, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_slasher = ISlasher::new(self.slasher, provider);

        let operator_is_frozen = contract_slasher
            .isFrozen(operator_addr)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let ISlasher::isFrozenReturn { _0: is_froze } = operator_is_frozen;
        Ok(is_froze)
    }

    /// Check if the service manager can slash the operator
    ///
    /// # Arguments
    ///
    /// * `operator_addr` - The operator's address
    /// * `service_manager_addr` - The service manager's address
    ///
    /// # Returns
    ///
    /// * `u32` - The block number until the operator can be slashed
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails    
    pub async fn service_manager_can_slash_operator_until_block(
        &self,
        operator_addr: Address,
        service_manager_addr: Address,
    ) -> Result<u32, ElContractsError> {
        let provider = get_provider(&self.provider);

        let contract_slasher = ISlasher::new(self.slasher, provider);

        let service_manager_can_slash_operator_until_block = contract_slasher
            .contractCanSlashOperatorUntilBlock(operator_addr, service_manager_addr)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let ISlasher::contractCanSlashOperatorUntilBlockReturn { _0: can_slash } =
            service_manager_can_slash_operator_until_block;

        Ok(can_slash)
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
            earnings_receiver_address: operator_details.__deprecated_earningsReceiver,
            delegation_approver_address: operator_details.delegationApprover,
            staker_opt_out_window_blocks: operator_details.stakerOptOutWindowBlocks,
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
        anvil_constants::{
            get_delegation_manager_address, get_erc20_mock_strategy, get_service_manager_address,
        },
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
        let slasher_address_return = delegation_manager_contract.slasher().call().await.unwrap();
        let DelegationManager::slasherReturn {
            _0: slasher_address,
        } = slasher_address_return;
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

        ELChainReader::new(
            get_test_logger(),
            slasher_address,
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
    async fn check_strategy_shares_and_operator_frozen() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let operator_addr = Address::from_str(OPERATOR_ADDRESS).unwrap();
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;

        let chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let shares = chain_reader
            .get_operator_shares_in_strategy(operator_addr, strategy_addr)
            .await
            .unwrap();

        let zero = U256::ZERO;
        assert!(shares > zero);

        // test if operator is frozen
        let frozen = chain_reader
            .operator_is_frozen(operator_addr)
            .await
            .unwrap();
        assert!(!frozen);

        let service_manager_address = get_service_manager_address(http_endpoint).await;
        let ret_can_slash = chain_reader
            .service_manager_can_slash_operator_until_block(operator_addr, service_manager_address)
            .await
            .unwrap();

        println!("ret_can_slash: {ret_can_slash}");
        assert!(ret_can_slash == 0);
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
