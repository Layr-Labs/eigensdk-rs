//! Task manager

use alloy::primitives::B256;
use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
};
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigen_types::avs::TaskResponseDigest;
use eigen_utils::slashing::middleware::{
    iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
    iblssignaturechecker::BN254::{G1Point, G2Point},
};
use error::TaskProcessorError;
use std::sync::Arc;
use std::time::Duration;
use std::{collections::HashMap, fmt::Debug};
use task::Task;
use task_manager::TaskManagerContract;
use task_response::TaskResponse;
use tokio::sync::Mutex;
use tracing::info;

/// Task processor error
pub mod error;
/// Task
pub mod task;
/// Task manager trait
pub mod task_manager;
/// Task response
pub mod task_response;

/// Indexing task processor
#[derive(Debug, Clone)]
pub struct IndexingTaskProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Debug + Send + Sync + 'static + Clone,
    T: Transport + Clone + Send + Sync + 'static,
    P: Provider<T, N> + Clone + Send + Sync + 'static,
    N: Network,
{
    /// Hashmap to store the created tasks
    tasks: Arc<Mutex<HashMap<u32, Task<TM::Input>>>>,
    /// Hashmap to store the task responses
    task_responses: Arc<Mutex<HashMap<u32, HashMap<TaskResponseDigest, TaskResponse<TM::Output>>>>>,
    /// Avs writer
    task_manager: TM,
}

impl<TM, T, P, N> IndexingTaskProcessor<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Debug + Send + Sync + 'static + Clone,
    T: Transport + Clone + Send + Sync + 'static,
    P: Provider<T, N> + Clone + Send + Sync + 'static,
    N: Network,
{
    /// Create a new task processor
    ///
    /// # Arguments
    ///
    /// * `task_manager` - The task manager
    ///
    /// # Returns
    ///
    /// A new task processor
    pub fn new(task_manager: TM) -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::default())),
            task_responses: Arc::new(Mutex::new(HashMap::default())),
            task_manager,
        }
    }

    /// Recieves a event and creates the [`TaskMetadata`]
    ///
    /// # Arguments
    ///
    /// * `event` - The new task event
    ///
    /// # Returns
    ///
    /// The [`TaskMetadata`]
    pub async fn handle_new_task(
        &mut self,
        task_index: u32,
        task: Task<TM::Input>,
    ) -> TaskMetadata {
        self.tasks.lock().await.insert(task_index, task.clone());

        let quorum_numbers: Vec<u8> = task.quorum_numbers.into();
        let quorum_threshold_percentages =
            std::iter::repeat_n(task.quorum_threshold_percentage as u8, quorum_numbers.len())
                .collect();
        TaskMetadata::new(
            task_index,
            task.task_created_block.into(),
            quorum_numbers,
            quorum_threshold_percentages,
            std::time::Duration::from_secs(60), // TODO: Make this configurable
        )
        .with_window_duration(Duration::from_secs(15)) // TODO: Make this configurable
    }

    /// Processes a task response
    ///
    /// # Arguments
    ///
    /// * `response` - The task response
    ///
    /// # Returns
    ///
    /// The task response digest
    pub async fn process_task_response(&mut self, response: TaskResponse<TM::Output>) -> B256 {
        let digest = alloy::primitives::keccak256(response.encode());

        self.task_responses
            .lock()
            .await
            .entry(response.task_index)
            .or_default()
            .entry(digest)
            .or_insert(response);

        digest
    }

    /// Processes an aggregated response and sends it to the contract
    ///
    /// # Arguments
    ///
    /// * `response` - The BLS Aggregated Response
    ///
    /// # Returns
    ///
    /// The aggregated response digest
    pub async fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        info!(
            "Aggregated response received for task {}: {:?}",
            response.task_index, response.task_response_digest
        );

        let mut non_signer_pub_keys = Vec::<G1Point>::new();
        // TODO: Review if x is some
        for pub_key in response.non_signers_pub_keys_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1())?;
            non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let mut quorum_apks = Vec::<G1Point>::new();
        for pub_key in response.quorum_apks_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1())?;
            quorum_apks.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
            nonSignerPubkeys: non_signer_pub_keys,
            nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
            quorumApks: quorum_apks,
            apkG2: G2Point {
                X: convert_to_g2_point(response.signers_apk_g2.g2())?.X,
                Y: convert_to_g2_point(response.signers_apk_g2.g2())?.Y,
            },
            sigma: G1Point {
                X: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())?.X,
                Y: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())?.Y,
            },
            quorumApkIndices: response.quorum_apk_indices,
            totalStakeIndices: response.total_stake_indices,
            nonSignerStakeIndices: response.non_signer_stake_indices,
        };

        let (task, task_response) = {
            let tasks_lock = self.tasks.lock().await;
            let task = tasks_lock
                .get(&response.task_index)
                .ok_or(TaskProcessorError::TaskNotFound)?
                .clone();

            let responses_lock = self.task_responses.lock().await;
            let task_response = responses_lock
                .get(&response.task_index)
                .and_then(|map| map.get(&response.task_response_digest))
                .ok_or(TaskProcessorError::TaskResponseNotFound)?
                .clone();

            (task, task_response)
        };

        self.task_manager
            .respond_to_task(task.clone(), task_response, non_signer_stakes_and_signature)
            .await
            .map_err(TaskProcessorError::TaskManagerError)?;

        info!("Aggregated response sent to contract");
        Ok(())
    }
}
