use eigen_types::avs::SignatureVerificationError;
use thiserror::Error;

/// Possible errors raised in BLS aggregation
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BlsAggregationServiceError {
    /// Task expired error
    #[error("task expired error")]
    TaskExpired,
    /// Task not found error
    #[error("task not found error")]
    TaskNotFound,
    /// Signature verification error. Wraps the error from the [`SignatureVerificationError`] enum.
    #[error("signature verification error")]
    SignatureVerificationError(SignatureVerificationError),
    /// Signature channel closed error
    #[error("signatures channel was closed, can't send signatures to aggregator")]
    SignaturesChannelClosed,
    /// AVS registry error
    #[error("Avs Registry Error")]
    RegistryError,
    /// Duplicate task index error
    #[error("duplicate task index error")]
    DuplicateTaskIndex,
    /// Sending to service error
    #[error("error sending to service")]
    SenderError,
    /// Receiving from service error
    #[error("error receiving from service")]
    ReceiverError,
}
