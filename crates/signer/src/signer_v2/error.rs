use alloy::signers::{aws::AwsSignerError, local::LocalSignerError};
use thiserror::Error;

/// Error returned when creating a signer
#[derive(Error, Debug)]
pub enum SignerError {
    /// LocalSigner error
    #[error("local signer error: {0}")]
    LocalSignerError(#[from] LocalSignerError),
    /// AwsSigner error.
    /// Boxed because [`AwsSignerError`] is much larger (344+ bytes) than other variants (32 bytes).
    #[error("aws signer error: {0}")]
    AwsSignerError(#[from] Box<AwsSignerError>),
    /// Invalid endpoint URL
    #[error("invalid endpoint URL")]
    InvalidEndpointUrl,
    /// Environment variable not found
    #[error("environment variable not found: {0}")]
    EnvVariableNotFound(String),
}
