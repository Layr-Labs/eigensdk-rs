use alloy::primitives::Address;
use eigen_signer::SignerConfig;
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
    /// Operator set ID
    pub operator_set_id: Option<u32>,
    /// New magnitude to allocate
    pub new_magnitude: Vec<u64>,
    /// Deposit tokens amount
    pub deposit_tokens: Option<String>,
    /// Allocation manager address
    pub allocation_manager_address: Option<Address>,
    /// Registry coordinator address
    pub registry_coordinator_address: Option<Address>,
    /// Delegation manager address
    pub delegation_manager_address: Option<Address>,
    /// AVS directory address
    pub avs_directory_address: Option<Address>,
    /// Strategy manager address
    pub strategy_manager_address: Option<Address>,
    /// ERC20 strategy address
    pub erc20_strategy_address: Option<Address>,
    /// AVS address
    pub avs_address: Option<Address>,
    /// Strategies addresses
    pub strategies_addresses: Option<Vec<Address>>,
}
