#![allow(missing_docs)]

use alloy::{
    primitives::{address, keccak256, B256, U256},
    sol_types::SolValue,
};
use bindings::iincrediblesquaringtaskmanager::IIncredibleSquaringTaskManager;
use eigen_aggregator::TaskResponse;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::BlsKeyPair;
use eigen_logging::get_logger;
use eigen_operator::{
    client::ClientAggregator, operator_task_processor::OperatorTaskProcessor, Operator,
};
use eigen_testing_utils::anvil_constants::{FIRST_ADDRESS, OPERATOR_BLS_KEY};
use serde::{Deserialize, Serialize};

// Allow warnings in auto-generated code
#[allow(warnings)]
pub mod bindings;

// 1. Implement the `TaskResponse` trait
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FakeResponse {
    task_index: u32,
    number_squared: U256,
}

impl TaskResponse for FakeResponse {
    fn digest(&self) -> B256 {
        let response = IIncredibleSquaringTaskManager::TaskResponse {
            referenceTaskIndex: self.task_index,
            numberSquared: self.number_squared,
        };
        keccak256(response.abi_encode())
    }

    fn task_index(&self) -> u32 {
        self.task_index
    }
}

// 2. Implement the OperatorTaskProcessor trait
struct OperatorTaskProcessorImpl;

impl OperatorTaskProcessor for OperatorTaskProcessorImpl {
    type NewTaskEvent = IIncredibleSquaringTaskManager::NewTaskCreated;
    type TaskResponse = FakeResponse;

    fn process_new_task(&self, new_task_created: Self::NewTaskEvent) -> Self::TaskResponse {
        FakeResponse {
            task_index: new_task_created.taskIndex,
            number_squared: new_task_created.task.numberToBeSquared,
        }
    }
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

    let avs_registry_reader = AvsRegistryChainReader::new(
        get_logger(),
        registry_coordiator_address,
        operator_state_retriever_address,
        http_rpc_url,
    )
    .await
    .unwrap();
    let operator_task_processor = OperatorTaskProcessorImpl;
    let client_aggregator = ClientAggregator::new(server_address).await.unwrap();
    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();

    // Initialize the operator
    let operator = Operator::new(
        &avs_registry_reader,
        &bls_key_pair,
        FIRST_ADDRESS,
        "OPERATOR NAME",
        &client_aggregator,
        &ws_rpc_url,
        operator_task_processor,
    )
    .await
    .unwrap();

    // Subscribe to the new task events and start listening. When a new task is created,
    // the operator will process it and send the signed task response to the aggregator.
    operator.start().await.unwrap();
}
