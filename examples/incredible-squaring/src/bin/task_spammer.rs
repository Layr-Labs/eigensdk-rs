use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use eigensdk::task_spammer::TaskSpammerBuilder;
use incredible_squaring::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;
use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() {
    let http_rpc_url = "http://localhost:8545".to_string();
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x742d35cc6634c0532925a3b844f51254ab06f58e").unwrap();
    let url = Url::parse(&http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

    TaskSpammerBuilder::new(contract)
        .with_iter((0..).map(U256::from))
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(10))
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();
}
