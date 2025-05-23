//! This example shows how to initialize an aggregator
//! Follow the [`eigen-aggregator` crate documentation](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/aggregator/src/lib.rs#L1-L84)
//! to set up an aggregator.

use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use eigensdk::aggregator::task_processor::IndexingTaskProcessor;
use eigensdk::aggregator::{Aggregator, AggregatorConfig};
use eigensdk::logging::get_logger;
use eigensdk::logging::init_logger;
use eigensdk::logging::log_level::LogLevel;
use incredible_squaring::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use incredible_squaring::utils::load_config;
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);

    // 1. Define your types for the task manager (we do this in `ISTaskManager`: lib.rs)

    // 2. Create the aggregator configuration from the toml file
    let config: AggregatorConfig = load_config("./src/config/squaring-aggregator.toml").unwrap();
    let logger = get_logger();

    // 3. Create the task manager instance
    let signer = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
    let task_manager_address =
        Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3").unwrap();
    let url = Url::parse(&config.http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

    // 4. Create the task processor
    let task_processor =
        IndexingTaskProcessor::new(contract, Duration::from_secs(60), Duration::from_secs(15));

    // 5. Create and start the aggregator
    let aggregator = Aggregator::new(config, task_processor, logger)
        .await
        .unwrap();
    aggregator.start().await.unwrap();
}
