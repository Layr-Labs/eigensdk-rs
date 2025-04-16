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

/// Task Challenge Window Block : 100 blocks
const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
const BLOCK_TIME_SECONDS: u32 = 12;

#[derive(Debug, Clone)]
pub struct NewTaskEventGeneric<Input>
where
    Input: Clone,
{
    pub task_index: u32,
    pub task: Task<Input>,
}

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

        // REVIEW: window_duration?
        Ok(TaskMetadata::new(
            event.task_index,
            u64::from(event.task.task_created_block),
            event.task.quorum_numbers.to_vec(),
            vec![event.task.quorum_threshold_percentage],
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
