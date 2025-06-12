use alloy::{contract::Error as AlloyError, providers::PendingTransactionError};
use eigen_client_avsregistry::error::AvsRegistryError;
use eigen_crypto_bls::error::BlsError;
use eigen_signer::error::SignerError;
use eigen_task_manager::{event_decoder::AbiDecodeError, TaskManagerError};
use rust_bls_bn254::errors::KeystoreError;
use tarpc::client::RpcError;
use thiserror::Error;

/// Error returned by AvsRegistry
#[derive(Debug, Error)]
pub enum OperatorError {
    /// AvsRegistry Error
    #[error("AvsRegistry Error")]
    AvsRegistry(#[from] AvsRegistryError),
    /// Operator Id Error
    #[error("Failed to get operator id")]
    OperatorIdError,
    /// Operator ID from BLS key pair does not match operator ID from contract
    #[error("Operator ID from BLS key pair does not match operator ID from contract")]
    OperatorIdMismatch,
    /// Operator Subscribe Logs Error
    #[error("Failed to subscribe logs")]
    SubscribeLogsError,
    /// Operator Transport Error
    #[error("Could not connect")]
    TransportError,
    /// Bls Keystore error
    #[error("Bls Keystore error ")]
    BlsKeystoreError(#[from] KeystoreError),
    /// Bls crate(SDK) error
    #[error("Bls crate(SDK) error")]
    EigenBlsError(#[from] BlsError),
    /// Failed to send signed task response
    #[error("Failed to send signed task response")]
    SendSignedTaskResponseError(#[from] RpcError),
    /// Max retry attempts exceeded
    #[error("Max retry attempts exceeded")]
    MaxRetryExceeded,
    /// Invalid failure rate
    #[error("Invalid failure rate. Must be between 0 and 100.")]
    InvalidFailureRate,
    /// Failed to encode signed task response
    #[error("Failed to encode signed task response")]
    FailedToEncodeSignedTaskResponse,
    /// Failed when computing a task response
    #[error("Failed when computing a task response")]
    ComputingTaskResponseError(#[from] TaskManagerError),
    /// Alloy contract error
    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),
    /// Decoding of the new task event failed
    #[error("Decoding of new task failed")]
    LogDecodeFailed(#[from] AbiDecodeError),
    /// Invalid deposit tokens
    #[error("Invalid deposit tokens")]
    InvalidDepositTokens,
    /// Registration Error
    #[error("Registration Error")]
    RegistrationError(#[from] OperatorRegistrationError),
}

/// Errors when registering an operator
#[derive(Debug, Error)]
pub enum OperatorRegistrationError {
    /// Registration Config Error
    #[error("Registration Config Missing")]
    RegistrationConfigMissing,
    /// Alloy pending Transaction error
    #[error("Alloy pending Transaction error {0}")]
    AlloyPendingTransactionError(#[from] PendingTransactionError),
    /// Alloy contract error
    #[error("Alloy contract error: {0}")]
    AlloyContractError(#[from] AlloyError),
    /// Signer Error
    #[error("Signer Error")]
    SignerError(#[from] SignerError),
    /// Failed to parse U256
    #[error("Failed to parse U256")]
    U256ParseError,
    /// Failed to parse http url
    #[error("Failed to parse http url")]
    HttpUrlParseError,
    /// BLS conversion error
    #[error("BLS conversion error")]
    BlsConversionError(#[from] BlsError),
}
