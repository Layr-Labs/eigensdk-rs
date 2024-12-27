use eigen_types::avs::SignatureVerificationError;
use thiserror::Error;

/// Possible errors raised in BLS aggregation
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BlsAggregationServiceError {
    #[error("task expired error")]
    TaskExpired,
    #[error("task not found error")]
    TaskNotFound,
    #[error("signature verification error")]
    SignatureVerificationError(SignatureVerificationError),
    #[error("signatures channel was closed, can't send signatures to aggregator")]
    SignaturesChannelClosed,
    #[error("error sending to channel")]
    ChannelError,
    #[error("Avs Registry Error")]
    RegistryError,
    #[error("duplicate task index error")]
    DuplicateTaskIndex,
}
