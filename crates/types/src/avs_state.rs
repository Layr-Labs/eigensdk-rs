use crate::operator::OperatorId;
use crate::operator::OperatorInfo;
use crate::operator::QuorumNum;
use alloy::primitives::Address;
use alloy::primitives::U256;
use eigen_crypto_bls::BlsG1Point;
use ethers::types::U64;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OperatorAvsState {
    pub operator_id: OperatorId,
    pub operator_info: OperatorInfo,
    pub stake_per_quorum: HashMap<QuorumNum, U256>,
    pub block_num: U64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct QuorumAvsState {
    pub quorum_num: u8,
    pub total_stake: U256,
    pub agg_pub_key_g1: BlsG1Point,
    pub block_num: u64,
}

///An operator set identified by the AVS address and an identifier
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OperatorSet {
    /// The unique identifier for the operator set
    pub id: u32,
    /// The address of the AVS this operator set belongs to
    pub avs: Address,
}

impl From<OperatorSet>
    for eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet
{
    fn from(operator_set: OperatorSet) -> Self {
        eigen_utils::slashing::core::allocationmanager::AllocationManager::OperatorSet {
            id: operator_set.id,
            avs: operator_set.avs,
        }
    }
}
