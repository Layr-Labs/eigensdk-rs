use std::collections::HashMap;

use alloy_primitives::{BlockNumber, FixedBytes, U256};
use ark_bn254::G1Projective;
use ark_ec::{short_weierstrass::Affine, AffineRepr, CurveGroup};
use async_trait::async_trait;
use eigen_client_avsregistry::error::AvsRegistryError;
use eigen_crypto_bls::{BlsG1Point, OperatorId, PublicKey};
use eigen_types::{
    operator::{OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState, QuorumNum},
    test::TestOperator,
};
use eigen_utils::operatorstateretriever::OperatorStateRetriever::CheckSignaturesIndices;

use crate::AvsRegistryService;

/// A fake implementation of the `AvsRegistryService` trait that can be used for testing or debug purposes.
#[derive(Clone)]
pub struct FakeAvsRegistryService {
    operators: HashMap<BlockNumber, HashMap<OperatorId, OperatorAvsState>>,
}

impl FakeAvsRegistryService {
    /// Create a new instance of the `FakeAvsRegistryService` with the given `block_number` and `test_operators`.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number at which the operators are registered.
    /// * `test_operators` - A list of `TestOperator` instances to be included in the registry.
    ///
    /// # Returns
    ///
    /// A new instance of the `FakeAvsRegistryService`.
    pub fn new(block_number: BlockNumber, test_operators: Vec<TestOperator>) -> Self {
        let mut state_per_operator_id: HashMap<OperatorId, OperatorAvsState> = HashMap::new();

        // populate the inner hashmap
        for op in test_operators {
            let state = OperatorAvsState {
                operator_id: op.operator_id.into(),
                operator_info: OperatorInfo {
                    pub_keys: Some(OperatorPubKeys::from(op.bls_keypair)),
                },
                block_num: block_number.into(),
                stake_per_quorum: op.stake_per_quorum,
            };
            state_per_operator_id.insert(op.operator_id, state);
        }

        Self {
            operators: HashMap::from([(block_number, state_per_operator_id)]),
        }
    }
}

#[async_trait]
impl AvsRegistryService for FakeAvsRegistryService {
    async fn get_operators_avs_state_at_block(
        &self,
        block_number: u32,
        _quorum_nums: &[u8],
    ) -> Result<HashMap<FixedBytes<32>, OperatorAvsState>, AvsRegistryError> {
        self.operators
            .get(&(block_number as u64))
            .ok_or(AvsRegistryError::GetOperatorState)
            .cloned()
    }

    async fn get_quorums_avs_state_at_block(
        &self,
        quorum_nums: &[u8],
        block_num: u32,
    ) -> Result<HashMap<u8, QuorumAvsState>, AvsRegistryError> {
        let operator_avs_state = self
            .operators
            .get(&(block_num as u64))
            .ok_or(AvsRegistryError::GetOperatorState)?;
        let mut quorum_avs_state: HashMap<QuorumNum, QuorumAvsState> = HashMap::new();
        for quorum_num in quorum_nums {
            let mut pub_key_g1 = G1Projective::from(PublicKey::zero());
            let mut total_stake = U256::ZERO;
            for operator in operator_avs_state.values() {
                // only include operators that have a stake in this quorum
                if let Some(stake) = operator.stake_per_quorum.get(quorum_num) {
                    if let Some(pub_keys) = &operator.operator_info.pub_keys {
                        pub_key_g1 += pub_keys.g1_pub_key.g1();
                        total_stake += stake;
                    }
                }
            }

            // this check is needed because the conversion into affine fails if pub_key_g1 is zero.
            let agg_pub_key_g1 = if pub_key_g1 == G1Projective::from(PublicKey::zero()) {
                BlsG1Point::new(Affine::zero())
            } else {
                BlsG1Point::new(pub_key_g1.into_affine())
            };
            quorum_avs_state.insert(
                *quorum_num,
                QuorumAvsState {
                    quorum_num: *quorum_num,
                    total_stake,
                    agg_pub_key_g1,
                    block_num,
                },
            );
        }
        Ok(quorum_avs_state)
    }

    async fn get_check_signatures_indices(
        &self,
        _reference_block_number: u32,
        _quorum_numbers: Vec<u8>,
        _non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<CheckSignaturesIndices, AvsRegistryError> {
        Ok(CheckSignaturesIndices {
            nonSignerQuorumBitmapIndices: vec![],
            quorumApkIndices: vec![],
            totalStakeIndices: vec![],
            nonSignerStakeIndices: vec![],
        })
    }
}
