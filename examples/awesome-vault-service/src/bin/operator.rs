#![allow(missing_docs)]

use alloy::primitives::{Address, U256};
use awesome_vault_service::{
    task_manager::{save_value, save_wrong_value, ISTaskManager},
    utils::setup_operator,
};
use eigen_operator::failing_response_calculator;
use eigensdk::{
    crypto_bls::BlsKeyPair,
    logging::{get_logger, init_logger, log_level::LogLevel},
    operator::{config::OperatorConfig, Operator},
    testing_utils::anvil_constants::{FIRST_ADDRESS, FIRST_PRIVATE_KEY, OPERATOR_BLS_KEY},
};

use std::{collections::BTreeMap, str::FromStr, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;

// This example shows how to initialize an operator and start to listen for new task events.
// For this example, Operator should be registered.
#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();
    let operator_address = FIRST_ADDRESS;
    let operator_private_key = FIRST_PRIVATE_KEY;
    let operator_name = "dot-product-god";
    let logger = get_logger();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let http_rpc_url = "http://localhost:8545".to_string();
    let aggregator_ip_port = "127.0.0.1:8080".to_string();
    let allocation_manager =
        Address::from_str("0x2279b7a0a67db372996a5fab50d91eaa73d2ebe6").unwrap();
    let delegation_manager_address =
        Address::from_str("0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0").unwrap();
    let avs_directory_address =
        Address::from_str("0x610178da211fef7d417bc0e6fed39f05609ad788").unwrap();
    let strategy_manager_address =
        Address::from_str("0x0165878a594ca255338adfa4d48449f69242eb8f").unwrap();
    let rewards_coordinator_address =
        Address::from_str("0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0").unwrap();
    let permission_controller_address =
        Address::from_str("0x59b670e9fa9d0a427751af201d676719a970857b").unwrap();
    let strategy_address = Address::from_str("0x2b961e3959b79326a8e7f64ef0d2d825707669b5").unwrap();
    let avs = Address::from_str("0x5f3f1dbd7b74c6b46e8c44f98792a1daf8d69154").unwrap();
    let registry_coordinator_address =
        Address::from_str("0x7bc06c482dead17c0e297afbc32f6e63d3846650").unwrap();
    let operator_state_retriever_address =
        Address::from_str("0x4c5859f0f772848b2d91f1d83e2fe57935348029").unwrap();

    setup_operator(
        bls_key_pair.clone(),
        Some(operator_private_key.to_string()),
        "ecdsa_keystore_path".to_string(),
        "ecdsa_keystore_password".to_string(),
        http_rpc_url.clone(),
        "metadata_uri".to_string(),
        "socket".to_string(),
        0,
        0,
        U256::from_str("5000000000000000000000").unwrap(),
        vec![1000000000000000000],
        permission_controller_address,
        rewards_coordinator_address,
        allocation_manager,
        registry_coordinator_address,
        delegation_manager_address,
        avs_directory_address,
        strategy_manager_address,
        strategy_address,
        avs,
    )
    .await
    .unwrap();

    info!("Operator setup complete");

    let operator_config = OperatorConfig {
        bls_key_pair,
        operator_address,
        operator_name: operator_name.to_string(),
        ws_rpc_url: ws_rpc_url.to_string(),
        http_rpc_url: http_rpc_url.to_string(),
        registry_coordinator_address,
        operator_state_retriever_address,
        aggregator_ip_port,
        registration: None,
    };

    // Initialize the operator
    let operator = Operator::new(logger, operator_config).await.unwrap();

    let redis_state = Arc::new(Mutex::new(BTreeMap::<String, String>::new()));

    // TESTING PURPOSES ONLY
    let compute = failing_response_calculator(
        save_value(redis_state.clone()).await,
        save_wrong_value(redis_state).await,
        50,
    );

    // let compute = save_value(redis_state);

    operator
        .start::<ISTaskManager>(compute.await)
        .await
        .unwrap();
}
