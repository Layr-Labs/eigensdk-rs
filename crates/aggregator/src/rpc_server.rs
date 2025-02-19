use eigen_crypto_bls::Signature;
use eigen_types::operator::OperatorId;
use serde::{Deserialize, Serialize};

/// Signed Task Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedTaskResponse<T> {
    /// A response to a task
    pub task_response: T,
    /// Signature of the task response
    pub signature: Signature,
    /// ID of the operator corresponding to the signature
    pub operator_id: OperatorId,
}

impl<T: Serialize + for<'de> Deserialize<'de>> SignedTaskResponse<T> {
    /// Create a new [`SignedTaskResponse`]
    pub fn new(task_response: T, bls_signature: Signature, operator_id: OperatorId) -> Self {
        Self {
            task_response,
            signature: bls_signature,
            operator_id,
        }
    }
}
