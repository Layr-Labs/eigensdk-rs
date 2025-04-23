//! Task manager

pub mod task;
pub mod task_response;

pub struct IndexingTaskProcessor<TM: TaskManagerContract> {
    /// Hashmap to store the created tasks
    tasks: HashMap<u32, Task<TM::Input>>,
    /// Hashmap to store the task responses
    task_responses: HashMap<TaskIndex, HashMap<TaskResponseDigest, TaskResponse<TM::Output>>>,
    /// Avs writer
    task_manager: TM,
}
