#[cfg(test)]
pub mod integration_test {
    use crate::bls_agg::{BlsAggregationServiceResponse, BlsAggregatorService};
    use alloy_primitives::{hex, Bytes, FixedBytes, B256, U256};
    use alloy_provider::Provider;
    use eigen_client_avsregistry::{
        reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter,
    };
    use eigen_crypto_bls::{
        convert_to_bls_checker_g1_point, convert_to_bls_checker_g2_point, BlsKeyPair,
    };
    use eigen_logging::get_test_logger;
    use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
    use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
    use eigen_testing_utils::anvil_constants::{
        get_erc20_mock_strategy, get_operator_state_retriever_address,
        get_registry_coordinator_address, get_service_manager_address,
    };
    use eigen_types::{avs::TaskIndex, operator::QuorumThresholdPercentages};
    use eigen_utils::{
        binding::{
            IBLSSignatureChecker::{self, G1Point, NonSignerStakesAndSignature},
            RegistryCoordinator::{self, OperatorSetParam, StrategyParams},
        },
        get_provider, get_signer,
    };
    use serial_test::serial;
    use sha2::{Digest, Sha256};
    use std::time::Duration;
    use std::{
        process::{Command, Stdio},
        thread::sleep,
    };
    use tokio::sync::watch;

    const PRIVATE_KEY_1: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const PRIVATE_KEY_2: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

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
            .collect(); // fails with invalid G1Affine

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

    #[tokio::test]
    #[serial]
    async fn test_1_quorum_1_operator() {
        let registry_coordinator_address = get_registry_coordinator_address().await;
        let operator_state_retriever_address = get_operator_state_retriever_address().await;
        let service_manager_address = get_service_manager_address().await;
        let http_endpoint = "http://localhost:8545".to_string();
        let ws_endpoint = "ws://localhost:8545".to_string();
        let provider = get_provider(&http_endpoint);
        let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let bls_key_pair = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let quorum_nums = Bytes::from([0]);

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();
        let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint.clone(),
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();

        // create quorum
        let contract_registry_coordinator = RegistryCoordinator::new(
            registry_coordinator_address,
            get_signer(
                "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
                &http_endpoint,
            ),
        );
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 100,
            kickBIPsOfTotalStake: 1000,
        };
        let strategy_params = StrategyParams {
            strategy: get_erc20_mock_strategy().await,
            multiplier: 1,
        };

        let _ = contract_registry_coordinator
            .createQuorum(operator_set_params, 0, vec![strategy_params])
            .send()
            .await
            .unwrap();

        // Register operator
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair.clone(),
                salt,
                U256::from_be_slice(&[0xff; 32]), // TODO: check expiry time
                quorum_nums.clone(),
                "socket".to_string(),
            )
            .await
            .unwrap();

        // Sleep is needed so registered operators are accesible to the OperatorInfoServiceInMemory
        sleep(Duration::from_millis(500));

        // Create aggregation service
        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await;

        let (shutdown_tx, shutdown_rx) = watch::channel(());
        operators_info
            .start_service(0, 0, shutdown_rx)
            .await
            .unwrap();

        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service);

        let operator_id =
            hex!("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9").into();

        let current_block_num = provider.get_block_number().await.unwrap();

        // Advance the block number by 1
        Command::new("cast")
            .args(&["rpc", "anvil_mine", "1", "--rpc-url", &http_endpoint])
            .stdout(Stdio::null())
            .output()
            .expect("Failed to execute command");

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];
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
        let task_response = 123; // Initialize with appropriate data
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
        let _ = shutdown_tx.send(());

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
    #[serial]
    async fn test_1_quorum_2_operators() {
        let registry_coordinator_address = get_registry_coordinator_address().await;
        let operator_state_retriever_address = get_operator_state_retriever_address().await;
        let service_manager_address = get_service_manager_address().await;
        let http_endpoint = "http://localhost:8545".to_string();
        let ws_endpoint = "ws://localhost:8545".to_string();
        let provider = get_provider(&http_endpoint);
        let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);

        let bls_key_pair_1 = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id_1 =
            hex!("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9").into();

        let bls_key_pair_2 = BlsKeyPair::new(
            "14610126902690889134622698668747132666439281256983827313388062967626731803500".into(),
        )
        .unwrap();
        let operator_id_2 =
            hex!("7213614953817d00866957a5f866c67a5fb8d4e392af501701f7ab35294dc4b3").into();

        let quorum_nums = Bytes::from([1u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];

        let contract_registry_coordinator = RegistryCoordinator::new(
            registry_coordinator_address,
            get_signer(PRIVATE_KEY_1.to_string(), &http_endpoint),
        );

        // TODO: check quorum params
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 1,
            kickBIPsOfTotalStake: 1,
        };
        let strategy_params = vec![StrategyParams {
            strategy: get_erc20_mock_strategy().await,
            multiplier: 1,
        }];

        // Create quorum
        let _ = contract_registry_coordinator
            .createQuorum(operator_set_params.clone(), 0, strategy_params.clone())
            .send()
            .await
            .unwrap();

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();

        // Register operator
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair_1.clone(),
                salt,
                U256::from_be_slice(&[0xff; 32]), // TODO: check expiry time
                quorum_nums.clone(),
                "socket".to_string(),
            )
            .await
            .unwrap();

        let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint.clone(),
            PRIVATE_KEY_2.to_string(),
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair_2.clone(),
                salt,
                U256::from_be_slice(&[0xff; 32]), // TODO: check expiry time
                quorum_nums.clone(),
                "socket".to_string(),
            )
            .await
            .unwrap();

        // Sleep is needed so registered operators are accesible to the OperatorInfoServiceInMemory
        sleep(Duration::from_millis(500));

        // Create aggregation service
        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await;

        let (shutdown_tx, shutdown_rx) = watch::channel(());
        operators_info
            .start_service(0, 0, shutdown_rx)
            .await
            .unwrap();

        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service);

        let current_block_num = provider.get_block_number().await.unwrap();

        // Advance the block number by 1
        Command::new("cast")
            .args(&["rpc", "anvil_mine", "1", "--rpc-url", &http_endpoint])
            .stdout(Stdio::null())
            .output()
            .expect("Failed to execute command");

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
        let task_response = 123; // Initialize with appropriate data
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
        let _ = shutdown_tx.send(());

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
    #[serial]
    async fn test_2_quorums_2_operators() {
        let registry_coordinator_address = get_registry_coordinator_address().await;
        let operator_state_retriever_address = get_operator_state_retriever_address().await;
        let service_manager_address = get_service_manager_address().await;
        let http_endpoint = "http://localhost:8545".to_string();
        let ws_endpoint = "ws://localhost:8545".to_string();
        let provider = get_provider(&http_endpoint);
        let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);

        let bls_key_pair_1 = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id_1 =
            hex!("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9").into();

        let bls_key_pair_2 = BlsKeyPair::new(
            "14610126902690889134622698668747132666439281256983827313388062967626731803500".into(),
        )
        .unwrap();
        let operator_id_2 =
            hex!("7213614953817d00866957a5f866c67a5fb8d4e392af501701f7ab35294dc4b3").into();

        let quorum_nums = Bytes::from([2u8, 3u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];

        let contract_registry_coordinator = RegistryCoordinator::new(
            registry_coordinator_address,
            get_signer(PRIVATE_KEY_1.to_string(), &http_endpoint),
        );

        // TODO: check quorum params
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 1,
            kickBIPsOfTotalStake: 1,
        };
        let strategy_params = vec![StrategyParams {
            strategy: get_erc20_mock_strategy().await,
            multiplier: 1,
        }];

        // Create quorum 0
        let _ = contract_registry_coordinator
            .createQuorum(operator_set_params.clone(), 0, strategy_params.clone())
            .send()
            .await
            .unwrap();

        // Create quorum 1
        let _ = contract_registry_coordinator
            .createQuorum(operator_set_params, 0, strategy_params)
            .send()
            .await
            .unwrap();

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();

        // Register operator
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair_1.clone(),
                salt,
                U256::from_be_slice(&[0xff; 32]), // TODO: check expiry time
                Bytes::from([2]),
                "socket".to_string(),
            )
            .await
            .unwrap();

        let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            // TODO: check if needed
            get_test_logger(),
            http_endpoint.clone(),
            PRIVATE_KEY_2.to_string(),
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair_2.clone(),
                salt,
                U256::from_be_slice(&[0xff; 32]), // TODO: check expiry time
                Bytes::from([3]),
                "socket".to_string(),
            )
            .await
            .unwrap();

        // Sleep is needed so registered operators are accesible to the OperatorInfoServiceInMemory
        sleep(Duration::from_millis(500));

        // Create aggregation service
        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await;

        let (shutdown_tx, shutdown_rx) = watch::channel(());
        operators_info
            .start_service(0, 0, shutdown_rx)
            .await
            .unwrap();

        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service);

        let current_block_num = provider.get_block_number().await.unwrap();

        // Advance the block number by 1
        Command::new("cast")
            .args(&["rpc", "anvil_mine", "1", "--rpc-url", &http_endpoint])
            .stdout(Stdio::null())
            .output()
            .expect("Failed to execute command");

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
        let task_response = 123; // Initialize with appropriate data
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
        let _ = shutdown_tx.send(());
        dbg!(&bls_agg_response);

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
    #[serial]
    async fn test_2_quorums_1_operator() {
        let registry_coordinator_address = get_registry_coordinator_address().await;
        let operator_state_retriever_address = get_operator_state_retriever_address().await;
        let service_manager_address = get_service_manager_address().await;
        let http_endpoint = "http://localhost:8545".to_string();
        let ws_endpoint = "ws://localhost:8545".to_string();
        let provider = get_provider(&http_endpoint);
        let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);

        let bls_key_pair_1 = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id_1 =
            hex!("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9").into();

        let quorum_nums = Bytes::from([0u8, 1u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];

        let contract_registry_coordinator = RegistryCoordinator::new(
            registry_coordinator_address,
            get_signer(PRIVATE_KEY_1.to_string(), &http_endpoint),
        );

        // TODO: check quorum params
        let operator_set_params = OperatorSetParam {
            maxOperatorCount: 10,
            kickBIPsOfOperatorStake: 1,
            kickBIPsOfTotalStake: 1,
        };
        let strategy_params = vec![StrategyParams {
            strategy: get_erc20_mock_strategy().await,
            multiplier: 1,
        }];

        // Create quorum
        let _ = contract_registry_coordinator
            .createQuorum(operator_set_params.clone(), 0, strategy_params.clone())
            .send()
            .await
            .unwrap();

        // Create quorum
        let _ = contract_registry_coordinator
            .createQuorum(operator_set_params, 0, strategy_params)
            .send()
            .await
            .unwrap();

        // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();

        // Register operator
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair_1.clone(),
                salt,
                U256::from_be_slice(&[0xff; 32]), // TODO: check expiry time
                quorum_nums.clone(),
                "socket".to_string(),
            )
            .await
            .unwrap();

        // Sleep is needed so registered operators are accesible to the OperatorInfoServiceInMemory
        sleep(Duration::from_millis(500));

        // Create aggregation service
        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await;

        let (shutdown_tx, shutdown_rx) = watch::channel(());
        operators_info
            .start_service(0, 0, shutdown_rx)
            .await
            .unwrap();

        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service);

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(1);

        // Initialize the task
        let current_block_num = provider.get_block_number().await.unwrap();

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

        // Advance the block number by 1
        Command::new("cast")
            .args(&["rpc", "anvil_mine", "1", "--rpc-url", &http_endpoint])
            .stdout(Stdio::null())
            .output()
            .expect("Failed to execute command");

        // Compute the signature and send it to the aggregation service
        let task_response = 123; // Initialize with appropriate data
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

        let bls_agg_response = BlsAggregationServiceResponse {
            task_index: 0,
            task_response_digest: hex!(
                "41f1c4ddd1183083b48396129dec579e9b7ae61bcf24b743cfe59b7d558a2676"
            )
            .into(),
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1: vec![bls_key_pair_1.public_key(), bls_key_pair_1.public_key()],
            signers_apk_g2: bls_key_pair_1.public_key_g2(),
            signers_agg_sig_g1: bls_signature_1,
            non_signer_quorum_bitmap_indices: vec![],
            quorum_apk_indices: vec![1, 1],
            total_stake_indices: vec![1, 1],
            non_signer_stake_indices: vec![vec![], vec![]],
        };

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        let _ = shutdown_tx.send(());
        dbg!(&bls_agg_response);

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
