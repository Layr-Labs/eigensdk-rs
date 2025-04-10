use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use bindings::iincrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::IIncredibleSquaringTaskManagerInstance;
use eigen_task_generator::TaskGenerator;
use std::{str::FromStr, sync::Arc, time::Duration};
use tracing::info;

// Allow warnings in auto-generated code
#[allow(warnings)]
pub mod bindings;

#[tokio::main]
async fn main() {
    let http_rpc_url = "http://localhost:8545".to_string();
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x742d35cc6634c0532925a3b844f51254ab06f58e").unwrap();
    let url = Url::parse(&http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

    TaskGenerator::builder()
        .with_iter(0..10)
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(10))
        .run({
            let contract_arc = Arc::new(IIncredibleSquaringTaskManagerInstance::new(
                task_manager_address,
                provider,
            ));
            move |i, quorum_threshold, quorums| {
                let contract = Arc::clone(&contract_arc);
                async move {
                    let number_to_be_squared = U256::from(i * i);
                    contract
                        .createNewTask(
                            number_to_be_squared,
                            quorum_threshold.into(),
                            quorums.into(),
                        )
                        .send()
                        .await
                        .unwrap()
                        .get_receipt()
                        .await
                        .unwrap();

                    info!("Task {} created", i);
                    Ok(())
                }
            }
        })
        .await
        .unwrap();
}
