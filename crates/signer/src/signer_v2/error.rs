use alloy::signers::{aws::AwsSignerError, local::LocalSignerError};
use thiserror::Error;

/// Possible errors raised in signer creation
#[derive(Error, Debug)]
pub enum SignerError {
    /// LocalSigner error
    #[error("local signer error: {0}")]
    LocalSignerError(#[from] LocalSignerError),
    /// AwsSigner error
    #[error("aws signer error: {0}")]
    AwsSignerError(#[from] AwsSignerError),
    /// Invalid endpoint URL
    #[error("invalid endpoint URL")]
    InvalidEndpointUrl,
}
