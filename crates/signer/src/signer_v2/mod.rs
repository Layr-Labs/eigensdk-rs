use alloy::{
    network::TxSigner,
    primitives::Address,
    signers::{
        aws::AwsSigner,
        local::{LocalSigner, PrivateKeySigner},
        Signature,
    },
};
use aws_config::BehaviorVersion;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

use crate::web3_signer::Web3Signer;

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
    Web3 {
        endpoint: String,
        address: Address,
    },
    Aws {
        key_id: String,
        chain_id: Option<u64>,
    },
}

pub async fn tx_signer_from_config(config: SignerConfig) -> impl TxSigner<Signature> {
    match config {
        SignerConfig::PrivateKey { private_key_hex } => {
            PrivateKeySigner::from_str(&private_key_hex).unwrap()
        }
        SignerConfig::Keystore { path, password } => {
            // Support for ECDSA
            LocalSigner::decrypt_keystore(path, password).unwrap()
        }
        SignerConfig::Web3 { endpoint, address } => {
            let url: Url = endpoint.parse().unwrap();
            Web3Signer::new(address, url)
        }
        SignerConfig::Aws { key_id, chain_id } => {
            // Review default values
            let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
            let client = aws_sdk_kms::Client::new(&config);
            AwsSigner::new(client, key_id, chain_id)
        }
    }
}
