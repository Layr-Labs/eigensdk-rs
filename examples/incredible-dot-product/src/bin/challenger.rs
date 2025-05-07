use alloy::primitives::U256;
use eigensdk::{
    challenger::{
        challenger_processor::IndexingChallengerProcessor, error::ChallengerError, Challenger,
    },
    common::get_signer,
    task_processor::{task::Task, task_response::TaskResponse},
};
use incredible_bindings::incredibledotproducttaskmanager::{
    IIncredibleDotProductTaskManager::DotProductInput,
    IncredibleDotProductTaskManager::IncredibleDotProductTaskManagerInstance,
};
use incredible_dot_product::{config::Config, TaskManagerWrapper};

#[tokio::main]
async fn main() {
    let config = Config::load_from("config.toml");
    let wallet = get_signer(&config.rpc_config.signer, &config.rpc_config.http_rpc_url);

    let contract =
        IncredibleDotProductTaskManagerInstance::new(config.contract_address.task_manager, wallet);
    let contract_wrapper = TaskManagerWrapper(contract);

    let task_processor = IndexingChallengerProcessor::new(contract_wrapper, is_response_correct);
    let mut challenger = Challenger::new(
        config.rpc_config.ws_rpc_url.to_string(),
        config.rpc_config.http_rpc_url.to_string(),
        task_processor,
    );
    challenger.start_challenger().await.unwrap();
}

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
