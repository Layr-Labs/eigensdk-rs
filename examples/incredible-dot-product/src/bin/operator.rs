//! Incredible Dot Product Operator

use eigensdk::{
    logging::{get_logger, init_logger, log_level::LogLevel},
    operator::{config::OperatorConfig, failing_response_calculator, Operator},
};
use eyre::Result;
use incredible_dot_product::{
    load_config,
    task_manager::{dot_product, invalid_dot_product, ISTaskManager},
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
            config.bls_key_pair.clone(),
            config.http_rpc_url.clone(),
        )
        .await
        .unwrap();
        info!("Operator setup complete");
    }

    let operator = Operator::new(logger, config)
        .await
        .map_err(|e| eyre::eyre!("Operator new error: {}", e))?;

    let logic = failing_response_calculator(dot_product, invalid_dot_product, 40);

    // TODO: Review bounds in SDK. I have to derive Serialize and Deserialize for TaskResponse in the bindings
    operator
        .start::<ISTaskManager>(logic.await)
        .await
        .map_err(|e| eyre::eyre!("Operator start error: {}", e))?;

    Ok(())
}
