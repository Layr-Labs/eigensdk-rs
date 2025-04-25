use crate::{task::Task, task_response::TaskResponse};
use alloy::sol_types::SolValue;
use eigen_utils::slashing::middleware::iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature;

/// Task manager contract trait. It wraps the contract's types and functions.
pub trait TaskManagerContract {
    /// Type for inputs of each task
    type Input: Clone + SolValue;

    /// Type for outputs of each task
    type Output: Clone + SolValue;

    /// New task event
    type EventSignature: AsRef<str>;

    // DAMIAN:
    // We have a problem with these methods: they are very tightly coupled to
    // the bindings. For example, `create_new_task` from the binding expects
    // an `IncredibleSquaringTaskManager::Task`, and the same applies to
    // `TaskResponse`. If we want to use this interface, the user should
    // re-create the `IncredibleSquaringTaskManager::Task` using the values
    // of our Task struct. I think it feels weird.

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
    );
}
