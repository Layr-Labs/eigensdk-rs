use std::{collections::BTreeMap, str::FromStr, sync::Arc, time::Duration};

use alloy::{
    network::EthereumWallet,
    primitives::{Address, Keccak256, B256},
    sol_types::SolEvent,
};
use eigensdk::{
    common::get_signer,
    logging::{get_test_logger, init_logger, log_level::LogLevel},
    task_manager::{
        impl_task_manager_from_defs_and_contract, response_calculator::ResponseCalculator,
        TaskManagerDefs, TaskManagerError,
    },
    testing_utils::anvil::start_anvil_with_state,
};
use rand::Rng;
use tokio::sync::Mutex;

use crate::{
    bindings::awesomevaulttaskmanager::{
        AwesomeVaultTaskManager::{AwesomeVaultTaskManagerInstance, NewTaskCreated, TaskResponded},
        IAwesomeVaultTaskManager::TaskInput,
    },
    generic_avs::{start_avs, AvsConfig},
};

// Contract addresses
const TASK_MANAGER_ADDRESS: &str = "0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3";

// Task spammer config
const NUM_TASKS: u64 = 3;
const TASK_INTERVAL: u64 = 5;
const QUORUM_THRESHOLD: u8 = 50;
const QUORUMS: [u8; 1] = [0];

// Aggregator config
const TIME_TO_EXPIRY: Duration = Duration::from_secs(5);
const WINDOW_DURATION: Duration = Duration::from_secs(3);

// Strategy config
const NEW_MAGNITUDE: [u64; 1] = [1000000000000000000];
const DEPOSIT_TOKENS: &str = "5000000000000000000000";

// Signers
const AGGREGATOR_SIGNER: &str =
    "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
const CHALLENGER_SIGNER: &str =
    "0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba";
// This one must match the `task_generator_addr` passed to the Task manager in deployment
const TASK_SPAMMER_SIGNER: &str =
    "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";

// Anvil state path
const AWESOME_VAULT_SERVICE_STATE_PATH: &str =
    "./examples/awesome-vault-service/contracts/anvil/awesome-vault-service-anvil-state/state.json";

type TaskManagerInstance = AwesomeVaultTaskManagerInstance<
    (),
    alloy::providers::fillers::FillProvider<
        alloy::providers::fillers::JoinFill<
            alloy::providers::fillers::JoinFill<
                alloy::providers::Identity,
                alloy::providers::fillers::JoinFill<
                    alloy::providers::fillers::GasFiller,
                    alloy::providers::fillers::JoinFill<
                        alloy::providers::fillers::BlobGasFiller,
                        alloy::providers::fillers::JoinFill<
                            alloy::providers::fillers::NonceFiller,
                            alloy::providers::fillers::ChainIdFiller,
                        >,
                    >,
                >,
            >,
            alloy::providers::fillers::WalletFiller<EthereumWallet>,
        >,
        alloy::providers::RootProvider,
    >,
>;

// Test the awesome vault service
#[tokio::test]
async fn test_awesome_vault_service() {
    let (_container, http_endpoint, ws_endpoint) =
        start_anvil_with_state(AWESOME_VAULT_SERVICE_STATE_PATH).await;

    init_logger(LogLevel::Info);
    let logger = get_test_logger();

    // Task spammer should finish when all tasks are created (`NUM_TASKS` * `TASK_INTERVAL`)
    // so we add 5 seconds to the timeout
    let timeout_duration = Duration::from_secs(NUM_TASKS * TASK_INTERVAL + 5);

    // Vault service response calculator

    // Create the AVS config
    let config = AvsConfig::with_default_addresses_and_keys(
        create_task_manager_contract(&http_endpoint, AGGREGATOR_SIGNER),
        create_task_manager_contract(&http_endpoint, CHALLENGER_SIGNER),
        create_task_manager_contract(&http_endpoint, TASK_SPAMMER_SIGNER),
        || VaultServiceResponseCalculator {
            vault: Arc::new(Mutex::new(BTreeMap::new())),
        },
        generate_input,
        logger,
        timeout_duration,
        http_endpoint.to_string(),
        ws_endpoint.to_string(),
        "awesome-vault".to_string(),
        TIME_TO_EXPIRY,
        WINDOW_DURATION,
        TASK_INTERVAL,
        QUORUM_THRESHOLD,
        QUORUMS.to_vec(),
        NUM_TASKS,
        DEPOSIT_TOKENS.to_string(),
        NEW_MAGNITUDE.to_vec(),
    );

    // Start the AVS
    start_avs(&config).await;

    // Give some time to the aggregator to process the last task
    tokio::time::sleep(Duration::from_secs(TASK_INTERVAL)).await;

    // Verify that all tasks have been completed
    verify_tasks_completed(&http_endpoint).await;
}

/// Verify that all tasks that have been created by the task spammer have been completed
async fn verify_tasks_completed(http_endpoint: &str) {
    let contract = create_task_manager_contract(http_endpoint, AGGREGATOR_SIGNER);
    let latest_task_num = contract.latestTaskNum().call().await.unwrap()._0;
    assert_eq!(latest_task_num, NUM_TASKS as u32);

    // Verify that all tasks have responses
    for task_index in 0..latest_task_num {
        let response_hash = contract
            .allTaskResponses(task_index)
            .call()
            .await
            .unwrap()
            ._0;
        assert_ne!(B256::default(), response_hash,);
    }
}

/// Create the task manager contract for a specific signer
fn create_task_manager_contract(http_endpoint: &str, signer: &str) -> TaskManagerInstance {
    let task_manager_address = Address::from_str(TASK_MANAGER_ADDRESS).unwrap();
    let provider = get_signer(signer, http_endpoint);
    AwesomeVaultTaskManagerInstance::new(task_manager_address, provider)
}

/// Build the task manager struct for the awesome vault task manager
#[derive(Debug, Clone)]
pub struct AwesomeVaultTaskManager;
impl TaskManagerDefs for AwesomeVaultTaskManager {
    type Input = TaskInput;
    type Output = B256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(AwesomeVaultTaskManager => AwesomeVaultTaskManagerInstance);

/// Response Calculator for the Vault Service
/// We want to simulate a vault service that stores key-value pairs in a BTreeMap
/// and computes the vault root as the hash of the leaves.
#[derive(Clone)]
pub struct VaultServiceResponseCalculator {
    pub vault: Arc<Mutex<BTreeMap<String, String>>>,
}

/// Implement the [`ResponseCalculator`] trait for the Vault Service
/// This trait is used to compute the response for a task.
impl ResponseCalculator<TaskInput, B256> for VaultServiceResponseCalculator {
    async fn compute_response(
        &self,
        _task_index: u32,
        input: TaskInput,
    ) -> Result<B256, TaskManagerError> {
        let mut vault = self.vault.lock().await;
        vault.insert(input.key, input.value);
        compute_vault_root(&vault)
    }
}

/// Compute the vault root
pub fn compute_vault_root(map: &BTreeMap<String, String>) -> Result<B256, TaskManagerError> {
    if map.is_empty() {
        return Ok(Default::default());
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
    let root = B256::from_slice(&leaves[0]);
    Ok(root)
}

/// Generate random input for the vault
fn generate_input(_: u64) -> TaskInput {
    let mut rng = rand::thread_rng();
    TaskInput {
        key: format!("key_{}", rng.gen_range(0..1000000)),
        value: format!("value_{}", rng.gen_range(0..1000000)),
    }
}

/// Hash two nodes of the Merkle tree
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

/// Hash a leaf of the tree (key-value)
fn hash_leaf(key: &str, value: &str) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    hasher.update(key.as_bytes());
    hasher.update(value.as_bytes());
    hasher.finalize().into()
}
