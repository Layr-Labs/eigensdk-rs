#[cfg(test)]
pub mod integration_test {
    use crate::{
        bls_agg::{BlsAggregatorService, TaskMetadata, TaskSignature},
        bls_aggregation_service_response::BlsAggregationServiceResponse,
    };
    use alloy::providers::Provider;
    use alloy::providers::WalletProvider;
    use alloy::{
        primitives::{Address, Bytes, FixedBytes, B256, U256},
        transports::http,
    };
    use eigen_client_avsregistry::{
        reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter,
    };
    use eigen_client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
    use eigen_common::{get_provider, get_signer};
    use eigen_crypto_bls::{
        convert_to_bls_checker_g1_point, convert_to_bls_checker_g2_point, BlsKeyPair,
    };
    use eigen_logging::get_test_logger;
    use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
    use eigen_services_operatorsinfo::{
        operatorsinfo_inmemory::OperatorInfoServiceInMemory,
        operatorsinfo_onchain::OperatorInfoOnChain,
    };
    use eigen_testing_utils::{
        anvil::{mine_anvil_blocks, start_anvil_container, start_m2_anvil_container},
        anvil_constants::{
            get_allocation_manager_address, get_bls_apk_registry_address, get_erc20_mock_strategy,
            get_operator_state_retriever_address, get_registry_coordinator_address,
            get_service_manager_address, get_socket_registry_address, get_strategy_manager_address,
        },
        chain_clients::{
            create_operator_set, create_quorum, create_total_delegated_stake_operator_set,
        },
        test_data::TestData,
        transaction::wait_transaction,
    };
    use eigen_types::{
        avs::TaskIndex,
        operator::{operator_id_from_g1_pub_key, QuorumNum, QuorumThresholdPercentages},
    };

    use eigen_utils::slashing::{
        core::allocationmanager::{AllocationManager::OperatorSet, IAllocationManagerTypes},
        middleware::{
            blsapkregistry::BLSApkRegistry,
            iblssignaturechecker::{
                IBLSSignatureChecker::{self},
                IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
                BN254::G1Point,
            },
        },
    };
    use serde::Deserialize;
    use sha2::{Digest, Sha256};
    use std::time::Duration;
    use tokio::{task, time::sleep};
    use tokio_util::sync::CancellationToken;

    const PRIVATE_KEY_1: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"; // the owner addr
    const PRIVATE_KEY_2: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
    const BLS_KEY_1: &str =
        "12248929636257230549931416853095037629726205319386239410403476017439825112537";
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

    #[tokio::test]
    async fn test_bls_agg_operator_sets_enabled() {
        // test 1 quorum, 1 operator
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        // if TEST_DATA_PATH is set, load the test data from the json file
        let default_input = Input {
            bls_key: BLS_KEY_1.to_string(),
            quorum_numbers: vec![0],
            quorum_threshold_percentages: vec![100_u8],
        };
        let test_data: TestData<Input> = TestData::new(default_input);
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let erc20_strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let strategy_manager_address = get_strategy_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(&http_endpoint);
        create_total_delegated_stake_operator_set(
            &http_endpoint,
            erc20_strategy_address,
            avs_address,
        )
        .await;

        // Register operator
        let bls_key_pair = BlsKeyPair::new(
            "12248929636257230549931416853095037629726205319386239410403476017439825112537"
                .to_string(),
        )
        .unwrap();
        let operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key()).unwrap();
        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            None,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            None,
            http_endpoint.clone(),
        );
        let el_chain_writer = ELChainWriter::new(
            strategy_manager_address,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader,
            http_endpoint.clone(),
            PRIVATE_KEY_2.to_string(),
        );
        let operator_2_address = get_signer(PRIVATE_KEY_2, &http_endpoint).default_signer_address();
        let s = el_chain_writer
            .deposit_erc20_into_strategy(erc20_strategy_address, "10000000".parse().unwrap())
            .await
            .unwrap();
        let modify = el_chain_writer
            .modify_allocations(
                operator_2_address,
                [IAllocationManagerTypes::AllocateParams {
                    operatorSet: OperatorSet {
                        avs: get_service_manager_address(http_endpoint.clone()).await,
                        id: 0,
                    },
                    strategies: [erc20_strategy_address].to_vec(),
                    newMagnitudes: [10000000].to_vec(),
                }]
                .to_vec(),
            )
            .await
            .unwrap();
        let a = get_provider(&http_endpoint)
            .get_transaction_receipt(s)
            .await
            .unwrap()
            .unwrap()
            .status();
        assert!(a);
        let b = get_provider(&http_endpoint)
            .get_transaction_receipt(modify)
            .await
            .unwrap()
            .unwrap()
            .status();
        assert!(b);
        el_chain_writer
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_2, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        // // Create avs clients to interact with contracts deployed on anvil
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
        // // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);
        let quorum_nums = Bytes::from(test_data.input.quorum_numbers);
        let quorum_threshold_percentages: QuorumThresholdPercentages =
            test_data.input.quorum_threshold_percentages;
        // Initialize the task
        let metadata = TaskMetadata::new(
            task_index,
            current_block_num,
            quorum_nums.to_vec(),
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut agg_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);
        let bls_signature = bls_key_pair.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature,
                operator_id,
            ))
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = agg_response.receive_aggregated_response().await.unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(avs_address, provider);
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
    async fn test_bls_agg_operator_sets_enabled_using_operatorinfo_onchain() {
        // test 1 quorum, 1 operator
        let (container, http_endpoint, _ws_endpoint) = start_anvil_container().await;
        // if TEST_DATA_PATH is set, load the test data from the json file
        let default_input = Input {
            bls_key: BLS_KEY_1.to_string(),
            quorum_numbers: vec![0],
            quorum_threshold_percentages: vec![100_u8],
        };
        let test_data: TestData<Input> = TestData::new(default_input);
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let erc20_strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let provider = get_provider(&http_endpoint.clone());
        let bls_apk_registry_address = get_bls_apk_registry_address(http_endpoint.clone()).await;
        let socket_registry_address = get_socket_registry_address(http_endpoint.clone()).await;
        create_total_delegated_stake_operator_set(
            &http_endpoint,
            erc20_strategy_address,
            avs_address,
        )
        .await;

        // Register operator
        let bls_key_pair = BlsKeyPair::new(
            "12248929636257230549931416853095037629726205319386239410403476017439825112537"
                .to_string(),
        )
        .unwrap();
        let operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key()).unwrap();
        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            None,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            None,
            http_endpoint.clone(),
        );
        let el_chain_writer = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader,
            http_endpoint.clone(),
            PRIVATE_KEY_2.to_string(),
        );

        el_chain_writer
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_2, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        // // Create avs clients to interact with contracts deployed on anvil
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_address,
            operator_state_retriever_address,
            http_endpoint.clone(),
        )
        .await
        .unwrap();
        let operators_info_on_chain = OperatorInfoOnChain::new(
            &http_endpoint,
            bls_apk_registry_address,
            socket_registry_address,
        );

        // Create aggregation service
        let avs_registry_service = AvsRegistryServiceChainCaller::new(
            avs_registry_reader.clone(),
            operators_info_on_chain,
        );

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());
        let current_block_num = provider.get_block_number().await.unwrap();

        mine_anvil_blocks(&container, 1).await;
        // // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);
        let quorum_nums = Bytes::from(test_data.input.quorum_numbers);
        let quorum_threshold_percentages: QuorumThresholdPercentages =
            test_data.input.quorum_threshold_percentages;
        // Initialize the task
        let metadata = TaskMetadata::new(
            task_index,
            current_block_num,
            quorum_nums.to_vec(),
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut agg_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);
        let bls_signature = bls_key_pair.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature,
                operator_id,
            ))
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = agg_response.receive_aggregated_response().await.unwrap();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(avs_address, provider);
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
    async fn test_bls_agg_m2() {
        let (container, http_endpoint, ws_endpoint) = start_m2_anvil_container().await;

        let default_input = Input {
            bls_key: BLS_KEY_1.to_string(),
            quorum_numbers: vec![0],
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

        let provider = get_provider(http_endpoint.as_str());

        // Create Quorum
        create_quorum(PRIVATE_KEY_1, &http_endpoint).await;
        let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint.to_string(),
            PRIVATE_KEY_1.to_string(),
            registry_coordinator_address,
            service_manager_address,
        )
        .await
        .unwrap();

        let digest_hash: FixedBytes<32> = FixedBytes::from([0x02; 32]);

        // this is set to U256::MAX so that the registry does not take the signature as expired.
        let signature_expiry = U256::MAX;
        let s = avs_registry_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair.clone(),
                digest_hash,
                signature_expiry,
                quorum_nums.clone(),
                "m2-register".to_string(),
            )
            .await
            .unwrap();
        let a = wait_transaction(&http_endpoint, s).await.unwrap().status();

        assert!(a);

        mine_anvil_blocks(&container, 1).await;

        let current_block_num = provider.get_block_number().await.unwrap();
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

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);

        // Initialize the task
        let metadata = TaskMetadata::new(
            task_index,
            current_block_num,
            quorum_nums.to_vec(),
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut agg_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);
        let bls_signature = bls_key_pair.sign_message(task_response_digest.as_ref());
        let bls_apk_registry = BLSApkRegistry::new(
            registry_coordinator_address,
            get_provider(&http_endpoint.clone()),
        );
        let s = bls_apk_registry
            .getOperatorId(get_signer(PRIVATE_KEY_1, &http_endpoint).default_signer_address())
            .call()
            .await
            .unwrap()
            ._0;
        let operator_id = operator_id_from_g1_pub_key(bls_key_pair.public_key()).unwrap();
        assert_eq!(s, operator_id);
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature,
                operator_id,
            ))
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = agg_response.receive_aggregated_response().await.unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(service_manager_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                (current_block_num - 1) as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_1_quorum_2_operators() {
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let erc20_strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let quorum_nums = Bytes::from([0u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];
        create_total_delegated_stake_operator_set(
            &http_endpoint,
            erc20_strategy_address,
            avs_address,
        )
        .await;

        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            None,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            None,
            http_endpoint.clone(),
        );
        let el_chain_writer_key_1 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader.clone(),
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
        );
        let el_chain_writer_key_2 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader,
            http_endpoint.clone(),
            PRIVATE_KEY_2.to_string(),
        );

        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();

        el_chain_writer_key_1
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_1, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair_1.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        let bls_key_pair_2 = BlsKeyPair::new(BLS_KEY_2.to_string()).unwrap();
        el_chain_writer_key_2
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_2, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair_2.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        mine_anvil_blocks(&container, 1).await;
        let current_block_num = provider.get_block_number().await.unwrap();
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

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);

        // Initialize the task
        let metadata = TaskMetadata::new(
            task_index,
            current_block_num,
            quorum_nums.to_vec(),
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut agg_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);
        let operator_id_1 = operator_id_from_g1_pub_key(bls_key_pair_1.public_key()).unwrap();
        let operator_id_2 = operator_id_from_g1_pub_key(bls_key_pair_2.public_key()).unwrap();
        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());

        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_1,
                operator_id_1,
            ))
            .await
            .unwrap();

        let bls_signature_2 = bls_key_pair_2.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_2,
                operator_id_2,
            ))
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = agg_response.receive_aggregated_response().await.unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(avs_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                (current_block_num - 1) as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_2_quorums_2_operators_separated() {
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let erc20_strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let quorum_nums = Bytes::from([0u8, 1u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
        create_total_delegated_stake_operator_set(
            http_endpoint.as_str(),
            erc20_strategy_address,
            avs_address,
        )
        .await;
        create_total_delegated_stake_operator_set(
            http_endpoint.as_str(),
            erc20_strategy_address,
            avs_address,
        )
        .await;

        // Register operators
        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();

        let bls_key_pair_2 = BlsKeyPair::new(BLS_KEY_2.to_string()).unwrap();

        let operator_id_1 = operator_id_from_g1_pub_key(bls_key_pair_1.public_key()).unwrap();
        let operator_id_2 = operator_id_from_g1_pub_key(bls_key_pair_2.public_key()).unwrap();
        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            None,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            None,
            http_endpoint.clone(),
        );
        let el_chain_writer_key_1 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader.clone(),
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
        );

        let el_chain_writer_key_2 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader.clone(),
            http_endpoint.clone(),
            PRIVATE_KEY_2.to_string(),
        );

        el_chain_writer_key_2
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_2, &http_endpoint).default_signer_address(),
                avs_address,
                vec![1],
                bls_key_pair_2.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        el_chain_writer_key_1
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_1, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair_1.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        mine_anvil_blocks(&container, 1).await;
        let current_block_num = provider.get_block_number().await.unwrap();
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

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);

        // Initialize the task
        let metadata = TaskMetadata::new(
            task_index,
            current_block_num,
            quorum_nums.to_vec(),
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut agg_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);

        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_1,
                operator_id_1,
            ))
            .await
            .unwrap();

        let bls_signature_2 = bls_key_pair_2.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_2,
                operator_id_2,
            ))
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = agg_response.receive_aggregated_response().await.unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(avs_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                (current_block_num - 1) as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_2_quorums_2_operators_shared() {
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let erc20_strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();
        let bls_key_pair_2 = BlsKeyPair::new(BLS_KEY_2.to_string()).unwrap();

        // Create quorums
        let quorum_nums = Bytes::from([0u8, 1u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
        create_total_delegated_stake_operator_set(
            http_endpoint.as_str(),
            erc20_strategy_address,
            avs_address,
        )
        .await;
        create_total_delegated_stake_operator_set(
            http_endpoint.as_str(),
            erc20_strategy_address,
            avs_address,
        )
        .await;

        let operator_id_1 = operator_id_from_g1_pub_key(bls_key_pair_1.public_key()).unwrap();
        let operator_id_2 = operator_id_from_g1_pub_key(bls_key_pair_2.public_key()).unwrap();
        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            None,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            None,
            http_endpoint.clone(),
        );
        let el_chain_writer_key_1 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader.clone(),
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
        );

        let el_chain_writer_key_2 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader.clone(),
            http_endpoint.clone(),
            PRIVATE_KEY_2.to_string(),
        );

        el_chain_writer_key_2
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_2, &http_endpoint).default_signer_address(),
                avs_address,
                vec![1],
                bls_key_pair_2.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        el_chain_writer_key_1
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_1, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair_1.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        mine_anvil_blocks(&container, 1).await;
        let current_block_num = provider.get_block_number().await.unwrap();
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

        // Create aggregation service
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(1);

        // Initialize the task
        let metadata = TaskMetadata::new(
            task_index,
            current_block_num,
            quorum_nums.to_vec(),
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut agg_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();
        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);

        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_1,
                operator_id_1,
            ))
            .await
            .unwrap();

        let bls_signature_2 = bls_key_pair_2.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_2,
                operator_id_2,
            ))
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = agg_response.receive_aggregated_response().await.unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(avs_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                (current_block_num - 1) as u32,
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
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let erc20_strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();
        let operator_id_1 = operator_id_from_g1_pub_key(bls_key_pair_1.public_key()).unwrap();
        // Create quorums
        let quorum_nums = Bytes::from([0u8, 1u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
        create_total_delegated_stake_operator_set(
            http_endpoint.as_str(),
            erc20_strategy_address,
            avs_address,
        )
        .await;
        create_total_delegated_stake_operator_set(
            http_endpoint.as_str(),
            erc20_strategy_address,
            avs_address,
        )
        .await;

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

        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            None,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            None,
            http_endpoint.clone(),
        );
        let el_chain_writer_key_1 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader.clone(),
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
        );

        el_chain_writer_key_1
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_1, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair_1.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        el_chain_writer_key_1
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_1, &http_endpoint).default_signer_address(),
                avs_address,
                vec![1],
                bls_key_pair_1.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        mine_anvil_blocks(&container, 1).await;
        let current_block_num = provider.get_block_number().await.unwrap();

        // Create aggregation service
        let avs_registry_service =
            AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);

        let bls_agg_service = BlsAggregatorService::new(avs_registry_service, get_test_logger());

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(1);

        // Initialize the task
        let metadata = TaskMetadata::new(
            task_index,
            current_block_num,
            quorum_nums.to_vec(),
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut agg_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);

        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_1,
                operator_id_1,
            ))
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = agg_response.receive_aggregated_response().await.unwrap();
        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(avs_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                (current_block_num - 1) as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_create_quorum_after_service_start() {
        let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        let allocation_manager_address =
            get_allocation_manager_address(http_endpoint.clone()).await;
        let registry_coordinator_address =
            get_registry_coordinator_address(http_endpoint.clone()).await;
        let operator_state_retriever_address =
            get_operator_state_retriever_address(http_endpoint.clone()).await;
        let erc20_strategy_address = get_erc20_mock_strategy(http_endpoint.clone()).await;
        let avs_address = get_service_manager_address(http_endpoint.clone()).await;
        let provider = get_provider(http_endpoint.as_str());

        let quorum_nums = Bytes::from([0u8]);
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];

        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            None,
            Address::ZERO,
            Address::ZERO,
            Address::ZERO,
            None,
            http_endpoint.clone(),
        );
        let el_chain_writer_key_1 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader.clone(),
            http_endpoint.clone(),
            PRIVATE_KEY_1.to_string(),
        );
        let el_chain_writer_key_2 = ELChainWriter::new(
            Address::ZERO,
            Address::ZERO,
            None,
            Some(allocation_manager_address),
            registry_coordinator_address,
            el_chain_reader,
            http_endpoint.clone(),
            PRIVATE_KEY_2.to_string(),
        );

        mine_anvil_blocks(&container, 1).await;

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

        // Create the operator set and register the operators
        create_total_delegated_stake_operator_set(
            &http_endpoint,
            erc20_strategy_address,
            avs_address,
        )
        .await;

        let bls_key_pair_1 = BlsKeyPair::new(BLS_KEY_1.to_string()).unwrap();
        el_chain_writer_key_1
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_1, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair_1.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        let bls_key_pair_2 = BlsKeyPair::new(BLS_KEY_2.to_string()).unwrap();
        el_chain_writer_key_2
            .register_for_operator_sets(
                get_signer(PRIVATE_KEY_2, &http_endpoint).default_signer_address(),
                avs_address,
                vec![0],
                bls_key_pair_2.clone(),
                "operator-sets",
            )
            .await
            .unwrap();

        mine_anvil_blocks(&container, 1).await;

        // Create the task related parameters
        let task_index: TaskIndex = 0;
        let time_to_expiry = Duration::from_secs(10);

        let current_block_num = provider.get_block_number().await.unwrap();

        // Initialize the task
        let metadata = TaskMetadata::new(
            task_index,
            current_block_num,
            quorum_nums.to_vec(),
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut agg_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        // Compute the signature and send it to the aggregation service
        let task_response = 123;
        let task_response_digest = hash(task_response);
        let operator_id_1 = operator_id_from_g1_pub_key(bls_key_pair_1.public_key()).unwrap();
        let operator_id_2 = operator_id_from_g1_pub_key(bls_key_pair_2.public_key()).unwrap();
        let bls_signature_1 = bls_key_pair_1.sign_message(task_response_digest.as_ref());

        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_1,
                operator_id_1,
            ))
            .await
            .unwrap();

        let bls_signature_2 = bls_key_pair_2.sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_2,
                operator_id_2,
            ))
            .await
            .unwrap();

        // Wait for the response from the aggregation service
        let bls_agg_response = agg_response.receive_aggregated_response().await.unwrap();

        // Send the shutdown signal to the OperatorInfoServiceInMemory
        cancellation_token.cancel();

        // Check the response
        let service_manager = IBLSSignatureChecker::new(avs_address, provider);
        service_manager
            .checkSignatures(
                task_response_digest,
                quorum_nums,
                (current_block_num - 1) as u32,
                agg_response_to_non_signer_stakes_and_signature(bls_agg_response),
            )
            .call()
            .await
            .unwrap();
    }
}
