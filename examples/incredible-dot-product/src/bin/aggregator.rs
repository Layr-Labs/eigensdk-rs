//! Incredible Dot Product Aggregator

use alloy::primitives::Address;
use eigensdk::{
    aggregator::{task_processor::IndexingTaskProcessor, Aggregator, AggregatorConfig},
    common::get_signer,
    logging::{get_logger, init_logger, log_level::LogLevel},
};
use eyre::Result;
use incredible_dot_product::{
    utils::load_config, IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use std::{str::FromStr, time::Duration};

/// This example shows how to initialize an aggregator
/// Follow the [`Aggregator`] documentation to set up an aggregator.
/// The process can be split into 5 steps:
///
/// 1. Define your types for the task manager (Done in [`ISTaskManager`](incredible_dot_product::task_manager::ISTaskManager))
/// 2. Create the [`AggregatorConfig`]
/// 3. Instantiate the task manager instance from your bindings
/// 4. Create the task processor using the [`IndexingTaskProcessor`]
/// 5. Create and start the aggregator
#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);
    let logger = get_logger();

    // 2. Create the aggregator configuration from the toml file
    let config: AggregatorConfig = load_config("./src/config/dot-aggregator.toml")?;

    // 3. Instantiate the task manager instance from your bindings
    let wallet = get_signer(
        "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6",
        &config.http_rpc_url,
    );
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;
    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);

    // 4. Create the task processor
    let task_processor =
        IndexingTaskProcessor::new(contract, Duration::from_secs(10), Duration::from_secs(2));

    // 5. Create and start the aggregator
    let aggregator = Aggregator::new(config, task_processor, logger)
        .await
        .map_err(|e| eyre::eyre!("Aggregator new error: {}", e))?;
    aggregator
        .start()
        .await
        .map_err(|e| eyre::eyre!("Aggregator start error: {}", e))?;

    Ok(())
}
