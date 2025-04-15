#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewTaskCreated {
    #[allow(missing_docs)]
    pub taskIndex: u32,
    #[allow(missing_docs)]
    pub task: <IIncredibleSquaringTaskManager::Task as alloy::sol_types::SolType>::RustType,
}
