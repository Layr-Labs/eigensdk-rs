use std::{fmt::Debug, future::Future};

use crate::{task::Task, task_response::TaskResponse, TaskProcessorError};
use alloy::sol_types::{SolEvent, SolValue};
use eigen_services_blsaggregation::bls_agg::TaskMetadata;
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;
use serde::{de::DeserializeOwned, Serialize};

/// Task manager contract trait. It wraps the contract's types and functions.
pub trait TaskManagerContract {
    /// Type for inputs of each task
    type Input: Clone + SolValue + Send + Sync + 'static + Debug;

    /// Type for outputs of each task
    type Output: Clone + SolValue + Send + Sync + 'static + Debug + Serialize + DeserializeOwned;

    /// New task event
    type NewTaskEvent: SolEvent + Send + Sync + 'static;

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

    /// Process a new task
    ///
    /// # Arguments
    ///
    /// * `event` - The new task event
    ///
    /// # Returns
    ///
    /// * `task_index` - The task index
    /// * `task` - The task
    /// * `task_metadata` - The task metadata
    fn process_new_task(
        &mut self,
        event: Self::NewTaskEvent,
    ) -> impl Future<Output = Result<(u32, Task<Self::Input>, TaskMetadata), TaskProcessorError>> + Send;
}
