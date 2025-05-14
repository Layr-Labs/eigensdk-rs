//! Incredible Dot Product Challenger

use alloy::primitives::Address;
use eigensdk::{
    challenger::{
        challenger_processor::{verifier_from_compute_function, IndexingChallengerProcessor},
        config::ChallengerConfig,
        Challenger,
    },
    common::get_signer,
    logging::{init_logger, log_level::LogLevel},
    testing_utils::anvil_constants::FIRST_PRIVATE_KEY,
};
use eyre::Result;
use incredible_dot_product::{
    task_manager::dot_product,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);
    let http_rpc_url = "http://localhost:8545".to_string();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let wallet = get_signer(FIRST_PRIVATE_KEY, &http_rpc_url);
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;

    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);

    let verifier = verifier_from_compute_function(dot_product);
    let task_processor = IndexingChallengerProcessor::new(contract, verifier);
    let config = ChallengerConfig {
        http_rpc_url,
        ws_rpc_url,
    };
    let mut challenger = Challenger::new(config, task_processor);
    challenger
        .start_challenger()
        .await
        .map_err(|e| eyre::eyre!("Challenger start error: {}", e))?;

    Ok(())
}
