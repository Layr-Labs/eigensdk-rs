use alloy_primitives::Address;
use alloy_signer_aws::{AwsSigner, AwsSignerError};
use alloy_signer_local::PrivateKeySigner;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_kms;
use eth_keystore::decrypt_key;
use std::path::Path;
use thiserror::Error;
use url::Url;

use crate::web3_signer::Web3Signer;

#[derive(Debug)]
/// Represents the input params to create a signer
pub enum Config {
    /// Hexadecimal private key
    PrivateKey(String),
    /// Keystore path and password
    Keystore(String, String),
    /// Web3 endpoint and address
    Web3(String, String),
}

#[derive(Error, Debug)]
/// Possible errors raised in signer creation
pub enum SignerError {
    #[error("invalid private key")]
    InvalidPrivateKey,
    #[error("invalid keystore password")]
    InvalidPassword,
    #[error("invalid address")]
    InvalidAddress,
    #[error("invalid url")]
    InvalidUrl,
}

impl Config {
    /// Creates a signer from given config.
    pub fn signer_from_config(c: Config) -> Result<PrivateKeySigner, SignerError> {
        // TODO: check chain id to select signer
        match c {
            Config::PrivateKey(key) => key
                .parse::<PrivateKeySigner>()
                .map_err(|_| SignerError::InvalidPrivateKey),
            Config::Keystore(path, password) => {
                let keypath = Path::new(&path);
                let private_key =
                    decrypt_key(keypath, password).map_err(|_| SignerError::InvalidPassword)?;
                PrivateKeySigner::from_slice(&private_key)
                    .map_err(|_| SignerError::InvalidPrivateKey)
            }
            Config::Web3(endpoint, address) => {
                let url: Url = endpoint.parse().map_err(|_| SignerError::InvalidUrl)?;
                let address = Address::from_slice(
                    &hex::decode(address).map_err(|_| SignerError::InvalidAddress)?,
                );
                Web3Signer::new(address, url)
                // TODO: return Web3Signer using Signer trait
            }
        }
    }
    /// Creates a signer from a key ID in AWS Key Management Service
    pub async fn aws_signer(
        key_id: String,
        chain_id: Option<u64>,
        region: Region,
    ) -> Result<AwsSigner, AwsSignerError> {
        let config = aws_config::load_defaults(BehaviorVersion::latest())
            .await
            .to_builder()
            .region(Some(region))
            .build();
        let client = aws_sdk_kms::Client::new(&config);
        Ok(AwsSigner::new(client, key_id, chain_id).await?)
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use super::*;
    use alloy_consensus::{SignableTransaction, TxLegacy};
    use alloy_network::{TxSigner, TxSignerSync};
    use alloy_primitives::hex_literal::hex;
    use alloy_primitives::{address, bytes, U256};
    use alloy_signer::Signature;
    use alloy_signer_local::PrivateKeySigner;
    use std::str::FromStr;

    #[test]
    fn sign_transaction_with_private_key() {
        let private_key =
            "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".to_owned();
        let config = Config::PrivateKey(private_key);
        let mut tx = TxLegacy {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        let signer = Config::signer_from_config(config);

        let signature = signer.unwrap().sign_transaction_sync(&mut tx).unwrap();
        let expected_signature = Signature::from_rs_and_parity(
            U256::from_str(
                "99963972037857174861280476053118856715670512199525969754644366601434507134123",
            )
            .unwrap(),
            U256::from_str(
                "54587766196536123534774489028213321677166972433316011091332824361042811624091",
            )
            .unwrap(),
            37,
        )
        .unwrap();
        assert_eq!(signature, expected_signature);
    }

    #[test]
    fn sign_transaction_with_keystore() {
        let path = "mockdata/dummy.key.json".to_owned();
        let password = "testpassword".to_owned();
        let config = Config::Keystore(path, password);
        let mut tx = TxLegacy {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        let private_key = hex!("7a28b5ba57c53603b0b07b56bba752f7784bf506fa95edc395f5cf6c7514fe9d");
        let expected_signer = PrivateKeySigner::from_slice(&private_key).unwrap();
        let expected_signature = expected_signer.sign_transaction_sync(&mut tx).unwrap();

        let signer = Config::signer_from_config(config);
        let signature = signer.unwrap().sign_transaction_sync(&mut tx).unwrap();

        assert_eq!(signature, expected_signature);
    }

    #[tokio::test]
    async fn sign_transaction_with_aws_signer() {
        let key_id = "1234abcd-12ab-34cd-56ef-1234567890ab".to_string();
        let chain_id = Some(1);
        let region = Region::from_static("us-west-2a");
        let signer = Config::aws_signer(key_id, chain_id, region).await.unwrap();
        let mut tx = TxLegacy {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        // This request fails because key_id is not a valid key
        let sig = signer.sign_transaction(&mut tx).await.unwrap();

        let mut encoded_tx = Vec::new();
        tx.encode_for_signing(&mut encoded_tx);

        assert_eq!(
            sig.recover_address_from_msg(encoded_tx).unwrap(),
            signer.address()
        );
    }
}
