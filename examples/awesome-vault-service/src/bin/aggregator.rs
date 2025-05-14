//! Incredible Dot Product Aggregator

use alloy::primitives::Address;
use awesome_vault_service::bindings::awesomevaulttaskmanager::AwesomeVaultTaskManager::AwesomeVaultTaskManagerInstance;
use eigensdk::{
    aggregator::{task_processor::IndexingTaskProcessor, Aggregator, AggregatorConfig},
    common::get_signer,
    logging::{init_logger, log_level::LogLevel},
};
use eyre::Result;

use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);
    let http_rpc_url = "http://localhost:8545".to_string();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let wallet = get_signer(
        "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
        &http_rpc_url,
    );
    let aggregator_ip_port = "127.0.0.1:8080".to_string();
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;
    let registry_coordinator_address =
        Address::from_str("0x7bc06c482dead17c0e297afbc32f6e63d3846650")?;
    let operator_state_retriever_address =
        Address::from_str("0x4c5859f0f772848b2d91f1d83e2fe57935348029")?;

    let contract = AwesomeVaultTaskManagerInstance::new(task_manager_address, wallet);

    let task_processor =
        IndexingTaskProcessor::new(contract, Duration::from_secs(10), Duration::from_secs(2));

    let config = AggregatorConfig {
        server_address: aggregator_ip_port,
        http_rpc_url,
        ws_rpc_url,
        registry_coordinator: registry_coordinator_address,
        operator_state_retriever: operator_state_retriever_address,
    };

    let aggregator = Aggregator::new(config, task_processor)
        .await
        .map_err(|e| eyre::eyre!("Aggregator new error: {}", e))?;
    aggregator
        .start()
        .await
        .map_err(|e| eyre::eyre!("Aggregator start error: {}", e))?;

    Ok(())
}
