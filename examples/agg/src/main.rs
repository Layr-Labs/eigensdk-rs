#![allow(missing_docs)]
use alloy::{
    primitives::{keccak256, B256, U256},
    sol_types::SolValue,
};
use bindings::iincrediblesquaringtaskmanager::IIncredibleSquaringTaskManager;
use eigen_aggregator::{
    traits::{
        task_processor::{TaskProcessor, TaskProcessorError},
        task_response::TaskResponse,
    },
    Aggregator,
};
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::info;

// Allow warnings in auto-generated code
#[allow(warnings)]
mod bindings;

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

// 2. Implement the `TaskProcessor` trait
#[derive(Debug)]
struct TaskProcessorImpl {}

impl TaskProcessorImpl {
    fn new() -> Self {
        Self {}
    }
}
impl TaskProcessor for TaskProcessorImpl {
    type NewTaskEvent = IIncredibleSquaringTaskManager::NewTaskCreated;
    type TaskResponse = FakeResponse;

    async fn process_new_task(
        &self,
        event: Self::NewTaskEvent,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        let quorum_numbers: Vec<u8> = event.task.quorumNumbers.into();
        let quorum_threshold_percentages =
            std::iter::repeat_n(event.task.quorumThresholdPercentage, quorum_numbers.len())
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect();
        Ok(TaskMetadata::new(
            event.taskIndex,
            event.task.taskCreatedBlock.into(),
            quorum_numbers,
            quorum_threshold_percentages,
            std::time::Duration::from_secs(60),
        )
        .with_window_duration(Duration::from_secs(15)))
    }

    async fn process_task_response(
        &self,
        response: Self::TaskResponse,
    ) -> Result<B256, TaskProcessorError> {
        Ok(response.digest())
    }

    async fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        info!(
            "Aggregated response received for task {}: {:?}",
            response.task_index, response.task_response_digest
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let regcoord_address: [u8; 20] = hex::decode("7969c5ed335650692bc04293b07f5bf2e7a673c0")
        .unwrap()
        .try_into()
        .unwrap();
    let opstate_address: [u8; 20] = hex::decode("1429859428c0abc9c2c47c8ee9fbaf82cfa0f20f")
        .unwrap()
        .try_into()
        .unwrap();
    let config = eigen_aggregator::config::AggregatorConfig {
        server_address: "http://localhost:8080".to_string(),
        http_rpc_url: "http://localhost:8545".to_string(),
        ws_rpc_url: "ws://localhost:8545".to_string(),
        registry_coordinator: regcoord_address.into(),
        operator_state_retriever: opstate_address.into(),
    };

    let tp = TaskProcessorImpl::new();
    let aggregator = Aggregator::new(config, tp).await.unwrap();

    aggregator
        .start("ws://localhost:8545".to_string())
        .await
        .unwrap();
}
