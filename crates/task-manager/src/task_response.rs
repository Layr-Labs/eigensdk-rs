use alloy::sol_types::SolValue;

/// Task response struct with generic response
#[derive(Debug, Clone)]
pub struct TaskResponse<Output>
where
    Output: Clone + SolValue,
{
    /// Task index
    pub task_index: u32,
    /// Response
    pub response: Output,
}

impl<Output: Clone + SolValue> TaskResponse<Output> {
    /// Abi encode the task response
    ///
    /// # Returns
    ///
    /// The abi encoded task response
    pub fn encode(&self) -> Vec<u8> {
        (self.task_index, self.response.clone()).abi_encode()
    }
}
