use alloy::primitives::Address;
use eigen_signer::SignerConfig;
use serde::{Deserialize, Serialize};

/// Operator registration config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRegistrationConfig {
    /// Signer of the operator
    pub signer: SignerConfig,
    /// Operator config for EigenLayer registration
    /// Hacer option esto
    pub operator_global_config: Option<OperatorELConfig>,
    /// AVS registration configs
    pub avs_registration_configs: Vec<AvsRegistrationConfig>,
}

/// TODO: Improve name
/// This configuration is used to register the operator to EigenLayer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorELConfig {
    /// Metadata URI
    pub metadata_uri: String,
    /// Allocation delay
    pub allocation_delay: u32,
    /// Delegation manager address
    pub delegation_manager_address: Address,
}

// TODO: Improve name
/// This configuration is used to register the operator to an AVS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvsRegistrationConfig {
    /// AVS address
    pub avs_address: Address,
    /// Socket address for this AVS
    pub socket: Option<String>,
    /// Allocation manager address
    pub allocation_manager_address: Option<Address>,
    /// Registry coordinator address
    pub registry_coordinator_address: Option<Address>,
    /// Strategy manager address
    pub strategy_manager_address: Address,
    /// Operator sets for this AVS
    pub operator_sets: Vec<OperatorSet>,
    /// Deposits for this AVS
    pub deposits: Vec<DepositInfo>,
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

///An operator set identified by the AVS address and an identifier
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorSet {
    /// The unique identifier for the operator set
    pub id: u32,
    /// The address of the AVS this operator set belongs to
    pub avs_address: Address,
}

impl From<OperatorSet>
    for eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet
{
    fn from(operator_set: OperatorSet) -> Self {
        eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet {
            id: operator_set.id,
            avs: operator_set.avs_address,
        }
    }
}
