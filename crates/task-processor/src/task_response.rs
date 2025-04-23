use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse<Output> {
    #[allow(missing_docs)]
    pub referenceTaskIndex: u32,
    #[allow(missing_docs)]
    pub respose: Output,
}
