use alloy::primitives::Address;
use async_trait::async_trait;
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::operator::{OperatorInfo, OperatorPubKeys};

use crate::{operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceError};

/// A fake implementation of the `OperatorInfoService` trait that can be used for testing or debug purposes.
pub struct FakeOperatorInfoService {
    pub pubkeys: OperatorInfo,
    pub socket: String,
}

impl FakeOperatorInfoService {
    pub fn new(pubkeys: BlsKeyPair) -> Self {
        Self {
            pubkeys: OperatorInfo {
                pub_keys: Some(OperatorPubKeys::from(pubkeys)),
            },
            socket: String::default(),
        }
    }
}

#[async_trait]
impl OperatorInfoService for FakeOperatorInfoService {
    async fn get_operator_info(
        &self,
        _address: Address,
    ) -> Result<Option<OperatorPubKeys>, OperatorInfoServiceError> {
        Ok(self.pubkeys.pub_keys.clone())
    }

    async fn get_operator_socket(
        &self,
        _address: Address,
    ) -> Result<Option<String>, OperatorInfoServiceError> {
        Ok(Some(self.socket.clone()))
    }
}
