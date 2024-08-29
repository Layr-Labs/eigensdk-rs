use alloy_primitives::Address;
use async_trait::async_trait;
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::operator::{OperatorInfo, OperatorPubKeys};

use crate::operator_info::OperatorInfoService;

pub struct FakeOperatorInfoService {
    pub pubkeys: OperatorInfo,
}

impl FakeOperatorInfoService {
    pub fn new(pubkeys: BlsKeyPair) -> Self {
        Self {
            pubkeys: OperatorInfo {
                pub_keys: Some(OperatorPubKeys::from(pubkeys)),
            },
        }
    }
}

#[async_trait]
impl OperatorInfoService for FakeOperatorInfoService {
    async fn get_operator_info(&self, _address: Address) -> Option<OperatorPubKeys> {
        self.pubkeys.pub_keys.clone()
    }
}
