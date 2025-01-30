use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, PartialEq, Eq, Serialize)]
#[serde(default)]
pub struct BlsConfig {
    pub keystore_path: String,
    pub keystore_password: String,
    pub keystore_2_path: String,
    pub keystore_2_password: String,
}
