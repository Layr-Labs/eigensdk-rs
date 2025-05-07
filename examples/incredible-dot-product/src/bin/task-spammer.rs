use alloy::primitives::{Address, U256};
use eigensdk::{
    common::get_signer,
    logging::{init_logger, log_level::LogLevel},
    task_spammer::TaskSpammerBuilder,
};
use incredible_bindings::incredibledotproducttaskmanager::{
    IIncredibleDotProductTaskManager::DotProductInput,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use incredible_dot_product::TaskManagerWrapper;
use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let key = "4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";
    let http_rpc_url = "http://localhost:8545".to_string();
    let wallet = get_signer(key, &http_rpc_url);
    let task_manager_address =
        Address::from_str("0x7bc06c482dead17c0e297afbc32f6e63d3846650").unwrap();

    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);
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
