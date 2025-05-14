//! Incredible Dot Product Task Spammer

use alloy::primitives::Address;
use awesome_vault_service::bindings::awesomevaulttaskmanager::{
    AwesomeVaultTaskManager::AwesomeVaultTaskManagerInstance, IAwesomeVaultTaskManager::TaskInput,
};
use eigensdk::{
    common::get_signer,
    logging::{init_logger, log_level::LogLevel},
    task_spammer::TaskSpammerBuilder,
};
use eyre::Result;
use rand::Rng;

use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);
    let key = "4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";
    let http_rpc_url = "http://localhost:8545".to_string();
    let wallet = get_signer(key, &http_rpc_url);
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;

    let contract = AwesomeVaultTaskManagerInstance::new(task_manager_address, wallet);

    TaskSpammerBuilder::new(contract)
        .with_iter((1..).map(|_| {
            let random_key = format!("key_{}", rand::thread_rng().gen_range(0..1000000));
            let random_value = format!("value_{}", rand::thread_rng().gen_range(0..1000000));
            TaskInput {
                key: random_key.clone(),
                value: random_value.clone(),
            }
        }))
        .with_quorum(50, vec![0])
        .with_interval(Duration::from_secs(10))
        .build()
        .map_err(|e| eyre::eyre!("TaskSpammer build error: {}", e))?
        .run()
        .await
        .map_err(|e| eyre::eyre!("TaskSpammer run error: {}", e))?;

    Ok(())
}
