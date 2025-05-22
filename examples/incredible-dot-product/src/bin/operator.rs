//! Incredible Dot Product Operator

use alloy::primitives::U256;
use eigensdk::{
    logging::{get_logger, init_logger, log_level::LogLevel},
    operator::{config::OperatorConfig, Operator},
    task_manager::response_calculator::response_calculator_from_fn,
    testing_utils::task_processor::failing_response_calculator,
};
use eyre::Result;
use incredible_dot_product::{
    task_manager::{dot_product, ISTaskManager},
    utils::load_config,
};

/// This example shows how to initialize an operator and start processing tasks.
/// For this example, Operator should be registered.
/// Follow the [`Operator`](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/operator/src/lib.rs#L1-L110)
/// documentation to set up an operator.
///
/// 1. Define your types for the task manager (Done in [`ISTaskManager`])
/// 2. Create the [`OperatorConfig`]
/// 3. Create the logic to compute the task (Done in [`dot_product`])
/// 4. Build the [`ResponseCalculator`](eigensdk::task_manager::response_calculator::ResponseCalculator)
/// 5. Use the [`failing_response_calculator`] to test how the operator behaves when
///    it responds incorrectly to a task and how slashing works
/// 6. Start the operator
#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);
    let logger = get_logger();

    // 2. Create the `OperatorConfig`
    let config: OperatorConfig = load_config("./src/config/dot-operator.toml").unwrap();

    // 4. Build the `ResponseCalculator` with the computation function
    let response_calculator = response_calculator_from_fn(dot_product);

    // 5. Use the `failing_response_calculator` with the wrong `Output` type and a given failure rate
    let logic = failing_response_calculator(response_calculator, || U256::MAX, 40);

    // 6. Start the operator
    let operator = Operator::new(logger, config).await.unwrap();
    operator
        .start::<ISTaskManager>(logic)
        .await
        .map_err(|e| eyre::eyre!("Operator start error: {}", e))?;

    Ok(())
}
