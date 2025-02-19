use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct RpcConfig {
    pub chain_id: u16,
    pub http_rpc_url: String,
    pub ws_rpc_url: String,
    pub signer: String,
}
