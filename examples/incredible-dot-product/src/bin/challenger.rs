use std::str::FromStr;

use alloy::primitives::{Address, U256};
use eigensdk::{
    challenger::{
        challenger_processor::IndexingChallengerProcessor, error::ChallengerError, Challenger,
    },
    common::get_signer,
    logging::{init_logger, log_level::LogLevel},
    task_processor::{task::Task, task_response::TaskResponse},
    testing_utils::anvil_constants::FIRST_PRIVATE_KEY,
};
use incredible_bindings::incredibledotproducttaskmanager::{
    IIncredibleDotProductTaskManager::DotProductInput,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use incredible_dot_product::TaskManagerWrapper;

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let http_rpc_url = "http://localhost:8545".to_string();
    let ws_rpc_url = "ws://localhost:8545".to_string();
    let wallet = get_signer(FIRST_PRIVATE_KEY, &http_rpc_url);
    let task_manager_address =
        Address::from_str("0x7bc06c482dead17c0e297afbc32f6e63d3846650").unwrap();

    let contract = IncredibleDotProductTaskManagerInstance::new(task_manager_address, wallet);
    let contract_wrapper = TaskManagerWrapper(contract);

    let task_processor = IndexingChallengerProcessor::new(contract_wrapper, is_response_correct);
    let mut challenger = Challenger::new(http_rpc_url, ws_rpc_url, task_processor);
    challenger.start_challenger().await.unwrap();
}

/// Checks if the operator response is correct
///
/// # Arguments
///
/// * `task` - The task
/// * `task_response` - The task response
///
/// # Returns
///
/// * `Result<bool, ChallengerError>` - The result of the operation
fn is_response_correct(
    task: Task<DotProductInput>,
    task_response: TaskResponse<U256>,
) -> Result<bool, ChallengerError> {
    let input = task.input;

    let result = input
        .X
        .iter()
        .zip(input.Y.iter())
        .fold(U256::ZERO, |acc, (a, b)| acc + (*a) * (*b));

    Ok(result == task_response.response)
}
