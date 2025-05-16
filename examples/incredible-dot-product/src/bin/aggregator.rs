//! Incredible Dot Product Aggregator

use std::{str::FromStr, time::Duration};

use alloy::primitives::Address;
use eigensdk::{
    aggregator::{task_processor::IndexingTaskProcessor, Aggregator, AggregatorConfig},
    common::get_signer,
    logging::{init_logger, log_level::LogLevel},
};
use eyre::Result;
use incredible_dot_product::{
    load_config, IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);
    let config: AggregatorConfig = load_config("./src/config/dot-aggregator.toml")?;
    let wallet = get_signer(
        "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
        &config.http_rpc_url,
    );
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;

    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);

    let task_processor =
        IndexingTaskProcessor::new(contract, Duration::from_secs(10), Duration::from_secs(2));

    let aggregator = Aggregator::new(config, task_processor)
        .await
        .map_err(|e| eyre::eyre!("Aggregator new error: {}", e))?;
    aggregator
        .start()
        .await
        .map_err(|e| eyre::eyre!("Aggregator start error: {}", e))?;

    Ok(())
}
