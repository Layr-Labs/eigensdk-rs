//! This example shows how to initialize a challenger and start processing tasks.
//! Follow the [`eigen-challenger` crate documentation](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/challenger/src/lib.rs#L1-L112)
//! to set up a challenger.

use alloy::primitives::Address;
use eigensdk::{
    challenger::{
        challenger_processor::{verifier_from_compute_function, IndexingChallengerProcessor},
        config::ChallengerConfig,
        Challenger,
    },
    common::get_signer,
    task_manager::response_calculator::response_calculator_from_fn,
    testing_utils::anvil_constants::FIRST_PRIVATE_KEY,
};
use eyre::Result;
use incredible_dot_product::{
    task_manager::dot_product, utils::load_config,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(
        tracing_subscriber::fmt::Subscriber::builder()
            .with_max_level(Level::INFO)
            .with_ansi(false)
            .finish(),
    )
    .unwrap();

    // 1. Define your types for the task manager (we do this in `ISTaskManager`: lib.rs)

    // 2. Create the `ChallengerConfig` from the toml file
    let config: ChallengerConfig = load_config("./src/config/dot-challenger.toml")?;

    // 3. Create the logic to compute the task (we do this in `dot_product`: lib.rs)

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
    let challenger = Challenger::new(config, task_processor);
    challenger
        .run()
        .await
        .map_err(|e| eyre::eyre!("Challenger start error: {}", e))?;

    Ok(())
}
