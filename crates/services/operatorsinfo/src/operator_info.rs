use alloy::primitives::Address;
use async_trait::async_trait;
use eigen_types::operator::OperatorPubKeys;

use crate::operatorsinfo_inmemory::OperatorInfoServiceError;

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
    async fn get_operator_info(
        &self,
        address: Address,
    ) -> Result<Option<OperatorPubKeys>, OperatorInfoServiceError>;

    async fn get_operator_socket(
        &self,
        address: Address,
    ) -> Result<Option<String>, OperatorInfoServiceError>;
}
