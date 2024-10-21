use crate::error::ElContractsError;
use crate::reader::ELChainReader;
use alloy_primitives::{Address, FixedBytes, TxHash, U256};
pub use eigen_types::operator::Operator;
use eigen_utils::{
    delegationmanager::{
        DelegationManager::{self},
        IDelegationManager::OperatorDetails,
    },
    erc20::ERC20,
    get_signer,
    irewardscoordinator::IRewardsCoordinator::{self, RewardsMerkleClaim},
    strategymanager::StrategyManager,
};

use tracing::info;

/// Gas limit for registerAsOperator in [`DelegationManager`]
pub const GAS_LIMIT_REGISTER_AS_OPERATOR_DELEGATION_MANAGER: u128 = 300000;

/// Chain Writer to interact with EigenLayer contracts onchain
#[derive(Debug, Clone)]
pub struct ELChainWriter {
    delegation_manager: Address,
    strategy_manager: Address,
    rewards_coordinator: Address,
    el_chain_reader: ELChainReader,
    provider: String,
    signer: String,
}

impl ELChainWriter {
    pub fn new(
        delegation_manager: Address,
        strategy_manager: Address,
        rewards_coordinator: Address,
        el_chain_reader: ELChainReader,
        provider: String,
        signer: String,
    ) -> Self {
        Self {
            delegation_manager,
            strategy_manager,
            rewards_coordinator,
            el_chain_reader,
            provider,
            signer,
        }
    }

    /// Register an operator to EigenLayer, and wait for the transaction to be mined.
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator to register
    ///
    /// # Returns
    ///
    /// * `FixedBytes<32>` - The transaction hash if successful, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn register_as_operator(
        &self,
        operator: Operator,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        info!("registering operator {:?} to EigenLayer", operator.address);
        let op_details = OperatorDetails {
            __deprecated_earningsReceiver: operator.earnings_receiver_address,
            delegationApprover: operator.delegation_approver_address,
            stakerOptOutWindowBlocks: operator.staker_opt_out_window_blocks,
        };
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let binding = {
            let contract_call = contract_delegation_manager
                .registerAsOperator(op_details, operator.metadata_url.unwrap_or_default());
            contract_call.gas(300000)
        };

        let binding_tx = binding
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let receipt = binding_tx
            .get_receipt()
            .await
            .map_err(ElContractsError::AlloyPendingTransactionError)?;

        let tx_status = receipt.status();
        let hash = receipt.transaction_hash;
        if tx_status {
            info!(tx_hash = %receipt.transaction_hash, "tx successfully included");
        } else {
            info!(tx_hash = %receipt.transaction_hash, "failed to register operator");
        };
        Ok(hash)
    }

    /// Update operator details on EigenLayer
    ///
    /// # Arguments
    ///
    /// * `operator` - The operator to update
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash if successful, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn update_operator_details(
        &self,
        operator: Operator,
    ) -> Result<TxHash, ElContractsError> {
        info!(
            "updating operator detils of operator {:?} to EigenLayer",
            operator.address
        );
        let operator_details = OperatorDetails {
            __deprecated_earningsReceiver: operator.earnings_receiver_address,
            delegationApprover: operator.delegation_approver_address,
            stakerOptOutWindowBlocks: operator.staker_opt_out_window_blocks,
        };
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let contract_call_modify_operator_details =
            contract_delegation_manager.modifyOperatorDetails(operator_details);

        let modify_operator_tx = contract_call_modify_operator_details
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        info!(tx_hash = %modify_operator_tx.tx_hash(), operator = %operator.address, "updated operator details tx");

        let contract_call_update_metadata_uri = contract_delegation_manager
            .updateOperatorMetadataURI(operator.metadata_url.unwrap_or_default());

        let metadata_tx = contract_call_update_metadata_uri.send().await?;

        Ok(*metadata_tx.tx_hash())
    }

    /// Deposit ERC20 tokens into a strategy on EigenLayer
    ///
    /// # Arguments
    ///
    /// * `strategy_addr` - The address of the strategy to deposit into
    /// * `amount` - The amount of tokens to deposit
    ///
    /// # Returns
    ///
    /// * `TxHash` - The transaction hash if successful, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn deposit_erc20_into_strategy(
        &self,
        strategy_addr: Address,
        amount: U256,
    ) -> Result<TxHash, ElContractsError> {
        info!("depositing {amount:?} tokens into strategy {strategy_addr:?}");
        let tokens = self
            .el_chain_reader
            .get_strategy_and_underlying_erc20_token(strategy_addr)
            .await?;
        let (_, underlying_token_contract, underlying_token) = tokens;
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_underlying_token = ERC20::new(underlying_token_contract, &provider);

        let contract_call = contract_underlying_token.approve(self.strategy_manager, amount);

        let _approve = contract_call.send().await?;

        let contract_strategy_manager = StrategyManager::new(self.strategy_manager, &provider);

        let deposit_contract_call =
            contract_strategy_manager.depositIntoStrategy(strategy_addr, underlying_token, amount);

        let tx = deposit_contract_call.send().await?;

        info!("deposited {amount:?} tokens into strategy {strategy_addr:?}");
        Ok(*tx.tx_hash())
    }

    /// Set a claimer for a given address on EigenLayer
    ///
    /// # Arguments
    ///
    /// * `claimer` - The address to set as the claimer
    ///
    /// # Returns
    ///
    /// * `FixedBytes<32>` - The transaction hash if the operation is sent, otherwise an error
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails
    pub async fn set_claimer_for(
        &self,
        claimer: Address,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let set_claimer_for_call = contract_rewards_coordinator.setClaimerFor(claimer);

        let tx = set_claimer_for_call.send().await?;
        Ok(*tx.tx_hash())
    }

    /// Process a claim for rewards to a given address.
    /// This function interacts with the RewardsCoordinator contract to execute the claim operation for a given address.
    ///
    /// # Arguments
    ///
    /// * `earnerAddress` - The address of the earner for whom to process the claim.
    /// * `claim` - The RewardsMerkleClaim object containing the claim.
    ///
    /// # Returns
    ///
    /// * `Result<FixedBytes<32>, ElContractsError>` - The transaction hash if the claim is sent, otherwise an error.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn process_claim(
        &self,
        earner_address: Address,
        claim: RewardsMerkleClaim,
    ) -> Result<FixedBytes<32>, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let process_claim_call = contract_rewards_coordinator.processClaim(claim, earner_address);

        let tx = process_claim_call.send().await?;
        Ok(*tx.tx_hash())
    }
}

#[cfg(test)]
mod tests {
    use super::ELChainWriter;
    use crate::reader::ELChainReader;
    use alloy::providers::Provider;
    use alloy_primitives::{address, Address, FixedBytes, U256};
    use alloy_signer_local::PrivateKeySigner;
    use anvil_constants::CONTRACTS_REGISTRY;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::start_anvil_container,
        anvil_constants::{
            self, get_delegation_manager_address, get_erc20_mock_strategy,
            get_rewards_coordinator_address, get_service_manager_address,
            get_strategy_manager_address,
        },
        transaction::wait_transaction,
    };
    use eigen_types::operator::Operator;
    use eigen_utils::{
        contractsregistry::ContractsRegistry::{self, get_test_valuesReturn},
        delegationmanager::DelegationManager,
        get_provider,
        irewardscoordinator::IRewardsCoordinator::{EarnerTreeMerkleLeaf, RewardsMerkleClaim},
        mockavsservicemanager::MockAvsServiceManager,
    };
    use std::str::FromStr;

    /// Returns a new instance of ELChainWriter and the address of the delegation manager contract
    ///
    /// # Returns
    ///
    /// A tuple containing an instance of ELChainWriter and the address of the delegation manager contract
    async fn setup_el_chain_reader(http_endpoint: String) -> (ELChainReader, Address) {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let delegation_manager_contract = DelegationManager::new(
            delegation_manager_address,
            get_provider(http_endpoint.as_str()),
        );
        let slasher_address_return = delegation_manager_contract.slasher().call().await.unwrap();
        let DelegationManager::slasherReturn {
            _0: slasher_address,
        } = slasher_address_return;

        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let service_manager_contract = MockAvsServiceManager::new(
            service_manager_address,
            get_provider(http_endpoint.as_str()),
        );
        let avs_directory_address_return = service_manager_contract
            .avsDirectory()
            .call()
            .await
            .unwrap();

        let MockAvsServiceManager::avsDirectoryReturn {
            _0: avs_directory_address,
        } = avs_directory_address_return;

        (
            ELChainReader::new(
                get_test_logger().clone(),
                slasher_address,
                delegation_manager_address,
                avs_directory_address,
                http_endpoint,
            ),
            delegation_manager_address,
        )
    }

    async fn new_test_writer(http_endpoint: String) -> ELChainWriter {
        let (el_chain_reader, _) = setup_el_chain_reader(http_endpoint.clone()).await;
        let operator_addr = Address::from_str("90F79bf6EB2c4f870365E785982E1f101E93b906").unwrap();
        let operator_private_key =
            "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6".to_string();
        let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;

        ELChainWriter::new(
            operator_addr,
            strategy_manager,
            rewards_coordinator,
            el_chain_reader,
            http_endpoint.clone(),
            operator_private_key,
        )
    }

    #[tokio::test]
    async fn test_register_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);

        let (el_chain_reader, _delegation_manager_address) =
            setup_el_chain_reader(http_endpoint).await;

        let operator_pvt_key = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
        let operator: PrivateKeySigner = (operator_pvt_key)
            .parse()
            .expect("failed to generate wallet");

        let contract_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, provider);
        // Use these value in tests when needed
        let operator_index = "1".parse().unwrap();
        let get_test_values_return = contract_registry
            .get_test_values("test_register_operator".to_string(), operator_index)
            .call()
            .await
            .unwrap();
        let get_test_valuesReturn {
            _0: _timestamp,
            _1: _blocknumber,
            _2: _index,
        } = get_test_values_return;

        // operator who registered at index 1
        let operator_address = operator.address();
        assert!(el_chain_reader
            .is_operator_registered(operator_address)
            .await
            .unwrap());
    }

    #[tokio::test]
    async fn test_register_and_update_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);

        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        // define an operator
        let wallet = PrivateKeySigner::from_str(
            "bead471191bea97fc3aeac36c9d74c895e8a6242602e144e43152f96219e96e8",
        )
        .expect("no key");

        let operator = Operator {
            address: wallet.address(),
            earnings_receiver_address: wallet.address(),
            delegation_approver_address: wallet.address(),
            staker_opt_out_window_blocks: 3,
            metadata_url: Some("eigensdk-rs".to_string()),
        };

        // First test: register as an operator
        let tx_hash = el_chain_writer
            .register_as_operator(operator)
            .await
            .unwrap();

        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();
        assert!(receipt.unwrap().status());

        let wallet_modified = PrivateKeySigner::from_str(
            "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
        )
        .expect("no key");

        let operator_modified = Operator {
            address: wallet_modified.address(),
            earnings_receiver_address: wallet_modified.address(),
            delegation_approver_address: wallet_modified.address(),
            staker_opt_out_window_blocks: 3,
            metadata_url: Some("eigensdk-rs".to_string()),
        };

        // Second test: update operator details
        let tx_hash = el_chain_writer
            .update_operator_details(operator_modified)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_deposit_erc20_into_strategy() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        let amount = U256::from_str("100").unwrap();
        let strategy_addr = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let tx_hash = el_chain_writer
            .deposit_erc20_into_strategy(strategy_addr, amount)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_set_claimer_for() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        let claimer = address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2");

        let tx_hash = el_chain_writer.set_claimer_for(claimer).await.unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    #[tokio::test]
    async fn test_process_claim() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        let earner_address = address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2");
        let claim = RewardsMerkleClaim {
            rootIndex: 0,
            earnerIndex: 0,
            earnerTreeProof: vec![].into(),
            earnerLeaf: EarnerTreeMerkleLeaf {
                earner: address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2"),
                earnerTokenRoot: FixedBytes::from([0; 32]),
            },
            tokenIndices: vec![],
            tokenTreeProofs: vec![],
            tokenLeaves: vec![],
        };

        let tx_hash = el_chain_writer
            .process_claim(earner_address, claim)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }
}
