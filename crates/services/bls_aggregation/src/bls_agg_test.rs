#[cfg(test)]
pub mod integration_test {
    use crate::bls_agg::{BlsAggregationServiceResponse, BlsAggregatorService};
    use alloy_primitives::{hex, Bytes, FixedBytes, B256, U256};
    use alloy_provider::Provider;
    use eigen_client_avsregistry::{
        reader::AvsRegistryChainReader,
        writer::{AvsRegistryChainWriter, GAS_LIMIT_REGISTER_OPERATOR_REGISTRY_COORDINATOR},
    };
    use eigen_crypto_bls::{
        convert_to_bls_checker_g1_point, convert_to_bls_checker_g2_point, BlsKeyPair,
    };
    use eigen_logging::get_test_logger;
    use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
    use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
    use eigen_testing_utils::{
        anvil_constants::{
            get_operator_state_retriever_address, get_registry_coordinator_address,
            get_service_manager_address,
        },
        m2_holesky_constants::StrategyBase_ETHx,
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
    use tokio::sync::watch;

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
            strategy: StrategyBase_ETHx,
            multiplier: 1,
        };

        let _ = contract_registry_coordinator
            .createQuorum(operator_set_params, 0, vec![strategy_params])
            .gas(GAS_LIMIT_REGISTER_OPERATOR_REGISTRY_COORDINATOR)
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
        let operator_id =
            hex!("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9").into();

        // Create the task related parameters
        let current_block_num = provider.get_block_number().await.unwrap();
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
    async fn test_2_quorums_1_operator() {
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
        let quorum_nums = Bytes::from([0u8, 1u8]);

        // create quorums
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
            strategy: StrategyBase_ETHx,
            multiplier: 1,
        };

        let _ = contract_registry_coordinator
            .createQuorum(
                operator_set_params.clone(),
                0,
                vec![strategy_params.clone()],
            )
            .gas(GAS_LIMIT_REGISTER_OPERATOR_REGISTRY_COORDINATOR)
            .send()
            .await
            .unwrap();

        let _ = contract_registry_coordinator
            .createQuorum(operator_set_params, 0, vec![strategy_params])
            .gas(GAS_LIMIT_REGISTER_OPERATOR_REGISTRY_COORDINATOR)
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
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(),
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();

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

        // Register operator
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair.clone(),
                salt,
                U256::from_be_slice(&[0xff; 32]), // TODO: check expiry time
                Bytes::from([0]),
                "socket".to_string(),
            )
            .await
            .unwrap();

        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair.clone(),
                salt,
                U256::from_be_slice(&[0xff; 32]), // TODO: check expiry time
                Bytes::from([1]),
                "socket".to_string(),
            )
            .await
            .unwrap();
        let operator_id =
            hex!("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9").into();

        // Create the task related parameters
        let current_block_num = provider.get_block_number().await.unwrap();
        let task_index: TaskIndex = 0;
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
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
