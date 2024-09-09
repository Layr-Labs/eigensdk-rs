use alloy_primitives::{Bytes, FixedBytes, U256};
use ark_bn254::G1Projective;
use ark_ec::{short_weierstrass::Affine, AffineRepr, CurveGroup};
use async_trait::async_trait;
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
    /// * `operators_info_service` - The operator info service
    pub fn new(avs_registry: R, operators_info_service: S) -> Self {
        Self {
            avs_registry,
            operators_info_service,
        }
    }
}

#[async_trait]
impl<R: AvsRegistryReader + Sync, S: OperatorInfoService + Sync> AvsRegistryService
    for AvsRegistryServiceChainCaller<R, S>
{
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
                // BUG! check this. Should it be indexed by quorum_num?
                let info = self.get_operator_info(*operator.operatorId).await?;
                let stake_per_quorum = HashMap::new();
                let avs_state = operators_avs_state
                    .entry(FixedBytes(*operator.operatorId))
                    .or_insert_with(|| OperatorAvsState {
                        operator_id: *operator.operatorId,
                        operator_info: OperatorInfo {
                            pub_keys: Some(info),
                        },
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
        Ok(quorum_nums
            .iter()
            .map(|quorum_num| {
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
    async fn get_operator_info(
        &self,
        operator_id: [u8; 32],
    ) -> Result<OperatorPubKeys, AvsRegistryError> {
        let operator_addr = self.avs_registry.get_operator_from_id(operator_id).await?;

        self.operators_info_service
            .get_operator_info(operator_addr)
            .await
            .unwrap_or(None)
            .ok_or(AvsRegistryError::GetOperatorInfo)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::str::FromStr;

    use crate::AvsRegistryService;

    use super::AvsRegistryServiceChainCaller;
    use alloy_primitives::{Address, B256, U256};
    use eigen_client_avsregistry::fake_reader::{FakeAvsRegistryReader, FakeOperator};
    use eigen_crypto_bls::{BlsG1Point, BlsKeyPair};
    use eigen_services_operatorsinfo::fake_operator_info::FakeOperatorInfoService;
    use eigen_types::operator::{
        OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState, QuorumNum,
    };
    use eigen_types::test::TestOperator;
    use serde::Deserialize;

    const PRIVATE_KEY_DECIMAL: &str =
        "13710126902690889134622698668747132666439281256983827313388062967626731803599";
    const OPERATOR_ID: &str = "48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9";
    const OPERATOR_ADDRESS: &str = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720";

    fn build_test_operator() -> FakeOperator {
        FakeOperator {
            operator_id: B256::from_str(OPERATOR_ID).unwrap(),
            pubkeys: OperatorPubKeys::from(BlsKeyPair::new(PRIVATE_KEY_DECIMAL.into()).unwrap()),
            stake_per_quorum: HashMap::from([(1u8, U256::from(123))]),
        }
    }

    fn build_avs_registry_service_chaincaller(
        test_operator: FakeOperator,
    ) -> AvsRegistryServiceChainCaller<FakeAvsRegistryReader, FakeOperatorInfoService> {
        let operator_address = Address::from_str(OPERATOR_ADDRESS).unwrap();
        let avs_registry = FakeAvsRegistryReader::new(test_operator.clone(), operator_address);
        let operator_info_service = FakeOperatorInfoService::new(test_operator.pubkeys);
        AvsRegistryServiceChainCaller::new(avs_registry, operator_info_service)
    }

    #[tokio::test]
    async fn test_get_operator_info() {
        let test_operator = build_test_operator();

        let service = build_avs_registry_service_chaincaller(test_operator.clone());
        let operator_info = service
            .get_operator_info(test_operator.operator_id.into())
            .await
            .unwrap();
        let expected_operator_info = Some(test_operator.pubkeys);
        assert_eq!(expected_operator_info, Some(operator_info));
    }

    #[derive(Deserialize, Debug)]
    struct Input {
        query_quorum_numbers: Vec<QuorumNum>,
        query_block_num: u32,
    }

    #[derive(Deserialize, Debug)]
    struct TestData {
        input: Input,
    }

    #[tokio::test]
    async fn test_get_operator_avs_state() {
        let test_data_path = std::env::var("TEST_DATA_PATH").unwrap();
        let file = File::open(test_data_path).unwrap();
        let reader = BufReader::new(file);
        let data: TestData = serde_json::from_reader(reader).unwrap();

        let block_num = data.input.query_block_num;
        let quorum_nums = data.input.query_quorum_numbers.as_slice();
        let test_operator = build_test_operator();
        let service = build_avs_registry_service_chaincaller(test_operator.clone());

        let operator_avs_state = service
            .get_operators_avs_state_at_block(block_num, quorum_nums)
            .await
            .unwrap();

        // get the expected stakes for the selected quorums
        let expected_stake_per_quorum = quorum_nums
            .into_iter()
            .filter_map(|key| {
                test_operator
                    .stake_per_quorum
                    .get(&key)
                    .map(|value| (*key, value.clone()))
            })
            .collect();

        let expected_operator_avs_state = OperatorAvsState {
            operator_id: test_operator.operator_id.into(),
            operator_info: OperatorInfo {
                pub_keys: Some(test_operator.pubkeys),
            },
            stake_per_quorum: expected_stake_per_quorum,
            block_num: block_num.into(),
        };
        let operator_state = operator_avs_state.get(&test_operator.operator_id).unwrap();
        assert_eq!(expected_operator_avs_state, *operator_state);
    }

    #[tokio::test]
    async fn test_get_quorum_avs_state() {
        let test_operator = build_test_operator();
        let quorum_num = 1;
        let block_num = 1u32;
        let service = build_avs_registry_service_chaincaller(test_operator.clone());
        let quorum_state_per_number = service
            .get_quorums_avs_state_at_block(&[quorum_num], 1)
            .await
            .unwrap();

        let total_stake = test_operator
            .stake_per_quorum
            .get(&(block_num as u8))
            .unwrap();

        let expected_quorum_state = QuorumAvsState {
            quorum_num,
            total_stake: *total_stake,
            agg_pub_key_g1: test_operator.pubkeys.g1_pub_key,
            block_num,
        };
        let expected_quorum_state_per_number =
            HashMap::from([(block_num as u8, expected_quorum_state)]);

        assert_eq!(expected_quorum_state_per_number, quorum_state_per_number);
    }
}
