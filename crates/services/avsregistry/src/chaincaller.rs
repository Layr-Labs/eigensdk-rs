use alloy_primitives::{Bytes, FixedBytes, U256};
use ark_bn254::G1Projective;
use ark_ec::{short_weierstrass::Affine, AffineRepr, CurveGroup};
use eigen_client_avsregistry::{
    error::AvsRegistryError,
    reader::{AvsRegistryChainReader, AvsRegistryReader},
};
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
    /// Create a new instance of the AvsRegistryServiceChainCaller
    ///
    /// # Arguments
    ///
    /// * `avs_registry` - The AVS registry chain reader
    /// * `operators_info_service` - The operator info service
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
            .avs_registry
            .get_operator_from_id(operator_id)
            .await
            .ok()?;
        dbg!(operator_addr);
        self.operators_info_service
            .get_operator_info(operator_addr)
            .await
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::AvsRegistryServiceChainCaller;
    use alloy_primitives::{Bytes, FixedBytes, U256};
    use eigen_client_avsregistry::reader::AvsRegistryChainReader;
    use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::get_test_logger;
    use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
    use eigen_testing_utils::anvil_constants::{
        get_operator_state_retriever_address, get_registry_coordinator_address,
    };
    use eigen_types::operator::OperatorPubKeys;
    use eigen_types::test::TestOperator;

    #[tokio::test]
    async fn test_get_operator_keys() {
        let bls_keypair = BlsKeyPair::new(
            "13710126902690889134622698668747132666439281256983827313388062967626731803599".into(),
        )
        .unwrap();
        let operator_id = FixedBytes::<32>::from_slice(
            hex::decode("48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9")
                .unwrap()
                .as_slice(),
        );

        let test_operator_1 = TestOperator {
            operator_id,
            bls_keypair: bls_keypair.clone(),
            stake_per_quorum: HashMap::default(),
        };

        let registry_coordinator_addr = get_registry_coordinator_address().await;
        let operator_state_retriever_addr = get_operator_state_retriever_address().await;

        let avs_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            "http://localhost:8545".into(),
            "2a871d0798f97d79848a013d4936a73bf4cc922c825d33c1cf7073dff6d409c6".into(),
            registry_coordinator_addr,
            operator_state_retriever_addr,
        )
        .await
        .unwrap();
        let quorum_nums = Bytes::from([0]);
        let salt = [0x02; 32].into();
        avs_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_keypair,
                salt,
                U256::MAX,
                quorum_nums,
                "".into(),
            )
            .await
            .unwrap();

        let reader = AvsRegistryChainReader::new(
            get_test_logger(),
            registry_coordinator_addr,
            operator_state_retriever_addr,
            "http://localhost:8545".into(),
        )
        .await
        .unwrap();

        let op_info_service = OperatorInfoServiceInMemory::new(
            get_test_logger(),
            reader.clone(),
            "ws://localhost:8545".into(),
        )
        .await;
        let (tx, rx) = tokio::sync::watch::channel(());
        op_info_service.start_service(200, 205, rx).await.unwrap();

        let avs_registry_service = AvsRegistryServiceChainCaller::new(reader, op_info_service);
        let operator_info = avs_registry_service
            .get_operator_info(test_operator_1.operator_id.into())
            .await
            .unwrap();
        let expected_operator_info = OperatorPubKeys::from(test_operator_1.bls_keypair);

        assert_eq!(operator_info, expected_operator_info);
    }
}

// func TestAvsRegistryServiceChainCaller_getOperatorPubkeys(t *testing.T) {
// 	logger := testutils.GetTestLogger()
// 	testOperator1 := fakes.TestOperator{
// 		OperatorAddr: common.HexToAddress("0x1"),
// 		OperatorId:   types.OperatorId{1},
// 		OperatorInfo: types.OperatorInfo{
// 			Pubkeys: types.OperatorPubkeys{
// 				G1Pubkey: bls.NewG1Point(big.NewInt(1), big.NewInt(1)),
// 				G2Pubkey: bls.NewG2Point(
// 					[2]*big.Int{big.NewInt(1), big.NewInt(1)},
// 					[2]*big.Int{big.NewInt(1), big.NewInt(1)},
// 				),
// 			},
// 			Socket: "localhost:8080",
// 		},
// 	}

// 	// TODO(samlaf): add error test cases
// 	var tests = []struct {
// 		name             string
// 		operator         *fakes.TestOperator
// 		queryOperatorId  types.OperatorId
// 		wantErr          error
// 		wantOperatorInfo types.OperatorInfo
// 	}{
// 		{
// 			name:             "should return operator info",
// 			operator:         &testOperator1,
// 			queryOperatorId:  testOperator1.OperatorId,
// 			wantErr:          nil,
// 			wantOperatorInfo: testOperator1.OperatorInfo,
// 		},
// 	}

// 	for _, tt := range tests {
// 		t.Run(tt.name, func(t *testing.T) {
// 			// Create mocks
// 			mockAvsRegistryReader := fakes.NewFakeAVSRegistryReader(tt.operator, nil)
// 			mockOperatorsInfoService := newFakeOperatorInfoService(tt.operator.OperatorInfo)

// 			// Create a new instance of the avsregistry service
// 			service := NewAvsRegistryServiceChainCaller(mockAvsRegistryReader, mockOperatorsInfoService, logger)

// 			// Call the GetOperatorPubkeys method with the test operator address
// 			gotOperatorInfo, gotErr := service.getOperatorInfo(context.Background(), tt.queryOperatorId)
// 			if !errors.Is(gotErr, tt.wantErr) {
// 				t.Fatalf("GetOperatorPubkeys returned wrong error. Got: %v, want: %v.", gotErr, tt.wantErr)
// 			}
// 			if tt.wantErr == nil && !reflect.DeepEqual(tt.wantOperatorInfo, gotOperatorInfo) {
// 				t.Fatalf(
// 					"GetOperatorPubkeys returned wrong operator pubkeys. Got: %v, want: %v.",
// 					gotOperatorInfo,
// 					tt.wantOperatorInfo,
// 				)
// 			}
// 		})
// 	}
// }
