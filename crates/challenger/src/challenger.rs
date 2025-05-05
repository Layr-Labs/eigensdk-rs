use std::future::Future;

use alloy::{
    rpc::types::Log,
    sol_types::{SolEvent, SolValue},
};
use eigen_task_processor::{task::Task, task_response::TaskResponse};

use crate::error::ChallengerError;

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
    /// * `log` - The log of the new task creation
    ///
    /// # Returns
    ///
    /// * `Result<(), ChallengerError>` - The result of the operation
    fn handle_task_creation(
        &mut self,
        log: Log,
    ) -> impl Future<Output = Result<(), ChallengerError>> + Send;

    /// Handle the response of a task when a task response event is received
    ///
    /// # Arguments
    ///
    /// * `log` - The log of the task response
    /// * `is_response_correct` - The logic to check if the response is correct
    ///
    /// # Returns
    ///
    /// * `Result<(), ChallengerError>` - The result of the operation
    fn handle_task_response(
        &mut self,
        log: Log,
        is_response_correct: impl Fn(Task<Self::Input>, TaskResponse<Self::Output>) -> Result<bool, ChallengerError>
            + Send,
    ) -> impl Future<Output = Result<(), ChallengerError>> + Send;
}
