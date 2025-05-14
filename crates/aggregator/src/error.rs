use crate::task_processor::TaskProcessorError;
use alloy::transports::{RpcError, TransportErrorKind};
use eigen_client_avsregistry::error::AvsRegistryError;
use eigen_crypto_bls::error::BlsError;
use eigen_services_blsaggregation::bls_aggregation_service_error::BlsAggregationServiceError;
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceError;
use eigen_task_manager::event_decoder::AbiDecodeError;
use tarpc::ServerError;
use thiserror::Error;

/// Error returned by chainio
#[derive(Debug, Error)]
pub enum AggregatorError {
    /// Bls Aggregation Service Error
    #[error("Bls Aggregation Service Error : {0}")]
    BlsAggregationServiceError(#[from] BlsAggregationServiceError),

    /// Task Response not found
    #[error("Task panicked or got cancelled")]
    JoinError,

    /// Decoding of event failed
    #[error("Decoding of event failed")]
    LogDecodeFailed(#[from] AbiDecodeError),

    /// Build avs registry chain reader
    #[error("Failed to build avs registry chain reader ")]
    BuildAvsRegistryChainReader(#[from] AvsRegistryError),

    /// alloy rpc error
    #[error("Alloy rpc error")]
    AlloyRpc(#[from] RpcError<TransportErrorKind>),

    /// IO error
    #[error("IO error")]
    IOError(#[from] std::io::Error),

    /// Operator Info service error
    #[error("Operator Info Service error")]
    OperatorInfoServiceError(#[from] OperatorInfoServiceError),

    /// Error returned by the [`TaskProcessor`](crate::traits::TaskProcessor)
    #[error("Task Processing failed")]
    TaskProcessorError(#[from] Box<dyn std::error::Error + Send>),

    /// Task index missing in topics
    #[error("Task index missing in topics")]
    TaskIndexMissingInTopics,

    /// Invalid task data
    #[error("Invalid task data")]
    InvalidTaskData,

    /// Invalid task index conversion
    #[error("Invalid task index conversion")]
    InvalidTaskIndexConversion,

    /// Task processor error
    #[error("Task processor error")]
    IndexingTaskProcessorError(#[from] TaskProcessorError),

    /// Tarpc error
    #[error("Tarpc error")]
    TarpcError(#[from] ServerError),

    /// BLS Key error
    #[error("BLS Key error")]
    BlsKeyError(#[from] BlsError),

    /// Alloy error
    #[error("Alloy error")]
    AlloyError(#[from] alloy::sol_types::Error),
}
