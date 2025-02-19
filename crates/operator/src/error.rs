use eigen_config::error::ConfigError;
use eigen_crypto_bls::error::BlsError;
use rust_bls_bn254::errors::KeystoreError;
use thiserror::Error;

/// Error returned by AvsRegistry
#[derive(Debug, Error)]
pub enum OperatorError {
    /// Operator Registration Error
    #[error("Failed to register operator")]
    RegistrationError,
    /// Operator Subscribe Logs Error
    #[error("Failed to subscribe logs")]
    SubscribeLogsError,
    /// Operator Transport Error
    #[error("Could not connect")]
    TransportError,
    /// Failed to parse config
    #[error("Config error {0}")]
    ConfigParseError(#[from] ConfigError),
    /// Bls Keystore error
    #[error("Bls Keystore error ")]
    BlsKeystoreError(#[from] KeystoreError),
    /// Bls crate(SDK) error
    #[error("Bls crate(SDK) error")]
    EigenBlsError(#[from] BlsError),
}
