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

/// This example shows how to initialize a task spammer and start generating tasks
/// Follow the [`TaskSpammer`](eigensdk::task_spammer::TaskSpammerBuilder) documentation to set up a task spammer.
///
/// 1. Define your types for the task manager (Done in [`ISTaskManager`](incredible_squaring::ISTaskManager))
/// 2. Create the task manager instance from your bindings
/// 3. Build the [`TaskSpammerBuilder`]
/// 4. Define an iterator that creates appropriate input values for your specific AVS
/// 5. Set the quorum and the quorum threshold percentage
/// 6. Set the interval to wait between spamming tasks
/// 7. Build and run the task spammer
#[tokio::main]
async fn main() {
    let http_rpc_url = "http://localhost:8545".to_string();

    // 2. Create the task manager instance
    let signer = "0x2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6";
    let task_manager_address =
        Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3").unwrap();
    let url = Url::parse(&http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

    TaskSpammerBuilder::new(contract) // (3) Initialize the `TaskSpammerBuilder`
        .with_iter((0..).map(U256::from)) // (4) Define an iterator
        .with_quorum(50, vec![0]) // (5) Set the quorum and threshold percentage
        .with_interval(Duration::from_secs(10)) // (6) Set the interval to wait between spamming tasks
        .build() // (7) Build the `TaskSpammer`
        .unwrap()
        .run() // (7) Run the `TaskSpammer`
        .await
        .unwrap();
}
