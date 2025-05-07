use alloy::primitives::{Address, U256};
use eigen_crypto_bls::BlsKeyPair;
use serde::{Deserialize, Serialize};

pub struct OperatorRegistrationConfig {
    pub bls_key_pair: BlsKeyPair,
    pub operator_pvt_key: Option<String>,
    pub ecdsa_keystore_path: String,
    pub ecdsa_keystore_password: String,
    pub rpc_url: String,
    pub metadata_uri: String,
    pub socket: String,
    pub allocation_delay: u32,
    pub operator_set_id: u32,
    pub new_magnitude: Vec<u64>,
    pub deposit_tokens: U256,
    pub permission_controller_address: Address,
    pub rewards_coordinator: Address,
    pub allocation_manager: Address,
    pub registry_coordinator_address: Address,
    pub delegation_manager_address: Address,
    pub avs_directory_address: Address,
    pub strategy_manager_address: Address,
    pub erc20_strategy_address: Address,
    pub avs: Address,
    pub strategies: Vec<Address>,
}
