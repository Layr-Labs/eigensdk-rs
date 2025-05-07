use super::operator_register_config::OperatorRegistrationConfig;
use alloy::primitives::Address;
use eigen_crypto_bls::BlsKeyPair;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Operator configuration struct
pub struct OperatorConfig {
    /// BLS key pair of the operator
    pub bls_key_pair: BlsKeyPair,
    /// Address of the operator
    pub operator_address: Address,
    /// Name of the operator
    /// This is used for logging and debugging purposes.
    pub operator_name: String,
    /// Ethereum WebSocket RPC URL
    pub ws_rpc_url: String,
    /// Ethereum HTTP RPC URL
    pub http_rpc_url: String,
    /// Address of the registry coordinator
    /// Used to check the operator is registered.
    pub registry_coordinator_address: Address,
    /// Address of the operator state retriever
    pub operator_state_retriever_address: Address,
    /// IP and port of the aggregator
    pub aggregator_ip_port: String,

    pub registration: Option<OperatorRegistrationConfig>,
}
