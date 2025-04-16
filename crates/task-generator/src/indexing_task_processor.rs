use crate::new_task_event_generic::{
    NewTaskEventGeneric, BLOCK_TIME_SECONDS, TASK_CHALLENGE_WINDOW_BLOCK,
};
use crate::task::Task;
use crate::task_manager_contract::TaskManagerContract;
use crate::task_response::TaskResponse;
use alloy::dyn_abi::SolType;
use alloy::primitives::keccak256;
use eigen_aggregator::{TaskMetadata, TaskProcessorError};
use eigen_services_blsaggregation::bls_aggregation_service_response::BlsAggregationServiceResponse;
use eigen_types::avs::{TaskIndex, TaskResponseDigest};
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
}
