use alloy::dyn_abi::{abi::TokenSeq, SolType};
use alloy::sol_types::SolValue;
use eigen_crypto_bls::{alloy_g1_point_to_g1_affine, convert_to_g1_point, Signature};
use eigen_task_processor::task_response::TaskResponse;
use eigen_types::operator::OperatorId;
use eigen_utils::slashing::middleware::registrycoordinator::BN254::G1Point;

use crate::AggregatorError;

/// The tuple for SignedTaskResponse: (TaskResponse<Output>, G1Point, OperatorId)
pub type SignedTaskResponseTuple<Output> = (
    (<u32 as SolValue>::SolType, <Output as SolValue>::SolType),
    <G1Point as SolValue>::SolType,
    <OperatorId as SolValue>::SolType,
);

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
    T: From<<<T as SolValue>::SolType as SolType>::RustType>,
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

    /// Abi encode the task response
    ///
    /// # Returns
    ///
    /// The abi encoded task response
    pub fn encode(&self) -> Result<Vec<u8>, AggregatorError> {
        let g1_point = convert_to_g1_point(self.signature.g1_point().g1())?;

        Ok(<SignedTaskResponseTuple<T>>::abi_encode(&(
            (
                self.task_response.task_index,
                self.task_response.response.clone(),
            ),
            g1_point,
            self.operator_id,
        )))
    }

    /// Abi decode a signed task response
    ///
    /// # Arguments
    ///
    /// * `data` - The data to decode
    ///
    /// # Returns
    ///
    /// A new [`SignedTaskResponse`]
    pub fn decode(data: &[u8]) -> Result<Self, AggregatorError> {
        let (task_response, g1_point, operator_id) =
            decode_params::<SignedTaskResponseTuple<T>>(data, false)?;

        let task_response = TaskResponse {
            task_index: task_response.0,
            response: task_response.1.into(),
        };

        let g1_affine = alloy_g1_point_to_g1_affine(g1_point);

        Ok(Self {
            task_response,
            signature: Signature::new(g1_affine),
            operator_id,
        })
    }
}

// TODO: Move this to a common utils file
/// Decode generic type
///
/// # Arguments
///
/// * `data` - The data to decode
/// * `validate` - Whether to validate the data
///
/// # Returns
///
/// * `Result<T::RustType, AggregatorError>` - The decoded data
pub fn decode_params<T>(data: &[u8], validate: bool) -> Result<T::RustType, AggregatorError>
where
    T: SolType,
    for<'de> <T as SolType>::Token<'de>: TokenSeq<'de>,
{
    Ok(T::abi_decode_params(data, validate)?)
}
