#![allow(missing_docs)]

use alloy::primitives::U256;
use eigen_logging::get_logger;
use eigen_operator::{config::OperatorConfig, failing_response_calculator, Operator};
use incredible_squaring::{square, utils::load_config, ISTaskManager};

// This example shows how to initialize an operator and start to listen for new task events.
// For this example, Operator should be registered.
#[tokio::main]
async fn main() {
    let logger = get_logger();
    let config: OperatorConfig = load_config("./src/config/squaring-operator.toml").unwrap();

    // Initialize the operator
    let operator = Operator::new(logger, config).await.unwrap();

    let logic = failing_response_calculator(square, |_, _| async { Ok(U256::from(42)) }, 60);

    // Subscribe to the new task events and start listening. When a new task is created,
    // the operator will process it and send the signed task response to the aggregator.
    operator.start::<ISTaskManager>(logic.await).await.unwrap();
}
