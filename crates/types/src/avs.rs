use alloy::primitives::FixedBytes;
use thiserror::Error;
pub type TaskIndex = u32;

pub type TaskResponseDigest = FixedBytes<32>;

/// Error type for signature verification
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum SignatureVerificationError {
    #[error("incorrect signature error")]
    IncorrectSignature,
    #[error("operator public key not found")]
    OperatorPublicKeyNotFound,
    #[error("operator not found")]
    OperatorNotFound,
    #[error("duplicate signature error")]
    DuplicateSignature,
}
