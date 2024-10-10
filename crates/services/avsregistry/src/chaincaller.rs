use alloy_primitives::{Bytes, FixedBytes, U256};
use ark_bn254::G1Projective;
use ark_ec::{short_weierstrass::Affine, AffineRepr, CurveGroup};
use async_trait::async_trait;
use eigen_client_avsregistry::{error::AvsRegistryError, reader::AvsRegistryReader};
use eigen_crypto_bls::{BlsG1Point, PublicKey};
use eigen_services_operatorsinfo::operator_info::OperatorInfoService;
use eigen_types::operator::{OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState};
use eigen_utils::operatorstateretriever::OperatorStateRetriever::CheckSignaturesIndices;
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
    /// Returns the operator info for the given operator id
    ///
    /// # Arguments
    ///
    /// * `operator_id` - The operator id
    ///
    /// # Returns
    ///
    /// The operator info
    ///
    /// # Errors
    ///
    /// An error is returned if the operator info is not found or can not be retrieved
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
    use std::str::FromStr;

    use super::AvsRegistryServiceChainCaller;
    use crate::AvsRegistryService;
    use alloy_primitives::{Address, FixedBytes, U256};
    use eigen_client_avsregistry::fake_reader::FakeAvsRegistryReader;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_services_operatorsinfo::fake_operator_info::FakeOperatorInfoService;
    use eigen_testing_utils::test_data::TestData;
    use eigen_types::operator::{
        OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumAvsState, QuorumNum,
    };
    use eigen_types::test::TestOperator;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    struct InputOperatorInfo {
        private_key_decimal: String,
        operator_id: String,
        operator_address: String,
    }

    #[derive(Deserialize, Debug)]
    struct InputOperatorAvsState {
        quorum_numbers: Vec<QuorumNum>,
        block_num: u32,
        private_key_decimal: String,
        operator_id: String,
        operator_address: String,
    }

    const PRIVATE_KEY_DECIMAL: &str =
        "13710126902690889134622698668747132666439281256983827313388062967626731803599";
    const OPERATOR_ID: &str = "48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9";
    const OPERATOR_ADDRESS: &str = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720";

    /// Helper function to build a TestOperator given a private key and an operator id
    ///
    /// # Arguments
    ///
    /// * `private_key_decimal` - The private key of the operator in decimal format
    /// * `operator_id` - The operator id
    ///
    /// # Returns
    ///
    /// The TestOperator element
    fn build_test_operator(private_key_decimal: &str, operator_id: &str) -> TestOperator {
        let bls_keypair = BlsKeyPair::new(private_key_decimal.into()).unwrap();
        let operator_id =
            FixedBytes::<32>::from_slice(hex::decode(operator_id).unwrap().as_slice());
        TestOperator {
            operator_id,
            bls_keypair: bls_keypair.clone(),
            stake_per_quorum: HashMap::from([(1u8, U256::from(123))]),
        }
    }

    fn build_avs_registry_service_chaincaller(
        test_operator: TestOperator,
        operator_address: &str,
    ) -> AvsRegistryServiceChainCaller<FakeAvsRegistryReader, FakeOperatorInfoService> {
        let operator_address = Address::from_str(operator_address).unwrap();
        let avs_registry = FakeAvsRegistryReader::new(test_operator.clone(), operator_address);
        let operator_info_service = FakeOperatorInfoService::new(test_operator.bls_keypair.clone());
        AvsRegistryServiceChainCaller::new(avs_registry, operator_info_service)
    }

    #[tokio::test]
    async fn test_get_operator_pubkeys() {
        let default_input = InputOperatorInfo {
            private_key_decimal: PRIVATE_KEY_DECIMAL.to_owned(),
            operator_id: OPERATOR_ID.to_owned(),
            operator_address: OPERATOR_ADDRESS.to_owned(),
        };
        let test_data: TestData<InputOperatorInfo> = TestData::new(default_input);

        let test_operator = build_test_operator(
            test_data.input.private_key_decimal.as_str(),
            test_data.input.operator_id.as_str(),
        );
        let bls_keypair = test_operator.bls_keypair.clone();

        let service = build_avs_registry_service_chaincaller(
            test_operator.clone(),
            test_data.input.operator_address.as_str(),
        );
        let operator_info = service
            .get_operator_info(test_operator.operator_id.into())
            .await
            .unwrap();
        let expected_operator_info = Some(OperatorPubKeys::from(bls_keypair));
        assert_eq!(expected_operator_info, Some(operator_info));
    }

    #[tokio::test]
    async fn test_get_operators_avs_state() {
        let default_input = InputOperatorAvsState {
            quorum_numbers: vec![1],
            block_num: 1,
            private_key_decimal: PRIVATE_KEY_DECIMAL.to_owned(),
            operator_id: OPERATOR_ID.to_owned(),
            operator_address: OPERATOR_ADDRESS.to_owned(),
        };
        let test_data: TestData<InputOperatorAvsState> = TestData::new(default_input);

        let test_operator = build_test_operator(
            test_data.input.private_key_decimal.as_str(),
            test_data.input.operator_id.as_str(),
        );
        let service = build_avs_registry_service_chaincaller(
            test_operator.clone(),
            test_data.input.operator_address.as_str(),
        );

        let operator_avs_state = service
            .get_operators_avs_state_at_block(
                test_data.input.block_num,
                &test_data.input.quorum_numbers,
            )
            .await
            .unwrap();

        let expected_operator_avs_state = OperatorAvsState {
            operator_id: test_operator.operator_id.into(),
            operator_info: OperatorInfo {
                pub_keys: Some(OperatorPubKeys::from(test_operator.bls_keypair)),
            },
            stake_per_quorum: test_operator.stake_per_quorum,
            block_num: test_data.input.block_num.into(),
        };
        let operator_state = operator_avs_state.get(&test_operator.operator_id).unwrap();
        assert_eq!(expected_operator_avs_state, *operator_state);
    }

    #[tokio::test]
    async fn test_get_quorum_avs_state() {
        let test_operator = build_test_operator(PRIVATE_KEY_DECIMAL, OPERATOR_ID);
        let quorum_num = 1;
        let block_num = 1u32;
        let service =
            build_avs_registry_service_chaincaller(test_operator.clone(), OPERATOR_ADDRESS);
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
            agg_pub_key_g1: test_operator.bls_keypair.public_key(),
            block_num,
        };
        let expected_quorum_state_per_number =
            HashMap::from([(block_num as u8, expected_quorum_state)]);

        assert_eq!(expected_quorum_state_per_number, quorum_state_per_number);
    }
}
