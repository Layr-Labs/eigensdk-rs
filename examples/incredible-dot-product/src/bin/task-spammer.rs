use alloy::primitives::U256;
use eigensdk::{common::get_signer, task_spammer::TaskSpammerBuilder};
use incredible_bindings::incredibledotproducttaskmanager::{
    IIncredibleDotProductTaskManager::DotProductInput,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use incredible_dot_product::{config::Config, TaskManagerWrapper};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let config = Config::load_from("config.toml");
    let wallet = get_signer(&config.rpc_config.signer, &config.rpc_config.http_rpc_url);

    let contract =
        IncredibleDotProductTaskManagerInstance::new(config.contract_address.task_manager, wallet);
    let wrapper = TaskManagerWrapper(contract);

    TaskSpammerBuilder::new(wrapper)
        .with_iter((0..).map(|i| DotProductInput {
            X: vec![U256::from(i); 4],
            Y: vec![U256::from(i * 2); 4],
        }))
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(10))
        .build()
        .unwrap()
        .run()
        .await
        .unwrap();
}
