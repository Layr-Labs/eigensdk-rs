use alloy::primitives::Address;
use eigen_signer::SignerConfig;
use eigen_types::avs_state::OperatorSet;
use serde::{Deserialize, Serialize};

/// Operator registration config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegistrationConfig {
    /// Signer of the operator
    pub signer: SignerConfig,
    /// Metadata URI
    pub metadata_uri: Option<String>,
    /// Socket address
    pub socket: Option<String>,
    /// Allocation delay
    pub allocation_delay: Option<u32>,
    /// Allocation manager address
    pub allocation_manager_address: Option<Address>,
    /// Registry coordinator address
    pub registry_coordinator_address: Option<Address>,
    /// Delegation manager address
    pub delegation_manager_address: Option<Address>,
    /// Strategy manager addresses
    pub strategy_manager_address: Option<Address>,
    /// Operator sets
    pub operator_sets: Vec<OperatorSet>,
    /// Tokens to deposit
    pub deposits: Vec<DepositInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositInfo {
    /// Token address
    pub token_address: Address,
    /// Amount of tokens to deposit
    pub amount: String,
    /// Allocation magnitude
    pub allocation_magnitude: u64,
}
