//! Incredible Dot Product Aggregator

use std::{str::FromStr, time::Duration};

use alloy::primitives::Address;
use eigensdk::{
    aggregator::{Aggregator, AggregatorConfig},
    common::get_signer,
    logging::{init_logger, log_level::LogLevel},
    task_processor::IndexingTaskProcessor,
};
use eyre::Result;
use incredible_bindings::incredibledotproducttaskmanager::IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance;
use incredible_dot_product::TaskManagerWrapper;

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
    let task_manager_address = Address::from_str("0x7bc06c482dead17c0e297afbc32f6e63d3846650")?;
    let registry_coordinator_address =
        Address::from_str("0xfd471836031dc5108809d173a067e8486b9047a3")?;
    let operator_state_retriever_address =
        Address::from_str("0x5f3f1dbd7b74c6b46e8c44f98792a1daf8d69154")?;

    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);
    let wrapper_contract = TaskManagerWrapper(contract);

    let task_processor = IndexingTaskProcessor::new(
        wrapper_contract,
        Duration::from_secs(10),
        Duration::from_secs(2),
    );

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
