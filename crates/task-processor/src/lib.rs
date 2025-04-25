//! Task manager

use alloy::primitives::B256;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigen_types::avs::TaskResponseDigest;
use eigen_utils::slashing::middleware::{
    iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
    iblssignaturechecker::BN254::{G1Point, G2Point},
};
use new_task_event_generic::NewTaskEventGeneric;
use std::{collections::HashMap, fmt::Debug, time::Duration};
use task::Task;
use task_manager::TaskManagerContract;
use task_response::TaskResponse;
use tracing::info;

/// New task event generic
pub mod new_task_event_generic;
/// Task
pub mod task;
/// Task manager trait
pub mod task_manager;
/// Task response
pub mod task_response;

/// Indexing task processor
#[derive(Debug, Clone)]
pub struct IndexingTaskProcessor<TM>
where
    TM: TaskManagerContract + Debug,
{
    /// Hashmap to store the created tasks
    tasks: HashMap<u32, Task<TM::Input>>,
    /// Hashmap to store the task responses
    task_responses: HashMap<u32, HashMap<TaskResponseDigest, TaskResponse<TM::Output>>>,
    /// Avs writer
    task_manager: TM,
    /// Task timeout
    task_timeout: Duration,
    /// Window duration
    window_duration: Duration,
}

impl<TM> IndexingTaskProcessor<TM>
where
    TM: TaskManagerContract + Debug,
{
    /// Create a new task processor
    ///
    /// # Arguments
    ///
    /// * `task_manager` - The task manager
    /// * `task_timeout` - The task timeout
    /// * `window_duration` - The window duration
    ///
    /// # Returns
    ///
    /// A new task processor
    pub fn new(task_manager: TM, task_timeout: Duration, window_duration: Duration) -> Self {
        Self {
            tasks: HashMap::default(),
            task_responses: HashMap::default(),
            task_manager,
            task_timeout,
            window_duration,
        }
    }

    /// Recieves a event and creates the [`TaskMetadata`]
    ///
    /// # Arguments
    ///
    /// * `event` - The new task event
    /// * `task_timeout` - The task timeout
    /// * `window_duration` - The window duration
    ///
    /// # Returns
    ///
    /// The [`TaskMetadata`]
    pub async fn process_new_task(
        &mut self,
        event: NewTaskEventGeneric<TM::Input>,
    ) -> TaskMetadata {
        self.tasks.insert(event.task_index, event.task.clone());

        TaskMetadata::new(
            event.task_index,
            u64::from(event.task.task_created_block),
            event.task.quorum_numbers.to_vec(),
            vec![event.task.quorum_threshold_percentage],
            self.task_timeout,
        )
        .with_window_duration(self.window_duration)
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
    pub async fn process_aggregated_response(&self, response: BlsAggregationServiceResponse) {
        info!(
            "Aggregated response received for task {}: {:?}",
            response.task_index, response.task_response_digest
        );

        let mut non_signer_pub_keys = Vec::<G1Point>::new();
        // TODO: Review if x is some
        for pub_key in response.non_signers_pub_keys_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1()).unwrap();
            non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let mut quorum_apks = Vec::<G1Point>::new();
        for pub_key in response.quorum_apks_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1()).unwrap();
            quorum_apks.push(G1Point { X: g1.X, Y: g1.Y })
        }

        let non_signer_stakes_and_signature = NonSignerStakesAndSignature {
            nonSignerPubkeys: non_signer_pub_keys,
            nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
            quorumApks: quorum_apks,
            apkG2: G2Point {
                X: convert_to_g2_point(response.signers_apk_g2.g2()).unwrap().X,
                Y: convert_to_g2_point(response.signers_apk_g2.g2()).unwrap().Y,
            },
            sigma: G1Point {
                X: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .unwrap()
                    .X,
                Y: convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())
                    .unwrap()
                    .Y,
            },
            quorumApkIndices: response.quorum_apk_indices,
            totalStakeIndices: response.total_stake_indices,
            nonSignerStakeIndices: response.non_signer_stake_indices,
        };

        let task = self.tasks.get(&response.task_index).unwrap();

        let task_response = self
            .task_responses
            .get(&response.task_index)
            .and_then(|map| map.get(&response.task_response_digest))
            .cloned()
            .unwrap();

        self.task_manager.respond_to_task(
            task.clone(),
            task_response,
            non_signer_stakes_and_signature,
        );

        info!("Aggregated response sent to contract");
    }
}
