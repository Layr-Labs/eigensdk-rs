//! Example AVS which squares a number

use std::collections::BTreeMap;
use std::sync::Arc;

use alloy::primitives::FixedBytes;
use alloy::primitives::Keccak256;
use alloy::primitives::B256;
use alloy::sol_types::SolEvent;
use eigensdk::task_processor::impl_task_manager_from_defs_and_contract;
use eigensdk::task_processor::task_manager::TaskManagerDefs;
use eigensdk::task_processor::task_manager::TaskManagerError;
use tokio::sync::Mutex;
use tracing::info;

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

/// Set the key and value in the Redis state
///
/// # Arguments
///
/// * `redis_state` - The Redis state
///
/// # Returns
///
/// * `Result<Bytes, TaskManagerError>` - The hash of the new key and value
pub async fn save_value(
    redis_state: Arc<Mutex<BTreeMap<String, String>>>,
) -> impl AsyncFn(u32, TaskInput) -> Result<FixedBytes<32>, TaskManagerError> {
    move |_task_index, input: TaskInput| {
        let redis_state = redis_state.clone();
        async move {
            info!("Setting key: {} with value: {}", input.key, input.value);
            let mut map = redis_state.lock().await;
            map.insert(input.key.clone(), input.value.clone());
            dbg!(&map);
            hash_entry(_task_index, input)
        }
    }
}

pub async fn save_wrong_value(
    redis_state: Arc<Mutex<BTreeMap<String, String>>>,
) -> impl AsyncFn(u32, TaskInput) -> Result<FixedBytes<32>, TaskManagerError> {
    move |_task_index, input: TaskInput| {
        let redis_state = redis_state.clone();
        async move {
            let wrong_input = TaskInput {
                key: format!("WRONG_{}", input.key),
                value: format!("WRONG_{}", input.value),
            };
            info!(
                "Setting key: {} with value: {}",
                wrong_input.key, wrong_input.value
            );
            let mut map = redis_state.lock().await;
            map.insert(wrong_input.key.clone(), wrong_input.value.clone());
            dbg!(&map);
            hash_entry(_task_index, wrong_input)
        }
    }
}

/// Hash the new key and value
///
/// # Arguments
///
/// * `task_index` - The index of the task
/// * `input` - The input of the task
///
/// # Returns
///
/// The hash of the new key and value
pub fn hash_entry(_task_index: u32, input: TaskInput) -> Result<FixedBytes<32>, TaskManagerError> {
    let mut keccak = Keccak256::new();
    keccak.update(input.key.as_bytes());
    keccak.update(input.value.as_bytes());
    Ok(keccak.finalize())
}
