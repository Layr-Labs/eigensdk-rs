use std::str::FromStr;

use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::ProviderBuilder;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::reqwest::Url;
use eigen_challenger::{
    challenger_processor::IndexingChallengerProcessor, error::ChallengerError, Challenger,
};
use eigen_task_processor::{task::Task, task_response::TaskResponse};
use incredible_squaring::bindings::incrediblesquaringtaskmanager::IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance;

fn is_response_correct(
    task: Task<U256>,
    task_response: TaskResponse<U256>,
) -> Result<bool, ChallengerError> {
    Ok(task.input * task.input == task_response.response)
}

#[tokio::main]
async fn main() {
    let http_rpc_url = "http://localhost:8545".to_string();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let signer = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let task_manager_address =
        Address::from_str("0x742d35cc6634c0532925a3b844f51254ab06f58e").unwrap();
    let url = Url::parse(&http_rpc_url).unwrap();
    let wallet = EthereumWallet::new(PrivateKeySigner::from_str(signer).unwrap());
    let provider = ProviderBuilder::new().wallet(wallet).on_http(url);

    let contract = IncredibleSquaringTaskManagerInstance::new(task_manager_address, provider);
    let task_processor = IndexingChallengerProcessor::new(contract, is_response_correct);

    let mut challenger = Challenger::new(http_rpc_url, ws_rpc_url, task_processor);
    challenger.start_challenger().await.unwrap();
}
