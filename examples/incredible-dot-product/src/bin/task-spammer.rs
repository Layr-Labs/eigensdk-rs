use std::time::Duration;

use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
    primitives::U256,
};
use eigensdk::{
    common::get_signer,
    task_spammer::{
        error::TaskSpammerError, task_manager::TaskManagerContract, TaskSpammerBuilder,
    },
    types::operator::{QuorumNum, QuorumThresholdPercentage},
};
use incredible_bindings::incrediblesquaringtaskmanager::{
    IIncredibleSquaringTaskManager::DotProductInput,
    IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance,
};
use incredible_dot_product::config::Config;

#[tokio::main]
async fn main() {
    let config = Config::load_from("config.toml");
    let wallet = get_signer(&config.rpc_config.signer, &config.rpc_config.http_rpc_url);

    let contract =
        IncredibleSquaringTaskManagerInstance::new(config.contract_address.task_manager, wallet);
    let wrapper = TaskManagerWrapper(contract);

    TaskSpammerBuilder::new(wrapper)
        .with_iter((0..).map(|i| DotProductInput {
            a: vec![U256::from(i); 4],
            b: vec![U256::from(i * 2); 4],
        }))
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(10))
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();
}
