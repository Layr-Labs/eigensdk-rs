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

    /// Handle a new task creation when a new task event is received
    fn handle_task_creation(
        &mut self,
        log: Log,
    ) -> impl Future<Output = Result<(), ChallengerError>> + Send;

    /// Handle a task response when a task response event is received
    fn handle_task_response(
        &mut self,
        log: Log,
        check_response: impl Fn(Task<Self::Input>, TaskResponse<Self::Output>) -> Result<bool, ChallengerError>
            + Send,
    ) -> impl Future<Output = Result<(), ChallengerError>> + Send;
}
