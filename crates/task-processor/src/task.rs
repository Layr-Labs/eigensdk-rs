use alloy::sol_types::{private::Bytes, SolValue};
/// Task abstraction
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Task<Input>
where
    Input: Clone + SolValue,
{
    #[allow(missing_docs)]
    pub input: Input,
    #[allow(missing_docs)]
    pub task_created_block: u32,
    #[allow(missing_docs)]
    pub quorum_numbers: Bytes,
    #[allow(missing_docs)]
    pub quorum_threshold_percentage: u8,
}
