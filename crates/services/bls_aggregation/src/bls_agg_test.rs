#[cfg(test)]
pub mod integration_test {
    use alloy_node_bindings::Anvil;
    use eigen_client_avsregistry::reader::AvsRegistryChainReader;
    use eigen_client_avsregistry::subscriber::AvsRegistryChainSubscriber;
    use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants::{
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_service_manager_address, ANVIL_RPC_URL,
    };
    use eigen_types::operator::operator_id_from_g1_pub_key;
    use eigen_utils::binding::mockAvsServiceManager;

    #[tokio::test]
    async fn test_1_quorum_1_operator() {
        let http_endpoint = "http://localhost:8545".to_string();
        let bls_key_pair = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key());
        let registry_coordinator_address = get_registry_coordinator_address().await;
        let operator_state_retriever_address = get_operator_state_retriever_address().await;
        let service_manager_address = get_service_manager_address().await;

        // create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();
        let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint.clone(),
            "".to_string(), // TODO: signer key?
            registry_coordinator_address,
            operator_state_retriever_address,
        )
        .await
        .unwrap();
        let avs_registry_subscriber = AvsRegistryChainSubscriber::new(http_endpoint); // is AvsRegistryChainSubscriber fully implemented?

        let service_manager_contract =
            mockAvsServiceManager::new(service_manager_address, ANVIL_RPC_URL.clone());
    }
}
