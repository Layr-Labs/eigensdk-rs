use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default)]
pub struct EcdsaConfig {
    pub keystore_path: String,
    pub keystore_password: String,
}
