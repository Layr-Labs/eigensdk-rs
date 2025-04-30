use alloy::primitives::Address;
use eigen_aggregator::AggregatorConfig;
use serde::Deserialize;
use std::fs;

const CONFIG_PATH: &str = "examples/incredible-summation/config.toml";

/// Rpc config
#[derive(Deserialize, Debug, Default)]
pub struct RpcConfig {
    pub chain_id: u64,
    pub http_rpc_url: String,
    pub ws_rpc_url: String,
    pub signer: String,
}

/// Task manager config
#[derive(Deserialize, Debug, Default)]
pub struct TaskManagerConfig {
    pub signer: String,
}

/// Contracts addresses
#[derive(Deserialize, Debug, Default)]
pub struct ContractAddress {
    pub task_manager: Address,
    pub service_manager: Address,
    pub erc20_mock_strategy: Address,
    pub registry_coordinator: Address,
    pub operator_state_retriever: Address,
    pub delegation_manager: Address,
    pub avs_directory: Address,
    pub strategy_manager: Address,
    pub rewards_coordinator: Address,
    pub permission_controller: Address,
    pub allocation_manager: Address,
}

/// Config for the incredible summation example
#[derive(Deserialize, Debug, Default)]
pub struct Config {
    /// Agregator config
    pub aggregator_config: AggregatorConfig,
    /// Rpc config
    pub rpc_config: RpcConfig,
    /// Task manager config
    pub task_manager_config: TaskManagerConfig,
    /// Contracts addresses
    pub contracts_addresses: ContractAddress,
}

impl Config {
    pub fn new() -> Self {
        let s = fs::read_to_string(CONFIG_PATH).unwrap();
        let config: Config = toml::from_str(&s).unwrap();
        config
    }
}
