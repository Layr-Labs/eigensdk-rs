//! Example AVS which squares a number

use alloy::primitives::B256;
use alloy::primitives::U256;
use alloy::sol_types::SolEvent;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::NewTaskCreated;
use bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::TaskResponded;
use eigensdk::task_manager::impl_task_manager_from_defs_and_contract;
use eigensdk::task_manager::TaskManagerDefs;
use eigensdk::task_manager::TaskManagerError;

// Allow warnings in auto-generated code
#[allow(warnings)]
pub mod bindings;
pub mod utils;

/// Compute the square of a number
///
/// # Arguments
///
/// * `_task_index` - The index of the task
/// * `number_to_be_squared` - The number to be squared
///
/// # Returns
///
/// * `Result<U256, TaskManagerError>` - The square of the number
pub fn square(_task_index: u32, number_to_be_squared: U256) -> Result<U256, TaskManagerError> {
    Ok(number_to_be_squared * number_to_be_squared)
}

/// Task Manager Definition. This struct will be used to build the `TaskManager`
/// with the [`impl_task_manager_from_defs_and_contract`] macro.
pub struct ISTaskManager;

/// Implement the [`TaskManagerDefs`] trait for a unit struct.
/// You need to specify the input and output types of the task. In this case, U256.
/// You also need to define the selectors for the new task event and the task responded event.
impl TaskManagerDefs for ISTaskManager {
    type Input = U256;
    type Output = U256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(ISTaskManager => IncredibleSquaringTaskManagerInstance);
