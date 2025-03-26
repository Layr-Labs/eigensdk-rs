//! AvsRegistry methods for reading, writing and subscribing purposes.

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

/// Reader module
pub mod reader;

/// Writer module
pub mod writer;

/// Avs registry error message
pub mod error;

/// Fake avs registry module
pub mod fake_reader;

#[cfg(test)]
pub(crate) mod test_utils {

    use crate::reader::AvsRegistryChainReader;
    use crate::writer::AvsRegistryChainWriter;
    use alloy::{
        primitives::{aliases::U96, Address, Bytes, FixedBytes, U256},
        providers::WalletProvider,
        sol_types::SolCall,
    };
    use eigen_common::get_signer;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants::{
        get_allocation_manager_address, get_erc20_mock_strategy,
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_service_manager_address, FIRST_PRIVATE_KEY,
    };
    use eigen_testing_utils::transaction::wait_transaction;
    use eigen_utils::slashing::{
        core::allocationmanager::AllocationManager,
        middleware::registrycoordinator::{
            ISlashingRegistryCoordinatorTypes::OperatorSetParam,
            IStakeRegistryTypes::StrategyParams, RegistryCoordinator,
        },
        sdk::mockavsservicemanager::MockAvsServiceManager,
    };

    pub(crate) async fn create_operator_set(http_endpoint: &str, avs_address: Address) {
        let allocation_manager_addr =
            get_allocation_manager_address(http_endpoint.to_string()).await;
        let default_signer = get_signer(FIRST_PRIVATE_KEY, http_endpoint);
        let allocation_manager =
            AllocationManager::new(allocation_manager_addr, default_signer.clone());
        let registry_coordinator_addr =
            get_registry_coordinator_address(http_endpoint.to_string()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.to_string()).await;
        let service_manager =
            MockAvsServiceManager::new(service_manager_address, default_signer.clone());
        service_manager
            .setAppointee(
                default_signer.default_signer_address(),
                allocation_manager_addr,
                alloy::primitives::FixedBytes(AllocationManager::setAVSRegistrarCall::SELECTOR),
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        allocation_manager
            .setAVSRegistrar(avs_address, registry_coordinator_addr)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        // Create slashable quorum
        let contract_registry_coordinator =
            RegistryCoordinator::new(registry_coordinator_addr, default_signer.clone());
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 100,
            kickBIPsOfTotalStake: 1000,
        };
        let strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        service_manager
            .setAppointee(
                registry_coordinator_addr,
                allocation_manager_addr,
                alloy::primitives::FixedBytes(AllocationManager::createOperatorSetsCall::SELECTOR),
            )
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        let strategy_params = StrategyParams {
            strategy,
            multiplier: U96::from(1),
        };

        contract_registry_coordinator
            .createSlashableStakeQuorum(operator_set_params, U96::from(0), vec![strategy_params], 0)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
    }

    pub(crate) async fn build_avs_registry_chain_writer(
        http_endpoint: String,
        private_key: String,
    ) -> AvsRegistryChainWriter {
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let service_manager_addr = get_service_manager_address(http_endpoint.clone()).await;
        AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint,
            private_key,
            registry_coordinator_address,
            service_manager_addr,
        )
        .await
        .unwrap()
    }

    pub(crate) async fn build_avs_registry_chain_reader(
        http_endpoint: String,
    ) -> AvsRegistryChainReader {
        let registry_coordinator_addr =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;

        AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_addr,
            operator_state_retriever_address,
            http_endpoint.to_string(),
        )
        .await
        .unwrap()
    }

    // this function is called from test_avs_writer_methods
    pub(crate) async fn test_register_operator(
        avs_writer: &AvsRegistryChainWriter,
        private_key_decimal: String,
        quorum_nums: Bytes,
        http_url: String,
    ) {
        let bls_key_pair = BlsKeyPair::new(private_key_decimal).unwrap();
        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);

        // this is set to U256::MAX so that the registry does not take the signature as expired.
        let signature_expiry = U256::MAX;
        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                digest_hash,
                signature_expiry,
                quorum_nums.clone(),
                "".into(),
            )
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_url, tx_hash).await.unwrap().status();
        assert!(tx_status);
    }

    // this function is caller from test_avs_writer_methods
    pub(crate) async fn test_deregister_operator(
        avs_writer: &AvsRegistryChainWriter,
        quorum_nums: Bytes,
        http_url: String,
    ) {
        let tx_hash = avs_writer.deregister_operator(quorum_nums).await.unwrap();

        let tx_status = wait_transaction(&http_url, tx_hash).await.unwrap().status();
        assert!(tx_status);
    }
}
// TODO: Move this tests package to somewhere else
#[cfg(test)]
mod tests {

    use crate::test_utils::build_avs_registry_chain_writer;
    use alloy::{
        primitives::{aliases::U96, keccak256, FixedBytes, U256, U8},
        sol_types::SolValue,
    };
    use eigen_client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
    use eigen_common::{get_provider, get_signer};
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::{
        anvil::start_anvil_container,
        anvil_constants::{
            get_allocation_manager_address, get_avs_directory_address,
            get_delegation_manager_address, get_erc20_mock_strategy,
            get_registry_coordinator_address, get_rewards_coordinator_address,
            get_strategy_manager_address, FIRST_ADDRESS, FIRST_PRIVATE_KEY,
        },
        transaction::wait_transaction,
    };
    use eigen_utils::slashing::{
        core::{
            delegationmanager::DelegationManager,
            irewardscoordinator::{
                IRewardsCoordinator,
                IRewardsCoordinatorTypes::{
                    EarnerTreeMerkleLeaf, RewardsMerkleClaim, TokenTreeMerkleLeaf,
                },
            },
        },
        middleware::servicemanagerbase::IRewardsCoordinatorTypes::{
            RewardsSubmission, StrategyAndMultiplier,
        },
        sdk::mockerc20::MockERC20,
    };

    async fn build_el_chain_reader(http_endpoint: String) -> ELChainReader {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;

        ELChainReader::build(
            get_test_logger().clone(),
            delegation_manager_address,
            avs_directory_address,
            rewards_coordinator,
            &http_endpoint,
        )
        .await
        .unwrap()
    }

    async fn new_test_writer(http_endpoint: String, private_key: String) -> ELChainWriter {
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;
        let delegation_manager = get_delegation_manager_address(http_endpoint.clone()).await;
        let allocation_manager = get_allocation_manager_address(http_endpoint.clone()).await;
        let contract_delegation_manager =
            DelegationManager::new(delegation_manager, get_provider(&http_endpoint));
        let permission_controller = contract_delegation_manager
            .permissionController()
            .call()
            .await
            .unwrap()
            ._0;
        let registry_coordinator = get_registry_coordinator_address(http_endpoint.clone()).await;

        ELChainWriter::new(
            strategy_manager,
            rewards_coordinator,
            Some(permission_controller),
            Some(allocation_manager),
            registry_coordinator,
            el_chain_reader,
            http_endpoint.clone(),
            private_key,
        )
    }

    /// The claim can be submitted from [`FIRST_PRIVATE_KEY`]
    async fn new_claim(
        http_endpoint: &str,
        cumulative_earnings: U256,
    ) -> (FixedBytes<32>, RewardsMerkleClaim) {
        let signer = get_signer(FIRST_PRIVATE_KEY, http_endpoint);
        let rewards_coordinator_address =
            get_rewards_coordinator_address(http_endpoint.to_string()).await;

        let el_chain_reader = build_el_chain_reader(http_endpoint.to_string()).await;
        let mock_strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        let (_, token_address) = el_chain_reader
            .get_strategy_and_underlying_token(mock_strategy)
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
        let earner_address = FIRST_ADDRESS;
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
        let next_root_index = el_chain_reader
            .get_distribution_roots_length()
            .await
            .unwrap();
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

        // Fetch the current timestamp to increase it
        let curr_rewards_calculation_end_timestamp = el_chain_reader
            .curr_rewards_calculation_end_timestamp()
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
    async fn test_process_claim() {
        let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        let signer = get_signer(FIRST_PRIVATE_KEY, &http_endpoint);

        let el_chain_writer =
            new_test_writer(http_endpoint.to_string(), FIRST_PRIVATE_KEY.to_string()).await;
        let el_chain_reader = build_el_chain_reader(http_endpoint.to_string()).await;

        let private_key = FIRST_PRIVATE_KEY.to_string();
        let avs_writer =
            build_avs_registry_chain_writer(http_endpoint.clone(), private_key.clone()).await;

        let rewards_coordinator_address =
            get_rewards_coordinator_address(http_endpoint.clone()).await;
        let provider = get_provider(&http_endpoint);
        let rewards_coordinator = IRewardsCoordinator::new(rewards_coordinator_address, &provider);

        let mock_strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;

        let (_, token_address) = el_chain_reader
            .get_strategy_and_underlying_token(mock_strategy)
            .await
            .unwrap();

        let token = MockERC20::new(token_address, &signer);
        let receipt = token
            .mint(FIRST_ADDRESS, U256::from(1000))
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        assert!(receipt.status());

        let rewards_duration = rewards_coordinator
            .MAX_REWARDS_DURATION()
            .call()
            .await
            .unwrap()
            ._0;

        let calculation_interval_seconds = rewards_coordinator
            .CALCULATION_INTERVAL_SECONDS()
            .call()
            .await
            .unwrap()
            ._0;

        let current_timestamp: u32 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap();
        let intervals_since_genesis = current_timestamp / calculation_interval_seconds;
        let start_timestamp = (intervals_since_genesis + 1) * calculation_interval_seconds;

        let strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let (_, token) = el_chain_reader
            .get_strategy_and_underlying_token(strategy_address)
            .await
            .unwrap();

        let strategies_and_multipliers = vec![StrategyAndMultiplier {
            strategy: strategy_address,
            multiplier: U96::from(1),
        }];
        let rewards_submissions = vec![RewardsSubmission {
            strategiesAndMultipliers: strategies_and_multipliers,
            token,
            amount: U256::from(1_000),
            startTimestamp: start_timestamp,
            duration: rewards_duration,
        }];

        let tx_hash = avs_writer
            .create_avs_rewards_submission(rewards_submissions)
            .await
            .unwrap();

        let tx_status = wait_transaction(&http_endpoint, tx_hash)
            .await
            .unwrap()
            .status();

        assert!(tx_status);

        // Check claimer balance at strategy before claim
        let token = MockERC20::new(token_address, &signer);
        let initial_balance = token.balanceOf(FIRST_ADDRESS).call().await.unwrap()._0;

        let expected_initial_balance = U256::from_str_radix("10000000000000000000", 10).unwrap();
        assert!(initial_balance == expected_initial_balance);

        let rewards_amount = U256::from(42);
        let (_root, claim) = new_claim(&http_endpoint, rewards_amount).await;

        let tx_hash = el_chain_writer
            .process_claim(claim, FIRST_ADDRESS)
            .await
            .unwrap();

        let receipt = wait_transaction(&http_endpoint, tx_hash).await.unwrap();
        assert!(receipt.status());

        // Check balance at strategy after claim
        let balance_after_claim = token.balanceOf(FIRST_ADDRESS).call().await.unwrap()._0;

        let expected_balance_after_claim = expected_initial_balance + rewards_amount;
        assert!(balance_after_claim == expected_balance_after_claim);
    }
}
