use crate::error::ElContractsError;
use crate::reader::ELChainReader;
use alloy_primitives::{Address, FixedBytes, TxHash, U256};
use eigen_common::get_signer;
pub use eigen_types::operator::Operator;
use eigen_utils::deploy::irewardscoordinator::IRewardsCoordinator::{self, RewardsMerkleClaim};
use eigen_utils::middleware::{
    delegationmanager::{
        DelegationManager::{self},
        IDelegationManager::OperatorDetails,
    },
    erc20::ERC20,
    strategymanager::StrategyManager,
};

use tracing::info;

/// Gas limit for registerAsOperator in [`DelegationManager`]
pub const GAS_LIMIT_REGISTER_AS_OPERATOR_DELEGATION_MANAGER: u128 = 300000;

/// Chain Writer to interact with EigenLayer contracts onchain
#[derive(Debug, Clone)]
pub struct ELChainWriter {
    strategy_manager: Address,
    rewards_coordinator: Address,
    el_chain_reader: ELChainReader,
    provider: String,
    signer: String,
}

impl ELChainWriter {
    pub fn new(
        strategy_manager: Address,
        rewards_coordinator: Address,
        el_chain_reader: ELChainReader,
        provider: String,
        signer: String,
    ) -> Self {
        Self {
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

        let contract_delegation_manager =
            DelegationManager::new(self.el_chain_reader.delegation_manager, provider);

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

        let contract_delegation_manager =
            DelegationManager::new(self.el_chain_reader.delegation_manager, provider);

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
    use alloy::{providers::Provider, sol_types::SolValue};
    use alloy_primitives::{address, keccak256, Address, FixedBytes, U256, U8};
    use alloy_signer_local::PrivateKeySigner;
    use anvil_constants::CONTRACTS_REGISTRY;
    use eigen_common::{get_provider, get_signer};
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::{set_account_balance, start_anvil_container},
        anvil_constants::{
            self, get_delegation_manager_address, get_erc20_mock_strategy,
            get_rewards_coordinator_address, get_service_manager_address,
            get_strategy_manager_address, ANVIL_FIRST_ADDRESS,
        },
        transaction::wait_transaction,
    };
    use eigen_types::operator::Operator;
    use eigen_utils::{
        deploy::{
            contractsregistry::ContractsRegistry::{self, get_test_valuesReturn},
            irewardscoordinator::IRewardsCoordinator::{
                self, getDistributionRootsLengthReturn, EarnerTreeMerkleLeaf, RewardsMerkleClaim,
                TokenTreeMerkleLeaf,
            },
            mockavsservicemanager::MockAvsServiceManager,
            mockerc20::MockERC20,
        },
        middleware::delegationmanager::DelegationManager,
    };
    use serial_test::serial;
    use std::str::FromStr;

    /// Address of an unregistered account in EigenLayer. Used for testing the operator registration.
    pub const UNREGISTERED_ACCOUNT_ADDRESS: &str = "0x480EbE61a1881D1732ec47A8f6778fCC4Ee820dF";
    /// Private key of an unregistered account in EigenLayer. Used for testing the operator registration.
    pub const UNREGISTERED_ACCOUNT_PRIVATE_KEY: &str =
        "98540e7b77adc6201562c7591cb3b9c203b8e270bbd3b8a49f96c54042c44c3b";

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

    /// Creates an instance of ELChainWriter.
    ///
    /// # Arguments
    ///
    /// * `http_endpoint` - The HTTP endpoint used to send transactions.
    /// * `private_key` - The private key used to sign transactions.
    ///
    /// # Returns
    ///
    /// A new instance of ELChainWriter.
    async fn new_test_writer_with_private_key(
        http_endpoint: String,
        private_key: String,
    ) -> ELChainWriter {
        let (el_chain_reader, _) = setup_el_chain_reader(http_endpoint.clone()).await;
        let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;

        ELChainWriter::new(
            strategy_manager,
            rewards_coordinator,
            el_chain_reader,
            http_endpoint.clone(),
            private_key,
        )
    }

    async fn new_test_writer(http_endpoint: String) -> ELChainWriter {
        let private_key =
            "7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6".to_string();
        new_test_writer_with_private_key(http_endpoint, private_key).await
    }

    #[tokio::test]
    #[serial]
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
    #[serial]
    async fn test_register_and_update_operator() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let provider = get_provider(&http_endpoint);

        // Use a different account since all funded accounts are already registered as operators
        let private_key = UNREGISTERED_ACCOUNT_PRIVATE_KEY.to_string();
        let el_chain_writer =
            new_test_writer_with_private_key(http_endpoint.clone(), private_key).await;

        // Fund the unregistered account
        set_account_balance(&_container, UNREGISTERED_ACCOUNT_ADDRESS).await;

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
    #[serial]
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
    #[serial]
    async fn test_set_claimer_for() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let el_chain_writer = new_test_writer(http_endpoint.clone()).await;

        let claimer = address!("5eb15C0992734B5e77c888D713b4FC67b3D679A2");

        let tx_hash = el_chain_writer.set_claimer_for(claimer).await.unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }

    /// The claim can be submitted from [`ANVIL_FIRST_PRIVATE_KEY`]
    /// This is taken from the slashing PR, it is slightly changed since
    /// some chain reader functions are not available in this branch yet.
    pub async fn new_claim(
        http_endpoint: &str,
        cumulative_earnings: U256,
    ) -> (FixedBytes<32>, RewardsMerkleClaim) {
        let signer = get_signer(ANVIL_FIRST_PRIVATE_KEY, http_endpoint);
        let rewards_coordinator_address =
            get_rewards_coordinator_address(http_endpoint.to_string()).await;

        let (el_chain_reader, _) = setup_el_chain_reader(http_endpoint.to_string()).await;
        let mock_strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        let (_, token_address, _) = el_chain_reader
            .get_strategy_and_underlying_erc20_token(mock_strategy)
            .await
            .unwrap();

        // Initialize the rewards coordinator bindings
        let rewards_coordinator = IRewardsCoordinator::new(rewards_coordinator_address, &signer);

        // Mint tokens for the rewards coordinator
        let token = MockERC20::new(token_address, &signer);
        let receipt = token
            .mint(rewards_coordinator_address, cumulative_earnings)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        assert!(receipt.status());

        // Generate token tree leaf
        // For the tree structure, see https://github.com/Layr-Labs/eigenlayer-contracts/blob/a888a1cd1479438dda4b138245a69177b125a973/docs/core/RewardsCoordinator.md#rewards-merkle-tree-structure
        let earner_address = ANVIL_FIRST_ADDRESS;
        let token_leaves = vec![TokenTreeMerkleLeaf {
            token: token_address,
            cumulativeEarnings: cumulative_earnings,
        }];
        // Hash token tree leaf to get root
        let encoded_token_leaf = [
            // uint8 internal constant TOKEN_LEAF_SALT = 1;
            U8::from(1).to_be_bytes_vec(),
            token_leaves[0].token.abi_encode_packed(),
            token_leaves[0].cumulativeEarnings.abi_encode_packed(),
        ]
        .concat();
        let earner_token_root = keccak256(encoded_token_leaf);

        // Generate earner tree leaf
        let earner_leaf = EarnerTreeMerkleLeaf {
            earner: earner_address,
            earnerTokenRoot: earner_token_root,
        };
        // Hash earner tree leaf to get root
        let encoded_earner_leaf = [
            // uint8 internal constant EARNER_LEAF_SALT = 0;
            U8::from(0).to_be_bytes_vec(),
            earner_leaf.earner.abi_encode_packed(),
            earner_leaf.earnerTokenRoot.abi_encode_packed(),
        ]
        .concat();
        let earner_tree_root = keccak256(encoded_earner_leaf);

        // Fetch the next root index from contract
        let distribution_roots_length_return = rewards_coordinator
            .getDistributionRootsLength()
            .call()
            .await
            .unwrap();
        let getDistributionRootsLengthReturn {
            _0: next_root_index,
        } = distribution_roots_length_return;
        // Construct the claim
        let claim = RewardsMerkleClaim {
            rootIndex: next_root_index.try_into().unwrap(),
            earnerIndex: 0,
            // Empty proof because leaf == root
            earnerTreeProof: vec![].into(),
            earnerLeaf: earner_leaf,
            tokenIndices: vec![0],
            tokenTreeProofs: vec![
                // Empty proof because leaf == root
                vec![].into(),
            ],
            tokenLeaves: token_leaves,
        };

        let root = earner_tree_root;

        let activation_delay = 0;
        // Set the activation delay to zero so that the claim can be processed
        // right after setting the root
        let set_activation_delay = rewards_coordinator
            .setActivationDelay(activation_delay)
            .send()
            .await
            .unwrap();

        let receipt = set_activation_delay.get_receipt().await.unwrap();
        assert!(receipt.status());

        // Set the rewards updater so that we can submit the root
        let rewards_updater = ANVIL_FIRST_ADDRESS;
        let set_rewards_updater_tx = rewards_coordinator
            .setRewardsUpdater(rewards_updater)
            .send()
            .await
            .unwrap();
        let receipt = set_rewards_updater_tx.get_receipt().await.unwrap();
        assert!(receipt.status());

        // Fetch the current timestamp to increase it
        let curr_rewards_calculation_end_timestamp_return = rewards_coordinator
            .currRewardsCalculationEndTimestamp()
            .call()
            .await
            .unwrap();
        let IRewardsCoordinator::currRewardsCalculationEndTimestampReturn {
            _0: curr_rewards_calculation_end_timestamp,
        } = curr_rewards_calculation_end_timestamp_return;

        // Submit the root
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
    async fn test_process_claim() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let private_key = ANVIL_FIRST_PRIVATE_KEY.to_string();
        let el_chain_writer =
            new_test_writer_with_private_key(http_endpoint.to_string(), private_key).await;
        let earnings = U256::from(42);
        let (_root, claim) = new_claim(&http_endpoint, earnings).await;
        let earner = ANVIL_FIRST_ADDRESS;
        let tx_hash = el_chain_writer.process_claim(earner, claim).await.unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());
    }
}
