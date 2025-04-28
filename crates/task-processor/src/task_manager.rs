use std::{fmt::Debug, future::Future};

use crate::{task::Task, task_response::TaskResponse, TaskProcessorError};
use alloy::sol_types::{SolEvent, SolValue};
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use serde::de::DeserializeOwned;

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
    fn respond_to_task(
        &self,
        task: Task<Self::Input>,
        response: TaskResponse<Self::Output>,
        non_signer_stakes_and_signature: NonSignerStakesAndSignature,
    ) -> impl Future<Output = Result<(), TaskProcessorError>> + Send;
}
