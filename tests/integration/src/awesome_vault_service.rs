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
    testing_utils::anvil::start_anvil_container_with_state,
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
const AVS_ADDRESS: &str = "0x5f3f1dbd7b74c6b46e8c44f98792a1daf8d69154";
const REGISTRY_COORDINATOR: &str = "0x7bc06c482dead17c0e297afbc32f6e63d3846650";
const OPERATOR_STATE_RETRIEVER_ADDRESS: &str = "0x4c5859f0f772848b2d91f1d83e2fe57935348029";
const ALLOCATION_MANAGER_ADDRESS: &str = "0x2279b7a0a67db372996a5fab50d91eaa73d2ebe6";
const DELEGATION_MANAGER_ADDRESS: &str = "0x9fe46736679d2d9a65f0992f2272de9f3c7fa6e0";
const STRATEGY_MANAGER_ADDRESS: &str = "0x0165878a594ca255338adfa4d48449f69242eb8f";
const ERC20_STRATEGY_ADDRESS: &str = "0x2b961e3959b79326a8e7f64ef0d2d825707669b5";
const REWARDS_COORDINATOR_ADDRESS: &str = "0xa51c1fc2f0d1a1b8494ed1fe312d7c3a78ed91c0";
const AVS_DIRECTORY_ADDRESS: &str = "0x610178da211fef7d417bc0e6fed39f05609ad788";
const PERMISSION_CONTROLLER_ADDRESS: &str = "0x59b670e9fa9d0a427751af201d676719a970857b";

// Task spammer configuration
const NUM_TASKS: u64 = 3;
const TASK_INTERVAL: u64 = 5;

// Aggregator configuration
const AGGREGATOR_RPC_URL: &str = "127.0.0.1:8080";

// Signers
const AGGREGATOR_SIGNER: &str =
    "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
const OPERATOR_SIGNER: &str = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
const TASK_SPAMMER_SIGNER: &str =
    "0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";

// Operator configuration
const OPERATOR_ADDRESS: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const OPERATOR_BLS_SIGNER: &str =
    "1371012690269088913462269866874713266643928125698382731338806296762673180359922";

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
// This test will deploy the AVS, start the aggregator, operator, challenger and task spammer
// and verify that all tasks (`NUM_TASKS`) have been completed
#[tokio::test]
async fn test_awesome_vault_service() {
    let (_container, http_endpoint, ws_endpoint) =
        start_anvil_container_with_state(AWESOME_VAULT_SERVICE_STATE_PATH).await;

    init_logger(LogLevel::Info);
    let logger = get_test_logger();

    // Create the AVS config
    let config = create_avs_config(
        &http_endpoint,
        &ws_endpoint,
        create_task_manager_contract(&http_endpoint, AGGREGATOR_SIGNER),
        create_task_manager_contract(&http_endpoint, OPERATOR_SIGNER),
        create_task_manager_contract(&http_endpoint, TASK_SPAMMER_SIGNER),
    );

    // Build the response calculator, which is used to compute the response for a task
    let response_calculator = VaultServiceResponseCalculator {
        vault: Arc::new(Mutex::new(BTreeMap::new())),
    };

    // Start the AVS
    let (aggregator_handle, operator_handle, challenger_handle, spammer_handle) =
        start_avs(config, response_calculator, logger, |_| generate_input()).await;

    // Wait until `NUM_TASKS` tasks are created
    spammer_handle.await.unwrap();

    // Give some time to the aggregator to process the last task
    tokio::time::sleep(Duration::from_secs(TASK_INTERVAL)).await;

    // Abort the aggregator, operator and challenger handles
    for handle in [aggregator_handle, operator_handle, challenger_handle] {
        handle.abort();
    }

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
        assert_ne!(
            B256::default(),
            response_hash,
            "Tarea {} sin respuesta",
            task_index
        );
    }
}

/// Create the AVS config with hardcoded values
fn create_avs_config(
    http_endpoint: &str,
    ws_endpoint: &str,
    aggregator_task_manager: TaskManagerInstance,
    challenger_task_manager: TaskManagerInstance,
    task_spammer_task_manager: TaskManagerInstance,
) -> AvsConfig<TaskManagerInstance> {
    AvsConfig {
        task_manager_address: Address::from_str(TASK_MANAGER_ADDRESS).unwrap(),
        http_rpc_url: http_endpoint.to_string(),
        ws_rpc_url: ws_endpoint.to_string(),
        avs_address: Address::from_str(AVS_ADDRESS).unwrap(),
        registry_coordinator_address: Address::from_str(REGISTRY_COORDINATOR).unwrap(),
        operator_state_retriever_address: Address::from_str(OPERATOR_STATE_RETRIEVER_ADDRESS)
            .unwrap(),
        allocation_manager_address: Address::from_str(ALLOCATION_MANAGER_ADDRESS).unwrap(),
        delegation_manager_address: Address::from_str(DELEGATION_MANAGER_ADDRESS).unwrap(),
        strategy_manager_address: Address::from_str(STRATEGY_MANAGER_ADDRESS).unwrap(),
        strategy_address: Address::from_str(ERC20_STRATEGY_ADDRESS).unwrap(),
        rewards_coordinator_address: Address::from_str(REWARDS_COORDINATOR_ADDRESS).unwrap(),
        avs_directory_address: Address::from_str(AVS_DIRECTORY_ADDRESS).unwrap(),
        permission_controller_address: Address::from_str(PERMISSION_CONTROLLER_ADDRESS).unwrap(),
        operator_bls_private_key: OPERATOR_BLS_SIGNER.to_string(),
        aggregator_ip_port: AGGREGATOR_RPC_URL.to_string(),
        task_interval: TASK_INTERVAL,
        quorum_threshold: 50,
        quorums: vec![0],
        num_tasks: NUM_TASKS,
        operator_private_key: OPERATOR_SIGNER.to_string(),
        challenger_private_key: OPERATOR_SIGNER.to_string(),
        aggregator_private_key: AGGREGATOR_SIGNER.to_string(),
        operator_address: Address::from_str(OPERATOR_ADDRESS).unwrap(),
        operator_name: "awesome-vault".to_string(),
        metadata_uri: "metadata".to_string(),
        socket: "127.0.0.1:8080".to_string(),
        allocation_delay: 0,
        operator_set_id: 0,
        new_magnitude: vec![1000000000000000000],
        deposit_tokens: "5000000000000000000000".to_string(),
        time_to_expiry: Duration::from_secs(5),
        window_duration: Duration::from_secs(2),
        aggregator_task_manager,
        challenger_task_manager,
        task_spammer_task_manager,
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
pub struct ISTaskManager;
impl TaskManagerDefs for ISTaskManager {
    type Input = TaskInput;
    type Output = B256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(ISTaskManager => AwesomeVaultTaskManagerInstance);

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
fn generate_input() -> TaskInput {
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
