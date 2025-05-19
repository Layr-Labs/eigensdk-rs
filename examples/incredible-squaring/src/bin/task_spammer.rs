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
    let signer = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
    let task_manager_address =
        Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3").unwrap();
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
