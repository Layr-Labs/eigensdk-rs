use alloy::{
    network::TxSigner,
    primitives::Address,
    signers::{
        aws::AwsSigner,
        local::{LocalSigner, PrivateKeySigner},
        Signature,
    },
};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_kms::config::{Credentials, SharedCredentialsProvider};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

use crate::{signer_v2::error::SignerError, web3_signer::Web3Signer};

/// Error types for the signer v2 module
pub mod error;

/// Configuration for the existing signers
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SignerConfig {
    /// Hexadecimal private key
    PrivateKey { private_key_hex: String },
    /// Keystore path and password
    /// Right now, only ECDSA keystore format is supported
    Keystore { path: String, password: String },
    /// Web3Signer endpoint and address
    Web3 { endpoint: String, address: Address },
    /// AWS KMS key ID and chain ID
    Aws {
        key_id: String,
        chain_id: Option<u64>,
        access_key: String,
        secret_access_key: String,
        region: String,
        endpoint_url: String,
    },
}

/// Creates a transaction signer from a configuration
pub async fn tx_signer_from_config(
    config: SignerConfig,
) -> Result<Box<dyn TxSigner<Signature>>, SignerError> {
    match config {
        SignerConfig::PrivateKey { private_key_hex } => {
            Ok(Box::new(PrivateKeySigner::from_str(&private_key_hex)?))
        }
        SignerConfig::Keystore { path, password } => {
            // Support for ECDSA
            Ok(Box::new(LocalSigner::decrypt_keystore(path, password)?))
        }
        SignerConfig::Web3 { endpoint, address } => {
            let url: Url = endpoint
                .parse()
                .map_err(|_| SignerError::InvalidEndpointUrl)?;
            Ok(Box::new(Web3Signer::new(address, url)))
        }
        SignerConfig::Aws {
            key_id,
            chain_id,
            access_key,
            secret_access_key,
            region,
            endpoint_url,
        } => {
            // Review default values
            let creds = Credentials::new(access_key, secret_access_key, None, None, "Static");
            let aws_region = Region::new(region);
            let config = aws_config::load_defaults(BehaviorVersion::latest())
                .await
                .to_builder()
                .credentials_provider(SharedCredentialsProvider::new(creds))
                .endpoint_url(endpoint_url)
                .region(Some(aws_region))
                .build();
            let client = aws_sdk_kms::Client::new(&config);
            Ok(Box::new(AwsSigner::new(client, key_id, chain_id).await?))
        }
    }
}
