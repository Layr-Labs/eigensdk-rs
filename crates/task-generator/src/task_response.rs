use alloy_sol_types::SolType;

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct TaskResponse<OUTPUT: SolType> {
    #[allow(missing_docs)]
    pub referenceTaskIndex: u32,
    #[allow(missing_docs)]
    pub respose: OUTPUT,
}
