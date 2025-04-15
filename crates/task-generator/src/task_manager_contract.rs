use crate::task::Task;
use crate::task_response::TaskResponse;

pub trait TaskManagerContract {
    /// Type for task indices
    //type Index;

    /// Type for inputs of each task
    type Input;

    /// Type for outputs of each task
    type Output;

    fn create_new_task(&self, task: Task<Self::Input>) -> Task<Self::Input>;

    fn respond_to_task(&self, task: Task<Self::Input>, response: TaskResponse<Self::Output>);

    // fn submit_challenge(&self) ->
}
