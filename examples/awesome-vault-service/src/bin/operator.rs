//! This example shows how to initialize an operator and start processing tasks.
//! Follow the [`eigen-operator` crate documentation](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/operator/src/lib.rs#L1-L131)
//! to set up an operator.

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

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let logger = get_logger();

    // 1. Define your types for the task manager (we do this in `ISTaskManager`: lib.rs)

    // 2. Create the `OperatorConfig`
    let config: OperatorConfig = load_config("./src/config/awesome-operator.toml").unwrap();

    // 3. Create the logic to compute the task (we do this in `compute_vault_root`: lib.rs)

    // 4. Build the `VaultServiceResponseCalculator`, which implements the `ResponseCalculator` trait
    let vault_service_response_calculator = VaultServiceResponseCalculator {
        vault: Arc::new(Mutex::new(BTreeMap::new())),
    };

    // 5. Use the `failing_response_calculator` with the wrong `Output` type and a given failure rate
    let logic = failing_response_calculator(vault_service_response_calculator, B256::default, 50);

    // 6. Start the operator
    let operator = Operator::new(logger, config, logic).await.unwrap();
    operator.run::<ISTaskManager>().await.unwrap();
}
