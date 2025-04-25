use alloy::sol_types::{SolEvent, SolValue};

use crate::task::Task;

/// Task Challenge Window Block : 100 blocks
pub const TASK_CHALLENGE_WINDOW_BLOCK: u32 = 100;
/// Block Time Seconds : 12 seconds
pub const BLOCK_TIME_SECONDS: u32 = 12;

/// Simulate the new task event with generic input
#[derive(Debug, Clone)]
pub struct NewTaskEventGeneric<Input>
where
    Input: Clone + SolValue,
{
    /// Task index
    pub task_index: u32,
    /// Task
    pub task: Task<Input>,
}
