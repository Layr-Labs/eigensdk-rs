use std::{str::FromStr, sync::Arc, time::Duration};

use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::{Address, U256},
    providers::{
        fillers::{
            BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
            WalletFiller,
        },
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
    transports::http::reqwest::Url,
};

use bindings::iincrediblesquaringtaskmanager::IIncredibleSquaringTaskManager::IIncredibleSquaringTaskManagerInstance;

use async_trait::async_trait;
use eigen_task_generator::{error::TaskGeneratorError, TaskGenerator, TaskProcess};
use eigen_types::operator::{QuorumNum, QuorumThresholdPercentage};
use tracing::info;

// Allow warnings in auto-generated code
#[allow(warnings)]
pub mod bindings;

type NumberToSquare = u32;

type DefaultTransport = ();
type DefaultNetwork = Ethereum;
type DefaultProvider = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider,
>;
type DefaultContract =
    IIncredibleSquaringTaskManagerInstance<DefaultTransport, DefaultProvider, DefaultNetwork>;

pub struct TaskGeneratorImpl {
    task_manager_contract: DefaultContract,
}

impl TaskGeneratorImpl {
    pub fn new(task_manager_address: Address, signer: String, rpc_url: String) -> Self {
        let url = Url::parse(&rpc_url).expect("Wrong rpc url");
        let pk_signer = PrivateKeySigner::from_str(&signer).expect("Wrong signer");
        let wallet = EthereumWallet::new(pk_signer);
        let pr = ProviderBuilder::new().wallet(wallet).on_http(url);

        let task_manager_contract =
            IIncredibleSquaringTaskManagerInstance::new(task_manager_address, pr);

        Self {
            task_manager_contract,
        }
    }
}

// 1. Implement TaskProcess trait for TaskGeneratorImpl
#[async_trait]
impl TaskProcess<NumberToSquare> for TaskGeneratorImpl {
    async fn create_new_task(
        &self,
        task_index: u32,
        input: NumberToSquare,
        quorum_threshold: QuorumThresholdPercentage,
        quorums: Vec<QuorumNum>,
    ) -> Result<(), TaskGeneratorError> {
        self.task_manager_contract
            .createNewTask(U256::from(input), quorum_threshold.into(), quorums.into())
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();

        info!("Task created with index: {}", task_index);
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let http_rpc_url = "http://localhost:8545".to_string();
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x742d35cc6634c0532925a3b844f51254ab06f58e").unwrap();

    let task_generator = Arc::new(TaskGeneratorImpl::new(
        task_manager_address,
        signer.to_string(),
        http_rpc_url,
    ));

    TaskGenerator::builder()
        .with_iter(0..10)
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(10))
        .run(|i, quorum_threshold, quorums| {
            let task_generator = task_generator.clone();
            async move {
                // This should create a new task with the number to square
                task_generator
                    .create_new_task(i, i, quorum_threshold, quorums)
                    .await
            }
        })
        .await
        .unwrap();
}
