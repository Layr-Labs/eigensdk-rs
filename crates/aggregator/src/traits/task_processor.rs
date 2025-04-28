use super::task_response::TaskResponse;
use alloy::dyn_abi::SolType;
use alloy::sol_types::SolEvent;
use alloy::{primitives::B256, sol_types::SolValue};
use alloy_rlp::Decodable;
use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use serde::de::DeserializeOwned;
use serde::Deserializer;
use std::fmt::Debug;
use std::future::Future;

/// Error returned by the task processor
pub type TaskProcessorError = Box<dyn core::error::Error + Send>;

/// Utility function for boxing errors
pub fn box_error<E: core::error::Error + Send + 'static>(e: E) -> TaskProcessorError {
    Box::new(e)
}

/// Abstracts task-specific behaviour
pub trait TaskProcessor {
    type NewTaskEvent: SolEvent + Send + Sync + 'static;

    /// Response type expected by the task processor
    type TaskResponse: TaskResponse + Send + Sync + 'static;

    /// Input type expected by the task processor
    type Input: SolValue + Debug;

    /// Processes a task, returning metadata related to signature aggregation
    fn process_new_task(
        &mut self,
        event: Self::NewTaskEvent,
    ) -> impl Future<Output = Result<TaskMetadata, TaskProcessorError>> + Send;

    /// Processes a task response, returning the response's digest
    fn process_task_response(
        &mut self,
        event: Self::TaskResponse,
    ) -> impl Future<Output = Result<B256, TaskProcessorError>> + Send;

    /// Process the result of a BLS aggregation
    fn process_aggregated_response(
        &self,
        response: BlsAggregationServiceResponse,
    ) -> impl Future<Output = Result<(), TaskProcessorError>> + Send;
}
