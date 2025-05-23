//! This example shows how to initialize an operator and start processing tasks.
//! Follow the [`eigen-operator` crate documentation`](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/operator/src/lib.rs#L1-L131)
//! to set up an operator.

use alloy::primitives::U256;
use eigensdk::logging::log_level::LogLevel;
use eigensdk::logging::{get_logger, init_logger};
use eigensdk::operator::{config::OperatorConfig, Operator};
use eigensdk::task_manager::response_calculator::response_calculator_from_fn;
use eigensdk::testing_utils::task_processor::failing_response_calculator;
use incredible_squaring::{square, utils::load_config, ISTaskManager};

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let logger = get_logger();

    // 1. Define your types for the task manager (we do this in `ISTaskManager`: lib.rs)

    // 2. Create the `OperatorConfig`
    let config: OperatorConfig = load_config("./src/config/squaring-operator.toml").unwrap();

    // 3. Create the logic to compute the task (we do this in `square`: lib.rs)

    // 4. Build the `ResponseCalculator` with the computation function
    let response_calculator = response_calculator_from_fn(square);

    // 5. Use the `failing_response_calculator` with the wrong `Output` type and a given failure rate
    let logic = failing_response_calculator(response_calculator, || U256::from(42), 60);

    // 6. Initialize the operator
    let operator = Operator::new(logger, config).await.unwrap();
    operator.start::<ISTaskManager>(logic).await.unwrap();
}
