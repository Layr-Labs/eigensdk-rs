//! This example shows how to initialize a task spammer and start generating tasks
//! Follow the [`eigen-task-spammer` crate documentation](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/task-spammer/src/lib.rs#L1-L104)
//! to set up a task spammer.

use alloy::primitives::{Address, U256};
use eigensdk::{common::get_signer, task_spammer::TaskSpammerBuilder};
use eyre::Result;
use incredible_dot_product::{
    IIncredibleDotProductTaskManager::DotProductInput,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Define your types for the task manager (we do this in `ISTaskManager`: lib.rs)

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
        .run() // 7. Run the `TaskSpammer`
        .await
        .map_err(|e| eyre::eyre!("TaskSpammer run error: {}", e))?;

    Ok(())
}
