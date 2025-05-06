#![allow(missing_docs)]

use alloy::primitives::{address, U256};
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_logger;
use eigen_operator::{config::TestingOperatorConfig, error::OperatorError, TestingOperator};
use eigen_task_processor::task_response::TaskResponse;
use eigen_testing_utils::anvil_constants::{FIRST_ADDRESS, OPERATOR_BLS_KEY};
use incredible_squaring::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;

fn square(event: NewTaskCreated) -> Result<TaskResponse<U256>, OperatorError> {
    let square = event.task.numberToBeSquared * event.task.numberToBeSquared;
    Ok(TaskResponse {
        task_index: event.taskIndex,
        response: square,
    })
}

fn wrong_square(event: NewTaskCreated) -> Result<TaskResponse<U256>, OperatorError> {
    Ok(TaskResponse {
        task_index: event.taskIndex,
        response: U256::from(3),
    })
}

// This example shows how to initialize a testing operator and start to listen for new task events.
// This operator have two different logic to compute the task response and depends on the failure rate,
// it will fail the task.
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

    let config = TestingOperatorConfig {
        bls_key_pair,
        operator_address: FIRST_ADDRESS,
        operator_name: "OPERATOR NAME".to_string(),
        ws_rpc_url,
        http_rpc_url,
        registry_coordinator_address,
        operator_state_retriever_address,
        aggregator_ip_port: server_address,
        failure_rate: 80,
    };

    // Initialize the testing operator
    let operator = TestingOperator::new(logger, config).await.unwrap();

    // Subscribe to the new task events and start listening. When a new task is created,
    // the operator will process it. There are a `failure_rate` chance that the operator will fail the task.
    operator.start(square, wrong_square).await.unwrap();
}
