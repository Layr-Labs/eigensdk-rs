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

// This example shows how to initialize an operator and start to listen for new task events.
// For this example, Operator should be registered.
#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let logger = get_logger();
    let config: OperatorConfig = load_config("./src/config/awesome-operator.toml").unwrap();

    // Initialize the operator
    let operator = Operator::new(logger, config).await.unwrap();

    let vault_service_response_calculator = VaultServiceResponseCalculator {
        vault: Arc::new(Mutex::new(BTreeMap::new())),
    };

    let logic = failing_response_calculator(vault_service_response_calculator, B256::default, 50);

    operator.start::<ISTaskManager>(logic).await.unwrap();
}
