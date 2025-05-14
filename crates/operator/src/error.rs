use eigen_client_avsregistry::error::AvsRegistryError;
// use eigen_config::error::ConfigError;
use eigen_crypto_bls::error::BlsError;
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
    /// Operator Registration Error
    #[error("Failed to register operator")]
    RegistrationError,
    /// Operator Id Error
    #[error("Failed to get operator id")]
    OperatorIdError,
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

    /// Decoding of the new task event failed
    #[error("Decoding of new task failed")]
    LogDecodeFailed(#[from] AbiDecodeError),
}
