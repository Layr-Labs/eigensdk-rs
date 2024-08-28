use alloy_primitives::Address;
use async_trait::async_trait;
use eigen_types::operator::OperatorPubKeys;

#[async_trait]
pub trait OperatorInfoService {
    /// Get the operator info from the operator id
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator id
    ///
    /// # Returns
    ///
    /// The operator public keys
    async fn get_operator_info(&self, address: Address) -> Option<OperatorPubKeys>;
}
