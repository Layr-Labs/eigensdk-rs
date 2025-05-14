use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for the [`Challenger`]
pub struct ChallengerConfig {
    /// The rpc url
    pub rpc_url: String,
    /// The websocket url
    pub ws_url: String,
}

impl ChallengerConfig {
    /// Create a new challenger config
    ///
    /// # Arguments
    ///
    /// * `rpc_url` - The rpc url
    /// * `ws_url` - The websocket url
    ///
    /// # Returns
    ///
    /// * `Self` - The challenger config
    pub fn new(rpc_url: String, ws_url: String) -> Self {
        Self { rpc_url, ws_url }
    }
}
