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

use ark_ec::AffineRepr;
use std::{sync::Arc, time::Duration};

use alloy::{
    primitives::{address, keccak256, map::HashMap, Address, B256, U256},
    sol_types::SolValue,
};
use eigen_aggregator::{
    config::AggregatorConfig,
    traits::{
        task_processor::{TaskProcessor, TaskProcessorError},
        task_response::TaskResponse,
    },
    Aggregator,
};
use eigen_aggregator::{BlsAggregationServiceResponse, TaskMetadata};
use eigen_common::get_provider;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point, error::BlsError};
pub use eigen_types::operator::Operator;
use incredible_squaring::bindings::incrediblesquaringtaskmanager::{
    IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
    IIncredibleSquaringTaskManager::{Task, TaskResponse as IncredibleTaskResponse},
    IncredibleSquaringTaskManager::{self, NewTaskCreated},
    BN254::{G1Point, G2Point},
};
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
        let task_response = IncredibleTaskResponse {
            referenceTaskIndex: self.task_index,
            numberSquared: self.number_squared,
        };
        keccak256(task_response.abi_encode())
    }

    fn task_index(&self) -> u32 {
        self.task_index
    }
}

// 2. Implement the `TaskProcessor` trait
#[derive(Debug, Clone)]
struct IncredibleTaskProcessor {
    tasks: Arc<Mutex<HashMap<u32, Task>>>,
    task_responses: Arc<Mutex<HashMap<B256, NumberSquared>>>,
    task_manager_address: Address,
    http_rpc_url: String,
}

impl IncredibleTaskProcessor {
    fn new(task_manager_address: Address, http_rpc_url: String) -> Self {
        Self {
            tasks: Default::default(),
            task_responses: Default::default(),
            task_manager_address,
            http_rpc_url,
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
        let quorum_numbers: Vec<u8> = event.task.quorumNumbers.clone().into();
        let quorum_threshold_percentages =
            std::iter::repeat_n(event.task.quorumThresholdPercentage, quorum_numbers.len())
                .into_iter()
                .map(|x| x.try_into().unwrap())
                .collect();
        let metadata = TaskMetadata::new(
            event.taskIndex,
            event.task.taskCreatedBlock.into(),
            quorum_numbers,
            quorum_threshold_percentages,
            std::time::Duration::from_secs(60),
        )
        .with_window_duration(Duration::from_secs(15));
        self.tasks.lock().await.insert(event.taskIndex, event.task);
        Ok(metadata)
    }

    async fn process_task_response(
        &mut self,
        response: Self::TaskResponse,
    ) -> Result<B256, TaskProcessorError> {
        let digest = response.digest();
        self.task_responses.lock().await.insert(digest, response);
        Ok(digest)
    }

    async fn process_aggregated_response(
        &self,
        agg_response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        info!(
            "Aggregated response received for task {}: {:?}",
            agg_response.task_index, agg_response.task_response_digest
        );
        let number_squared_response = self
            .task_responses
            .lock()
            .await
            .get(&agg_response.task_response_digest)
            .unwrap()
            .clone();
        let task = self
            .tasks
            .lock()
            .await
            .get(&agg_response.task_index)
            .unwrap()
            .clone();

        let task_manager = IncredibleSquaringTaskManager::new(
            self.task_manager_address,
            get_provider(&self.http_rpc_url),
        );
        let task_response = IncredibleTaskResponse {
            referenceTaskIndex: number_squared_response.task_index,
            numberSquared: number_squared_response.number_squared,
        };

        let non_signer_stakes_and_signature =
            get_non_signer_stakes_and_signature(agg_response).unwrap();

        task_manager
            .respondToTask(task, task_response, non_signer_stakes_and_signature)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        Ok(())
    }
}

fn get_non_signer_stakes_and_signature(
    agg_response: BlsAggregationServiceResponse,
) -> Result<NonSignerStakesAndSignature, BlsError> {
    let mut non_signer_pub_keys = Vec::<G1Point>::new();
    for pub_key in agg_response.non_signers_pub_keys_g1.iter() {
        if pub_key.g1().x().is_some() {
            let g1 = convert_to_g1_point(pub_key.g1())?;
            non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
        } else {
            info!(
                "Zero non_signers for the task index :{:?}",
                agg_response.task_index
            );
        }
    }

    let mut quorum_apks = Vec::<G1Point>::new();
    for pub_key in agg_response.quorum_apks_g1.iter() {
        let g1 = convert_to_g1_point(pub_key.g1())?;
        quorum_apks.push(G1Point { X: g1.X, Y: g1.Y })
    }

    let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
        nonSignerPubkeys: non_signer_pub_keys,
        nonSignerQuorumBitmapIndices: agg_response.non_signer_quorum_bitmap_indices,
        quorumApks: quorum_apks,
        apkG2: G2Point {
            X: convert_to_g2_point(agg_response.signers_apk_g2.g2())?.X,
            Y: convert_to_g2_point(agg_response.signers_apk_g2.g2())?.Y,
        },
        sigma: G1Point {
            X: convert_to_g1_point(agg_response.signers_agg_sig_g1.g1_point().g1())?.X,
            Y: convert_to_g1_point(agg_response.signers_agg_sig_g1.g1_point().g1())?.Y,
        },
        quorumApkIndices: agg_response.quorum_apk_indices,
        totalStakeIndices: agg_response.total_stake_indices,
        nonSignerStakeIndices: agg_response.non_signer_stake_indices,
    };
    Ok(non_signer_stakes_and_signature)
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
    let processor = IncredibleTaskProcessor::new(
        address!("0x7bc06c482dead17c0e297afbc32f6e63d3846650"),
        "http://localhost:8545".to_string(),
    );
    let aggregator = Aggregator::new(config, processor).await.unwrap();

    // Start the aggregator in the background
    aggregator.start().await.unwrap();
}
