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
    operators_info_service: OperatorInfoServiceInMemory,
}

impl AvsRegistryServiceChainCaller {
    /// Create a new instance of the AvsRegistryServiceChainCaller
    ///
    /// # Arguments
    ///
    /// * `operators_info_service` - The operator info service
    pub fn new(
        operators_info_service: OperatorInfoServiceInMemory,
    ) -> Self {
        Self {
            operators_info_service,
        }
    }
}

impl AvsRegistryService for AvsRegistryServiceChainCaller {
    /// Get the operators AVS state at a specific block number
    ///
    /// # Arguments
    ///
    /// * `block_num` - The block number to get the AVS state at
    /// * `quorum_nums` - The list of quorum numbers
    ///
    /// # Returns
    ///
    /// A hashmap containing the operator ID and the operator AVS state
    async fn get_operators_avs_state_at_block(
        &self,
        block_num: u32,
        quorum_nums: &[u8],
    ) -> Result<HashMap<FixedBytes<32>, OperatorAvsState>, AvsRegistryError> {
        let mut operators_avs_state: HashMap<FixedBytes<32>, OperatorAvsState> = HashMap::new();

        let operators_stakes_in_quorums = self
            .operators_info_service
            .avs_registry_reader
            .get_operators_stake_in_quorums_at_block(block_num, Bytes::from(Vec::from(quorum_nums)))
            .await?;

        if operators_stakes_in_quorums.len() != quorum_nums.len() {
            // the list of quorum nums and the list of operators stakes in quorums should have the same length
            return Err(AvsRegistryError::InvalidQuorumNums);
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

    /// Get the quorum AVS state at a specific block
    ///
    /// # Arguments
    ///
    /// * `quorum_nums` - The list of quorum numbers
    /// * `block_num` - The block number
    ///
    /// # Returns
    ///
    /// A hashmap containing the quorum number and the quorum AVS state.
    async fn get_quorums_avs_state_at_block(
        &self,
        quorum_nums: &[u8],
        block_num: u32,
    ) -> Result<HashMap<u8, QuorumAvsState>, AvsRegistryError> {
        let operators_avs_state = self
            .get_operators_avs_state_at_block(block_num, quorum_nums)
            .await?;

        Ok(quorum_nums
            .iter()
            .map(|quorum_num| {
                let mut pub_key_g1 = G1Projective::from(PublicKey::identity());
                let mut total_stake: U256 = U256::from(0);
                for operator in operators_avs_state.values() {
                    if !operator.stake_per_quorum[quorum_num].is_zero() {
                        if let Some(pub_keys) = &operator.operator_info.pub_keys {
                            pub_key_g1 += pub_keys.g1_pub_key.g1();
                            total_stake += operator.stake_per_quorum[quorum_num];
                        }
                    }
                }
                let agg_pub_key_g1 = if pub_key_g1 == G1Projective::from(PublicKey::zero()) {
                    BlsG1Point::new(Affine::zero())
                } else {
                    BlsG1Point::new(pub_key_g1.into_affine())
                };

                (
                    *quorum_num,
                    QuorumAvsState {
                        quorum_num: *quorum_num,
                        total_stake,
                        agg_pub_key_g1,
                        block_num,
                    },
                )
            })
            .collect())
    }

    /// Get the signatures indices of quorum members for a specific block and checks
    /// if the indices are valid
    ///
    /// # Arguments
    ///
    /// * `reference_block_number` - The reference block number
    /// * `quorum_numbers` - The list of quorum numbers
    /// * `non_signer_operator_ids` - The list of non-signer operator ids
    ///
    /// # Returns
    ///
    /// A struct containing the indices of the quorum members that signed,
    /// and the ones that didn't
    async fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<CheckSignaturesIndices, AvsRegistryError> {
        self.operators_info_service
            .avs_registry_reader
            .get_check_signatures_indices(
                reference_block_number,
                quorum_numbers,
                non_signer_operator_ids,
            )
            .await
    }
}

impl AvsRegistryServiceChainCaller {
    /// Get the operator info from the operator id
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator id
    ///
    /// # Returns
    ///
    /// The operator public keys
    async fn get_operator_info(&self, operator_id: [u8; 32]) -> Option<OperatorPubKeys> {
        let operator_addr = self
            .operators_info_service
            .avs_registry_reader
            .get_operator_from_id(operator_id)
            .await
            .ok()?;

        self.operators_info_service
            .get_operator_info(operator_addr)
            .await
            .unwrap()
    }
}
