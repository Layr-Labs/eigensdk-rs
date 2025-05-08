use alloy::sol_types::SolValue;
use eigen_crypto_bls::Signature;
use eigen_task_processor::task_response::TaskResponse;
use eigen_types::operator::OperatorId;

/// Signed Task Response
#[derive(Debug, Clone)]
pub struct SignedTaskResponse<T>
where
    T: SolValue + Clone,
{
    /// A response to a task
    pub task_response: TaskResponse<T>,
    /// Signature of the task response
    pub signature: Signature,
    /// ID of the operator corresponding to the signature
    pub operator_id: OperatorId,
}

impl<T> SignedTaskResponse<T>
where
    T: SolValue + Clone,
{
    /// Create a new [`SignedTaskResponse`]
    ///
    /// # Arguments
    ///
    /// * `task_response` - The task response
    /// * `bls_signature` - The BLS signature
    /// * `operator_id` - The operator ID
    ///
    /// # Returns
    ///
    /// A new [`SignedTaskResponse`] containing the task response, signature, and operator ID.
    pub fn new(
        task_response: TaskResponse<T>,
        bls_signature: Signature,
        operator_id: OperatorId,
    ) -> Self {
        Self {
            task_response,
            signature: bls_signature,
            operator_id,
        }
    }
}
