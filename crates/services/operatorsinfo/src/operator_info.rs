use alloy_primitives::Address;
use async_trait::async_trait;
use eigen_types::operator::OperatorPubKeys;

#[async_trait]
pub trait OperatorInfoService {
    async fn get_operator_info(&self, address: Address) -> Option<OperatorPubKeys>;
}
