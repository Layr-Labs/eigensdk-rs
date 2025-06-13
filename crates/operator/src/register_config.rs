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
    pub operator_set_ids: Option<Vec<u32>>,
    /// New magnitude to allocate
    pub new_magnitudes: Option<Vec<u64>>,
    /// Deposit tokens amount
    pub deposit_tokens_amounts: Option<Vec<String>>,
    /// Allocation manager address
    pub allocation_manager_address: Option<Address>,
    /// Registry coordinator address
    pub registry_coordinator_address: Option<Address>,
    /// Delegation manager address
    pub delegation_manager_address: Option<Address>,
    /// Strategy manager addresses
    pub strategy_manager_addresses: Option<Vec<Address>>,
    /// ERC20 strategy addresses
    pub erc20_strategy_addresses: Option<Vec<Address>>,
    /// AVS addresses
    pub avs_addresses: Option<Vec<Address>>,
}
