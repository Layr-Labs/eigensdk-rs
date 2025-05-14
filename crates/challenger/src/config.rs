use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Configuration for the [`Challenger`]
pub struct ChallengerConfig {
    /// HTTP RPC URL
    pub http_rpc_url: String,
    /// WebSocket RPC URL
    pub ws_rpc_url: String,
}
