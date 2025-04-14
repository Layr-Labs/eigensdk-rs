use alloy::contract::Error as AlloyError;
use alloy::sol_types::Error as AlloySolTypeError;
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

    /// Task Response is correct
    #[error("Task Response is not wrong")]
    TaskResponseIsCorrect,

    /// Task Response not found
    #[error("Task Response not found")]
    TaskResponseNotFound,

    /// Task not found
    #[error("Task not found")]
    TaskNotFound,

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
}
