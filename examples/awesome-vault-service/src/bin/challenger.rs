//! This example shows how to initialize a challenger and start processing tasks.
//! Follow the [`eigen-challenger` crate documentation](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/crates/challenger/src/lib.rs#L1-L112)
//! to set up a challenger.

use alloy::primitives::Address;
use awesome_vault_service::{
    bindings::awesome_vault_task_manager::AwesomeVaultTaskManager::AwesomeVaultTaskManagerInstance,
    response_calculator::VaultServiceResponseCalculator, utils::load_config,
};
use eigensdk::{
    challenger::{
        challenger_processor::{verifier_from_compute_function, IndexingChallengerProcessor},
        config::ChallengerConfig,
        Challenger,
    },
    common::get_signer,
    testing_utils::anvil_constants::FIRST_PRIVATE_KEY,
};
use eyre::Result;
use std::{collections::BTreeMap, str::FromStr, sync::Arc};
use tokio::sync::Mutex;

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

    // 2. Create the `ChallengerConfig`
    let config: ChallengerConfig = load_config("./src/config/awesome-challenger.toml")?;

    // 3. Create the logic to compute the task (we do this in `compute_vault_root`: lib.rs)

    // 4. Instantiate the task manager instance from your bindings
    let wallet = get_signer(FIRST_PRIVATE_KEY, &config.http_rpc_url);
    let task_manager_address = Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3")?;
    let contract = AwesomeVaultTaskManagerInstance::new(task_manager_address, wallet);

    // 5. Build the `VaultServiceResponseCalculator`, which implements the `ResponseCalculator` trait
    let vault_service_response_calculator = VaultServiceResponseCalculator {
        vault: Arc::new(Mutex::new(BTreeMap::new())),
    };

    // 6. Create the verifier using the `VaultServiceResponseCalculator`
    let is_response_correct = verifier_from_compute_function(vault_service_response_calculator);

    // 7. Initialize the `IndexingChallengerProcessor` with the contract and the verifier
    let task_processor = IndexingChallengerProcessor::new(contract, is_response_correct);

    // 8. Create and start the challenger
    let challenger = Challenger::new(config, task_processor);
    challenger
        .run()
        .await
        .map_err(|e| eyre::eyre!("Challenger start error: {}", e))?;

    Ok(())
}
