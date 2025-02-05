use alloy::primitives::{Address, Bytes, FixedBytes, U256};
use alloy::signers::local::PrivateKeySigner;
use eigen_client_avsregistry::{reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter};
use eigen_client_elcontracts::{
    reader::ELChainReader,
    writer::{ELChainWriter, Operator},
};
use eigen_common::get_provider;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_test_logger;
use eigen_services_operatorsinfo::{
    operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceInMemory,
};
use eigen_testing_utils::anvil_constants::get_allocation_manager_address;
use eigen_testing_utils::m2_holesky_constants::DELEGATION_MANAGER_ADDRESS;
use eigen_testing_utils::{
    anvil::{set_account_balance, start_anvil_container},
    anvil_constants::{
        get_avs_directory_address, get_delegation_manager_address,
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_rewards_coordinator_address, get_strategy_manager_address,
    },
    transaction::wait_transaction,
};
use eigen_utils::slashing::core::delegationmanager::DelegationManager;
use std::{
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{task, time::sleep};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let (container, http_endpoint, ws_endpoint) = start_anvil_container().await;
    let operator_private_key = "0x6b35c6d8110c888de06575b45181bf3f9e6c73451fa5cde812c95a6b31e66ddf";
    let operator_address = "009440d62dc85c73dbf889b7ad1f4da8b231d2ef";
    let operator_bls_key =
        "12248929636257230549931416853095037629726205319386239410403476017439825112537";
    set_account_balance(&container, operator_address, "http://localhost:8546").await;

    let avs_registry_chain_reader = AvsRegistryChainReader::new(
        get_test_logger().clone(),
        get_registry_coordinator_address(http_endpoint.clone()).await,
        get_operator_state_retriever_address(http_endpoint.clone()).await,
        http_endpoint.clone(),
    )
    .await
    .expect("failed to build avs registry chain reader");

    let operators_info =
        OperatorInfoServiceInMemory::new(get_test_logger(), avs_registry_chain_reader, ws_endpoint)
            .await
            .unwrap()
            .0;

    let operators_info_clone = operators_info.clone();
    let cancellation_token: CancellationToken = CancellationToken::new();
    let token_clone = cancellation_token.clone();
    // start the service with a particular block range
    // from block : 0
    // to block : 0 means current block
    task::spawn(async move { operators_info_clone.start_service(&token_clone, 0, 0).await });

    register_operator(operator_private_key, operator_bls_key, &http_endpoint).await;

    sleep(Duration::from_secs(2)).await;
    // send cancel token to stop the service
    cancellation_token.cancel();

    // query any operator info from their address
    let res = operators_info
        .get_operator_info(Address::from_str(operator_address).unwrap())
        .await;
    println!("public key for operator is  : {:?}", res.unwrap());
}

pub async fn register_operator(pvt_key: &str, bls_key: &str, http_endpoint: &str) {
    let signer = PrivateKeySigner::from_str(pvt_key).unwrap();

    let delegation_manager_address = get_delegation_manager_address(http_endpoint.to_owned()).await;
    let avs_directory_address = get_avs_directory_address(http_endpoint.to_owned()).await;
    let strategy_manager_address = get_strategy_manager_address(http_endpoint.to_owned()).await;
    let rewards_coordinator_address =
        get_rewards_coordinator_address(http_endpoint.to_owned()).await;
    let delegation_manager_contract =
        DelegationManager::new(DELEGATION_MANAGER_ADDRESS, get_provider(http_endpoint));
    let permission_controller = delegation_manager_contract
        .permissionController()
        .call()
        .await
        .unwrap()
        ._0;

    let el_chain_reader = ELChainReader::new(
        get_test_logger(),
        Some(Address::ZERO),
        delegation_manager_address,
        rewards_coordinator_address,
        avs_directory_address,
        Some(permission_controller),
        http_endpoint.to_owned(),
    );
    let allocation_manager = get_allocation_manager_address(http_endpoint.to_string()).await;
    let registry_coordinator = get_registry_coordinator_address(http_endpoint.to_string()).await;

    let el_chain_writer = ELChainWriter::new(
        strategy_manager_address,
        rewards_coordinator_address,
        Some(permission_controller),
        Some(allocation_manager),
        registry_coordinator,
        el_chain_reader,
        http_endpoint.to_string(),
        pvt_key.to_string(),
    );

    let operator_details = Operator {
        address: signer.address(),
        delegation_approver_address: signer.address(),
        staker_opt_out_window_blocks: 3,
        metadata_url: Some("eigensdk-rs".to_string()),
        allocation_delay: 1,
    };

    let _ = el_chain_writer
        .register_as_operator(operator_details)
        .await
        .unwrap();

    let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
        get_test_logger(),
        http_endpoint.to_string(),
        pvt_key.to_string(),
        get_registry_coordinator_address(http_endpoint.to_owned()).await,
        get_operator_state_retriever_address(http_endpoint.to_owned()).await,
    )
    .await
    .unwrap();

    let bls_key_pair = BlsKeyPair::new(bls_key.to_string()).unwrap();
    let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);
    let now = SystemTime::now();
    let seconds_since_epoch = now.duration_since(UNIX_EPOCH).unwrap().as_secs();
    let expiry = U256::from(seconds_since_epoch) + U256::from(10000);
    let quorum_numbers = Bytes::from_str("0x00").unwrap();
    let socket = "socket";

    let tx_hash = avs_registry_writer
        .register_operator_in_quorum_with_avs_registry_coordinator(
            bls_key_pair,
            salt,
            expiry,
            quorum_numbers,
            socket.to_string(),
        )
        .await
        .unwrap();
    wait_transaction(http_endpoint, tx_hash).await.unwrap();
}
