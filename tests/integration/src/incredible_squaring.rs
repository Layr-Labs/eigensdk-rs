use std::{str::FromStr, time::Duration};

use crate::{
    bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::{
        IncredibleSquaringTaskManagerInstance, NewTaskCreated, TaskResponded,
    },
    generic_avs::{start_avs, AvsConfig},
};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, B256, U256},
    sol_types::SolEvent,
};
use eigensdk::{
    common::get_signer,
    logging::{get_test_logger, init_logger, log_level::LogLevel},
    task_manager::{
        impl_task_manager_from_defs_and_contract, response_calculator::response_calculator_from_fn,
        TaskManagerDefs, TaskManagerError,
    },
    testing_utils::anvil::start_anvil_with_state,
};

// Contracts addresses
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
    "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";

// Anvil state path
const INCREDIBLE_SQUARING_STATE_PATH: &str =
    "./examples/incredible-squaring/contracts/anvil/incredible-squaring-anvil-state/state.json";

type IncredibleInstance = IncredibleSquaringTaskManagerInstance<
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

/// Test incredible squaring
#[tokio::test]
async fn test_incredible_squaring() {
    let (_container, http_endpoint, ws_endpoint) =
        start_anvil_with_state(INCREDIBLE_SQUARING_STATE_PATH).await;

    init_logger(LogLevel::Info);
    let logger = get_test_logger();

    // Task spammer should finish when all tasks are created (`NUM_TASKS` * `TASK_INTERVAL`)
    // so we add 5 seconds to the timeout
    let timeout_duration = Duration::from_secs(NUM_TASKS * TASK_INTERVAL + 5);

    // Input generator
    let input = |i| U256::from(i);

    // Response calculator
    let response_calculator = || response_calculator_from_fn(square);

    // Create the AVS config using defaults
    let config = AvsConfig::with_default_addresses_and_keys(
        create_task_manager_contract(&http_endpoint, AGGREGATOR_SIGNER),
        create_task_manager_contract(&http_endpoint, CHALLENGER_SIGNER),
        create_task_manager_contract(&http_endpoint, TASK_SPAMMER_SIGNER),
        response_calculator,
        input,
        logger,
        timeout_duration,
        http_endpoint.to_string(),
        ws_endpoint.to_string(),
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

    verify_tasks_completed(&http_endpoint).await;
}

/// Verify that all tasks created by the task spammer have been completed
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

/// Create the task manager contract with a specific signer
fn create_task_manager_contract(http_endpoint: &str, signer: &str) -> IncredibleInstance {
    let task_manager_address = Address::from_str(TASK_MANAGER_ADDRESS).unwrap();
    let provider = get_signer(signer, http_endpoint);
    IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider)
}

/// Compute the square of the number
fn square(_task_index: u32, number_to_be_squared: U256) -> Result<U256, TaskManagerError> {
    Ok(number_to_be_squared * number_to_be_squared)
}

/// Task manager definitions for incredible squaring
#[derive(Debug, Clone)]
pub struct ISTaskManager;
impl TaskManagerDefs for ISTaskManager {
    type Input = U256;
    type Output = U256;
    const NEW_TASK_EVENT_SELECTOR: B256 = NewTaskCreated::SIGNATURE_HASH;
    const TASK_RESPONDED_EVENT_SELECTOR: B256 = TaskResponded::SIGNATURE_HASH;
}

impl_task_manager_from_defs_and_contract!(ISTaskManager => IncredibleSquaringTaskManagerInstance);
