#![allow(missing_docs)]

use alloy::primitives::{address, U256};
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_logger;
use eigen_operator::{config::OperatorConfig, failing_response_calculator, Operator};
use eigen_testing_utils::anvil_constants::{FIRST_ADDRESS, OPERATOR_BLS_KEY};
use incredible_squaring::{square, ISTaskManager};

// This example shows how to initialize an operator and start to listen for new task events.
// For this example, Operator should be registered.
#[tokio::main]
async fn main() {
    let registry_coordinator_address = address!("0x7bc06c482dead17c0e297afbc32f6e63d3846650");
    let operator_state_retriever_address = address!("0xb0d4afd8879ed9f52b28595d31b441d079b2ca07");
    let server_address = "http://localhost:8080".to_string();
    let http_rpc_url = "http://localhost:8545".to_string();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let logger = get_logger();

    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();

    let config = OperatorConfig {
        bls_key_pair,
        operator_address: FIRST_ADDRESS,
        operator_name: "OPERATOR NAME".to_string(),
        ws_rpc_url,
        http_rpc_url,
        registry_coordinator_address,
        operator_state_retriever_address,
        aggregator_ip_port: server_address,
        registration: None,
    };

    // Initialize the operator
    let operator = Operator::new(logger, config).await.unwrap();

    let logic = failing_response_calculator(square, |_, _| async { Ok(U256::from(42)) }, 60);

    // Subscribe to the new task events and start listening. When a new task is created,
    // the operator will process it and send the signed task response to the aggregator.
    operator.start::<ISTaskManager>(logic.await).await.unwrap();
}
