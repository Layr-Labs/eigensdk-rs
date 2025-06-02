use alloy::primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EcdsaSignerConfig {
    PrivateKey(String),
    Keystore(String, String),
}

/// Operator registration config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegistrationConfig {
    /// Signer for the operator
    pub signer: EcdsaSignerConfig,
    /// Metadata URI
    pub metadata_uri: String,
    /// Socket address
    pub socket: String,
    /// Allocation delay
    pub allocation_delay: u32,
    /// Operator set ID
    pub operator_set_id: u32,
    /// New magnitude to allocate
    pub new_magnitude: Vec<u64>,
    /// Deposit tokens amount
    pub deposit_tokens: String,
    /// Permission controller address
    pub permission_controller_address: Address,
    /// Rewards coordinator address
    pub rewards_coordinator_address: Address,
    /// Allocation manager address
    pub allocation_manager_address: Address,
    /// Registry coordinator address
    pub registry_coordinator_address: Address,
    /// Delegation manager address
    pub delegation_manager_address: Address,
    /// AVS directory address
    pub avs_directory_address: Address,
    /// Strategy manager address
    pub strategy_manager_address: Address,
    /// ERC20 strategy address
    pub erc20_strategy_address: Address,
    /// AVS address
    pub avs_address: Address,
    /// Strategies addresses
    pub strategies_addresses: Vec<Address>,
}
