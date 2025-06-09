//! Signer v2
//!
//! This module provides a simple and unified way to produce cryptographic signatures.
//!
//! ## Overview
//!
//! We provide a set of signers that can be used to sign transactions since they implement the
//! [`TxSigner`] trait.
//!
//! * [`PrivateKeySigner`] - Signer that uses a private key.
//! * [`Web3Signer`] - Signer that uses a Web3 endpoint.
//! * [`AwsSigner`] - Signer that uses AWS KMS.
//!
//! To create a signer, you can use the [`tx_signer_from_config`] function.
//!
//! ## Differences from Signer v1
//!
//! ### Unified Interface
//!
//! **v1**: Returns different concrete types:
//!
//! ```rust,no_run
//! # use eigen_signer::signer::Config;
//! # use alloy::network::TxSigner;
//! # use alloy::consensus::TxLegacy;
//! # use alloy::primitives::{bytes, hex_literal::hex, Address, U256};
//! # const PRIVATE_KEY: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
//! # const ADDRESS: [u8; 20] = hex!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
//! # const ENDPOINT: &str = "http://localhost:8545";
//! # #[tokio::main]
//! # async fn main() {
//! #     let mut tx = TxLegacy {
//! #         to: Address::from(ADDRESS).into(),
//! #         value: U256::from(1_000_000_000),
//! #         gas_limit: 2_000_000,
//! #         nonce: 0,
//! #         gas_price: 21_000_000_000,
//! #         input: bytes!(),
//! #         chain_id: Some(1),
//! #     };
//! #
//! // Create a signer from the private key configuration
//! // This returns a `PrivateKeySigner`
//! let config = Config::PrivateKey(PRIVATE_KEY.into());
//! let private_signer = Config::signer_from_config(config).unwrap();
//! private_signer.sign_transaction(&mut tx).await.unwrap();
//!
//! // Create a web3 signer from the endpoint and address
//! // This returns a `Web3Signer`
//! let web3_signer = Config::web3_signer(ENDPOINT.to_string(), ADDRESS.into()).unwrap();
//! web3_signer.sign_transaction(&mut tx).await.unwrap();
//! # }
//! ```
//!
//! **v2**: Returns a unified trait object:
//!
//! ```rust,no_run
//! # use eigen_signer::signer_v2::{SignerConfig, tx_signer_from_config, PrivateKeyConfig, Web3Config};
//! # use alloy::primitives::{address, bytes, hex_literal::hex, Address, U256};
//! # use alloy::consensus::TxLegacy;
//! # use alloy::network::TxSigner;
//! # const PRIVATE_KEY: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";
//! # const ADDRESS: [u8; 20] = hex!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045");
//! # const ENDPOINT: &str = "http://localhost:8545";
//! # #[tokio::main]
//! # async fn main() {
//! #    let mut tx = TxLegacy {
//! #        to: Address::from(ADDRESS).into(),
//! #        value: U256::from(1_000_000_000),
//! #        gas_limit: 2_000_000,
//! #        nonce: 0,
//! #        gas_price: 21_000_000_000,
//! #        input: bytes!(),
//! #        chain_id: Some(1),
//! #    };
//! #
//! // Create a signer from the private key configuration
//! // This returns a `GenericSigner` that implements the `TxSigner` trait
//! let private_config = PrivateKeyConfig {
//!     private_key: PRIVATE_KEY.into(),
//! };
//! let signer = tx_signer_from_config(private_config.into()).await.unwrap();
//! signer.sign_transaction(&mut tx).await.unwrap();
//!
//! // Create a signer from the web3 configuration
//! // This returns a `GenericSigner` that implements the `TxSigner` trait
//! let web3_config = Web3Config {
//!     endpoint: ENDPOINT.to_string(),
//!     address: ADDRESS.into(),
//! };
//! let signer = tx_signer_from_config(web3_config.into()).await.unwrap();
//! signer.sign_transaction(&mut tx).await.unwrap();
//! # }
//! ```
//!
//! ### Configuration System
//!
//! **v1**: Simple enum with tuple variants. No serialization support, so configurations must be
//! created programmatically.
//!
//! ```rust
//! pub enum Config {
//!     PrivateKey(String),
//!     Keystore(String, String),
//! }
//! ```
//!
//! **v2**: Structured, serializable configuration with named fields.
//! This is useful for configuration files.
//!
//! ```rust
//! # use eigen_signer::signer_v2::{PrivateKeyConfig, KeystoreConfig, Web3Config, AwsConfig};
//! # use serde::{Serialize, Deserialize};
//! #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
//! pub enum SignerConfig {
//!     PrivateKey(PrivateKeyConfig),
//!     Keystore(KeystoreConfig),
//!     Web3(Web3Config),
//!     Aws(AwsConfig),
//! }
//! ```
//!
//! ## Examples
//!
//! Here are some examples of how to create a signer from a configuration file:
//!
//! - [Incredible Squaring](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/examples/incredible-squaring/src/config/squaring-operator.toml)
//! - [Incredible Dot Product](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/examples/incredible-dot-product/src/config/dot-operator.toml)
//! - [Awesome Vault Service](https://github.com/Layr-Labs/eigensdk-rs/blob/v2-dev-2/examples/awesome-vault-service/src/config/awesome-operator.toml)

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

/// Environment variable to use as password for the keystore signer
const OPERATOR_ECDSA_KEY_PASSWORD: &str = "OPERATOR_ECDSA_KEY_PASSWORD";

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
    /// Hex-encoded private key plaintext.
    ///
    /// Uses a raw hexadecimal private key for transaction signing.
    PrivateKey(PrivateKeyConfig),
    /// Encrypted keystore file
    ///
    /// Uses encrypted keystore files following the [Web3 Secret Storage](https://ethereum.org/es/developers/docs/data-structures-and-encoding/web3-secret-storage) standard.
    /// The private key is encrypted with a password and stored in a JSON file.
    ///
    /// If no password is provided, the signer will try to use the [`OPERATOR_ECDSA_KEY_PASSWORD`] environment variable.
    ///
    /// To create a keystore, you can use the `eigen-cli` tool.
    ///
    /// ```bash
    /// cargo run --package eigen-cli -- egnkey generate --key-type ecdsa
    /// ```
    Keystore(KeystoreConfig),
    /// Web3 signer
    ///
    /// Delegates transaction signing to an external JSON-RPC signing service
    /// compatible with the [Web3Signer](https://docs.web3signer.consensys.io/reference/api/json-rpc)
    /// API. The service must support the `eth_signTransaction` method.
    Web3(Web3Config),
    /// AWS KMS signer
    ///
    /// Uses an AWS KMS asymmetric key (curve `secp256k1`) to sign transactions.
    /// The private key material never leaves AWS's Hardware Security Module.
    ///
    /// <div class="warning">
    /// This signer requires AWS credentials to be set via environment variables.
    /// See the <a href="https://docs.aws.amazon.com/sdkref/latest/guide/environment-variables.html">AWS documentation</a>
    /// for details and the <a href="https://docs.aws.amazon.com/sdkref/latest/guide/settings-reference.html#EVarSettings">AWS Settings Reference</a>
    /// for a full list of available environment variables.
    ///
    /// To configure your shell, run:
    /// ```bash
    /// export AWS_ACCESS_KEY_ID=your-access-key-id
    /// export AWS_SECRET_ACCESS_KEY=your-secret-access-key
    /// ```
    /// </div>
    Aws(AwsConfig),
}

impl From<PrivateKeyConfig> for SignerConfig {
    /// Convert a [`PrivateKeyConfig`] into a [`SignerConfig`]
    fn from(config: PrivateKeyConfig) -> Self {
        SignerConfig::PrivateKey(config)
    }
}

impl From<Web3Config> for SignerConfig {
    /// Convert a [`Web3Config`] into a [`SignerConfig`]
    fn from(config: Web3Config) -> Self {
        SignerConfig::Web3(config)
    }
}

impl From<KeystoreConfig> for SignerConfig {
    /// Convert a [`KeystoreConfig`] into a [`SignerConfig`]
    fn from(config: KeystoreConfig) -> Self {
        SignerConfig::Keystore(config)
    }
}

impl From<AwsConfig> for SignerConfig {
    /// Convert a [`AwsConfig`] into a [`SignerConfig`]
    fn from(config: AwsConfig) -> Self {
        SignerConfig::Aws(config)
    }
}

/// Configuration for a private key signer
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrivateKeyConfig {
    /// Hex-encoded private key plaintext.
    pub private_key: String,
}

/// Configuration for a keystore signer
///
/// This must be a file in the [Web3 Secret Storage](https://ethereum.org/es/developers/docs/data-structures-and-encoding/web3-secret-storage)
/// format.
///
/// To create a keystore, you can use the `eigen-cli` tool.
///
/// ```bash
/// cargo run --package eigen-cli -- egnkey generate --key-type ecdsa
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeystoreConfig {
    /// Path to the keystore file
    pub path: String,
    /// Password to decrypt the keystore file
    /// If no password is provided, the signer will try to use the [`OPERATOR_ECDSA_KEY_PASSWORD`] environment variable.
    pub password: Option<String>,
}

/// Configuration for a web3 signer
///
/// Delegates transaction signing to an external JSON-RPC signing service
/// compatible with the [Web3Signer](https://docs.web3signer.consensys.io/reference/api/json-rpc)
/// API. The service must support the `eth_signTransaction` method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Web3Config {
    /// Endpoint URL
    pub endpoint: String,
    /// Address of the signer
    pub address: Address,
}

/// AWS KMS signer
///
/// Uses an AWS KMS asymmetric key (curve `secp256k1`) to sign transactions.
/// The private key material never leaves AWS's Hardware Security Module.
///
/// <div class="warning">
/// This signer requires AWS credentials to be set via environment variables.
/// See the <a href="https://docs.aws.amazon.com/sdkref/latest/guide/environment-variables.html">AWS documentation</a>
/// for details and the <a href="https://docs.aws.amazon.com/sdkref/latest/guide/settings-reference.html#EVarSettings">AWS Settings Reference</a>
/// for a full list of available environment variables.
///
/// To configure your shell, run:
/// ```bash
/// export AWS_ACCESS_KEY_ID=your-access-key-id
/// export AWS_SECRET_ACCESS_KEY=your-secret-access-key
/// ```
/// </div>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AwsConfig {
    /// Key ID
    pub key_id: String,
    /// Chain ID
    pub chain_id: Option<u64>,
    /// Region
    pub region: String,
    /// Endpoint URL
    pub endpoint_url: String,
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
        SignerConfig::Keystore(KeystoreConfig { path, password }) => {
            // If the config password is empty, try with the environment variable
            let password = password
                .or_else(|| std::env::var(OPERATOR_ECDSA_KEY_PASSWORD).ok())
                .ok_or(SignerError::MissingKeystorePassword)?;

            Ok(GenericSigner::PrivateKey(LocalSigner::decrypt_keystore(
                path, password,
            )?))
        }
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
        let config = PrivateKeyConfig {
            private_key: PRIVATE_KEY.into(),
        };
        let mut tx = TxLegacy {
            to: Address::from(ADDRESS).into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        let signer = tx_signer_from_config(config.into()).await.unwrap();

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
        let config = KeystoreConfig {
            path: KEYSTORE_PATH.into(),
            password: Some(KEYSTORE_PASSWORD.into()),
        };
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

        let signer = tx_signer_from_config(config.into()).await.unwrap();
        let signature = signer.sign_transaction(&mut tx).await.unwrap();

        assert_eq!(signature, expected_signature);
    }

    #[tokio::test]
    async fn sign_transaction_with_keystore_and_env_password() {
        let config = KeystoreConfig {
            path: KEYSTORE_PATH.into(),
            password: None,
        };
        env::set_var("OPERATOR_ECDSA_KEY_PASSWORD", KEYSTORE_PASSWORD);

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

        let signer = tx_signer_from_config(config.into()).await.unwrap();
        let signature = signer.sign_transaction(&mut tx).await.unwrap();

        assert_eq!(signature, expected_signature);
    }

    #[tokio::test]
    async fn sign_transaction_with_keystore_and_no_env_password() {
        let config = KeystoreConfig {
            path: KEYSTORE_PATH.into(),
            password: None,
        };

        // Will try to decrypt the keystore with the env var, but it's not set, so it will return an error
        let signer = tx_signer_from_config(config.into()).await;
        assert!(signer.is_err());
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
        let aws_config = AwsConfig {
            key_id: key_id.into(),
            chain_id,
            region: AWS_US_WEST_REGION.into(),
            endpoint_url: localstack_endpoint,
        };
        let signer = tx_signer_from_config(aws_config.into()).await.unwrap();

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
        let web3_config = Web3Config {
            endpoint: endpoint.clone(),
            address,
        };
        let signer = tx_signer_from_config(web3_config.into()).await.unwrap();
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
            password: Some("testpassword".into()),
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
