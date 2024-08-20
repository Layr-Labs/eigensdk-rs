#[cfg(test)]
pub mod integration_test {
    use std::time::Duration;

    use crate::bls_agg::BlsAggregatorService;
    use alloy_primitives::{Bytes, B256};
    use alloy_primitives::{FixedBytes, U256};
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
        get_service_manager_address, ANVIL_RPC_URL,
    };
    use eigen_types::avs::TaskIndex;
    use eigen_types::operator::{operator_id_from_g1_pub_key, QuorumThresholdPercentages};
    use eigen_utils::binding::mockAvsServiceManager;
    use eigen_utils::get_provider;
    use sha2::{Digest, Sha256};

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

        let digest_hash: FixedBytes<32> = FixedBytes::from([
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
            0x02, 0x02, 0x02, 0x02,
        ]);
        let bls_key_pair = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key());
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
            "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".to_string(), // TODO: use anvil private key
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();
        // let avs_registry_subscriber = AvsRegistryChainSubscriber::new(http_endpoint); // is AvsRegistryChainSubscriber fully implemented?

        let service_manager =
            mockAvsServiceManager::new(service_manager_address, ANVIL_RPC_URL.clone());

        // create aggregation service
        let operators_info = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_reader.clone(),
            ws_endpoint,
        )
        .await;
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service);

        // register operator
        // TODO: check if the operator is registered successfully
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair.clone(),
                digest_hash,
                U256::from(6300),
                quorum_nums.clone(),
                "socket".to_string(),
            )
            .await
            .unwrap();

        // create the task related parameters
        let provider = get_provider(&http_endpoint);
        let current_block_num = provider.get_block_number().await.unwrap();
        let task_index: TaskIndex = 0;
        let task_response = 123; // Initialize with appropriate data
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
        let task_response_digest = hash(task_response);
        let bls_sig_op_1 = bls_key_pair.sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
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

        // TODO: check the response signature with `service_manager.checkSignatures()`
    }
}
