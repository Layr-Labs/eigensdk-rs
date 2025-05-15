#![allow(missing_docs)]

use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use eigen_aggregator::task_processor::IndexingTaskProcessor;
use eigen_aggregator::{Aggregator, AggregatorConfig};
use incredible_squaring::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use incredible_squaring::utils::load_config;
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let config: AggregatorConfig = load_config("./src/config/squaring-aggregator.toml").unwrap();
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x742d35cc6634c0532925a3b844f51254ab06f58e").unwrap();
    let url = Url::parse(&config.http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);
    let task_processor =
        IndexingTaskProcessor::new(contract, Duration::from_secs(60), Duration::from_secs(15));

    let aggregator = Aggregator::new(config, task_processor).await.unwrap();
    aggregator.start().await.unwrap();
}
