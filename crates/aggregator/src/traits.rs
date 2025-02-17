use alloy::primitives::B256;
use alloy::sol_types::SolEvent;
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigen_types::avs::TaskIndex;
use serde::{Deserialize, Serialize};
use std::future::Future;

/// Error returned by the task processor
pub type TaskProcessorError = Box<dyn core::error::Error + Send>;

/// Abstracts task-specific behaviour
pub trait TaskProcessor {
    /// Event type expected by the task processor
    type NewTaskEvent: SolEvent + Send + Sync + 'static;

    /// Response type expected by the task processor
    type TaskResponse: TaskResponse + Send + Sync + 'static;

    /// Processes a task, returning metadata related to signature aggregation
    fn process_new_task(
        &self,
        event: Self::NewTaskEvent,
    ) -> impl Future<Output = Result<TaskMetadata, TaskProcessorError>> + Send;

    /// Processes a task response, returning the response's digest
    fn process_task_response(
        &self,
        event: Self::TaskResponse,
    ) -> impl Future<Output = Result<B256, TaskProcessorError>> + Send;

    /// Process the result of a BLS aggregation
    fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> impl Future<Output = Result<(), TaskProcessorError>> + Send;
}

/// Task response trait
pub trait TaskResponse: for<'de> Deserialize<'de> + Serialize {
    /// Returns the index of the task
    fn task_index(&self) -> TaskIndex;

    /// Returns a 32-byte digest of the response
    fn digest(&self) -> B256;
}

/// Utility function for boxing errors
pub fn box_error<E: core::error::Error + Send + 'static>(e: E) -> TaskProcessorError {
    Box::new(e)
}
