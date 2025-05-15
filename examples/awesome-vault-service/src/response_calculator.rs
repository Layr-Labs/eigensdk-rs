use crate::bindings::awesomevaulttaskmanager::IAwesomeVaultTaskManager::TaskInput;
use alloy::primitives::{FixedBytes, Keccak256};
use eigensdk::task_manager::{response_calculator::ResponseCalculator, TaskManagerError};
use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;

pub struct VaultServiceResponseCalculator {
    pub vault: Arc<Mutex<BTreeMap<String, String>>>,
}

impl ResponseCalculator<TaskInput, FixedBytes<32>> for VaultServiceResponseCalculator {
    async fn compute_response(
        &self,
        _task_index: u32,
        input: TaskInput,
    ) -> Result<FixedBytes<32>, TaskManagerError> {
        let mut vault = self.vault.lock().await;
        info!("Setting key: {} with value: {}", input.key, input.value);
        vault.insert(input.key, input.value);
        compute_vault_root(&vault)
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
