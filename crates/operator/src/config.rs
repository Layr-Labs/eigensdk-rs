use alloy::primitives::Address;
use eigen_crypto_bls::BlsKeyPair;

#[derive(Debug, Clone)]
/// Operator configuration struct
pub struct OperatorConfig {
    pub bls_key_pair: BlsKeyPair,
    pub operator_address: Address,
    pub operator_name: String,
    pub ws_rpc_url: String,
    pub http_rpc_url: String,
    pub registry_coordinator_address: Address,
    pub operator_state_retriever_address: Address,
    pub aggregator_ip_port: String,
}
