use std::collections::HashMap;

use alloy_primitives::{BlockNumber, Bytes, FixedBytes, U256};
use ark_bn254::G1Projective;
use ark_ec::CurveGroup;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::{
    alloy_registry_g1_point_to_g1_affine, convert_to_g1_point, BlsG1Point, OperatorId, PublicKey,
};
use eigen_types::{
    operator::{OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState, QuorumNum},
    test::TestOperator,
};
use eigen_utils::binding::BLSApkRegistry::G1Point;

use crate::AvsRegistryService;

pub struct FakeAvsRegistryService {
    operators: HashMap<BlockNumber, HashMap<OperatorId, OperatorAvsState>>,
}

impl FakeAvsRegistryService {
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

impl AvsRegistryService for FakeAvsRegistryService {
    // TODO: change this so that it returns a Result
    async fn get_operators_avs_state_at_block(
        &self,
        block_number: u32,
        _quorum_nums: Bytes,
    ) -> HashMap<FixedBytes<32>, OperatorAvsState> {
        self.operators.get(&(block_number as u64)).unwrap().clone()
    }

    fn get_avs_registry(&self) -> AvsRegistryChainReader {
        todo!()
    }

    async fn get_quorums_avs_state_at_block(
        &self,
        quorum_nums: Bytes,
        block_num: u32,
    ) -> HashMap<u8, QuorumAvsState> {
        let operator_avs_state = self.operators.get(&(block_num as u64)).unwrap();
        let mut quorum_avs_state: HashMap<QuorumNum, QuorumAvsState> = HashMap::new();
        for quorum_num in quorum_nums {
            let mut pub_key_g1 = G1Projective::from(PublicKey::identity());
            let mut total_stake = U256::ZERO;
            for operator in operator_avs_state.values() {
                // only include operators that have a stake in this quorum
                match operator.stake_per_quorum.get(&quorum_num) {
                    Some(stake) => {
                        if let Some(pub_keys) = &operator.operator_info.pub_keys {
                            pub_key_g1 += pub_keys.g1_pub_key.g1();
                            total_stake += stake;
                        }
                    }
                    None => {}
                }
            }
            let g1_point = convert_to_g1_point(pub_key_g1.into_affine()).unwrap();
            quorum_avs_state.insert(
                quorum_num,
                QuorumAvsState {
                    quorum_num: quorum_num,
                    total_stake,
                    agg_pub_key_g1: BlsG1Point::new(alloy_registry_g1_point_to_g1_affine(
                        G1Point {
                            X: g1_point.X,
                            Y: g1_point.Y,
                        },
                    )),
                    block_num,
                },
            );
        }
        todo!()
    }

    async fn get_operator_info(&self, operator_id: [u8; 32]) -> Option<OperatorPubKeys> {
        todo!()
    }
}
