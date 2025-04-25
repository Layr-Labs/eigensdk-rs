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
use std::{collections::HashMap, fmt::Debug};
use task::Task;
use task_manager::TaskManagerContract;
use task_response::TaskResponse;
use tracing::info;
/// Task
pub mod task;
/// Task manager trait
pub mod task_manager;
/// Task response
pub mod task_response;

/// Error returned by the task processor
pub type TaskProcessorError = Box<dyn core::error::Error + Send>;

/// Utility function for boxing errors
pub fn box_error<E: core::error::Error + Send + 'static>(e: E) -> TaskProcessorError {
    Box::new(e)
}

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
    tasks: HashMap<u32, Task<TM::Input>>,
    /// Hashmap to store the task responses
    task_responses: HashMap<u32, HashMap<TaskResponseDigest, TaskResponse<TM::Output>>>,
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
            tasks: HashMap::default(),
            task_responses: HashMap::default(),
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
        event: TM::NewTaskEvent,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        let (task_index, task, task_metadata) = self.task_manager.process_new_task(event).await?;
        self.tasks.insert(task_index, task);
        dbg!("CREANDO TASK");
        Ok(task_metadata)
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
        dbg!("PROCESANDO RESPUESTA");
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

        self.task_manager
            .respond_to_task(task.clone(), task_response, non_signer_stakes_and_signature)
            .await?;

        info!("Aggregated response sent to contract");
        Ok(())
    }
}
