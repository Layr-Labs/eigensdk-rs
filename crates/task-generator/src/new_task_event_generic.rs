use crate::task::Task;

/// Task Challenge Window Block : 100 blocks
pub const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
pub const BLOCK_TIME_SECONDS: u32 = 12;

#[derive(Debug, Clone)]
pub struct NewTaskEventGeneric<Input>
where
    Input: Clone,
{
    pub task_index: u32,
    pub task: Task<Input>,
}

impl<Input: Clone> NewTaskEventGeneric<Input> {
    pub fn get_task_created_block(&self) -> u64 {
        u64::from(self.task.task_created_block)
    }

    pub fn get_quorum_numbers(&self) -> Vec<u8> {
        self.task.quorum_numbers.to_vec()
    }

    pub fn get_quorum_threshold_percentage(&self) -> Vec<u8> {
        vec![self.task.quorum_threshold_percentage]
    }
}
