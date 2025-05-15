use alloy::primitives::{Address, U256};
use serde::{Deserialize, Serialize};

/// Operator registration config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegistrationConfig {
    /// Private key of the operator
    pub operator_pvt_key: Option<String>,
    /// Path to the ECDSA keystore file
    pub ecdsa_keystore_path: String,
    /// Password for the ECDSA keystore file
    pub ecdsa_keystore_password: String,
    /// HTTP RPC URL
    pub http_rpc_url: String,
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
    pub deposit_tokens: U256,
    /// Permission controller address
    pub permission_controller_address: Address,
    /// Rewards coordinator address
    pub rewards_coordinator: Address,
    /// Allocation manager address
    pub allocation_manager: Address,
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
    pub avs: Address,
    /// Strategies addresses
    pub strategies: Vec<Address>,
}
