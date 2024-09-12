use thiserror::Error;

#[derive(Debug, Error)]
pub enum NodeApiError {
    /// Service id not found
    #[error("Service with id {0} not found")]
    ServiceIdNotFound(String),
}
