use alloy::contract::Error as AlloyError;
use alloy::sol_types::Error as AlloySolTypeError;
use eigen_task_manager::event_decoder::AbiDecodeError;
use eigen_task_manager::TaskManagerError;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum ChallengerError {
    /// Alloy contract error
    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),

    /// Alloy Transport Error
    #[error("Alloy Transport Error")]
    AlloyError(#[from] alloy::transports::TransportError),

    /// Alloy sol types error
    #[error("Alloy sol types error :{0}")]
    AlloySolType(#[from] AlloySolTypeError),

    /// Transaction hash not found
    #[error("Tx hash not found")]
    TransactionHashNotFound,

    /// Decoded data empty
    #[error("Decoded data empty")]
    EmptyDecodedData,

    /// Failed to decode event
    #[error("Failed to decode event ")]
    DecodeEvent,

    /// Failed to parse ECDSA keystore signer
    #[error("Failed to parse ecdsa keystore signer")]
    ECDSAKeystoreSigner,

    /// Transaction not found
    #[error("Transaction {0} not found")]
    TransactionNotFound(String),

    /// Task Manager error
    #[error("Task Manager error")]
    TaskManagerError(#[from] TaskManagerError),

    /// Task Index missing in topics
    #[error("Task Index missing in topics")]
    TaskIndexMissingInTopics,

    /// Invalid task index conversion
    #[error("Invalid task index conversion")]
    InvalidTaskIndexConversion,

    /// Invalid log decode
    #[error("Invalid log decode")]
    InvalidLogDecode,

    /// Invalid calldata
    #[error("Invalid calldata")]
    InvalidCalldata,

    /// Decoding of event failed
    #[error("Decoding of event failed")]
    LogDecodeFailed(#[from] AbiDecodeError),
}
