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
    task_manager::response_calculator::response_calculator_from_fn,
    testing_utils::anvil_constants::FIRST_PRIVATE_KEY,
};
use eyre::Result;
use incredible_dot_product::{
    task_manager::dot_product, utils::load_config,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use std::str::FromStr;

/// This example shows how to initialize a challenger and start processing tasks.
/// Follow the [`Challenger`] documentation to set up a challenger.
///
/// 1. Define your types for the task manager (Done in [`ISTaskManager`](incredible_dot_product::task_manager::ISTaskManager))
/// 2. Create the [`ChallengerConfig`]
/// 3. Define the task verification logic (Done in [`dot_product`])
/// 4. Instantiate the task manager instance from your bindings
/// 5. Build the [`ResponseCalculator`](eigensdk::task_manager::response_calculator::ResponseCalculator)
/// 6. Create the verifier with [`verifier_from_compute_function`]
/// 7. Initialize the [`IndexingChallengerProcessor`]
/// 8. Create and start the challenger
#[tokio::main]
async fn main() -> Result<()> {
    init_logger(LogLevel::Info);

    // 2. Create the `ChallengerConfig` from the toml file
    let config: ChallengerConfig = load_config("./src/config/dot-challenger.toml")?;

    // 4. Instantiate the task manager instance from your bindings
    let wallet = get_signer(FIRST_PRIVATE_KEY, &config.http_rpc_url);
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;
    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);

    // 5. Build the `ResponseCalculator` with the compute function
    let response_calculator = response_calculator_from_fn(dot_product);

    // 6. Create the verifier using the `ResponseCalculator`
    let verifier = verifier_from_compute_function(response_calculator);

    // 7. Initialize the `IndexingChallengerProcessor` with the contract and verifier
    let task_processor = IndexingChallengerProcessor::new(contract, verifier);

    // 8. Create and start the challenger
    let mut challenger = Challenger::new(config, task_processor);
    challenger
        .start_challenger()
        .await
        .map_err(|e| eyre::eyre!("Challenger start error: {}", e))?;

    Ok(())
}
