//! Task generator for Eigen Layer

pub mod task_generator;

use alloy::providers::ProviderBuilder;
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, U256},
    rpc::types::TransactionReceipt,
    signers::local::PrivateKeySigner,
};
use reqwest::Url;
use std::io::Error;
use std::str::FromStr;
use task_generator::TaskGenerator;
use tracing::info;

#[derive(Debug)]
pub struct TaskManager<TG: TaskGenerator> {
    rpc_url: String,
    signer: String,
    task_generator: TG,
    task_manager_address: Address,
}

impl<TG: TaskGenerator> TaskManager<TG> {
    /// New [`TaskManager`] instance
    pub fn new(
        rpc_url: String,
        signer: String,
        task_generator: TG,
        task_manager_address: Address,
    ) -> Self {
        Self {
            rpc_url,
            signer,
            task_generator,
            task_manager_address,
        }
    }

    /// Creates new task every 10 seconds
    pub async fn start(&self) {
        let url = Url::parse(&self.rpc_url).unwrap();
        let signer = PrivateKeySigner::from_str(&self.signer).unwrap();
        let wallet = EthereumWallet::new(signer);
        let pr = ProviderBuilder::new().wallet(wallet).on_http(url);
        let mut task_num: U256 = U256::from(1);

        let task_manager_contract = self
            .task_generator
            .build_task_manager(pr, self.task_manager_address);

        loop {
            self.task_generator.generate_task(&task_manager_contract);
            task_num += U256::from(1);
            info!("New task created");
        }
    }
}
