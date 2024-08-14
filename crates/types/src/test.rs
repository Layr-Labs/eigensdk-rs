use alloy_primitives::{B256, U256};
use eigen_crypto_bls::BlsKeyPair;
use std::collections::HashMap;

type QuorumNum = u8;
type StakeAmount = U256;

pub struct TestOperator {
    operator_id: B256,
    stake_per_quorum: HashMap<QuorumNum, StakeAmount>,
    bls_keypair: BlsKeyPair,
}
