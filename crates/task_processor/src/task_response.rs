#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct TaskResponse<OUTPUT> {
    #[allow(missing_docs)]
    pub referenceTaskIndex: u32,
    #[allow(missing_docs)]
    pub respose: OUTPUT,
}
