use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(default)]
/// Configuration for the Aggregator
pub struct AggregatorConfig {
    /// IP address and port the aggregation server will use
    pub server_address: String,

    /// URL of the Ethereum HTTP RPC
    pub http_rpc_url: String,
    /// URL of the Ethereum WebSocket RPC
    pub ws_rpc_url: String,

    /// Address of the RegistryCoordinator contract
    pub registry_coordinator: Address,
    /// Address of the OperatorStateRetriever contract
    pub operator_state_retriever: Address,
}
