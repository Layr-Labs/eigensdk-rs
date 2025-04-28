use alloy::sol_types::SolValue;
use serde::{Deserialize, Serialize};

/// Task response struct with generic response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse<Output>
where
    Output: Clone + SolValue,
{
    /// Task index
    pub task_index: u32,
    /// Response
    pub response: Output,
}

impl<Output> TaskResponse<Output>
where
    Output: Clone + SolValue,
{
    /// Abi encode the task response
    ///
    /// # Returns
    ///
    /// The abi encoded task response
    pub fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(self.task_index.abi_encode());
        bytes.extend(self.response.abi_encode());
        bytes
    }
}
