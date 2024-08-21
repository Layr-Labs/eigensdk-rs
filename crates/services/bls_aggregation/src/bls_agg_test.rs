#[cfg(test)]
pub mod integration_test {
    use crate::bls_agg::BlsAggregatorService;
    use alloy_primitives::{Bytes, FixedBytes, B256, U256};
    use alloy_provider::Provider;
    use eigen_client_avsregistry::{
        reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter,
    };
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::get_test_logger;
    use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
    use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
    use eigen_testing_utils::anvil_constants::{
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_service_manager_address,
    };
    use eigen_types::{avs::TaskIndex, operator::QuorumThresholdPercentages};
    use eigen_utils::{binding::mockAvsServiceManager, get_provider};
    use sha2::{Digest, Sha256};
    use std::time::Duration;
    use tokio::sync::watch;

    fn hash(task_response: u64) -> B256 {
        let mut hasher = Sha256::new();
        hasher.update(task_response.to_be_bytes());
        B256::from_slice(hasher.finalize().as_ref())
    }

    #[tokio::test]
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

        // create avs clients to interact with contracts deployed on anvil
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

        // create aggregation service
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

        // register operator
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

        // this query advances the block number by 1
        let operators = avs_registry_reader
            .get_operators_stake_in_quorums_at_current_block(quorum_nums.clone())
            .await
            .unwrap();
        let operator_id = operators[0][0].operatorId;

        // create the task related parameters
        let current_block_num = provider.get_block_number().await.unwrap();
        let task_index: TaskIndex = 0;
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];
        let time_to_expiry = Duration::from_secs(1);

        // initialize the task
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

        // compute the signature and send it to the aggregation service
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response); // es igual
        let bls_signature = bls_key_pair.sign_message(task_response_digest.as_ref()); // es igual
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature,
                operator_id.into(),
            )
            .await
            .unwrap();

        // wait for the response from the aggregation service and check the signature
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

        // TODO: check the response signature with `service_manager.checkSignatures()`
        let service_manager =
            mockAvsServiceManager::new(service_manager_address, get_provider(&http_endpoint));
    }
}
