#![allow(missing_docs)]

use alloy::primitives::B256;
use awesome_vault_service::{
    response_calculator::VaultServiceResponseCalculator, task_manager::ISTaskManager,
    utils::load_config,
};
use eigensdk::{
    logging::{get_logger, init_logger, log_level::LogLevel},
    operator::{config::OperatorConfig, Operator},
    testing_utils::task_processor::failing_response_calculator,
};

use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::Mutex;

/// This example shows how to initialize an operator and start processing tasks.
/// For this example, Operator should be registered.
/// Follow the [`Operator`](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/operator/src/lib.rs#L1-L110)
/// documentation to set up an operator.
///
/// 1. Define your types for the task manager (Done in [`ISTaskManager`])
/// 2. Create the [`OperatorConfig`]
/// 3. Create the logic to compute the task (Done in [`compute_vault_root`](awesome_vault_service::response_calculator::compute_vault_root))
/// 4. Build a custom [`ResponseCalculator`](eigensdk::task_manager::response_calculator::ResponseCalculator)
///    implementation, since we want to save the state of the vault in memory
///    (Done in [`VaultServiceResponseCalculator`])
/// 5. Use the [`failing_response_calculator`] to test how the operator behaves when
///    it responds incorrectly to a task and how slashing works
/// 6. Start the operator
#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let logger = get_logger();

    // 2. Create the `OperatorConfig`
    let config: OperatorConfig = load_config("./src/config/awesome-operator.toml").unwrap();

    // 4. Build the `VaultServiceResponseCalculator`, which implements the `ResponseCalculator` trait
    let vault_service_response_calculator = VaultServiceResponseCalculator {
        vault: Arc::new(Mutex::new(BTreeMap::new())),
    };

    // 5. Use the `failing_response_calculator` with the wrong `Output` type and a given failure rate
    let logic = failing_response_calculator(vault_service_response_calculator, B256::default, 50);

    // 6. Start the operator
    let operator = Operator::new(logger, config).await.unwrap();
    operator.start::<ISTaskManager>(logic).await.unwrap();
}
