#[cfg(test)]
pub mod integration_test {
    use crate::{
        bls_agg::BlsAggregatorService,
        bls_aggregation_service_response::BlsAggregationServiceResponse,
    };
    use alloy::{providers::Provider, signers::local::PrivateKeySigner};
    use alloy_primitives::{aliases::U96, Address, Bytes, FixedBytes, B256, U256};
    use alloy_provider::WalletProvider;
    use eigen_client_avsregistry::{
        reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter,
    };
    use eigen_client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
    use eigen_crypto_bls::{
        convert_to_bls_checker_g1_point, convert_to_bls_checker_g2_point, BlsKeyPair,
    };
    use eigen_logging::get_test_logger;
    use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
    use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
    use eigen_testing_utils::{
        anvil::{mine_anvil_blocks, start_anvil_container},
        anvil_constants::{
            get_allocation_manager_address, get_avs_directory_address,
            get_delegation_manager_address, get_erc20_mock_strategy,
            get_operator_state_retriever_address, get_registry_coordinator_address,
            get_rewards_coordinator_address, get_service_manager_address,
        },
        test_data::TestData,
        transaction::wait_transaction,
    };
    use eigen_types::{
        avs::TaskIndex,
        operator::{QuorumNum, QuorumThresholdPercentages},
    };
    use eigen_utils::{
        allocationmanager::IAllocationManagerTypes,
        blsapkregistry::BLSApkRegistry,
        get_provider, get_signer,
        iblssignaturechecker::{
            IBLSSignatureChecker::{self, NonSignerStakesAndSignature},
            BN254::G1Point,
        },
        mockavsservicemanager::MockAvsServiceManager,
        registrycoordinator::{
            IRegistryCoordinator::OperatorSetParam, IStakeRegistry::StrategyParams,
            RegistryCoordinator,
        },
    };
    use serde::Deserialize;
    use sha2::{Digest, Sha256};
    use std::time::Duration;
    use std::{
        process::{Command, Stdio},
        str::FromStr,
    };
    use tokio::{task, time::sleep};
    use tokio_util::sync::CancellationToken;

    const PRIVATE_KEY_1: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // the owner addr
    const PRIVATE_KEY_2: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
    const BLS_KEY_1: &str =
        "1371012690269088913462269866874713266643928125698382731338806296762673180359922";
    const BLS_KEY_2: &str =
        "14610126902690889134622698668747132666439281256983827313388062967626731803500";

    fn hash(task_response: u64) -> B256 {
        let mut hasher = Sha256::new();
        hasher.update(task_response.to_be_bytes());
        B256::from_slice(hasher.finalize().as_ref())
    }

    fn agg_response_to_non_signer_stakes_and_signature(
        agg_response: BlsAggregationServiceResponse,
    ) -> NonSignerStakesAndSignature {
        let non_signer_pubkeys: Vec<G1Point> = agg_response
            .non_signers_pub_keys_g1
            .iter()
            .map(|point| convert_to_bls_checker_g1_point(point.g1()).unwrap())
            .collect();
        let quorum_apks = agg_response
            .quorum_apks_g1
            .iter()
            .map(|point| convert_to_bls_checker_g1_point(point.g1()).unwrap())
            .collect();

        NonSignerStakesAndSignature {
            nonSignerPubkeys: non_signer_pubkeys,
            quorumApks: quorum_apks,
            apkG2: convert_to_bls_checker_g2_point(agg_response.signers_apk_g2.g2()).unwrap(),
            sigma: convert_to_bls_checker_g1_point(agg_response.signers_agg_sig_g1.g1_point().g1())
                .unwrap(),
            nonSignerQuorumBitmapIndices: agg_response.non_signer_quorum_bitmap_indices,
            quorumApkIndices: agg_response.quorum_apk_indices,
            totalStakeIndices: agg_response.total_stake_indices,
            nonSignerStakeIndices: agg_response.non_signer_stake_indices,
        }
    }

    #[derive(Deserialize, Debug)]
    struct Input {
        bls_key: String,
        quorum_numbers: Vec<QuorumNum>,
        quorum_threshold_percentages: QuorumThresholdPercentages,
    }

    async fn register_operator(
        private_key: &str,
        rpc_url: &str,
        bls_key_pair: BlsKeyPair,
        quorum_nums: Bytes,
    ) -> FixedBytes<32> {
        let address = PrivateKeySigner::from_str(private_key).unwrap().address();

        let registry_coordinator = get_registry_coordinator_address(rpc_url.to_string()).await;
        let operator_state_retriever =
            get_operator_state_retriever_address(rpc_url.to_string()).await;
        let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            rpc_url.to_string(),
            private_key.to_string(),
            registry_coordinator,
            operator_state_retriever,
        )
        .await
        .unwrap();
        let tx_hash = avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                FixedBytes::from([0x02; 32]),
                U256::from_be_slice(&[0xff; 32]),
                quorum_nums,
                "socket".to_string(),
            )
            .await
            .unwrap();
        let registered_status = wait_transaction(rpc_url, tx_hash).await.unwrap().status();
        assert!(registered_status);

        let bls_apk_registry = BLSApkRegistry::new(registry_coordinator, get_provider(rpc_url));
        bls_apk_registry
            .getOperatorId(address)
            .call()
            .await
            .unwrap()
            ._0
    }

    async fn create_quorum(http_endpoint: &str) {
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.to_string()).await;

        let contract_registry_coordinator = RegistryCoordinator::new(
            registry_coordinator_address,
            get_signer(PRIVATE_KEY_1, http_endpoint),
        );
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 100,
            kickBIPsOfTotalStake: 1000,
        };
        let strategy_params = StrategyParams {
            strategy: get_erc20_mock_strategy(http_endpoint.to_string()).await,
            multiplier: U96::from(1),
        };
        let _ = contract_registry_coordinator
            .createTotalDelegatedStakeQuorum(
                operator_set_params,
                U96::from(0),
                vec![strategy_params],
            )
            .send()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_bls_agg() {
        // test 1 quorum, 1 operator
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        // if TEST_DATA_PATH is set, load the test data from the json file
        let default_input = Input {
            bls_key: BLS_KEY_1.to_string(),
            quorum_numbers: vec![1],
            quorum_threshold_percentages: vec![100_u8],
        };
        let test_data: TestData<Input> = TestData::new(default_input);

        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());
        let quorum_nums = Bytes::from(test_data.input.quorum_numbers);
        let quorum_threshold_percentages: QuorumThresholdPercentages =
            test_data.input.quorum_threshold_percentages;

        create_quorum(&http_endpoint).await;

        // Register operator
        let bls_key_pair = BlsKeyPair::new(test_data.input.bls_key).unwrap();
        let operator_id = register_operator(
            PRIVATE_KEY_1,
            http_endpoint.as_str(),
            bls_key_pair.clone(),
            quorum_nums.clone(),
        )
        .await;

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await
        .unwrap()
        .0;

        let cancellation_token = CancellationToken::new();
        let operators_info_clone = operators_info.clone();
        let token_clone = cancellation_token.clone();
        task::spawn(async move { operators_info_clone.start_service(&token_clone, 0, 0).await });
        // Sleep to wait for the operator info service to start
        sleep(Duration::from_secs(1)).await;

        // Create aggregation service
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());
        let current_block_num = provider.get_block_number().await.unwrap();
        mine_anvil_blocks(&container, 1).await;

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);

        // Initialize the task
        bls_agg_service
            .initialize_new_task(
                task_index,
                current_block_num as u32,
                quorum_nums.to_vec(),
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);
        let bls_signature = bls_key_pair.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(task_index, task_response_digest, bls_signature, operator_id)
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
            .unwrap()
            .unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(service_manager_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                current_block_num as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_bls_agg_operator_sets_enabled() {
        // let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        let http_endpoint = "http://localhost:8545".to_string();
        let ws_endpoint = "ws://localhost:8545".to_string();

        let default_input = Input {
            bls_key: BLS_KEY_1.to_string(),
            quorum_numbers: vec![1],
            quorum_threshold_percentages: vec![100_u8],
        };
        let test_data: TestData<Input> = TestData::new(default_input);
        let quorum_nums = Bytes::from(test_data.input.quorum_numbers);
        let quorum_threshold_percentages: QuorumThresholdPercentages =
            test_data.input.quorum_threshold_percentages;
        let bls_key_pair = BlsKeyPair::new(test_data.input.bls_key).unwrap();

        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;
        let rewards_coordinator_address =
            get_rewards_coordinator_address(http_endpoint.clone()).await;
        let mock_strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let signer = get_signer(PRIVATE_KEY_1, &http_endpoint);

        let registry_coordinator_instance =
            RegistryCoordinator::new(registry_coordinator_address, signer.clone());
        let enable_operator_sets_status = registry_coordinator_instance
            .enableOperatorSets()
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap()
            .status();
        assert!(enable_operator_sets_status);
        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            allocation_manager_address,
            delegation_manager_address,
            avs_directory_address,
            Address::ZERO,
            http_endpoint.clone(),
        );
        let el_chain_writer = ELChainWriter::new(
            delegation_manager_address,
            Address::ZERO,
            rewards_coordinator_address,
            Address::ZERO,
            allocation_manager_address,
            registry_coordinator_address,
            el_chain_reader,
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
        );

        create_quorum(&http_endpoint.clone()).await;
        let s = el_chain_writer
            .register_for_operator_sets(
                signer.default_signer_address(),
                service_manager_address,
                [1].to_vec(),
                bls_key_pair.clone(),
                "socket1",
            )
            .await
            .unwrap();
        let a = wait_transaction(&http_endpoint, s).await.unwrap().status();
        assert!(a);

        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await
        .unwrap()
        .0;

        let cancellation_token = CancellationToken::new();
        let operators_info_clone = operators_info.clone();
        let token_clone = cancellation_token.clone();
        task::spawn(async move { operators_info_clone.start_service(&token_clone, 0, 0).await });
        // Sleep to wait for the operator info service to start
        sleep(Duration::from_secs(1)).await;

        // Create aggregation service
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());
        let current_block_num = provider.get_block_number().await.unwrap();
        let current_block_number = get_provider(&http_endpoint)
            .get_block_number()
            .await
            .unwrap();

        fn mine_anvil_block(rpc_url: &str, blocks: u64) {
            Command::new("cast")
                .args([
                    "rpc",
                    "anvil_mine",
                    &blocks.to_string(),
                    "--rpc-url",
                    rpc_url,
                ])
                .stdout(Stdio::null())
                .output()
                .expect("Failed to execute command");
        }
        mine_anvil_block(&http_endpoint, current_block_number + 2);

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);

        // Initialize the task
        bls_agg_service
            .initialize_new_task(
                task_index,
                current_block_num as u32,
                quorum_nums.to_vec(),
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);
        let bls_signature = bls_key_pair.sign_message(task_response_digest.as_ref());
        let bls_apk_registry = BLSApkRegistry::new(
            registry_coordinator_address,
            get_provider(&http_endpoint.clone()),
        );
        let operator_id = bls_apk_registry
            .getOperatorId(signer.default_signer_address())
            .call()
            .await
            .unwrap()
            ._0;
        bls_agg_service
            .process_new_signature(task_index, task_response_digest, bls_signature, operator_id)
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
            .unwrap()
            .unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(service_manager_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                current_block_num as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_1_quorum_2_operators() {
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;

        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        // Anvil state has one single quorum created (with quorum number 0), so we create a quorum that will have quorum number 1
        let quorum_nums = Bytes::from([1u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];
        create_quorum(&http_endpoint).await;

        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();
        let operator_id_1 = register_operator(
            PRIVATE_KEY_1,
            http_endpoint.as_str(),
            bls_key_pair_1.clone(),
            quorum_nums.clone(),
        )
        .await;

        let bls_key_pair_2 = BlsKeyPair::new(BLS_KEY_2.to_string()).unwrap();
        let operator_id_2 = register_operator(
            PRIVATE_KEY_2,
            http_endpoint.as_str(),
            bls_key_pair_2.clone(),
            quorum_nums.clone(),
        )
        .await;

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await
        .unwrap()
        .0;

        let current_block_num = provider.get_block_number().await.unwrap();
        let cancellation_token = CancellationToken::new();
        let operators_info_clone = operators_info.clone();
        let token_clone = cancellation_token.clone();
        task::spawn(async move {
            operators_info_clone
                .start_service(&token_clone, 0, current_block_num)
                .await
        });
        // Sleep to wait for the operator info service to start
        sleep(Duration::from_secs(1)).await;

        // Create aggregation service
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());

        let current_block_num = provider.get_block_number().await.unwrap();

        mine_anvil_blocks(&container, 1).await;

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);

        // Initialize the task
        bls_agg_service
            .initialize_new_task(
                task_index,
                current_block_num as u32,
                quorum_nums.to_vec(),
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);

        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_1,
                operator_id_1,
            )
            .await
            .unwrap();

        let bls_signature_2 = bls_key_pair_2.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_2,
                operator_id_2,
            )
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
            .unwrap()
            .unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(service_manager_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                current_block_num as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_2_quorums_2_operators_separated() {
        // operator 1 stakes on quorum 1
        // operator 2 stakes on quorum 2
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;

        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let quorum_nums = Bytes::from([1u8, 2u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
        create_quorum(http_endpoint.as_str()).await;
        create_quorum(http_endpoint.as_str()).await;

        // Register operators
        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();
        let operator_id_1 = register_operator(
            PRIVATE_KEY_1,
            http_endpoint.as_str(),
            bls_key_pair_1.clone(),
            Bytes::from([1]),
        )
        .await;

        let bls_key_pair_2 = BlsKeyPair::new(BLS_KEY_2.to_string()).unwrap();
        let operator_id_2 = register_operator(
            PRIVATE_KEY_2,
            http_endpoint.as_str(),
            bls_key_pair_2.clone(),
            Bytes::from([2]),
        )
        .await;

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await
        .unwrap()
        .0;

        let cancellation_token = CancellationToken::new();
        let operators_info_clone = operators_info.clone();
        let token_clone = cancellation_token.clone();
        task::spawn(async move { operators_info_clone.start_service(&token_clone, 0, 0).await });
        // Sleep to wait for the operator info service to start
        sleep(Duration::from_secs(1)).await;

        // Create aggregation service
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());

        let current_block_num = provider.get_block_number().await.unwrap();

        mine_anvil_blocks(&container, 1).await;

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);

        // Initialize the task
        bls_agg_service
            .initialize_new_task(
                task_index,
                current_block_num as u32,
                quorum_nums.to_vec(),
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);

        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_1,
                operator_id_1,
            )
            .await
            .unwrap();

        let bls_signature_2 = bls_key_pair_2.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_2,
                operator_id_2,
            )
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
            .unwrap()
            .unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(service_manager_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                current_block_num as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_2_quorums_2_operators_shared() {
        // operator 1 stakes on quorums [1, 2]
        // operator 2 stakes on quorums [2]
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;

        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();
        let bls_key_pair_2 = BlsKeyPair::new(BLS_KEY_2.to_string()).unwrap();

        // Create quorums
        let quorum_nums = Bytes::from([1u8, 2u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
        create_quorum(http_endpoint.as_str()).await;
        create_quorum(http_endpoint.as_str()).await;

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.to_string(),
        )
        .await
        .unwrap();

        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint.to_string(),
        )
        .await
        .unwrap()
        .0;

        let cancellation_token = CancellationToken::new();
        let operators_info_clone = operators_info.clone();
        let token_clone = cancellation_token.clone();
        task::spawn(async move { operators_info_clone.start_service(&token_clone, 0, 0).await });
        // Sleep to wait for the operator info service to start
        sleep(Duration::from_secs(1)).await;

        // Register operator
        let operator_id_1 = register_operator(
            PRIVATE_KEY_1,
            http_endpoint.as_str(),
            bls_key_pair_1.clone(),
            quorum_nums.clone(),
        )
        .await;
        let operator_id_2 = register_operator(
            PRIVATE_KEY_2,
            http_endpoint.as_str(),
            bls_key_pair_2.clone(),
            Bytes::from([2]),
        )
        .await;

        // Create aggregation service
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());

        let current_block_num = provider.get_block_number().await.unwrap();

        mine_anvil_blocks(&container, 1).await;

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(1);

        // Initialize the task
        bls_agg_service
            .initialize_new_task(
                task_index,
                current_block_num as u32,
                quorum_nums.to_vec(),
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);

        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_1,
                operator_id_1,
            )
            .await
            .unwrap();

        let bls_signature_2 = bls_key_pair_2.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_2,
                operator_id_2,
            )
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
            .unwrap()
            .unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(service_manager_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                current_block_num as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_2_quorums_1_operator() {
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;

        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let service_manager_address = get_service_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();

        // Create quorums
        let quorum_nums = Bytes::from([1u8, 2u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
        create_quorum(http_endpoint.as_str()).await;
        create_quorum(http_endpoint.as_str()).await;

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.to_string(),
        )
        .await
        .unwrap();

        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint.to_string(),
        )
        .await
        .unwrap()
        .0;

        let cancellation_token = CancellationToken::new();
        let operators_info_clone = operators_info.clone();
        let token_clone = cancellation_token.clone();
        task::spawn(async move { operators_info_clone.start_service(&token_clone, 0, 0).await });
        sleep(Duration::from_secs(1)).await;

        // Register operator
        let operator_id_1 = register_operator(
            PRIVATE_KEY_1,
            http_endpoint.as_str(),
            bls_key_pair_1.clone(),
            quorum_nums.clone(),
        )
        .await;

        // Create aggregation service
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());

        let current_block_num = provider.get_block_number().await.unwrap();
        mine_anvil_blocks(&container, 1).await;

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(1);

        // Initialize the task
        bls_agg_service
            .initialize_new_task(
                task_index,
                current_block_num as u32,
                quorum_nums.to_vec(),
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);

        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_1.clone(),
                operator_id_1,
            )
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
            .unwrap()
            .unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(service_manager_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                current_block_num as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }
}
