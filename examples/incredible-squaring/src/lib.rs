//! Example AVS which squares a number

use alloy::primitives::B256;
use alloy::primitives::U256;
use alloy::sol_types::SolEvent;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::TaskResponded;
use eigen_task_processor::impl_task_manager;
use eigen_task_processor::task_manager::TaskManagerDefs;

// Allow warnings in auto-generated code
#[allow(warnings)]
pub mod bindings;

// Implement the TaskManager trait for the task manager contract.
// You need to specify the input type of the task. In this case, U256.
// You also need to specify the call type of the task manager contract. `createNewTask` uses `createNewTaskCall`.
// You also need to specify the provider and network types.
//
// NOTE: When you are implementing this, you will have an external trait `TaskManager` and an external struct
// `CONTRACT_NAME_INSTANCE`, so it will throw an error. You can wrap the external struct in a newtype to avoid this.
// Example:
// struct TaskManagerWrapper<T, P, N>(IncredibleSquaringTaskManagerInstance<T, P, N>);
//
// impl<T, P, N> TaskManager<U256, T, P, N> for TaskManagerWrapper<T, P, N> { ... }
impl_task_manager!(
    Contract = IncredibleSquaringTaskManagerInstance,
    Input = U256,
    Output = U256,
    NewTaskEvent = NewTaskCreated,
    TaskRespondedEvent = TaskResponded,
);

pub struct ISTaskManager;

impl TaskManagerDefs for ISTaskManager {
    type Input = U256;
    type Output = U256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}
