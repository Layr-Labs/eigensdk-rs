use crate::web3_signer::Web3Signer;
use alloy_primitives::Address;
use alloy_signer_aws::{AwsSigner, AwsSignerError};
use alloy_signer_local::PrivateKeySigner;
use eth_keystore::decrypt_key;
use std::path::Path;
use thiserror::Error;
use url::Url;

/// Represents the input params to create a signer
#[derive(Debug)]
pub enum Config {
    /// Hexadecimal private key
    PrivateKey(String),
    /// Keystore path and password
    Keystore(String, String),
}

/// Possible errors raised in signer creation
#[derive(Error, Debug)]
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
        }
    }
    /// Creates a signer from a key ID in AWS Key Management Service
    pub async fn aws_signer(
        key_id: String,
        chain_id: Option<u64>,
        client: aws_sdk_kms::Client,
    ) -> Result<AwsSigner, AwsSignerError> {
        AwsSigner::new(client, key_id, chain_id).await
    }

    pub fn web3_signer(endpoint: String, address: Address) -> Result<Web3Signer, SignerError> {
        let url: Url = endpoint.parse().map_err(|_| SignerError::InvalidUrl)?;
        Ok(Web3Signer::new(address, url))
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use alloy_consensus::{SignableTransaction, TxLegacy};
    use alloy_network::{TxSigner, TxSignerSync};
    use alloy_primitives::hex_literal::hex;
    use alloy_primitives::{address, bytes, keccak256, Address, U256};
    use alloy_signer::Signature;
    use alloy_signer_local::PrivateKeySigner;
    use aws_config::{BehaviorVersion, Region};
    use aws_sdk_kms::{
        self,
        config::{Credentials, SharedCredentialsProvider},
    };
    use std::str::FromStr;
    use testcontainers::runners::AsyncRunner;
    use testcontainers::ImageExt;
    use testcontainers::{
        core::{IntoContainerPort, WaitFor},
        GenericImage,
    };
    use tokio;

    const PRIVATE_KEY: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
    const ADDRESS: [u8; 20] = hex!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    const SIGNATURE_R: &str =
        "99963972037857174861280476053118856715670512199525969754644366601434507134123";
    const SIGNATURE_S: &str =
        "54587766196536123534774489028213321677166972433316011091332824361042811624091";
    const SIGNATURE_Y_PARITY: u64 = 37;
    const KEYSTORE_PATH: &str = "mockdata/dummy.key.json";
    const KEYSTORE_PASSWORD: &str = "testpassword";
    const LOCALSTACK_PORT: u16 = 4566;
    const AWS_SOUTH_AMERICA_REGION: &str = "sa-east-1";
    const LOCALSTACK_IMAGE_NAME: &str = "localstack/localstack";
    const LOCALSTACK_IMAGE_TAG: &str = "latest";
    #[test]
    fn sign_transaction_with_private_key() {
        let config = Config::PrivateKey(PRIVATE_KEY.into());
        let mut tx = TxLegacy {
            to: Address::from(ADDRESS).into(),
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
            U256::from_str(SIGNATURE_R).unwrap(),
            U256::from_str(SIGNATURE_S).unwrap(),
            SIGNATURE_Y_PARITY,
        )
        .unwrap();
        assert_eq!(signature, expected_signature);
    }

    #[test]
    fn sign_transaction_with_keystore() {
        let config = Config::Keystore(KEYSTORE_PATH.into(), KEYSTORE_PASSWORD.into());
        let mut tx = TxLegacy {
            to: Address::from(ADDRESS).into(),
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
        // Start the container running Localstack
        let _container = GenericImage::new(LOCALSTACK_IMAGE_NAME, LOCALSTACK_IMAGE_TAG)
            .with_exposed_port(LOCALSTACK_PORT.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready."))
            .with_mapped_port(LOCALSTACK_PORT, LOCALSTACK_PORT.tcp())
            .start()
            .await
            .unwrap();

        // Set up the client configuration
        let creds = Credentials::new("localstack", "localstack", None, None, "Static");
        let localstack_url: String = "http://localhost:".to_string() + &LOCALSTACK_PORT.to_string();
        let region = Region::from_static(&AWS_SOUTH_AMERICA_REGION);
        let config = aws_config::load_defaults(BehaviorVersion::latest())
            .await
            .to_builder()
            .credentials_provider(SharedCredentialsProvider::new(creds))
            .endpoint_url(localstack_url)
            .region(Some(region.clone()))
            .build();

        // Create an AWS KMS Client
        let client = aws_sdk_kms::Client::new(&config);

        // Create a key
        let key = client
            .create_key()
            .key_spec(aws_sdk_kms::types::KeySpec::EccSecgP256K1)
            .key_usage(aws_sdk_kms::types::KeyUsageType::SignVerify)
            .send()
            .await
            .unwrap();

        // Create a signer for the given key
        let key_id = key.key_metadata().unwrap().key_id();
        let chain_id = Some(1);
        let signer = Config::aws_signer(key_id.into(), chain_id, client.clone())
            .await
            .unwrap();

        let mut tx = TxLegacy {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        // Sign the transaction
        let signature = signer.sign_transaction(&mut tx).await.unwrap();

        // Recover the address
        let mut encoded_tx = Vec::new();
        tx.encode_for_signing(&mut encoded_tx);
        let prehash = keccak256(encoded_tx);
        let recovered_address = signature.recover_address_from_prehash(&prehash).unwrap();

        // Check that the recovered addresses are the same
        assert_eq!(signer.address(), recovered_address);
    }

    #[tokio::test]
    #[ignore = "work in progress"]
    async fn sign_transaction_with_web3_signer() {
        // TODO: replace hardcoded addresses with anvil
        let endpoint = "http://127.0.0.1:8545 ".to_string();
        let address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
        let signer = Config::web3_signer(endpoint, address).unwrap();
        let mut tx = TxLegacy {
            to: address!("70997970C51812dc3A010C7d01b50e0d17dc79C8").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 0x76c0,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        let signature = signer.sign_transaction(&mut tx).await.unwrap();

        let private_key = hex!("ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80");
        let expected_signer = PrivateKeySigner::from_slice(&private_key).unwrap();
        let expected_signature = expected_signer.sign_transaction_sync(&mut tx).unwrap();

        assert_eq!(signature, expected_signature);
    }
}
