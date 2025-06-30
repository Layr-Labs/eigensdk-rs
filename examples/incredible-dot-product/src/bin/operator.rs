//! Incredible Dot Product Operator

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
    let config: OperatorConfig = load_config("./src/config/dot-operator.toml").unwrap();

    let response_calculator = response_calculator_from_fn(dot_product);

    let logic = failing_response_calculator(response_calculator, || U256::MAX, 40);

    let operator = Operator::new(config).await.unwrap();
    operator
        .start::<ISTaskManager>(logic)
        .await
        .map_err(|e| eyre::eyre!("Operator start error: {}", e))?;

    Ok(())
}
