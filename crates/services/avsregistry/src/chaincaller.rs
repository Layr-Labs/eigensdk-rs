use eigensdk_client_avsregistry::reader::AvsRegistryChainReader;
use eigensdk_contract_bindings::BLSApkRegistry::{G1Point, G2Point};
use eigensdk_crypto_bls::attestation::G1Point as BlsG1Point;
use eigensdk_crypto_bn254::utils::u256_to_bigint256;
use eigensdk_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigensdk_types::operator::{
    self, OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState,
};
use ethers_core::types::{Bytes, U256, U64};
use std::collections::HashMap;

pub struct AvsRegistryServiceChainCaller {
    avs_registry: AvsRegistryChainReader,
    operators_info_service: OperatorInfoServiceInMemory,
}

impl AvsRegistryServiceChainCaller {
    fn new(
        avs_registry: AvsRegistryChainReader,
        operators_info_service: OperatorInfoServiceInMemory,
    ) -> Self {
        Self {
            avs_registry,
            operators_info_service,
        }
    }

    pub async fn get_operators_avs_state_at_block(
        &self,
        block_num: u32,
        quorum_nums: Bytes,
    ) -> HashMap<[u8; 32], OperatorAvsState> {
        let mut operators_avs_state: HashMap<[u8; 32], OperatorAvsState> = HashMap::new();

        let operators_stakes_in_quorums = self
            .avs_registry
            .get_operators_stake_in_quorums_at_block(block_num, quorum_nums.clone())
            .await
            .unwrap();

        if operators_stakes_in_quorums.len() != quorum_nums.len() {
            // throw error
        }

        for (quorum_id, quorum_num) in quorum_nums.iter().enumerate() {
            for operator in &operators_stakes_in_quorums[quorum_id] {
                let info = self.get_operator_info(operator.operator_id).await;
                let stake_per_quorum = HashMap::new();
                let avs_state = operators_avs_state
                    .entry(operator.operator_id)
                    .or_insert_with(|| OperatorAvsState {
                        operator_id: operator.operator_id,
                        operator_info: OperatorInfo { pub_keys: info },
                        stake_per_quorum: stake_per_quorum,
                        block_num: block_num.into(),
                    });
                avs_state
                    .stake_per_quorum
                    .insert(*quorum_num, U256::from(operator.stake));
            }
        }

        return operators_avs_state;
    }

    pub async fn get_quorums_avs_state_at_block(&self, quorum_nums: Bytes, block_num: u32) {
        let operators_avs_state = self
            .get_operators_avs_state_at_block(block_num, quorum_nums.clone())
            .await;

        let mut quorums_avs_state: HashMap<u8, QuorumAvsState> = HashMap::new();

        for (_i, quorum_num) in quorum_nums.iter().enumerate() {
            let mut pub_key_g1 = BlsG1Point::new(
                u256_to_bigint256(U256::from(0)),
                u256_to_bigint256(U256::from(0)),
            );
            let mut total_stake: U256 = U256::from(0);
            for (_i, operator) in &operators_avs_state {
                if !operator.stake_per_quorum[quorum_num].is_zero() {
                    if let Some(pubkeys) = &operator.operator_info.pub_keys {
                        let g1_point = BlsG1Point::new(
                            u256_to_bigint256(pubkeys.g1_pub_key.x),
                            u256_to_bigint256(pubkeys.g1_pub_key.y),
                        );
                        pub_key_g1.add(g1_point);
                        total_stake += operator.stake_per_quorum[quorum_num];
                    }
                }
            }
            quorums_avs_state.insert(
                *quorum_num,
                QuorumAvsState {
                    quorum_num: *quorum_num,
                    total_stake: total_stake,
                    agg_pub_key_g1: pub_key_g1,
                    block_num: block_num,
                },
            );
        }
    }

    pub async fn get_operator_info(&self, operator_id: [u8; 32]) -> Option<OperatorPubKeys> {
        let operator_addr = self
            .avs_registry
            .get_operator_from_id(operator_id)
            .await
            .unwrap();

        let info = self
            .operators_info_service
            .get_operator_info(operator_addr)
            .await;

        info
    }
}
