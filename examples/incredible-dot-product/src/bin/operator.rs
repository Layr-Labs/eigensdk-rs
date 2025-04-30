use alloy::primitives::U256;
use eigensdk::{
    crypto_bls::BlsKeyPair,
    logging::{get_logger, init_logger, log_level::LogLevel},
    operator::{error::OperatorError, Operator},
    testing_utils::anvil_constants::{FIRST_ADDRESS, OPERATOR_BLS_KEY},
};
use incredible_bindings::incrediblesquaringtaskmanager::{
    IIncredibleSquaringTaskManager::{DotProductResult, TaskResponse},
    IncredibleSquaringTaskManager::NewTaskCreated,
};
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

    let operator = Operator::new(
        &bls_key_pair,
        operator_address,
        operator_name,
        logger,
        &ws_rpc_url,
        &http_rpc_url,
        registry_coordinator_address,
        operator_state_retriever_address,
        aggregator_ip_port,
    )
    .await
    .unwrap();

    // TODO: Review bounds in SDK. I have to derive Serialize and Deserialize for TaskResponse in the bindings
    // FIX: Some methods related to the operator are <Response> or <SignedResponse<Response>>
    //      and needs to be updated to <TaskResponse<Response>> and SignedResponse<TaskResponse<Response>>
    //      This will break when sending the RPC request to the aggregator
    operator.start(dot_product).await.unwrap();
}

fn dot_product(event: NewTaskCreated) -> Result<TaskResponse, OperatorError> {
    let result = DotProductResult {
        resultHigh: U256::ONE,
        resultLow: U256::ONE,
    };

    Ok(TaskResponse {
        referenceTaskIndex: event.taskIndex,
        output: result,
    })
}
