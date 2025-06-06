//! Signer v2 module

use alloy::{
    network::TxSigner,
    primitives::Address,
    signers::{
        aws::AwsSigner,
        local::{LocalSigner, PrivateKeySigner},
        Signature,
    },
};
use async_trait::async_trait;
use aws_config::Region;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use url::Url;

use crate::{signer_v2::error::SignerError, web3_signer::Web3Signer};

/// Error types for the signer v2 module
pub mod error;

/// Enum that contains all possible signer types
#[derive(Debug)]
enum GenericSigner {
    /// Hexadecimal private key
    PrivateKey(PrivateKeySigner),
    /// Web3 signer
    Web3(Web3Signer),
    /// AWS KMS signer
    Aws(AwsSigner),
}

#[async_trait]
impl TxSigner<Signature> for GenericSigner {
    fn address(&self) -> Address {
        match self {
            GenericSigner::PrivateKey(signer) => signer.address(),
            GenericSigner::Web3(signer) => signer.address(),
            GenericSigner::Aws(signer) => signer.address(),
        }
    }

    async fn sign_transaction(
        &self,
        tx: &mut dyn alloy::consensus::SignableTransaction<Signature>,
    ) -> alloy::signers::Result<Signature> {
        match self {
            GenericSigner::PrivateKey(signer) => signer.sign_transaction(tx).await,
            GenericSigner::Web3(signer) => signer.sign_transaction(tx).await,
            GenericSigner::Aws(signer) => signer.sign_transaction(tx).await,
        }
    }
}

/// Configuration for the existing signers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
#[non_exhaustive]
pub enum SignerConfig {
    /// Hexadecimal private key
    PrivateKey(PrivateKeyConfig),
    /// Keystore
    Keystore(KeystoreConfig),
    /// Web3
    Web3(Web3Config),
    /// AWS KMS
    Aws(AwsConfig),
}

/// Configuration for a private key signer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivateKeyConfig {
    private_key: String,
}

/// Configuration for a keystore signer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeystoreConfig {
    path: String,
    password: String,
}

/// Configuration for a web3 signer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Web3Config {
    endpoint: String,
    address: Address,
}

/// Configuration for an AWS KMS signer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AwsConfig {
    key_id: String,
    chain_id: Option<u64>,
    region: String,
    endpoint_url: String,
}

/// Creates a transaction signer from a configuration
///
/// # Arguments
///
/// * `config` - The signer configuration
///
/// # Returns
///
/// * A signer that implements the [`TxSigner`] trait
pub async fn tx_signer_from_config(
    config: SignerConfig,
) -> Result<impl TxSigner<Signature>, SignerError> {
    match config {
        SignerConfig::PrivateKey(PrivateKeyConfig { private_key }) => Ok(
            GenericSigner::PrivateKey(PrivateKeySigner::from_str(&private_key)?),
        ),
        SignerConfig::Keystore(KeystoreConfig { path, password }) => Ok(GenericSigner::PrivateKey(
            LocalSigner::decrypt_keystore(path, password)?,
        )),
        SignerConfig::Web3(Web3Config { endpoint, address }) => {
            let url: Url = endpoint
                .parse()
                .map_err(|_| SignerError::InvalidEndpointUrl)?;
            Ok(GenericSigner::Web3(Web3Signer::new(address, url)))
        }
        SignerConfig::Aws(AwsConfig {
            key_id,
            chain_id,
            region,
            endpoint_url,
        }) => {
            let config = aws_config::from_env()
                .endpoint_url(endpoint_url)
                .region(Some(Region::new(region)))
                .load()
                .await;
            let client = aws_sdk_kms::Client::new(&config);
            Ok(GenericSigner::Aws(
                AwsSigner::new(client, key_id, chain_id)
                    .await
                    .map_err(|e| SignerError::AwsSignerError(Box::new(e)))?,
            ))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::signer_v2::{
        tx_signer_from_config, AwsConfig, KeystoreConfig, PrivateKeyConfig, Web3Config,
    };

    use super::SignerConfig;
    use alloy::consensus::{SignableTransaction, TxLegacy};
    use alloy::network::{TxSigner, TxSignerSync};
    use alloy::primitives::PrimitiveSignature;
    use alloy::primitives::{address, bytes, hex_literal::hex, keccak256, Address, U256};
    use alloy::signers::local::PrivateKeySigner;
    use aws_config::{BehaviorVersion, Region, SdkConfig};
    use aws_sdk_kms::{
        self,
        config::{Credentials, SharedCredentialsProvider},
        types::KeyMetadata,
    };
    use eigen_testing_utils::anvil::start_anvil_container;
    use eigen_testing_utils::test_data::TestData;
    use std::env;
    use std::str::FromStr;
    use testcontainers::{
        core::{IntoContainerPort, WaitFor},
        runners::AsyncRunner,
        ContainerAsync, GenericImage, ImageExt,
    };
    use tokio;

    const PRIVATE_KEY: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
    const ADDRESS: [u8; 20] = hex!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
    const SIGNATURE_R: &str =
        "99963972037857174861280476053118856715670512199525969754644366601434507134123";
    const SIGNATURE_S: &str =
        "54587766196536123534774489028213321677166972433316011091332824361042811624091";
    const KEYSTORE_PATH: &str = "mockdata/dummy.key.json";
    const KEYSTORE_PASSWORD: &str = "testpassword";
    const LOCALSTACK_PORT: u16 = 4566;
    // Add this port to avoid conflicts with the test of the signer v1
    const LOCALSTACK_MAPPED_PORT: u16 = 4567;
    const AWS_US_WEST_REGION: &str = "us-west-1";
    const LOCALSTACK_IMAGE_NAME: &str = "localstack/localstack";
    const LOCALSTACK_IMAGE_TAG: &str = "latest";

    #[tokio::test]
    async fn sign_transaction_with_private_key() {
        let config = SignerConfig::PrivateKey(PrivateKeyConfig {
            private_key: PRIVATE_KEY.into(),
        });
        let mut tx = TxLegacy {
            to: Address::from(ADDRESS).into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        let signer = tx_signer_from_config(config).await.unwrap();

        let signature: [u8; 65] = signer.sign_transaction(&mut tx).await.unwrap().into();
        let sig = PrimitiveSignature::try_from(&signature[..]).unwrap();
        let expected_signature = PrimitiveSignature::new(
            U256::from_str(SIGNATURE_R).unwrap(),
            U256::from_str(SIGNATURE_S).unwrap(),
            false,
        );
        assert_eq!(sig, expected_signature);
    }

    #[tokio::test]
    async fn sign_transaction_with_keystore() {
        let config = SignerConfig::Keystore(KeystoreConfig {
            path: KEYSTORE_PATH.into(),
            password: KEYSTORE_PASSWORD.into(),
        });
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

        let signer = tx_signer_from_config(config).await.unwrap();
        let signature = signer.sign_transaction(&mut tx).await.unwrap();

        assert_eq!(signature, expected_signature);
    }

    #[tokio::test]
    async fn test_sign_transaction_with_kms_signer() {
        // Start the container running Localstack
        let _container = start_localstack_container().await;

        // Set the environment variables for the AWS credentials
        env::set_var("AWS_ACCESS_KEY_ID", "localstack");
        env::set_var("AWS_SECRET_ACCESS_KEY", "localstack");

        let localstack_endpoint = format!("http://localhost:{}", LOCALSTACK_MAPPED_PORT);
        let config = get_aws_config(
            "localstack".into(),
            "localstack".into(),
            Region::from_static(AWS_US_WEST_REGION),
            localstack_endpoint.clone(),
        )
        .await;

        let default_tx = TxLegacy {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };
        let mut test_data: TestData<TxLegacy> = TestData::new(default_tx);

        // Create an AWS KMS Client
        let client = aws_sdk_kms::Client::new(&config);

        // Create a key
        let key_metadata = create_kms_key(&client).await;

        // Create a signer for the given key
        let key_id = key_metadata.key_id();
        let chain_id = Some(1);
        let signer = tx_signer_from_config(SignerConfig::Aws(AwsConfig {
            key_id: key_id.into(),
            chain_id,
            region: AWS_US_WEST_REGION.into(),
            endpoint_url: localstack_endpoint,
        }))
        .await
        .unwrap();

        // Sign the transaction
        let signature = signer.sign_transaction(&mut test_data.input).await.unwrap();

        // Recover the address
        let mut encoded_tx = Vec::new();
        test_data.input.encode_for_signing(&mut encoded_tx);
        let prehash = keccak256(encoded_tx);
        let recovered_address = signature.recover_address_from_prehash(&prehash).unwrap();

        // Check that the recovered addresses are the same
        assert_eq!(signer.address(), recovered_address);
    }

    #[tokio::test]
    async fn test_sign_legacy_transaction_with_web3_signer() {
        let (_container, endpoint, _ws_endpoint) = start_anvil_container().await;

        let address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
        let signer = tx_signer_from_config(SignerConfig::Web3(Web3Config { endpoint, address }))
            .await
            .unwrap();
        let mut tx = TxLegacy {
            to: address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 0x76c0,
            gas_price: 21_000_000_000,
            nonce: 0,
            input: bytes!(),
            chain_id: Some(31337),
        };

        let signature = signer.sign_transaction(&mut tx).await.unwrap();

        let private_key_hex = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let expected_signer = PrivateKeySigner::from_str(private_key_hex).unwrap();
        let expected_signature = expected_signer.sign_transaction_sync(&mut tx).unwrap();

        assert_eq!(signature, expected_signature);
    }

    #[test]
    fn test_private_key_config_serialization() {
        let original = PrivateKeyConfig {
            private_key: "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".into(),
        };
        let toml_str = toml::to_string(&original).unwrap();
        let parsed: PrivateKeyConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed, original);

        let toml_str = r#"
            private_key = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7"
        "#;
        let parsed: PrivateKeyConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_keystore_config_serialization() {
        let original = KeystoreConfig {
            path: "ecdsa.key.json".into(),
            password: "testpassword".into(),
        };
        let toml_str = toml::to_string(&original).unwrap();
        let parsed: KeystoreConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed, original);

        let toml_str = r#"
            path ="ecdsa.key.json"
            password = "testpassword"
        "#;
        let parsed: KeystoreConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_web3_config_serialization() {
        let original = Web3Config {
            endpoint: "http://localhost:8545".into(),
            address: Address::from(hex!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045")),
        };
        let toml_str = toml::to_string(&original).unwrap();
        let parsed: Web3Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed, original);

        let toml_str = r#"
            endpoint = "http://localhost:8545"
            address = "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045"
        "#;
        let parsed: Web3Config = toml::from_str(toml_str).unwrap();
        assert_eq!(parsed, original);
    }

    #[test]
    fn test_aws_config_serialization() {
        let original = AwsConfig {
            key_id: "1234abcd-12ab-34cd-56ef-1234567890ab".into(),
            chain_id: Some(1),
            region: "us-west-1".into(),
            endpoint_url: "http://localhost:4566".into(),
        };
        let toml_str = toml::to_string(&original).unwrap();
        let parsed: AwsConfig = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed, original);

        let toml_str = r#"
            key_id = "1234abcd-12ab-34cd-56ef-1234567890ab"
            chain_id = 1
            region = "us-west-1"
            endpoint_url = "http://localhost:4566"
        "#;
        let parsed: AwsConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(parsed, original);
    }

    async fn start_localstack_container() -> ContainerAsync<GenericImage> {
        GenericImage::new(LOCALSTACK_IMAGE_NAME, LOCALSTACK_IMAGE_TAG)
            .with_exposed_port(LOCALSTACK_PORT.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready."))
            .with_mapped_port(LOCALSTACK_MAPPED_PORT, LOCALSTACK_PORT.tcp())
            .start()
            .await
            .expect("Error starting localstack container")
    }

    async fn create_kms_key(client: &aws_sdk_kms::Client) -> KeyMetadata {
        client
            .create_key()
            .key_spec(aws_sdk_kms::types::KeySpec::EccSecgP256K1)
            .key_usage(aws_sdk_kms::types::KeyUsageType::SignVerify)
            .send()
            .await
            .unwrap()
            .key_metadata()
            .unwrap()
            .clone()
    }

    async fn get_aws_config(
        access_key: String,
        secret_access_key: String,
        region: Region,
        endpoint_url: String,
    ) -> SdkConfig {
        let creds = Credentials::new(access_key, secret_access_key, None, None, "Static");
        aws_config::load_defaults(BehaviorVersion::latest())
            .await
            .to_builder()
            .credentials_provider(SharedCredentialsProvider::new(creds))
            .endpoint_url(endpoint_url)
            .region(Some(region.clone()))
            .build()
    }
}
