use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ContractsConfig {
    pub task_manager_addr: String,
    pub service_manager_addr: String,
    pub erc20_mock_strategy_addr: String,
}
