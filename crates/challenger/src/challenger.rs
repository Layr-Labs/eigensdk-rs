use alloy::{rpc::types::Log, sol_types::SolEvent};

pub trait ChallengerTaskProcessor {
    /// Event type expected by the task processor
    type NewTaskEvent: SolEvent + Send + Sync + 'static;

    /// Response type expected by the task processor
    type TaskResponseEvent: SolEvent + Send + Sync + 'static;

    /// Handle a new task creation when a new task event is received
    fn handle_task_creation(&mut self, decoded: Log<Self::NewTaskEvent>);

    /// Handle a task response when a task response event is received
    fn handle_task_response(&mut self, decoded: Log<Self::TaskResponseEvent>);
}
