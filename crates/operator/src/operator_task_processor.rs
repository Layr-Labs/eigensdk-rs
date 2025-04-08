use alloy::primitives::B256;
use alloy::sol_types::SolEvent;
use eigen_aggregator::TaskResponse;

/// Operator methods
pub trait OperatorTaskProcessor {
    /// Task response
    type TaskResponse: TaskResponse + Send;

    /// New task event
    type NewTaskEvent: SolEvent + Send;

    /// Build the task response from the event
    ///
    /// # Arguments
    ///
    /// * `new_task_created` - The event of the new task
    ///
    /// # Returns
    ///
    /// * `Self::TaskResponse` - The task response
    fn process_new_task(&self, new_task_created: Self::NewTaskEvent) -> Self::TaskResponse;
}
