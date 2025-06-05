use alloy::{primitives::B256, sol_types::SolValue};
use eigen_services_blsaggregation::bls_agg::TaskMetadata;
use eigen_task_manager::task::Task;
use eigen_task_manager::task_response::TaskResponse;
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use std::future::Future;

/// Aggregator processor error
pub mod error;
/// Standar implementation of the aggregator processor
pub mod indexing;

pub use error::AggregatorProcessorError;
pub use indexing::IndexingAggregatorProcessor;

/// Abstracts task-specific behaviour
pub trait AggregatorProcessor {
    /// Input type expected by the task processor
    type Input: SolValue + Send + Sync + 'static + Clone;

    /// Response type expected by the task processor
    type Output: SolValue + Send + Sync + 'static + Clone;

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
    ) -> impl Future<Output = Result<TaskMetadata, AggregatorProcessorError>> + Send;

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
    ) -> impl Future<Output = Result<B256, AggregatorProcessorError>> + Send;

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
        task_index: u32,
        task_response_digest: B256,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> impl Future<Output = Result<(), AggregatorProcessorError>> + Send;
}
