use alloy::{
    consensus::Transaction,
    contract::private::{Provider as PrivateProvider, Transport},
    network::Network,
    primitives::U256,
    providers::Provider,
    sol_types::SolCall,
};
use eigensdk::{
    challenger::{
        challenger::ChallengerTaskProcessor, challenger_processor::IndexingChallengerProcessor,
        Challenger,
    },
    common::{get_provider, get_signer},
    task_processor::{task::Task, task_response::TaskResponse},
};
use incredible_bindings::incredibledotproducttaskmanager::{
    IIncredibleDotProductTaskManager::{DotProductInput, Task, TaskResponse, TaskResponseMetadata},
    IncredibleDotProductTaskManager::{
        respondToTaskCall, IncredibleDotProductTaskManagerInstance, NewTaskCreated, TaskResponded,
    },
    BN254::G1Point,
};
use incredible_dot_product::config::Config;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = Config::load_from("config.toml");
    let wallet = get_signer(&config.rpc_config.signer, &config.rpc_config.http_rpc_url);

    let contract =
        IncredibleDotProductTaskManagerInstance::new(config.contract_address.task_manager, wallet);
    let task_processor = IndexingChallengerProcessor::new(contract, is_response_correct);
    let mut challenger = Challenger::new(
        config.rpc_config.ws_rpc_url.to_string(),
        config.rpc_config.http_rpc_url.to_string(),
        task_processor,
    );
    challenger.start_challenger().await.unwrap();
}

fn is_response_correct(task: Task<DotProductInput>, task_response: TaskResponse<U256>) -> bool {
    let input = task.input;

    let result = input
        .X
        .iter()
        .zip(input.Y.iter())
        .fold(U256::ZERO, |acc, (a, b)| acc + (*a) * (*b));

    result == task_response.response
}
