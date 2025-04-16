#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct Task<INPUT>
where
    INPUT: Clone,
{
    #[allow(missing_docs)]
    pub input: INPUT,
    #[allow(missing_docs)]
    pub task_created_block: u32,
    #[allow(missing_docs)]
    pub quorum_numbers: alloy::sol_types::private::Bytes,
    #[allow(missing_docs)]
    pub quorum_threshold_percentage: u8,
}
