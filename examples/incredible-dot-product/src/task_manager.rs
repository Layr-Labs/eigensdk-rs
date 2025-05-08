use crate::IIncredibleDotProductTaskManager::DotProductInput;
use crate::IncredibleDotProductTaskManager::{
    IncredibleDotProductTaskManagerInstance, NewTaskCreated, TaskResponded,
};
use alloy::primitives::U256;
use eigensdk::task_processor::impl_task_manager;

impl_task_manager!(
    Contract = IncredibleDotProductTaskManagerInstance,
    Input = DotProductInput,
    Output = U256,
    NewTaskEvent = NewTaskCreated,
    TaskRespondedEvent = TaskResponded,
);
