use alloy_primitives::{Bytes, FixedBytes, U256};
use ark_bn254::G1Projective;
use ark_ec::{short_weierstrass::Affine, AffineRepr, CurveGroup};
use eigen_client_avsregistry::{error::AvsRegistryError, reader::AvsRegistryReader};
use eigen_crypto_bls::{BlsG1Point, PublicKey};
use eigen_services_operatorsinfo::operator_info::OperatorInfoService;
use eigen_types::operator::{OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState};
use eigen_utils::binding::OperatorStateRetriever::CheckSignaturesIndices;
use std::collections::HashMap;

use crate::AvsRegistryService;

#[derive(Debug, Clone)]
pub struct AvsRegistryServiceChainCaller<R: AvsRegistryReader, S: OperatorInfoService> {
    avs_registry: R,
    operators_info_service: S,
}

impl<R: AvsRegistryReader, S: OperatorInfoService> AvsRegistryServiceChainCaller<R, S> {
    /// Create a new instance of the AvsRegistryServiceChainCaller
    ///
    /// # Arguments
    ///
    /// * `avs_registry` - The AVS registry chain reader
    /// * `operators_info_service` - The operator info service
    pub fn new(avs_registry: R, operators_info_service: S) -> Self {
        Self {
            avs_registry,
            operators_info_service,
        }
    }
}

impl<R: AvsRegistryReader, S: OperatorInfoService> AvsRegistryService
    for AvsRegistryServiceChainCaller<R, S>
{
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
            .avs_registry
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
        self.avs_registry
            .get_check_signatures_indices(
                reference_block_number,
                quorum_numbers,
                non_signer_operator_ids,
            )
            .await
    }
}

impl<R: AvsRegistryReader, S: OperatorInfoService> AvsRegistryServiceChainCaller<R, S> {
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
            .avs_registry
            .get_operator_from_id(operator_id)
            .await
            .ok()?;
        self.operators_info_service
            .get_operator_info(operator_addr)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::str::FromStr;

    use crate::AvsRegistryService;

    use super::AvsRegistryServiceChainCaller;
    use alloy_primitives::{Address, FixedBytes, U256};
    use eigen_client_avsregistry::fake_reader::FakeAvsRegistryReader;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_services_operatorsinfo::fake_operator_info::FakeOperatorInfoService;
    use eigen_types::operator::{OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState};
    use eigen_types::test::TestOperator;

    #[tokio::test]
    async fn test_get_operator_info() {
        let bls_keypair = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id = FixedBytes::<32>::from_slice(
            hex::decode("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9")
                .unwrap()
                .as_slice(),
        );
        let operator_address =
            Address::from_str("0xa0Ee7A142d267C1f36714E4a8F75612F20a79720").unwrap();
        let test_operator_1 = TestOperator {
            operator_id,
            bls_keypair: bls_keypair.clone(),
            stake_per_quorum: HashMap::from([(1u8, U256::from(123))]),
        };
        let avs_registry = FakeAvsRegistryReader::new(test_operator_1, operator_address);
        let operator_info_service = FakeOperatorInfoService::new(bls_keypair.clone());
        let service = AvsRegistryServiceChainCaller::new(avs_registry, operator_info_service);
        let operator_info = service.get_operator_info(operator_id.into()).await;
        let expected_operator_info = Some(OperatorPubKeys::from(bls_keypair));
        assert_eq!(expected_operator_info, operator_info);
    }

    #[tokio::test]
    async fn test_get_operator_avs_state() {
        let bls_keypair = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id = FixedBytes::<32>::from_slice(
            hex::decode("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9")
                .unwrap()
                .as_slice(),
        );
        let operator_address =
            Address::from_str("0xa0Ee7A142d267C1f36714E4a8F75612F20a79720").unwrap();
        let test_operator_1 = TestOperator {
            operator_id: operator_id.clone(),
            bls_keypair: bls_keypair.clone(),
            stake_per_quorum: HashMap::from([(1u8, U256::from(123))]),
        };
        let avs_registry = FakeAvsRegistryReader::new(test_operator_1, operator_address);
        let operator_info_service = FakeOperatorInfoService::new(bls_keypair.clone());
        let service = AvsRegistryServiceChainCaller::new(avs_registry, operator_info_service);
        let operator_avs_state = service
            .get_operators_avs_state_at_block(1, &[1u8])
            .await
            .unwrap();

        let expected_operator_avs_state = OperatorAvsState {
            operator_id: operator_id.into(),
            operator_info: OperatorInfo {
                pub_keys: Some(OperatorPubKeys::from(bls_keypair)),
            },
            stake_per_quorum: HashMap::from([(1u8, U256::from(123))]),
            block_num: 1.into(),
        };
        let operator_id = FixedBytes::<32>::from_slice(
            hex::decode("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9")
                .unwrap()
                .as_slice(),
        );

        let operator_state = operator_avs_state.get(&operator_id).unwrap();
        assert_eq!(expected_operator_avs_state, *operator_state);
    }

    #[tokio::test]
    async fn test_get_quorum_avs_state() {
        let bls_keypair = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id = FixedBytes::<32>::from_slice(
            hex::decode("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9")
                .unwrap()
                .as_slice(),
        );
        let operator_address =
            Address::from_str("0xa0Ee7A142d267C1f36714E4a8F75612F20a79720").unwrap();
        let test_operator_1 = TestOperator {
            operator_id,
            bls_keypair: bls_keypair.clone(),
            stake_per_quorum: HashMap::from([(1u8, U256::from(123))]),
        };
        let quorum_num = 1;
        let block_num = 1;
        let avs_registry = FakeAvsRegistryReader::new(test_operator_1, operator_address);
        let operator_info_service = FakeOperatorInfoService::new(bls_keypair.clone());
        let service = AvsRegistryServiceChainCaller::new(avs_registry, operator_info_service);
        let quorum_state_per_number = service
            .get_quorums_avs_state_at_block(&[quorum_num], 1)
            .await
            .unwrap();

        let expected_quorum_state = QuorumAvsState {
            quorum_num,
            total_stake: U256::from(123),
            agg_pub_key_g1: bls_keypair.public_key(),
            block_num,
        };
        assert_eq!(
            expected_quorum_state,
            *quorum_state_per_number.get(&(block_num as u8)).unwrap()
        );
    }
}
