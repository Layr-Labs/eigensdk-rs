/// Main Challenger struct
#[derive(Debug)]
pub struct Challenger {
    avs_writer: AvsWriter,
    ws_url: String,
    rpc_url: String,
    tasks: HashMap<u32, Task>,
    task_responses: HashMap<u32, TaskResponseData>,
    delegation_manager_address: Address,
    strategy_address: Address,
    operator_1_address: Address,
    operator_2_address: Address,
}

pub struct Challenger {
    service_manager_address: Address,
    rpc_url: String,
    ws_url: String,
    tasks: HashMap<u32, Task>,
    max_response_interval_blocks: u32,
    operator_address: Address,
}
