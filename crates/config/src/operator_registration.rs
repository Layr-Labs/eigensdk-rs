use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OperatorRegistrationConfig {
    pub register_operator: bool,
    pub operator_to_avs_registration_sig_salt: String,
    pub socket: String,
    pub quorum_number: String,
    pub sig_expiry: String,
    /// Optional operator pvt key, if not provided, it will be taken from the [`EcdsaConfig`]
    pub operator_pvt_key: Option<String>,
}
