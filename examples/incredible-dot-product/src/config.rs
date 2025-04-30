use std::{fs, path::Path};

use alloy::primitives::Address;
use eigensdk::aggregator::AggregatorConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RpcConfig {
    pub chain_id: u64,
    pub http_rpc_url: String,
    pub ws_rpc_url: String,
    pub signer: String,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Config {
    pub aggregator_config: AggregatorConfig,
    pub contract_address: ContractAddress,
    pub rpc_config: RpcConfig,
}

impl Config {
    pub fn load_from<P: AsRef<Path>>(path: P) -> Config {
        let s = fs::read_to_string(path).unwrap();
        let config: Config = toml::from_str(&s).unwrap();
        config
    }
}
