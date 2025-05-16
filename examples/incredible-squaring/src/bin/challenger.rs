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

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let config: ChallengerConfig = load_config("./src/config/squaring-challenger.toml").unwrap();
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x2bdcc0de6be1f7d2ee689a0342d76f52e8efaba3").unwrap();
    let url = Url::parse(&config.http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);

    let response_calculator = response_calculator_from_fn(square);

    let logic = verifier_from_compute_function(response_calculator);

    let task_processor = IndexingChallengerProcessor::new(contract, logic);

    let mut challenger = Challenger::new(config, task_processor);
    challenger.start_challenger().await.unwrap();
}
