#![allow(missing_docs)]

use alloy::primitives::address;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_logger;
use eigen_operator::{error::OperatorError, Operator};
use eigen_testing_utils::anvil_constants::{FIRST_ADDRESS, OPERATOR_BLS_KEY};
use incredible_squaring::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;

fn square(event: NewTaskCreated) -> Result<NumberSquared, OperatorError> {
    let square = event.task.numberToBeSquared * event.task.numberToBeSquared;
    Ok(NumberSquared {
        task_index: event.taskIndex,
        number_squared: square,
    })
}

// This example shows how to initialize an operator and start to listen for new task events.
// For this example, Operator should be registered.
#[tokio::main]
async fn main() {
    let registry_coordiator_address = address!("0x7bc06c482dead17c0e297afbc32f6e63d3846650");
    let operator_state_retriever_address = address!("0xb0d4afd8879ed9f52b28595d31b441d079b2ca07");
    let server_address = "http://localhost:8080".to_string();
    let http_rpc_url = "http://localhost:8545".to_string();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let logger = get_logger();

    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();

    // Initialize the operator
    let operator = Operator::new(
        &bls_key_pair,
        FIRST_ADDRESS,
        "OPERATOR NAME",
        logger,
        &ws_rpc_url,
        &http_rpc_url,
        registry_coordiator_address,
        operator_state_retriever_address,
        server_address,
    )
    .await
    .unwrap();

    // Subscribe to the new task events and start listening. When a new task is created,
    // the operator will process it and send the signed task response to the aggregator.
    operator.start(square).await.unwrap();
}
