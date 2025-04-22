//! Task generator errors

use alloy::contract::Error as AlloyError;
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
    /// Alloy contract error
    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),
    /// Pending transaction error
    #[error("Pending transaction error: {0}")]
    PendingTransactionError(#[from] alloy::providers::PendingTransactionError),
}
