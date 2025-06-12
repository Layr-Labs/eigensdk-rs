use eigen_task_manager::TaskManagerError;
use thiserror::Error;

/// [`AggregatorProcessor`](crate::AggregatorProcessor) error
#[derive(Debug, Error)]
pub enum AggregatorProcessorError {
    /// Task not found
    #[error("Task not found")]
    TaskNotFound,

    /// Task response not found
    #[error("Task response not found")]
    TaskResponseNotFound,

    /// Task manager error
    #[error("Task manager error")]
    TaskManagerError(#[from] TaskManagerError),
}
