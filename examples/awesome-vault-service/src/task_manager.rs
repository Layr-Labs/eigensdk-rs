//! Example AVS which squares a number

use std::collections::BTreeMap;
use std::sync::Arc;

use alloy::primitives::FixedBytes;
use alloy::primitives::Keccak256;
use alloy::primitives::B256;
use alloy::sol_types::SolEvent;
use eigensdk::task_manager::impl_task_manager_from_defs_and_contract;
use eigensdk::task_manager::TaskManagerDefs;
use eigensdk::task_manager::TaskManagerError;
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
    async move |_task_index, input: TaskInput| {
        let redis_state = redis_state.clone();
        info!("Setting key: {} with value: {}", input.key, input.value);
        let mut map = redis_state.lock().await;
        map.insert(input.key.clone(), input.value.clone());
        compute_vault_root(&map)
    }
}

/// Simulate a wrong computation
///
/// # Arguments
///
/// * `redis_state` - The Redis state
///
/// # Returns
///
/// * `impl AsyncFn(u32, TaskInput) -> Result<FixedBytes<32>, TaskManagerError>` - Default FixedBytes<32> value
pub async fn save_wrong_value(
    _redis_state: Arc<Mutex<BTreeMap<String, String>>>,
) -> impl AsyncFn(u32, TaskInput) -> Result<FixedBytes<32>, TaskManagerError> {
    move |_task_index, _input: TaskInput| async move {
        info!("Wrong computation");
        Ok(FixedBytes::<32>::default())
    }
}

/// Compute the vault root
///
/// # Arguments
///
/// * `map` - The Redis state
///
/// # Returns
///
/// * `Result<FixedBytes<32>, TaskManagerError>` - The vault root
pub fn compute_vault_root(
    map: &BTreeMap<String, String>,
) -> Result<FixedBytes<32>, TaskManagerError> {
    if map.is_empty() {
        return Ok(FixedBytes::from_slice(&[0u8; 32]));
    }

    let mut leaves: Vec<[u8; 32]> = map.iter().map(|(k, v)| hash_leaf(k, v)).collect();

    while leaves.len() > 1 {
        leaves = leaves
            .chunks(2)
            .map(|pair| {
                let left = pair.first().unwrap();
                let right = pair.get(1).unwrap_or(left);
                hash_nodes(*left, *right)
            })
            .collect();
    }
    let root = FixedBytes::from_slice(&leaves[0]);
    info!("Vault root: {:?}", root);
    Ok(root)
}

/// Hash two nodes
///
/// # Arguments
///
/// * `left` - The left node
/// * `right` - The right node
///
/// # Returns
///
/// * `[u8; 32]` - The hash of the two nodes
fn hash_nodes(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let (a, b) = if left > right {
        (right, left)
    } else {
        (left, right)
    };
    let mut hasher = Keccak256::new();
    hasher.update(a);
    hasher.update(b);
    hasher.finalize().into()
}

/// Hash a leaf
///
/// # Arguments
///
/// * `key` - The key
/// * `value` - The value
///
/// # Returns
///
/// * `[u8; 32]` - The hash of the leaf
fn hash_leaf(key: &str, value: &str) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(key.as_bytes());
    hasher.update(value.as_bytes());
    hasher.finalize().into()
}
