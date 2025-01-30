use alloy::transports::{RpcError, TransportErrorKind};
use eigen_client_avsregistry::error::AvsRegistryError;
use eigen_crypto_bls::error::BlsError;
use eigen_services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError;
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceError;
use jsonrpc_core::serde_json::Error;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum AggregatorError {
    /// Bls Aggregation Service Error
    #[error("Bls Aggregation Service Error : {0}")]
    BlsAggregationServiceError(#[from] BlsAggregationServiceError),

    /// Task Response not found
    #[error("Task Response not found")]
    TaskResponseNotFound,

    /// Task Response not found
    #[error("Task panicked or got cancelled")]
    JoinError,

    /// Decoding of the new task event failed
    #[error("Log decode failed")]
    LogDecodeFailed(#[from] alloy::sol_types::Error),

    /// Build avs registry chain reader
    #[error("Failed to build avs registry chain reader ")]
    BuildAvsRegistryChainReader(#[from] AvsRegistryError),

    /// Bls crate error
    #[error("Bls Crate Error SDK")]
    Bls(#[from] BlsError),

    /// alloy rpc error
    #[error("Alloy rpc error")]
    AlloyRpc(#[from] RpcError<TransportErrorKind>),

    /// serde json error
    #[error("Serde json error")]
    SerdeError(#[from] Error),

    /// IO error
    #[error("IO error")]
    IOError(#[from] std::io::Error),

    /// Operator Info service error
    #[error("Operator Info Service error")]
    OperatorInfoServiceError(#[from] OperatorInfoServiceError),

    /// Error returned by the [`TaskProcessor`](crate::traits::TaskProcessor)
    #[error("Task Processing failed")]
    TaskProcessorError(#[from] Box<dyn std::error::Error + Send>),
}
