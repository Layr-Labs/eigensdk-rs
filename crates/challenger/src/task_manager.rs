use alloy::sol_types::{SolEvent, SolValue};
use eigen_task_processor::{task::Task, task_response::TaskResponse};
use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point; // Check which type to use
use serde::de::DeserializeOwned;
use std::{fmt::Debug, future::Future};

/// Error returned by the task processor
pub type TaskManagerError = Box<dyn core::error::Error + Send>;

/// Utility function for boxing errors
pub fn box_error<E: core::error::Error + Send + 'static>(e: E) -> TaskManagerError {
    Box::new(e)
}

/// Task manager contract trait. It wraps the contract's types and functions.
pub trait TaskManagerContract<T, P, N: alloy::network::Network> {
    /// Type for inputs of each task
    type Input: Clone + SolValue + Send + Sync + 'static + Debug;

    /// Type for outputs of each task
    type Output: Clone + SolValue + Send + Sync + 'static + Debug + DeserializeOwned;

    /// Type for task response metadata
    type TaskResponseMetadata: Clone + SolValue + Send + Sync + 'static + Debug;

    /// New task event
    type NewTaskEvent: SolEvent + Send + Sync + 'static;

    /// Task responded event
    type TaskRespondedEvent: SolEvent + Send + Sync + 'static;

    /// Raise challenge
    ///
    /// # Arguments
    ///
    /// * `task` - The task
    /// * `task_response` - The task response
    /// * `task_response_metadata` - The task response metadata
    /// * `pubkeys_of_non_signing_operators` - The pubkeys of non-signing operators
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskManagerError>` - The result of the operation
    fn raise_challenge(
        &self,
        task: Task<Self::Input>,
        task_response: TaskResponse<Self::Output>,
        task_response_metadata: Self::TaskResponseMetadata,
        pubkeys_of_non_signing_operators: Vec<G1Point>,
    ) -> impl Future<Output = Result<(), TaskManagerError>> + Send;
}
