#![allow(missing_docs)]

use awesome_vault_service::{
    task_manager::{save_value, save_wrong_value, ISTaskManager},
    utils::{load_config, setup_operator},
};
use eigen_operator::failing_response_calculator;
use eigensdk::{
    logging::{get_logger, init_logger, log_level::LogLevel},
    operator::{config::OperatorConfig, Operator},
};

use std::{collections::BTreeMap, sync::Arc};
use tokio::sync::Mutex;
use tracing::info;

// This example shows how to initialize an operator and start to listen for new task events.
// For this example, Operator should be registered.
#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let logger = get_logger();
    let config: OperatorConfig = load_config("./src/config/awesome-operator.toml").unwrap();

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

    // Initialize the operator
    let operator = Operator::new(logger, config).await.unwrap();

    let redis_state = Arc::new(Mutex::new(BTreeMap::<String, String>::new()));

    // TESTING PURPOSES ONLY
    let compute = failing_response_calculator(
        save_value(redis_state.clone()).await,
        save_wrong_value(redis_state).await,
        50,
    );

    operator
        .start::<ISTaskManager>(compute.await)
        .await
        .unwrap();
}
