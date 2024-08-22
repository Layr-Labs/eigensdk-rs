use alloy_primitives::{Bytes, FixedBytes, U256};
use ark_bn254::G1Projective;
use ark_ec::{short_weierstrass::Affine, AffineRepr, CurveGroup};
use eigen_client_avsregistry::{error::AvsRegistryError, reader::AvsRegistryChainReader};
use eigen_crypto_bls::{BlsG1Point, PublicKey};
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigen_types::operator::{OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState};
use eigen_utils::binding::OperatorStateRetriever::CheckSignaturesIndices;
use std::collections::HashMap;

use crate::AvsRegistryService;

#[derive(Debug, Clone)]
pub struct AvsRegistryServiceChainCaller {
    avs_registry: AvsRegistryChainReader,
    operators_info_service: OperatorInfoServiceInMemory,
}

impl AvsRegistryServiceChainCaller {
    pub fn new(
        avs_registry: AvsRegistryChainReader,
        operators_info_service: OperatorInfoServiceInMemory,
    ) -> Self {
        Self {
            avs_registry,
            operators_info_service,
        }
    }
}

impl AvsRegistryService for AvsRegistryServiceChainCaller {
    async fn get_operators_avs_state_at_block(
        &self,
        block_num: u32,
        quorum_nums: &[u8],
    ) -> Result<HashMap<FixedBytes<32>, OperatorAvsState>, AvsRegistryError> {
        let mut operators_avs_state: HashMap<FixedBytes<32>, OperatorAvsState> = HashMap::new();

        let operators_stakes_in_quorums = self
            .avs_registry
            .get_operators_stake_in_quorums_at_block(block_num, Bytes::from(Vec::from(quorum_nums)))
            .await?;

        if operators_stakes_in_quorums.len() != quorum_nums.len() {
            // TODO: throw error
        }

        for (quorum_id, quorum_num) in quorum_nums.iter().enumerate() {
            for operator in &operators_stakes_in_quorums[quorum_id] {
                let info = self.get_operator_info(*operator.operatorId).await;
                let stake_per_quorum = HashMap::new();
                let avs_state = operators_avs_state
                    .entry(FixedBytes(*operator.operatorId))
                    .or_insert_with(|| OperatorAvsState {
                        operator_id: *operator.operatorId,
                        operator_info: OperatorInfo { pub_keys: info },
                        stake_per_quorum,
                        block_num: block_num.into(),
                    });
                avs_state
                    .stake_per_quorum
                    .insert(*quorum_num, U256::from(operator.stake));
            }
        }

        Ok(operators_avs_state)
    }

    async fn get_quorums_avs_state_at_block(
        &self,
        quorum_nums: &[u8],
        block_num: u32,
    ) -> Result<HashMap<u8, QuorumAvsState>, AvsRegistryError> {
        let operators_avs_state = self
            .get_operators_avs_state_at_block(block_num, quorum_nums)
            .await?;

        let mut quorums_avs_state: HashMap<u8, QuorumAvsState> = HashMap::new();

        for quorum_num in quorum_nums.iter() {
            let mut pub_key_g1 = G1Projective::from(PublicKey::identity());
            let mut total_stake: U256 = U256::from(0);
            for operator in operators_avs_state.values() {
                if !operator
                    .stake_per_quorum
                    .get(quorum_num)
                    .unwrap_or(&U256::ZERO)
                    .is_zero()
                {
                    if let Some(pub_keys) = &operator.operator_info.pub_keys {
                        pub_key_g1 += pub_keys.g1_pub_key.g1();
                        total_stake += operator.stake_per_quorum[quorum_num];
                    }
                }
            }
            let agg_pub_key_g1 = if pub_key_g1 == G1Projective::from(PublicKey::zero()) {
                // TODO: check this. What should the apk be for a quorum without stakers?
                BlsG1Point::new(Affine::zero())
            } else {
                BlsG1Point::new(pub_key_g1.into_affine())
            };

            quorums_avs_state.insert(
                *quorum_num,
                QuorumAvsState {
                    quorum_num: *quorum_num,
                    total_stake,
                    agg_pub_key_g1,
                    block_num,
                },
            );
        }
        Ok(quorums_avs_state)
    }

    async fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<CheckSignaturesIndices, AvsRegistryError> {
        self.avs_registry
            .get_check_signatures_indices(
                reference_block_number,
                quorum_numbers,
                non_signer_operator_ids,
            )
            .await
    }
}

impl AvsRegistryServiceChainCaller {
    async fn get_operator_info(&self, operator_id: [u8; 32]) -> Option<OperatorPubKeys> {
        let operator_addr = self
            .avs_registry
            .get_operator_from_id(operator_id)
            .await
            .unwrap();

        self.operators_info_service
            .get_operator_info(operator_addr)
            .await
    }
}
