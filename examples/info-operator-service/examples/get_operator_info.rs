use alloy_primitives::{address, Address, Bytes, FixedBytes, U256};
use alloy_signer_local::PrivateKeySigner;
use eigen_client_avsregistry::{reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter};
use eigen_client_elcontracts::{
    reader::ELChainReader,
    writer::{ELChainWriter, Operator},
};
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_test_logger;
use eigen_services_operatorsinfo::{
    operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceInMemory,
};
use eigen_testing_utils::{
    anvil::start_anvil_container,
    anvil_constants::{
        get_avs_directory_address, get_delegation_manager_address,
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_strategy_manager_address,
    },
};
use std::{
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{task, time::Duration};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    let (_container, http_endpoint, ws_endpoint) = start_anvil_container().await;

    let avs_registry_chain_reader = AvsRegistryChainReader::new(
        get_test_logger().clone(),
        get_registry_coordinator_address(http_endpoint.clone()).await,
        get_operator_state_retriever_address(http_endpoint.clone()).await,
        http_endpoint,
    )
    .await
    .expect("failed to build avs registry chain reader");

    let operators_info =
        OperatorInfoServiceInMemory::new(get_test_logger(), avs_registry_chain_reader, ws_endpoint)
            .await;

    let operators_info_clone = operators_info.clone();
    let cancellation_token: CancellationToken = CancellationToken::new();
    let token_clone = cancellation_token.clone();
    // start the service with a particular block range
    // from block : 0
    // to block : 0 means current block
    task::spawn(async move { operators_info_clone.start_service(&token_clone, 0, 0).await });

    register_operator(
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        "12248929636257230549931416853095037629726205319386239410403476017439825112537",
    )
    .await;

    tokio::time::sleep(Duration::from_secs(2)).await;
    // send cancel token to stop the service
    cancellation_token.cancel();

    // query any operator info from their address
    let res = operators_info
        .get_operator_info(address!("f39fd6e51aad88f6f4ce6ab8827279cfffb92266"))
        .await;
    println!("public key for operator is  : {:?}", res.unwrap());
}

pub async fn register_operator(pvt_key: &str, bls_key: &str) {
    let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;

    let delegation_manager_address = get_delegation_manager_address(http_endpoint.clone()).await;
    let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;
    let strategy_manager_address = get_strategy_manager_address(http_endpoint.clone()).await;
    let el_chain_reader = ELChainReader::new(
        get_test_logger(),
        Address::ZERO,
        delegation_manager_address,
        avs_directory_address,
        http_endpoint.clone(),
    );
    let signer = PrivateKeySigner::from_str(pvt_key).unwrap();

    let el_chain_writer = ELChainWriter::new(
        delegation_manager_address,
        strategy_manager_address,
        el_chain_reader,
        http_endpoint.to_string(),
        pvt_key.to_string(),
    );

    let operator_details = Operator {
        address: signer.address(),
        earnings_receiver_address: signer.address(),
        delegation_approver_address: signer.address(),
        staker_opt_out_window_blocks: 3,
        metadata_url: Some("eigensdk-rs".to_string()),
    };

    let _ = el_chain_writer
        .register_as_operator(operator_details)
        .await
        .unwrap();

    let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
        get_test_logger(),
        http_endpoint.to_string(),
        pvt_key.to_string(),
        get_registry_coordinator_address(http_endpoint.clone()).await,
        get_operator_state_retriever_address(http_endpoint).await,
    )
    .await
    .unwrap();

    let bls_key_pair = BlsKeyPair::new(bls_key.to_string()).unwrap();
    let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);
    let now = SystemTime::now();
    let mut expiry: U256 = U256::from(0);
    // Convert SystemTime to a Duration since the UNIX epoch
    if let Ok(duration_since_epoch) = now.duration_since(UNIX_EPOCH) {
        // Convert the duration to seconds
        let seconds = duration_since_epoch.as_secs(); // Returns a u64

        // Convert seconds to U256
        expiry = U256::from(seconds) + U256::from(10000);
    } else {
        println!("System time seems to be before the UNIX epoch.");
    }
    let quorum_numbers = Bytes::from_str("0x00").unwrap();
    let socket = "socket";

    let _ = avs_registry_writer
        .register_operator_in_quorum_with_avs_registry_coordinator(
            bls_key_pair,
            salt,
            expiry,
            quorum_numbers,
            socket.to_string(),
        )
        .await
        .unwrap();
}
