//! Incredible Dot Product Task Spammer

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
    let key = "4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356";
    let http_rpc_url = "http://localhost:8545".to_string();
    let wallet = get_signer(key, &http_rpc_url);
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;

    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);

    TaskSpammerBuilder::new(contract)
        .with_iter((0..).map(|i| DotProductInput {
            X: vec![U256::from(i); 4],
            Y: vec![U256::from(i * 2); 4],
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
