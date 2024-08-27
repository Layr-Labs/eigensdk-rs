use crate::operator::QuorumNum;
use alloy_primitives::{B256, U256};
use eigen_crypto_bls::BlsKeyPair;
use std::collections::HashMap;

type StakeAmount = U256;

#[derive(Clone)]
pub struct TestOperator {
    pub operator_id: B256,
    pub stake_per_quorum: HashMap<QuorumNum, StakeAmount>,
    pub bls_keypair: BlsKeyPair,
}

impl TestOperator {
    pub fn get_bls_keypair(&self) -> &BlsKeyPair {
        &self.bls_keypair
    }
}
