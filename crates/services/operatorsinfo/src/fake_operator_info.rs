use alloy_primitives::Address;
use async_trait::async_trait;
use eigen_types::operator::{OperatorInfo, OperatorPubKeys};

use crate::{operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceError};

pub struct FakeOperatorInfoService {
    pub operator_info: OperatorInfo,
}

impl FakeOperatorInfoService {
    pub fn new(pub_keys: OperatorPubKeys) -> Self {
        Self {
            operator_info: OperatorInfo {
                pub_keys: Some(pub_keys),
            },
        }
    }
}

#[async_trait]
impl OperatorInfoService for FakeOperatorInfoService {
    async fn get_operator_info(
        &self,
        _address: Address,
    ) -> Result<Option<OperatorPubKeys>, OperatorInfoServiceError> {
        Ok(self.operator_info.pub_keys.clone())
    }
}
