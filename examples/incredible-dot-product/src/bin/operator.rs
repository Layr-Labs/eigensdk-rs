use alloy::primitives::U256;
use eigensdk::{
    crypto_bls::BlsKeyPair,
    logging::{get_logger, init_logger, log_level::LogLevel},
    operator::{config::OperatorConfig, error::OperatorError, Operator},
    task_processor::task_response::TaskResponse,
    testing_utils::anvil_constants::{FIRST_ADDRESS, OPERATOR_BLS_KEY},
};
use incredible_bindings::incredibledotproducttaskmanager::IncredibleDotProductTaskManager::NewTaskCreated;
use incredible_dot_product::config::Config;

#[tokio::main]
async fn main() {
    init_logger(LogLevel::Info);
    let config = Config::load_from("config.toml");
    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();
    let operator_address = FIRST_ADDRESS;
    let operator_name = "dot-product-god";
    let logger = get_logger();
    let ws_rpc_url = config.rpc_config.ws_rpc_url;
    let http_rpc_url = config.rpc_config.http_rpc_url;
    let registry_coordinator_address = config.contract_address.registry_coordinator;
    let operator_state_retriever_address = config.contract_address.operator_state_retriever;
    let aggregator_ip_port = config.aggregator_config.server_address;

    let operator_config = OperatorConfig {
        bls_key_pair,
        operator_address,
        operator_name: operator_name.to_string(),
        ws_rpc_url: ws_rpc_url.to_string(),
        http_rpc_url: http_rpc_url.to_string(),
        registry_coordinator_address,
        operator_state_retriever_address,
        aggregator_ip_port,
    };
    let operator = Operator::new(logger, operator_config).await.unwrap();

    // TODO: Review bounds in SDK. I have to derive Serialize and Deserialize for TaskResponse in the bindings
    operator.start(dot_product).await.unwrap();
}

/// Computes the dot product of a pair of points
///
/// # Arguments
///
/// * `event` - The event containing the task
///
/// # Returns
///
/// * `Result<TaskResponse<U256>, OperatorError>` - The task response
fn dot_product(event: NewTaskCreated) -> Result<TaskResponse<U256>, OperatorError> {
    let input = event.task.pointsToMultiply;

    let result = input
        .X
        .iter()
        .zip(input.Y.iter())
        .fold(U256::ZERO, |acc, (a, b)| acc + (*a) * (*b));

    Ok(TaskResponse {
        task_index: event.taskIndex,
        response: result,
    })
}

/// Computes an invalid dot product of a pair of points
/// This function is used to test the slashing mechanism when the operator returns a wrong response
///
/// # Arguments
///
/// * `event` - The event containing the task
///
/// # Returns
///
/// * `Result<TaskResponse<U256>, OperatorError>` - The wrong task response
#[allow(dead_code)]
fn invalid_dot_product(event: NewTaskCreated) -> Result<TaskResponse<U256>, OperatorError> {
    let result = U256::MAX;

    Ok(TaskResponse {
        task_index: event.taskIndex,
        response: result,
    })
}
