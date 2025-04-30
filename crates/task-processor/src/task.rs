use alloy::sol_types::private::Bytes;

/// Task abstraction
#[derive(Debug, Clone)]
pub struct Task<Input> {
    /// Generic input
    pub input: Input,
    /// Task created block
    pub task_created_block: u32,
    /// Quorum numbers
    pub quorum_numbers: Bytes,
    /// Quorum threshold percentage
    pub quorum_threshold_percentage: u32,
}
