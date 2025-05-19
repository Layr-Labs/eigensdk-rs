#![allow(missing_docs)]

use alloy::primitives::U256;
use eigensdk::logging::log_level::LogLevel;
use eigensdk::logging::{get_logger, init_logger};
use eigensdk::operator::{config::OperatorConfig, Operator};
use eigensdk::task_manager::response_calculator::response_calculator_from_fn;
use eigensdk::testing_utils::task_processor::failing_response_calculator;
use incredible_squaring::{square, utils::load_config, ISTaskManager};

// This example shows how to initialize an operator and start to listen for new task events.
// For this example, Operator should be registered.
#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let logger = get_logger();
    let config: OperatorConfig = load_config("./src/config/squaring-operator.toml").unwrap();

    // Initialize the operator
    let operator = Operator::new(logger, config).await.unwrap();

    let response_calculator = response_calculator_from_fn(square);
    let logic = failing_response_calculator(response_calculator, || U256::from(42), 60);

    // Subscribe to the new task events and start listening. When a new task is created,
    // the operator will process it and send the signed task response to the aggregator.
    operator.start::<ISTaskManager>(logic).await.unwrap();
}
