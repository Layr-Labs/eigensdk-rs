//! Task manager

use alloy::{dyn_abi::SolType, primitives::B256};
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigen_types::avs::TaskResponseDigest;
use new_task_event_generic::NewTaskEventGeneric;
use serde::Serialize;
use std::{collections::HashMap, time::Duration};
use task::Task;
use task_manager::TaskManagerContract;
use task_response::TaskResponse;

pub mod new_task_event_generic;
pub mod task;
pub mod task_manager;
pub mod task_response;

pub struct IndexingTaskProcessor<TM>
where
    TM: TaskManagerContract,
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
    TM: TaskManagerContract,
{
    pub fn new(task_manager: TM, task_timeout: Duration, window_duration: Duration) -> Self {
        Self {
            tasks: HashMap::default(),
            task_responses: HashMap::default(),
            task_manager,
            task_timeout,
            window_duration,
        }
    }

    // TODO: Review the return type
    pub fn process_new_task(
        &mut self,
        event: NewTaskEventGeneric<TM::Input>,
        task_timeout: Duration,    // TODO: Check if this is correct
        window_duration: Duration, // TODO: Check if this is correct
    ) {
        self.tasks.insert(event.task_index, event.task.clone());

        TaskMetadata::new(
            event.task_index,
            u64::from(event.task.task_created_block),
            event.task.quorum_numbers.to_vec(),
            vec![event.task.quorum_threshold_percentage],
            task_timeout,
        )
        .with_window_duration(window_duration);
    }

    async fn process_task_response(&mut self, response: TaskResponse<TM::Output>) -> B256 {
        let digest = alloy::primitives::keccak256(response.abi_encode());

        self.task_responses
            .entry(response.referenceTaskIndex)
            .or_default()
            .entry(digest)
            .or_insert(response);

        digest
    }
}
