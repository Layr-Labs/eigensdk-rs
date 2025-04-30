//! Simple example of how to use the aggregator module.
//!
//! 1. Create a contract that emits an event when a task is created.
//! 2. To use the aggregator, you should implement the [`TaskProcessor`] and
//!    [`TaskResponse`] traits and create your own logic for processing the task.
//! 3. Deploy the task contract
//! 4. Create an operator set with a total delegated stake quorum
//! 5. Register operators to the operator set
//! 6. Start the aggregator
//! 7. Emit a task with the contract
//! 8. Send an RPC request to the aggregator with the task response and signature of both operators
//! 9. Since the threshold is reached, the BLS aggregation service will send the aggregated response
//!    to the aggregator

use std::{sync::Arc, time::Duration};

use alloy::{primitives::{address, keccak256, map::HashMap, Address, B256, U256}, sol_types::SolValue};
use eigen_aggregator::{
    config::AggregatorConfig,
    traits::{
        task_processor::{TaskProcessor, TaskProcessorError},
        task_response::TaskResponse,
    },
    Aggregator,
};
use eigen_aggregator::{BlsAggregationServiceResponse, TaskMetadata};
pub use eigen_types::operator::Operator;
use incredible_squaring::bindings::iincrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::{NewTaskCreated, Task, TaskResponse as IncredibleTaskResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;
use tracing::info;

// 2. Implement the `TaskResponse` trait
#[derive(Debug, Clone, Serialize, Deserialize)]
struct NumberSquared {
    task_index: u32,
    number_squared: U256,
}

impl TaskResponse for NumberSquared {
    fn digest(&self) -> B256 {
        keccak256(
            IncredibleTaskResponse {
                referenceTaskIndex: self.task_index,
                numberSquared: self.number_squared,
            }
            .abi_encode(),
        )
    }

    fn task_index(&self) -> u32 {
        self.task_index
    }
}

// 2. Implement the `TaskProcessor` trait
#[derive(Debug, Clone)]
struct IncredibleTaskProcessor {
    tasks: Arc<Mutex<HashMap<u32, Task>>>,
    task_manager_address: Address,
}

impl IncredibleTaskProcessor {
    fn new(task_manager_address: Address) -> Self {
        Self {
            tasks: Default::default(),
            task_manager_address,
        }
    }
}
impl TaskProcessor for IncredibleTaskProcessor {
    type NewTaskEvent = NewTaskCreated;
    type TaskResponse = NumberSquared;

    async fn process_new_task(
        &mut self,
        event: Self::NewTaskEvent,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        let Self::NewTaskEvent {
            taskIndex, task
        } = event.clone();
        self.tasks.lock().await.insert(event.taskIndex, event.task);
        event.task
        Ok(TaskMetadata::new(
            event.taskIndex,
            event.task.taskCreatedBlock,
            vec![0],
            vec![50],
            std::time::Duration::from_secs(60),
        )
        .with_window_duration(Duration::from_secs(15)))
    }

    async fn process_task_response(
        &mut self,
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
    let config = AggregatorConfig {
        server_address: "http://localhost:8081".to_string(),
        registry_coordinator: address!("0x7bc06c482dead17c0e297afbc32f6e63d3846650"),
        operator_state_retriever: address!("0xb0d4afd8879ed9f52b28595d31b441d079b2ca07"),
        http_rpc_url: "http://localhost:8545".to_string(),
        ws_rpc_url: "ws://localhost:8545".to_string(),
    };
    let processor =
        IncredibleTaskProcessor::new(address!("0x7bc06c482dead17c0e297afbc32f6e63d3846650"));
    let aggregator = Aggregator::new(config, processor).await.unwrap();

    // Start the aggregator in the background
    aggregator.start().await.unwrap();
}
