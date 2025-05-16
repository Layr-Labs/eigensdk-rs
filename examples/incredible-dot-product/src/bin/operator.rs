//! Incredible Dot Product Operator

use alloy::primitives::U256;
use eigensdk::{
    crypto_bls::BlsKeyPair,
    logging::{get_logger, init_logger, log_level::LogLevel},
    operator::{config::OperatorConfig, Operator},
    task_manager::response_calculator::response_calculator_from_fn,
    testing_utils::task_processor::failing_response_calculator,
};
use eyre::Result;
use incredible_dot_product::{
    load_config,
    task_manager::{dot_product, ISTaskManager},
    utils::setup_operator,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);
    let logger = get_logger();
    let config: OperatorConfig = load_config("./src/config/dot-operator.toml").unwrap();

    if let Some(registration) = config.registration.clone() {
        setup_operator(
            registration,
            BlsKeyPair::new(config.bls_private_key.clone()).unwrap(),
            config.http_rpc_url.clone(),
        )
        .await
        .unwrap();
        info!("Operator setup complete");
    }

    let operator = Operator::new(logger, config)
        .await
        .map_err(|e| eyre::eyre!("Operator new error: {}", e))?;

    let response_calculator = response_calculator_from_fn(dot_product);

    let logic = failing_response_calculator(response_calculator, || U256::MAX, 40);

    // TODO: Review bounds in SDK. I have to derive Serialize and Deserialize for TaskResponse in the bindings
    operator
        .start::<ISTaskManager>(logic)
        .await
        .map_err(|e| eyre::eyre!("Operator start error: {}", e))?;

    Ok(())
}
