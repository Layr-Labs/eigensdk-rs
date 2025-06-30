use alloy::{
    primitives::{B256, U256},
    sol_types::SolEvent,
};
use eigensdk::task_manager::{
    impl_task_manager_from_defs_and_contract, TaskManagerDefs, TaskManagerError,
};

use crate::{
    IIncredibleDotProductTaskManager::DotProductInput,
    IncredibleDotProductTaskManager::{
        IncredibleDotProductTaskManagerInstance, NewTaskCreated, TaskResponded,
    },
};

// Implement the [`TaskManagerDefs`] trait for a unit struct.
// You need to specify the input and output types of the task. In this case, U256.
// You also need to specify the selectors for the new task event and the task responded event.
pub struct ISTaskManager;

impl TaskManagerDefs for ISTaskManager {
    type Input = DotProductInput;
    type Output = U256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(ISTaskManager => IncredibleDotProductTaskManagerInstance);

/// Computes the dot product of a pair of points
///
/// # Arguments
///
/// * `task_index` - The index of the task
/// * `input` - The input of the task
///
/// # Returns
///
/// * `Result<TaskResponse<U256>, OperatorError>` - The task response
pub fn dot_product(_task_index: u32, input: DotProductInput) -> Result<U256, TaskManagerError> {
    Ok(input
        .X
        .iter()
        .zip(input.Y.iter())
        .fold(U256::ZERO, |acc, (a, b)| acc + (*a) * (*b)))
}
