//! This example shows how to initialize an operator and start processing tasks.
//! Follow the [`eigen-operator` crate documentation](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/operator/src/lib.rs#L1-L131)
//! to set up an operator.

use alloy::primitives::U256;
use eigensdk::{
    operator::{config::OperatorConfig, Operator},
    task_manager::response_calculator::response_calculator_from_fn,
    testing_utils::task_processor::failing_response_calculator,
};
use eyre::Result;
use incredible_dot_product::{
    task_manager::{dot_product, ISTaskManager},
    utils::load_config,
};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Define your types for the task manager (we do this in `ISTaskManager`: lib.rs)

    // 2. Create the `OperatorConfig`
    let config: OperatorConfig = load_config("./src/config/dot-operator.toml").unwrap();

    // 3. Create the logic to compute the task (we do this in `dot_product`: lib.rs)

    // 4. Build the `ResponseCalculator` with the computation function
    let response_calculator = response_calculator_from_fn(dot_product);

    // 5. Use the `failing_response_calculator` with the wrong `Output` type and a given failure rate
    let logic = failing_response_calculator(response_calculator, || U256::MAX, 40);

    let operator = Operator::new(config, logic).await.unwrap();
    operator
        .run::<ISTaskManager>()
        .await
        .map_err(|e| eyre::eyre!("Operator start error: {}", e))?;

    Ok(())
}
