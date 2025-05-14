use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for the [`Challenger`]
pub struct ChallengerConfig {
    /// The rpc url
    rpc_url: String,
    /// The websocket url
    ws_url: String,
}

impl Default for ChallengerConfig {
    /// Default challenger config with localhost rpc and ws urls values
    ///
    /// # Returns
    ///
    /// * `Self` - The default challenger config
    fn default() -> Self {
        Self {
            rpc_url: "http://localhost:8545".to_string(),
            ws_url: "ws://localhost:8545".to_string(),
        }
    }
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
