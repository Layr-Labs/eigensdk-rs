use std::{fmt::Debug, future::Future};

use crate::{task::Task, task_response::TaskResponse};
use alloy::sol_types::{SolEvent, SolValue};
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use serde::de::DeserializeOwned;

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

    /// New task event
    type NewTaskEvent: SolEvent;

    /// Respond to a task
    ///
    /// # Arguments
    ///
    /// * `task` - The task
    /// * `response` - The response
    /// * `non_signer_stakes_and_signature` - The non-signer stakes and signature
    ///
    /// # Returns
    ///
    /// * `Result<(), TaskManagerError>` - The result of the operation
    fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> impl Future<Output = Result<(), TaskManagerError>> + Send;
}
