use std::str::FromStr;

use alloy::network::EthereumWallet;
use alloy::primitives::Address;
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use eigensdk::challenger::{
    challenger_processor::{verifier_from_compute_function, IndexingChallengerProcessor},
    config::ChallengerConfig,
    Challenger,
};
use eigensdk::logging::init_logger;
use eigensdk::logging::log_level::LogLevel;
use eigensdk::task_manager::response_calculator::response_calculator_from_fn;
use incredible_squaring::{
    bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance,
    square, utils::load_config,
};

/// This example shows how to initialize a challenger and start processing tasks.
/// Follow the [`Challenger`] documentation to set up a challenger.
///
/// 1. Define your types for the task manager (Done in [`ISTaskManager`](incredible_squaring::ISTaskManager))
/// 2. Create the [`ChallengerConfig`]
/// 3. Define the task verification logic (Done in [`square`])
/// 4. Initialize the task manager instance from your bindings
/// 5. Build the [`ResponseCalculator`](eigensdk::task_manager::response_calculator::ResponseCalculator)
/// 6. Create the verifier with [`verifier_from_compute_function`]
/// 7. Create the [`IndexingChallengerProcessor`]
/// 8. Create and start the challenger
#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);

    // 2. Create the `ChallengerConfig`
    let config: ChallengerConfig = load_config("./src/config/squaring-challenger.toml").unwrap();

    // 4. Initialize the task manager instance
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3").unwrap();
    let url = Url::parse(&config.http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);
    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

    // 5. Build the `ResponseCalculator` with the computation function
    let response_calculator = response_calculator_from_fn(square);

    // 6. Create the verifier using the `ResponseCalculator`
    let logic = verifier_from_compute_function(response_calculator);

    // 7. Create the `IndexingChallengerProcessor`
    let task_processor = IndexingChallengerProcessor::new(contract, logic);

    // 8. Create and start the challenger
    let mut challenger = Challenger::new(config, task_processor);
    challenger.start_challenger().await.unwrap();
}
