use crate::new_task_event_generic::{
    NewTaskEventGeneric, BLOCK_TIME_SECONDS, TASK_CHALLENGE_WINDOW_BLOCK,
};
use crate::task::Task;
use crate::task_manager_contract::TaskManagerContract;
use crate::task_response::TaskResponse;
use alloy::dyn_abi::SolType;
use alloy::primitives::keccak256;
use alloy::primitives::B256;
use ark_ec::AffineRepr;
use eigen_aggregator::{TaskMetadata, TaskProcessorError};
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigen_services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigen_types::avs::{TaskIndex, TaskResponseDigest};
use eigen_utils::slashing::middleware::{
    iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
    iblssignaturechecker::BN254::{G1Point, G2Point},
};
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::time::Duration;
use tracing::info;

#[derive(Debug)]
#[allow(missing_docs)]
pub struct IndexingTaskProcessor<T, R, N, Input, Output, TM>
where
    T: SolType,
    R: SolType,
    N: SolType,
    Input: Clone,
    Output: Clone,
    TM: TaskManagerContract<T, R, N>,
{
    pub tasks: HashMap<TaskIndex, Task<Input>>,

    pub task_responses: HashMap<TaskIndex, HashMap<TaskResponseDigest, TaskResponse<Output>>>,

    pub task_manager: TM,
    _phantom: PhantomData<(T, R, N)>,
}

impl<T, R, N, Input, Output, TM> IndexingTaskProcessor<T, R, N, Input, Output, TM>
where
    T: SolType,
    R: SolType,
    N: SolType,
    Input: Clone,
    Output: Clone,
    TM: TaskManagerContract<T, R, N>,
{
    pub fn new(task_manager: TM) -> Self {
        Self {
            tasks: HashMap::new(),
            task_responses: HashMap::new(),
            task_manager,
            _phantom: PhantomData,
        }
    }

    async fn process_new_task(
        &mut self,
        event: NewTaskEventGeneric<Input>,
    ) -> Result<TaskMetadata, TaskProcessorError> {
        self.tasks.insert(event.task_index, event.task.clone());

        let time_to_expiry = tokio::time::Duration::from_secs(
            (TASK_CHALLENGE_WINDOW_BLOCK * BLOCK_TIME_SECONDS).into(),
        );
        let task_created_block = event.get_task_created_block();
        let quorum_numbers = event.get_quorum_numbers();
        let quorum_threshold_percentages = vec![event.task.quorum_threshold_percentage];

        // REVIEW: window_duration?
        Ok(TaskMetadata::new(
            event.task_index,
            task_created_block,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        )
        .with_window_duration(Duration::from_secs(5)))
    }

    /*
    async fn process_task_response(
        &mut self,
        response: TaskResponse<Output>,
    ) -> Result<TaskResponseDigest, TaskProcessorError> {
        // alloy::primitives::keccak256(TaskResponse::<OUTPUT>::abi_encode(&self))

        self.task_responses
            .entry(response.referenceTaskIndex)
            .or_default()
            .entry(response.digest())
            .or_insert(response.clone());

        Ok(response.digest())
    }
    */

    async fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        info!(
            "Aggregated response received for task {}: {:?}",
            response.task_index, response.task_response_digest
        );

        info!("Aggregated response sent to contract");
        Ok(())
    }

    async fn send_aggregated_response_to_contract(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> Result<(), TaskProcessorError> {
        let mut non_signer_pub_keys = Vec::<G1Point>::new();
        for pub_key in response.non_signers_pub_keys_g1.iter() {
            if pub_key.g1().x().is_some() {
                let g1 = convert_to_g1_point(pub_key.g1()).unwrap();
                //.map_err(box_error)?;
                non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
            } else {
                info!(
                    "Zero non_signers for the task index :{:?}",
                    response.task_index
                );
            }
        }

        let mut quorum_apks = Vec::<G1Point>::new();
        for pub_key in response.quorum_apks_g1.iter() {
            let g1 = convert_to_g1_point(pub_key.g1()).unwrap();
            //.map_err(box_error)?;
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

        let task = &self.tasks[&response.task_index];

        let task_sol: <() as SolType>::RustType = task.into();

        let task_response = self
            .task_responses
            .get(&response.task_index)
            .and_then(|map| map.get(&response.task_response_digest))
            .cloned()
            .unwrap();

        self.task_manager
            .respond_to_task(task, task_response, non_signer_stakes_and_signature);

        Ok(())
    }
}
