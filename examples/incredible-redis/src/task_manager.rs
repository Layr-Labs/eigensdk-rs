//! Example AVS which squares a number

use std::collections::BTreeMap;
use std::sync::Arc;

use alloy::primitives::Bytes;
use alloy::primitives::Keccak256;
use alloy::primitives::B256;
use alloy::sol_types::SolEvent;
use eigensdk::task_processor::impl_task_manager_from_defs_and_contract;
use eigensdk::task_processor::task_manager::TaskManagerDefs;
use eigensdk::task_processor::task_manager::TaskManagerError;
use tokio::sync::Mutex;
use tracing::info;

use crate::bindings::incredibleredistaskmanager::IIncredibleRedisTaskManager::SetInput;
use crate::bindings::incredibleredistaskmanager::IncredibleRedisTaskManager::IncredibleRedisTaskManagerInstance;
use crate::bindings::incredibleredistaskmanager::IncredibleRedisTaskManager::NewTaskCreated;
use crate::bindings::incredibleredistaskmanager::IncredibleRedisTaskManager::TaskResponded;

// Implement the [`TaskManagerDefs`] trait for a unit struct.
// You need to specify the input and output types of the task. In this case, U256.
// You also need to specify the selectors for the new task event and the task responded event.
pub struct ISTaskManager;

impl TaskManagerDefs for ISTaskManager {
    // TODO SDK: Should we remove the `Debug` bound in TM::Input?
    type Input = SetInput;
    type Output = Bytes;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(ISTaskManager => IncredibleRedisTaskManagerInstance);

pub fn set(
    redis_state: Arc<Mutex<BTreeMap<String, String>>>,
) -> impl AsyncFn(u32, SetInput) -> Result<Bytes, TaskManagerError> {
    move |_task_index, input: SetInput| {
        let redis_state = redis_state.clone();
        async move {
            info!("Setting key: {} with value: {}", input.key, input.value);
            let mut map = redis_state.lock().await;
            map.insert(input.key.clone(), input.value.clone());
            dbg!(&map);
            Ok(hash_state(&map))
        }
    }
}

fn hash_state(state: &BTreeMap<String, String>) -> Bytes {
    let mut keccak = Keccak256::new();
    for (k, v) in state.iter() {
        keccak.update(k.as_bytes());
        keccak.update(v.as_bytes());
    }
    Bytes::from(keccak.finalize().to_vec())
}
