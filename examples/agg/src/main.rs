#![allow(missing_docs)]
use std::time::Duration;

use alloy::{primitives::B256, sol};
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
use tracing::info;

sol! {
    event NewTask(uint256 indexed taskIndex);
}

// 1. Implement the `TaskResponse` trait
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FakeResponse {
    task_index: u32,
    response_digest: B256,
}

impl FakeResponse {
    fn new(task_index: u32, response_digest: [u8; 32]) -> Self {
        Self {
            task_index,
            response_digest: response_digest.into(),
        }
    }
}

impl TaskResponse for FakeResponse {
    fn digest(&self) -> B256 {
        self.response_digest
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
    type NewTaskEvent = NewTask;
    type TaskResponse = FakeResponse;

    async fn process_new_task(
        &self,
        event: Self::NewTaskEvent,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        Ok(TaskMetadata::new(
            event.taskIndex.to::<u32>(),
            12345,
            vec![0],
            vec![50],
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
}
