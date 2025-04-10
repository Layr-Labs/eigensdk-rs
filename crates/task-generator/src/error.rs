//! Task generator errors

use thiserror::Error;
#[derive(Debug, Error)]
/// Task Generator Errors
pub enum TaskGeneratorError {
    /// Quorum threshold not set
    #[error("Quorum threshold not set")]
    QuorumThresholdNotSet,
    /// Quorum not set
    #[error("Quorum not set")]
    QuorumNotSet,
    /// Iterator not set
    #[error("Iterator not set")]
    IteratorNotSet,
    /// Task creation error
    #[error("Task creation error")]
    TaskCreation,
}
