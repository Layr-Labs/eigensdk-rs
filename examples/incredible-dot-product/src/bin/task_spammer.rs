//! Incredible Dot Product Task Spammer

use alloy::primitives::{Address, U256};
use eigensdk::{
    common::get_signer,
    logging::{init_logger, log_level::LogLevel},
    task_spammer::TaskSpammerBuilder,
};
use eyre::Result;
use incredible_dot_product::{
    IIncredibleDotProductTaskManager::DotProductInput,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use std::{str::FromStr, time::Duration};

/// This example shows how to initialize a task spammer and start generating tasks
/// Follow the [`TaskSpammer`](eigensdk::task_spammer::TaskSpammerBuilder) documentation to set up a task spammer.
///
/// 1. Define your types for the task manager (Done in [`ISTaskManager`](incredible_dot_product::task_manager::ISTaskManager))
/// 2. Create the task manager instance from your bindings
/// 3. Build the [`TaskSpammerBuilder`]
/// 4. Define an iterator that creates appropriate input values for your specific AVS
/// 5. Set the quorum and the quorum threshold percentage
/// 6. Set the interval to wait between spamming tasks
/// 7. Build and run the task spammer
#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);

    // 2. Create the task manager instance
    let key = "4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";
    let http_rpc_url = "http://localhost:8545".to_string();
    let wallet = get_signer(key, &http_rpc_url);
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;
    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);

    TaskSpammerBuilder::new(contract) // 3. Initialize the `TaskSpammerBuilder`
        .with_iter((0..).map(|i| DotProductInput {
            X: vec![U256::from(i); 4],
            Y: vec![U256::from(i * 2); 4],
        })) // 4. Define an iterator
        .with_quorum(50, vec![0]) // 5. Set the quorum and threshold percentage
        .with_interval(Duration::from_secs(10)) // 6. Set the interval to wait between spamming tasks
        .build() // 7. Build the `TaskSpammer`
        .map_err(|e| eyre::eyre!("TaskSpammer build error: {}", e))?
        .run() // 8. Run the `TaskSpammer`
        .await
        .map_err(|e| eyre::eyre!("TaskSpammer run error: {}", e))?;

    Ok(())
}
