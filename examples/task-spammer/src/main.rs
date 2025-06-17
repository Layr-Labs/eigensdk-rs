use alloy::transports::Transport;
use alloy::{contract::private::Provider, network::Network};
use alloy::{
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};
use bindings::iincrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::IIncredibleSquaringTaskManagerInstance;
use eigen_task_spammer::{
    error::TaskSpammerError, task_manager::TaskManagerContract, TaskSpammerBuilder,
};
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use std::{str::FromStr, time::Duration};

// 1. Implement the TaskManagerContract trait for the task manager contract.
// You need to specify the input type of the task. In this case, U256.
// You also need to specify the call type of the task manager contract. `createNewTask` uses `createNewTaskCall`.
// You also need to specify the provider and network types.
//
// NOTE: When you are implementing this, you will have an exteranl trait `TaskManagerContract` and and external struct
// `CONTRACT_NAME_INSTANCE`, so it will throw an error. You can wrap the external struct in a newtype to avoid this.
// Example:
// struct TaskManagerWrapper<T, P, N>(IncredibleSquaringTaskManagerInstance<T, P, N>);
//
// impl<T, P, N> TaskManagerContract<U256, T, P, N> for TaskManagerWrapper<T, P, N> { ... }
impl<T, P, N> TaskManagerContract<U256, T, P, N> for IIncredibleSquaringTaskManagerInstance<T, P, N>
where
    T: Transport + Clone + Send + Sync,
    P: Provider<N>,
    N: Network,
{
    async fn create_new_task(
        &self,
        input: U256,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> Result<N::ReceiptResponse, TaskSpammerError> {
        Ok(self
            .createNewTask(input, quorum_threshold.into(), quorums.into())
            .send()
            .await?
            .get_receipt()
            .await?)
    }
}

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

    let contract = IIncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

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
