//! Example AVS which squares a number

use alloy::primitives::FixedBytes;
use alloy::primitives::B256;
use alloy::sol_types::SolEvent;
use eigensdk::task_manager::impl_task_manager_from_defs_and_contract;
use eigensdk::task_manager::TaskManagerDefs;

use crate::bindings::awesomevaulttaskmanager::AwesomeVaultTaskManager::AwesomeVaultTaskManagerInstance;
use crate::bindings::awesomevaulttaskmanager::AwesomeVaultTaskManager::NewTaskCreated;
use crate::bindings::awesomevaulttaskmanager::AwesomeVaultTaskManager::TaskResponded;
use crate::bindings::awesomevaulttaskmanager::IAwesomeVaultTaskManager::TaskInput;

// Implement the [`TaskManagerDefs`] trait for a unit struct.
// You need to specify the input and output types of the task.
// You also need to specify the selectors for the new task event and the task responded event.
pub struct ISTaskManager;

impl TaskManagerDefs for ISTaskManager {
    // TODO SDK: Should we remove the `Debug` bound in TM::Input?
    type Input = TaskInput;
    type Output = FixedBytes<32>;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(ISTaskManager => AwesomeVaultTaskManagerInstance);
