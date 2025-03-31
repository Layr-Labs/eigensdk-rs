//! This is a simple example of how to use the aggregator.
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

use std::time::Duration;

use alloy::primitives::{FixedBytes, B256};
use alloy::sol;
use alloy::transports::http::reqwest;
use eigen_aggregator::{
    config::AggregatorConfig,
    traits::{
        task_processor::{box_error, TaskProcessor, TaskProcessorError},
        task_response::TaskResponse,
    },
    Aggregator,
};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_common::get_signer;
use eigen_crypto_bls::{BlsG1Point, BlsG2Point, BlsKeyPair, Signature};
use eigen_logging::get_test_logger;
use eigen_logging::{init_logger, log_level::LogLevel};
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigen_testing_utils::anvil::start_anvil_container;
use eigen_testing_utils::anvil_constants::{
    get_erc20_mock_strategy, get_operator_state_retriever_address,
    get_registry_coordinator_address, get_service_manager_address, FIRST_ADDRESS,
    FIRST_PRIVATE_KEY, OPERATOR_BLS_KEY, OPERATOR_BLS_KEY_2, SECOND_ADDRESS, SECOND_PRIVATE_KEY,
};
use eigen_testing_utils::chain_clients::{
    create_total_delegated_stake_operator_set, new_test_writer,
};
pub use eigen_types::operator::Operator;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::info;

// 1. Fake contract to emit event
sol! {
    #[allow(missing_docs)]
    #[derive(Debug)]
    // IF WE MODIFY THE CONTRACT, WE NEED TO UPDATE THE BYTECODE - `solc --bin <CONTRACT_NAME>.sol`
    #[sol(rpc, bytecode = "6080604052348015600e575f5ffd5b506101868061001c5f395ff3fe608060405234801561000f575f5ffd5b5060043610610034575f3560e01c80635867173014610038578063a747649314610056575b5f5ffd5b610040610060565b60405161004d91906100c3565b60405180910390f35b61005e610065565b005b5f5481565b5f5f81548092919061007690610109565b91905055505f547f0308e35d068a731dcf227a01af50313408b863f815670a72362dfa93fdd0686d60405160405180910390a2565b5f819050919050565b6100bd816100ab565b82525050565b5f6020820190506100d65f8301846100b4565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610113826100ab565b91507fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203610145576101446100dc565b5b60018201905091905056fea264697066735822122066967740649d975b00c00d92c8ad08ecca35b10d89341e538ea5d4e4c3d0852b64736f6c634300081d0033")]
    contract TaskContract {
        uint256 public taskCounter;

        event NewTask(uint256 indexed taskIndex);

        function createTask() external {
            taskCounter++;
            emit NewTask(taskCounter);
        }
    }
}

// 2. Implement the `TaskResponse` trait
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FakeResponse {
    response: String,
}

impl FakeResponse {
    pub fn new(response: String) -> Self {
        Self { response }
    }
}

impl TaskResponse for FakeResponse {
    fn digest(&self) -> B256 {
        B256::new([0x01; 32])
    }

    fn task_index(&self) -> u32 {
        1
    }
}

// 2. Implement the `TaskProcessor` trait
#[derive(Debug)]
struct MockTaskProcessor {
    aggregated_response: mpsc::Sender<BlsAggregationServiceResponse>,
}

impl MockTaskProcessor {
    fn new(sender: mpsc::Sender<BlsAggregationServiceResponse>) -> Self {
        Self {
            aggregated_response: sender,
        }
    }
}
impl TaskProcessor for MockTaskProcessor {
    type NewTaskEvent = TaskContract::NewTask;
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
        info!(
            "Aggregated response received for task {}: {:?}",
            response.task_index, response.task_response_digest
        );

        self.aggregated_response.send(response).await.map_err(|e| {
            box_error(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to send response: {}", e),
            ))
        })
    }
}

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let (_container, http_rpc, ws_rpc) = start_anvil_container().await;

    let el_chain_writer = new_test_writer(http_rpc.clone(), FIRST_PRIVATE_KEY.to_string()).await;
    let el_chain_writer_2 = new_test_writer(http_rpc.clone(), SECOND_PRIVATE_KEY.to_string()).await;
    let avs_registry = AvsRegistryChainReader::new(
        get_test_logger(),
        get_registry_coordinator_address(http_rpc.clone()).await,
        get_operator_state_retriever_address(http_rpc.clone()).await,
        http_rpc.clone(),
    )
    .await
    .unwrap();

    // 3. Deploy the task contract
    let provider = get_signer(FIRST_PRIVATE_KEY, &http_rpc);
    let task_contract = TaskContract::deploy(&provider).await.unwrap();
    let avs_address = get_service_manager_address(http_rpc.clone()).await;

    // 4. Create quorums and operator sets
    create_total_delegated_stake_operator_set(
        &http_rpc,
        get_erc20_mock_strategy(http_rpc.clone()).await,
        avs_address,
    )
    .await;
    info!("Operator set created");

    // 5. Register operator to operator set
    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();
    el_chain_writer
        .register_for_operator_sets(FIRST_ADDRESS, avs_address, vec![0], bls_key_pair, "socket")
        .await
        .unwrap();
    info!("First operator registered to operator set");

    // 5. Register second operator to operator set
    let bls_key_pair_2 = BlsKeyPair::new(OPERATOR_BLS_KEY_2.to_string()).unwrap();
    el_chain_writer_2
        .register_for_operator_sets(
            SECOND_ADDRESS,
            avs_address,
            vec![0],
            bls_key_pair_2,
            "socket",
        )
        .await
        .unwrap();
    info!("Second operator registered to operator set");

    let operator_id = avs_registry.get_operator_id(FIRST_ADDRESS).await.unwrap();
    let operator_id_2 = avs_registry.get_operator_id(SECOND_ADDRESS).await.unwrap();

    // 6. Set up the aggregator config and initialize the processor
    let registry_coordinator = get_registry_coordinator_address(http_rpc.clone()).await;
    let operator_state_retriever = get_operator_state_retriever_address(http_rpc.clone()).await;
    let config = AggregatorConfig {
        server_address: "127.0.0.1:8081".to_string(),
        registry_coordinator,
        operator_state_retriever,
        http_rpc_url: http_rpc.clone(),
        ws_rpc_url: ws_rpc.clone(),
    };

    let (sender, mut receiver) = mpsc::channel(1);
    let processor = MockTaskProcessor::new(sender);
    let aggregator = Aggregator::new(config, processor).await.unwrap();

    // 6. Start the aggregator in the background
    let aggregator_handle = tokio::spawn(aggregator.start(ws_rpc.clone()));

    // Wait for the aggregator to initialize
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // 7. Emit a new task event
    let result = task_contract
        .createTask()
        .send()
        .await
        .unwrap()
        .get_receipt()
        .await
        .unwrap();
    info!("Task created: {:?}", result.transaction_hash);

    tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    // Send fake response from operator
    info!("Simulating operator response");

    // 8. Send fake response from the first operator
    let fake_response = FakeResponse::new("Hello world".to_string());
    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();
    let bls_signature = bls_key_pair.sign_message(fake_response.digest().as_ref());
    send_signed_task_response(fake_response, bls_signature.clone(), operator_id).await;
    info!("Response from first operator sent");

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    info!("Threshold reached but there is a window to send another response");

    // 8. Send fake response from second operator
    let fake_response_2 = FakeResponse::new("Hello world".to_string());
    let bls_key_pair_2 = BlsKeyPair::new(OPERATOR_BLS_KEY_2.to_string()).unwrap();
    let bls_signature_2 = bls_key_pair_2.sign_message(fake_response_2.digest().as_ref());
    send_signed_task_response(fake_response_2, bls_signature_2.clone(), operator_id_2).await;
    info!("Response from second operator sent");

    // Aggregate the bls signatures manually
    let quorum_apks_g1 = aggregate_g1_public_keys(&[bls_key_pair.clone(), bls_key_pair_2.clone()]);
    let signers_apk_g2 = aggregate_g2_public_keys(&[bls_key_pair.clone(), bls_key_pair_2.clone()]);
    let signers_agg_sig_g1 =
        aggregate_g1_signatures(&[bls_signature.clone(), bls_signature_2.clone()]);

    // Wait for the aggregator to aggregate the responses and then compare the results
    let aggregated_response = receiver.recv().await.unwrap();
    assert_eq!(aggregated_response.quorum_apks_g1, vec![quorum_apks_g1]);
    assert_eq!(aggregated_response.signers_apk_g2, signers_apk_g2);
    assert_eq!(aggregated_response.signers_agg_sig_g1, signers_agg_sig_g1);
    info!("Compared aggregated response with manual aggregation and they are equal");

    // Keep the service running until the aggregator is closed
    aggregator_handle.abort();
    if aggregator_handle.await.is_err() {
        info!("Aggregator finished");
        return;
    }
}

async fn send_signed_task_response(
    task_response: FakeResponse,
    signature: Signature,
    operator_id: FixedBytes<32>,
) {
    let client = reqwest::Client::new();
    client
        .post("http://127.0.0.1:8081")
        .json(&serde_json::json!({
            "jsonrpc": "2.0",
            "method": "process_signed_task_response",
            "params": {
                "params": {
                    "task_response": task_response,
                    "signature": signature,
                    "operator_id": operator_id
                }
            },
            "id": 1
        }))
        .send()
        .await
        .unwrap();
}

// Aggregate the g1 public keys
fn aggregate_g1_public_keys(bls_key_pairs: &[BlsKeyPair]) -> BlsG1Point {
    bls_key_pairs
        .iter()
        .map(|bls| bls.public_key().g1())
        .reduce(|a, b| (a + b).into())
        .map(BlsG1Point::new)
        .unwrap()
}

// Aggregate the g2 public keys
fn aggregate_g2_public_keys(bls_key_pairs: &[BlsKeyPair]) -> BlsG2Point {
    bls_key_pairs
        .iter()
        .map(|bls| bls.public_key_g2().g2())
        .reduce(|a, b| (a + b).into())
        .map(BlsG2Point::new)
        .unwrap()
}

fn aggregate_g1_signatures(signatures: &[Signature]) -> Signature {
    let agg = signatures
        .iter()
        .map(|s| s.g1_point().g1())
        .reduce(|a, b| (a + b).into())
        .unwrap();
    Signature::new(agg)
}
