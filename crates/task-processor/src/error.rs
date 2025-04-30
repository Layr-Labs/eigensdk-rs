use eigen_crypto_bls::error::BlsError;
use thiserror::Error;

use crate::task_manager::TaskManagerError;

/// Task processor error
#[derive(Debug, Error)]
pub enum TaskProcessorError {
    /// Task not found
    #[error("Task not found")]
    TaskNotFound,

    /// Task response not found
    #[error("Task response not found")]
    TaskResponseNotFound,

    /// Error de conversión de puntos G1/G2
    #[error("Error de conversión de puntos G1/G2")]
    PointConversionError(#[from] BlsError),

    /// Task manager error
    #[error("Task manager error")]
    TaskManagerError(#[from] TaskManagerError),
}
