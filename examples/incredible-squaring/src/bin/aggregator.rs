#![allow(missing_docs)]

use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use eigen_aggregator::task_processor::IndexingTaskProcessor;
use eigen_aggregator::{Aggregator, AggregatorConfig};
use eigen_logging::get_logger;
use incredible_squaring::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use std::str::FromStr;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let registry_coordinator =
        Address::from_str("0x7969c5ed335650692bc04293b07f5bf2e7a673c0").unwrap();
    let operator_state_retriever =
        Address::from_str("0x1429859428c0abc9c2c47c8ee9fbaf82cfa0f20f").unwrap();
    let http_rpc_url = "http://localhost:8545".to_string();
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x742d35cc6634c0532925a3b844f51254ab06f58e").unwrap();
    let url = Url::parse(&http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let logger = get_logger();

    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);
    let task_processor =
        IndexingTaskProcessor::new(contract, Duration::from_secs(60), Duration::from_secs(15));

    let config = AggregatorConfig {
        server_address: "http://localhost:8080".to_string(),
        http_rpc_url: "http://localhost:8545".to_string(),
        ws_rpc_url: "ws://localhost:8545".to_string(),
        registry_coordinator,
        operator_state_retriever,
    };

    let aggregator = Aggregator::new(config, task_processor, logger)
        .await
        .unwrap();
    aggregator.start().await.unwrap();
}
