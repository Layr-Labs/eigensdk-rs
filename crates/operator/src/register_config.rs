use alloy::primitives::Address;
use eigen_signer::SignerConfig;
use eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet;
use serde::{Deserialize, Serialize};

/// Operator registration config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegistrationConfig {
    /// Signer of the operator
    pub signer: SignerConfig,
    /// Operator config for EigenLayer registration
    pub operator_global_config: OperatorELConfig,
    /// AVS registration config
    pub avs_registration_config: AvsRegistrationConfig,
}

/// This configuration is used to register the operator to EigenLayer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorELConfig {
    /// Metadata URI
    pub metadata_uri: Option<String>,
    /// Allocation delay
    pub allocation_delay: Option<u32>,
    /// Delegation manager address
    pub delegation_manager_address: Option<Address>,
}

/// This configuration is used to register the operator to an AVS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvsRegistrationConfig {
    /// AVS address
    pub avs_address: Address,
    /// Operator set IDs for this AVS
    pub operator_set_configs: Vec<OperatorSetConfig>,
    /// Socket address for this AVS
    pub socket: Option<String>,
    /// Allocation manager address
    pub allocation_manager_address: Option<Address>,
    /// Registry coordinator address
    pub registry_coordinator_address: Option<Address>,
    /// Strategy manager address
    pub strategy_manager_address: Option<Address>,
}

/// Operator set configuration for an AVS with its deposits
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorSetConfig {
    /// Operator set ID
    pub id: u32,
    /// Deposits for this operator set
    pub deposits: Vec<DepositInfo>,
}

impl OperatorSetConfig {
    /// Create an operator set from the AVS address and the operator set ID
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the operator set
    ///
    /// # Returns
    ///
    /// * `OperatorSet` - The operator set
    pub fn operator_set(&self, avs_address: Address) -> OperatorSet {
        OperatorSet {
            id: self.id,
            avs: avs_address,
        }
    }
}

/// Deposit information for an AVS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositInfo {
    /// Strategy address
    pub strategy_address: Address,
    /// Amount of tokens to deposit
    pub amount: String,
    /// Amount of shares to allocate
    pub allocation_magnitude: u64,
}
