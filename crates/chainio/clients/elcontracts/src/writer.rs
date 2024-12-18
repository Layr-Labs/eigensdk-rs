use crate::error::ElContractsError;
use crate::reader::ELChainReader;
use alloy_primitives::{Address, FixedBytes, TxHash, U256};
pub use eigen_types::operator::Operator;
use eigen_utils::{
    delegationmanager::DelegationManager,
    erc20::ERC20,
    get_signer,
    irewardscoordinator::{
        IRewardsCoordinator,
        IRewardsCoordinatorTypes::{self, RewardsMerkleClaim},
    },
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
        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let binding = {
            let contract_call = contract_delegation_manager.registerAsOperator(
                operator.address,
                operator.allocation_delay,
                operator.metadata_url.unwrap_or_default(),
            );
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

        let provider = get_signer(&self.signer.clone(), &self.provider);

        let contract_delegation_manager = DelegationManager::new(self.delegation_manager, provider);

        let contract_call_modify_operator_details = contract_delegation_manager
            .modifyOperatorDetails(operator.address, operator.delegation_approver_address);

        let modify_operator_tx = contract_call_modify_operator_details
            .send()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        info!(tx_hash = %modify_operator_tx.tx_hash(), operator = %operator.address, "updated operator details tx");

        let contract_call_update_metadata_uri = contract_delegation_manager
            .updateOperatorMetadataURI(operator.address, operator.metadata_url.unwrap_or_default());

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

        let set_claimer_for_call = contract_rewards_coordinator.setClaimerFor_0(claimer);

        let tx = set_claimer_for_call.send().await?;
        Ok(*tx.tx_hash())
    }

    /// Process a claim for rewards for a given earner address. Checks the claim against a given root
    /// (determined by the root_index on the claim). Earnings are cumulative so earners can claim to
    /// the latest distribution root and the contract will compute the difference between their earning
    /// and claimed amounts. The difference is transferred to the earner address.
    /// If a claimer has not been set (see [`set_claimer_for`]), only the earner can claim. Otherwise, only
    /// the claimer can claim.
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
    /// * `ElContractsError` - if the call to the contract fails. Also fails if no root has been submitted yet.
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

    /// Get the latest claimable distribution root.
    ///
    /// # Returns
    /// * `Result<DistributionRoot, ElContractsError>` - The latest claimable distribution root if the call is successful.
    ///
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_current_claimable_distribution_root(
        &self,
    ) -> Result<IRewardsCoordinatorTypes::DistributionRoot, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let cumulative_claimed_for_root_call = contract_rewards_coordinator
            .getCurrentClaimableDistributionRoot()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::getCurrentClaimableDistributionRootReturn {
            _0: cumulative_claimed_for_root_ret,
        } = cumulative_claimed_for_root_call;

        Ok(cumulative_claimed_for_root_ret)
    }

    /// Get the length of the distribution roots.
    ///
    /// # Returns
    ///
    /// * `Result<U256, ElContractsError>` - The length of the distribution roots if the call is successful,
    ///   otherwise an error.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_distribution_roots_length(&self) -> Result<U256, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let distribution_roots_lenght_call = contract_rewards_coordinator
            .getDistributionRootsLength()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::getDistributionRootsLengthReturn {
            _0: distribution_roots_length,
        } = distribution_roots_lenght_call;

        Ok(distribution_roots_length)
    }

    /// Get the current rewards calculation end timestamp (the timestamp until which rewards have been calculated).
    ///
    /// # Returns
    ///
    /// * `Result<u32, ElContractsError>` - The current rewards calculation
    ///   end timestamp if the call is successful.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_curr_rewards_calculation_end_timestamp(
        &self,
    ) -> Result<u32, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let distribution_roots_lenght_call = contract_rewards_coordinator
            .currRewardsCalculationEndTimestamp()
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::currRewardsCalculationEndTimestampReturn {
            _0: timestamp_return,
        } = distribution_roots_lenght_call;

        Ok(timestamp_return)
    }

    /// Get the root index from a given hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The hash to get the root index from.
    ///
    /// # Returns
    ///
    /// * `Result<u32, ElContractsError>` - The root index if the
    ///   call is successful.
    ///
    /// # Errors
    ///
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_root_index_from_hash(
        &self,
        hash: FixedBytes<32>,
    ) -> Result<u32, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let get_root_index_from_hash_call = contract_rewards_coordinator
            .getRootIndexFromHash(hash)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::getRootIndexFromHashReturn { _0: root_index } =
            get_root_index_from_hash_call;

        Ok(root_index)
    }

    /// Check if a claim would currently pass the validations in `process_claim`
    ///
    /// # Arguments
    /// * `claim` - The claim to check
    ///
    /// # Returns
    /// * `Result<bool, ElContractsError>` - True if the claim would pass the validations, false otherwise
    ///
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails. Also fails if no root has been submitted yet.
    pub async fn check_claim(&self, claim: RewardsMerkleClaim) -> Result<bool, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let check_claim_call = contract_rewards_coordinator
            .checkClaim(claim)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::checkClaimReturn { _0: claim_ret } = check_claim_call;

        Ok(claim_ret)
    }

    /// Get the cumulative claimed amount for a given earner address and token.
    ///
    /// # Arguments
    /// * `earner_address` - The address of the earner.
    /// * `token` - The address of the token.
    ///
    /// # Returns
    /// * `Result<U256, ElContractsError>` - The cumulative claimed amount if the call is successful.
    ///
    /// # Errors
    /// * `ElContractsError` - if the call to the contract fails.
    pub async fn get_cumulative_claimed(
        &self,
        earner_address: Address,
        token: Address,
    ) -> Result<U256, ElContractsError> {
        let provider = get_signer(&self.signer, &self.provider);

        let contract_rewards_coordinator =
            IRewardsCoordinator::new(self.rewards_coordinator, &provider);

        let cumulative_claimed_call = contract_rewards_coordinator
            .cumulativeClaimed(earner_address, token)
            .call()
            .await
            .map_err(ElContractsError::AlloyContractError)?;

        let IRewardsCoordinator::cumulativeClaimedReturn {
            _0: cumulative_claim_ret,
        } = cumulative_claimed_call;

        Ok(cumulative_claim_ret)
    }
}

#[cfg(test)]
mod tests {
    use super::ELChainWriter;
    use crate::reader::ELChainReader;
    use alloy::{hex::FromHex, providers::Provider};
    use alloy_primitives::{address, Address, Bytes, FixedBytes, U256};
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::start_anvil_container,
        anvil_constants::{
            get_allocation_manager_address, get_avs_directory_address,
            get_delegation_manager_address, get_erc20_mock_strategy,
            get_rewards_coordinator_address, get_strategy_manager_address,
        },
        transaction::wait_transaction,
    };
    use eigen_types::operator::Operator;
    use eigen_utils::{
        get_provider, get_signer,
        irewardscoordinator::{
            IRewardsCoordinator,
            IRewardsCoordinatorTypes::{
                EarnerTreeMerkleLeaf, RewardsMerkleClaim, TokenTreeMerkleLeaf,
            },
        },
    };
    use std::str::FromStr;

    const OPERATOR_ADDRESS: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    const OPERATOR_PRIVATE_KEY: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    /// Returns a new instance of ELChainWriter and the address of the delegation manager contract
    ///
    /// # Returns
    ///
    /// A tuple containing an instance of ELChainWriter and the address of the delegation manager contract
    async fn setup_el_chain_reader(http_endpoint: String) -> (ELChainReader, Address) {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        (
            ELChainReader::new(
                get_test_logger().clone(),
                allocation_manager_address,
                delegation_manager_address,
                avs_directory_address,
                http_endpoint,
            ),
            delegation_manager_address,
        )
    }

    async fn new_test_writer(http_endpoint: String) -> ELChainWriter {
        let (el_chain_reader, _) = setup_el_chain_reader(http_endpoint.clone()).await;
        let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;
        let delegation_manager = get_delegation_manager_address(http_endpoint.clone()).await;

        ELChainWriter::new(
            delegation_manager,
            strategy_manager,
            rewards_coordinator,
            el_chain_reader,
            http_endpoint.clone(),
            OPERATOR_PRIVATE_KEY.to_string(),
        )
    }

    async fn new_claim(http_endpoint: &str) -> (FixedBytes<32>, RewardsMerkleClaim) {
        let el_chain_writer = new_test_writer(http_endpoint.to_string()).await;

        let earner_address = address!("25a1b7322f9796b26a4bec125913b34c292b28d6");
        let claim = RewardsMerkleClaim {
            rootIndex: 0,
            earnerIndex: 7,
            earnerTreeProof: Bytes::from_hex("4bf5e16eaabbc36964f1e1639808669420f55d60e51adb7e9695b77145c479fd6777be59643947bb24d78e69d6605bf369c515b479f3a8967dd68a97c5bb4a4a262b28002eeb6cbbffb7e79e5741bf2be189a6073440a62fabcd8af4dbda94e3").unwrap(),
            earnerLeaf: EarnerTreeMerkleLeaf {
                earner: earner_address,
                earnerTokenRoot: FixedBytes::from_hex(
                    "f8e7e20b32aae1d818dcb593b98982841e9a0ed12c161ad603e3ee3948746cba",
                )
                .unwrap(),
            },
            tokenIndices: vec![7],
            tokenTreeProofs: vec![
                Bytes::from_hex("3cd04e8fc6f23812c570fe12292a30bb9e105e00f5913ac4b4938f23e65d8d10e6b1403d58c9d5450952e7d96c81305dad9fb966e8a27d3a42058e3958a0d30033148e91b455542d05deb81b8305b672e742cd3145f7022a0089bad2e6af9173").unwrap(),
            ],
            tokenLeaves: vec![TokenTreeMerkleLeaf {
                token: address!("7fbfdd1dfd80730385aee232cc9f79b8ae12a654"),
                cumulativeEarnings: U256::from_str("3000000000000000000").unwrap(),
            }],
        };

        // Using test data taken from
        // https://github.com/Layr-Labs/eigenlayer-contracts/blob/a888a1cd1479438dda4b138245a69177b125a973/src/test/test-data/rewardsCoordinator/processClaimProofs_MaxEarnerAndLeafIndices.json
        let root = FixedBytes::from_hex(
            "37550707c80f3d8907c467999730e52127ab89be3f17a5017a3f1ffb73a1445f",
        )
        .unwrap();

        let key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let rewards_coordinator = IRewardsCoordinator::new(
            get_rewards_coordinator_address(http_endpoint.to_string()).await,
            get_signer(key, http_endpoint),
        );
        let curr_rewards_calculation_end_timestamp = el_chain_writer
            .get_curr_rewards_calculation_end_timestamp()
            .await
            .unwrap();
        let submit_tx = rewards_coordinator
            .submitRoot(root, curr_rewards_calculation_end_timestamp + 1)
            .send()
            .await
            .unwrap();
        let submit_status = submit_tx.get_receipt().await.unwrap().status();
        assert!(submit_status);

        (root, claim)
    }

    #[tokio::test]
    async fn test_register_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let (el_chain_reader, _delegation_manager_address) =
            setup_el_chain_reader(http_endpoint.clone()).await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        let operator = Operator {
            address: OPERATOR_ADDRESS, // can only register the address corresponding to the signer used in the writer
            staker_opt_out_window_blocks: 3,
            delegation_approver_address: OPERATOR_ADDRESS,
            metadata_url: Some("metadata_uri".to_string()),
            allocation_delay: 1,
        };
        el_chain_writer
            .register_as_operator(operator)
            .await
            .unwrap();

        let is_registered = el_chain_reader
            .is_operator_registered(OPERATOR_ADDRESS)
            .await
            .unwrap();
        assert!(is_registered);
    }

    #[tokio::test]
    async fn test_register_and_update_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);

        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        let operator = Operator {
            address: OPERATOR_ADDRESS,
            staker_opt_out_window_blocks: 3,
            delegation_approver_address: OPERATOR_ADDRESS,
            metadata_url: Some("eigensdk-rs".to_string()),
            allocation_delay: 1,
        };

        // First test: register as an operator
        let tx_hash = el_chain_writer
            .register_as_operator(operator)
            .await
            .unwrap();

        let receipt = provider.get_transaction_receipt(tx_hash).await.unwrap();
        assert!(receipt.unwrap().status());

        let operator_modified = Operator {
            address: OPERATOR_ADDRESS,
            staker_opt_out_window_blocks: 3,
            delegation_approver_address: Address::ZERO,
            metadata_url: Some("new-metadata".to_string()),
            allocation_delay: 1,
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
    async fn test_check_claim() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        let (_, claim) = new_claim(&http_endpoint).await;

        let distribution_roots_length = el_chain_writer
            .get_distribution_roots_length()
            .await
            .unwrap();
        assert_eq!(distribution_roots_length, U256::from(1));

        let valid_claim = el_chain_writer.check_claim(claim.clone()).await.unwrap();
        assert!(valid_claim);
    }

    #[tokio::test]
    async fn test_get_distribution_roots_length() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;
        new_claim(&http_endpoint).await;

        let distribution_roots_length_ret = el_chain_writer
            .get_distribution_roots_length()
            .await
            .unwrap();

        assert_eq!(distribution_roots_length_ret, U256::from(1));
    }

    #[tokio::test]
    async fn test_get_root_index_from_hash() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;
        let (root, _) = new_claim(&http_endpoint).await;

        let index = el_chain_writer
            .get_root_index_from_hash(root)
            .await
            .unwrap();

        assert_eq!(index, 0);
    }

    #[tokio::test]
    async fn test_get_cumulative_claimed() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        let earner_address = address!("F2288D736d27C1584Ebf7be5f52f9E4d47251AeE");
        let (_, _, token_address) = el_chain_writer
            .el_chain_reader
            .get_strategy_and_underlying_erc20_token(
                get_erc20_mock_strategy(http_endpoint.clone()).await,
            )
            .await
            .unwrap();

        let cumulative_claimed_ret = el_chain_writer
            .get_cumulative_claimed(earner_address, token_address)
            .await
            .unwrap();

        // No claims so cumulative claimed should be zero
        assert_eq!(cumulative_claimed_ret, U256::from(0));
    }

    #[tokio::test]
    async fn test_get_cumulative_claimed_for_root() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;
        let (root, _) = new_claim(&http_endpoint).await;

        let distribution_root = el_chain_writer
            .get_current_claimable_distribution_root()
            .await
            .unwrap();

        assert_eq!(distribution_root.root, root);
    }
}
