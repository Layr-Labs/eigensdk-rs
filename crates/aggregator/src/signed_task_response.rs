use alloy::dyn_abi::SolType;
use alloy::sol_types::SolValue;
use eigen_crypto_bls::{alloy_g1_point_to_g1_affine, convert_to_g1_point, Signature};
use eigen_task_processor::task_response::TaskResponse;
use eigen_types::operator::OperatorId;
use eigen_utils::slashing::middleware::registrycoordinator::BN254::G1Point;

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
    pub fn encode(&self) -> Vec<u8> {
        let g1_point = convert_to_g1_point(self.signature.g1_point().g1()).unwrap();

        <((u32, T), G1Point, OperatorId)>::abi_encode(&(
            (
                self.task_response.task_index,
                self.task_response.response.clone(),
            ),
            g1_point,
            self.operator_id,
        ))
    }

    pub fn decode(data: &[u8]) -> Self {
        let ((task_index, response), g1_point, operator_id) =
            <(
                (<u32 as SolValue>::SolType, <T as SolValue>::SolType),
                <G1Point as SolValue>::SolType,
                <OperatorId as SolValue>::SolType,
            )>::abi_decode_params(data, false)
            .unwrap();

        let g1_affine = alloy_g1_point_to_g1_affine(g1_point);

        let task_response = TaskResponse {
            task_index,
            response: response.into(),
        };

        Self {
            task_response,
            signature: Signature::new(g1_affine),
            operator_id,
        }
    }
}
