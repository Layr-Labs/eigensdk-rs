use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SignerConfig {
    PrivateKey {
        private_key_hex: String,
    },

    Keystore {
        path: String,
        password: String,
    },

    /// Firma vía Web3Signer (JSON-RPC a un nodo remoto).
    Web3 {
        /// URL del endpoint HTTP(S) donde está el Web3Signer.
        endpoint: Url,
        address: Address,
    },

    Aws {
        key_id: String,
        chain_id: Option<u64>,
        access_key: String,
        secret_access_key: String,
        region: String,
        endpoint_url: String,
    },
}
