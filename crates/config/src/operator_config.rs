use serde::{Deserialize, Serialize};

// TODO fix this!!!
#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct OperatorConfig {
    pub operator_address: String,
    pub operator_id: String,
    pub operator_2_address: String,
    pub operator_2_id: String,
}
