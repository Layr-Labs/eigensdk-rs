//! Example AVS which squares a number

use alloy::primitives::B256;
use alloy::sol_types::SolEvent;
use eigensdk::task_manager::impl_task_manager_from_defs_and_contract;
use eigensdk::task_manager::TaskManagerDefs;

use crate::bindings::awesome_vault_task_manager::AwesomeVaultTaskManager::AwesomeVaultTaskManagerInstance;
use crate::bindings::awesome_vault_task_manager::AwesomeVaultTaskManager::NewTaskCreated;
use crate::bindings::awesome_vault_task_manager::AwesomeVaultTaskManager::TaskResponded;
use crate::bindings::awesome_vault_task_manager::IAwesomeVaultTaskManager::TaskInput;

/// Task Manager Definition. This struct will be used to build the `TaskManager`
/// with the [`impl_task_manager_from_defs_and_contract`] macro.
pub struct ISTaskManager;

// Implement the [`TaskManagerDefs`] trait for a unit struct.
// You need to specify the input and output types of the task.
// You also need to specify the selectors for the new task event and the task responded event.
impl TaskManagerDefs for ISTaskManager {
    type Input = TaskInput;
    type Output = B256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(ISTaskManager => AwesomeVaultTaskManagerInstance);
