use std::time::Duration;

use eigensdk::{aggregator::Aggregator, common::get_signer, task_processor::IndexingTaskProcessor};
use incredible_bindings::incredibledotproducttaskmanager::IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance;
use incredible_dot_product::{config::Config, TaskManagerWrapper};

#[tokio::main]
async fn main() {
    let config = Config::load_from("config.toml");
    let wallet = get_signer(&config.rpc_config.signer, &config.rpc_config.http_rpc_url);

    let contract =
        IncredibleDotProductTaskManagerInstance::new(config.contract_address.task_manager, wallet);
    let wrapper_contract = TaskManagerWrapper(contract);

    let task_processor = IndexingTaskProcessor::new(
        wrapper_contract,
        Duration::from_secs(10),
        Duration::from_secs(2),
    );

    let aggregator = Aggregator::new(config.aggregator_config, task_processor)
        .await
        .unwrap();
    aggregator.start().await.unwrap();
}
