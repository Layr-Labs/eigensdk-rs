use std::future::Future;

use alloy::sol_types::{SolEvent, SolValue};
use eigen_task_processor::{task::Task, task_response::TaskResponse};
use eigen_utils::slashing::middleware::iblssignaturechecker::BN254::G1Point;

use crate::{challenger_processor::TaskResponseMetadataSol, error::ChallengerError};

pub trait ChallengerTaskProcessor {
    /// Event type expected by the task processor
    type NewTaskEvent: SolEvent + Send + Sync + 'static;

    /// Response type expected by the task processor
    type TaskResponseEvent: SolEvent + Send + Sync + 'static;

    /// Input type of the task
    type Input: Clone + SolValue + Send + Sync + 'static;

    /// Output type of the task
    type Output: Clone + SolValue + Send + Sync + 'static;

    /// Handle the creation of a new task when a new task event is received
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task
    /// * `task` - The task
    ///
    /// # Returns
    ///
    /// * `Result<(), ChallengerError>` - The result of the operation
    fn handle_task_creation(
        &mut self,
        task_index: u32,
        task: Task<Self::Input>,
    ) -> impl Future<Output = Result<(), ChallengerError>> + Send;

    /// Handle the response of a task when a task response event is received
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task
    /// * `task_response` - The task response
    /// * `task_response_metadata` - The metadata of the task response
    /// * `non_signing_operator_pub_keys` - The public keys of the non-signing operators
    ///
    /// # Returns
    ///
    /// * `Result<(), ChallengerError>` - The result of the operation
    fn handle_task_response(
        &self,
        task_index: u32,
        task_response: TaskResponse<Self::Output>,
        task_response_metadata: TaskResponseMetadataSol,
        non_signing_operator_pub_keys: Vec<G1Point>,
    ) -> impl Future<Output = Result<(), ChallengerError>> + Send;
}
