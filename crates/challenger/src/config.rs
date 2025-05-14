use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for the [`Challenger`]
pub struct ChallengerConfig {
    /// The rpc url
    pub rpc_url: String,
    /// The websocket url
    pub ws_url: String,
}
