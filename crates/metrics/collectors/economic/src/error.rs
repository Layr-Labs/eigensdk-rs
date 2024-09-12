use alloy_primitives::ruint;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollectorMetricError {
    #[error("Collector metrics error  ")]
    ElContractsError(#[from] eigen_client_elcontracts::error::ElContractsError),

    #[error("Operator is not registered")]
    OperatorNotRegistered,

    #[error("Collector metric error")]
    AvsRegistryError(#[from] eigen_client_avsregistry::error::AvsRegistryError),

    /// Parse Error
    #[error("Parse Error :{0}")]
    ParseError(#[from] ruint::ParseError),
}
