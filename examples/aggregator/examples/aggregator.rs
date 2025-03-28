//! This is an example of how to use the aggregator.
use alloy::sol;
use alloy::{primitives::B256, transports::http::reqwest};
use eigen_aggregator::{
    config::AggregatorConfig,
    traits::{
        task_processor::{box_error, TaskProcessor, TaskProcessorError},
        task_response::TaskResponse,
    },
    Aggregator, AggregatorError,
};
use eigen_logging::{init_logger, log_level::LogLevel};
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigen_testing_utils::{
    anvil::start_anvil_container,
    anvil_constants::{get_operator_state_retriever_address, get_registry_coordinator_address},
};
use serde::{Deserialize, Serialize};

// Fake task event
sol! {
    #[derive(Debug)]
    event FakeTaskEvent(uint256 indexed taskIndex);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FakeResponse {
    pub response: String,
}

impl TaskResponse for FakeResponse {
    fn digest(&self) -> B256 {
        B256::ZERO
    }

    fn task_index(&self) -> u32 {
        1
    }
}

#[derive(Debug, Clone)]
struct MockTaskProcessor;

impl TaskProcessor for MockTaskProcessor {
    type NewTaskEvent = FakeTaskEvent;
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
        ))
    }

    async fn process_task_response(
        &self,
        response: Self::TaskResponse,
    ) -> Result<B256, TaskProcessorError> {
        if response.response.is_empty() {
            return Err(box_error(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Empty result",
            )));
        }
        Ok(response.digest())
    }

    async fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        println!(
            "Aggregated response received for task {}: {:?}",
            response.task_index, response.task_response_digest
        );
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), AggregatorError> {
    init_logger(LogLevel::Info);
    let (_container, http_rpc, ws_rpc) = start_anvil_container().await;

    let registry_coordinator = get_registry_coordinator_address(http_rpc.clone()).await;
    let operator_state_retriever = get_operator_state_retriever_address(http_rpc.clone()).await;

    let config = AggregatorConfig {
        server_address: "127.0.0.1:8081".to_string(),
        registry_coordinator,
        operator_state_retriever,
        http_rpc_url: http_rpc.clone(),
        ws_rpc_url: ws_rpc.clone(),
    };
    let processor = MockTaskProcessor;

    // Initialize the aggregator
    let aggregator = Aggregator::new(config, processor).await?;

    // Start the aggregator in the background
    let aggregator_handle = tokio::spawn(aggregator.start(ws_rpc.clone()));

    // Wait for the aggregator to initialize
    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    dbg!("Timer done");

    // EMIT A NEW TASK WITH EVENT

    // Send fake response from operator
    tokio::spawn(async move {
        dbg!("Sending response");
        let client = reqwest::Client::new();
        let response = client
            .post("http://127.0.0.1:8081")
            .json(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": "process_signed_task_response",
                "params": {
                    "params": {
                        "task_response": "Hello world",
                        "signature": "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
                        "operator_id": 1
                    }
                },
                "id": 1
            }))
            .send()
            .await;

        println!("Response: {:?}", response);
    });

    // Keep the service running
    let result = aggregator_handle
        .await
        .map_err(|_| AggregatorError::InternalError)?;

    println!("Result: {:?}", result);

    Ok(())
}
