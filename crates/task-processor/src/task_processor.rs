use alloy::{primitives::B256, sol_types::SolValue};
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use serde::de::DeserializeOwned;
use std::future::Future;

use crate::error::TaskProcessorError;
use crate::task::Task;
use crate::task_response::TaskResponse;

/// Abstracts task-specific behaviour
pub trait TaskProcessor {
    /// Input type expected by the task processor
    type Input: SolValue + Send + Sync + 'static + Clone;

    /// Response type expected by the task processor
    type Output: SolValue + Send + Sync + 'static + Clone + DeserializeOwned;

    /// Selector for the event signaling a new task
    const NEW_TASK_EVENT_SELECTOR: B256;

    /// Creates the [`TaskMetadata`]
    ///
    /// # Arguments
    ///
    /// * `task_index` - The task index
    /// * `task` - The task
    ///
    /// # Returns
    ///
    /// The [`TaskMetadata`]
    fn process_new_task(
        &mut self,
        task_index: u32,
        task: Task<Self::Input>,
    ) -> impl Future<Output = Result<TaskMetadata, TaskProcessorError>> + Send;

    /// Processes a task response
    ///
    /// # Arguments
    ///
    /// * `response` - The task response
    ///
    /// # Returns
    ///
    /// The task response digest
    fn process_task_response(
        &mut self,
        task_response: TaskResponse<Self::Output>,
    ) -> impl Future<Output = Result<B256, TaskProcessorError>> + Send;

    /// Processes an aggregated response and sends it to the contract
    ///
    /// # Arguments
    ///
    /// * `response` - The BLS Aggregated Response
    ///
    /// # Returns
    ///
    /// The aggregated response digest
    fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> impl Future<Output = Result<(), TaskProcessorError>> + Send;
}
