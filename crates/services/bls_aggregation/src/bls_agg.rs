use alloy_primitives::{FixedBytes, U256};
use ark_bn254::G2Affine;
use ark_ec::{AffineRepr, CurveGroup};
use eigen_crypto_bls::BlsG1Point;
use eigen_crypto_bls::{BlsG2Point, Signature};
use eigen_crypto_bn254::utils::verify_message;
use eigen_services_avsregistry::AvsRegistryService;
use eigen_types::{
    avs::{SignedTaskResponseDigest, TaskIndex, TaskResponseDigest},
    operator::{OperatorAvsState, QuorumThresholdPercentage, QuorumThresholdPercentages},
};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::time::{self, Duration};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlsAggregationServiceResponse {
    task_index: TaskIndex,
    task_response_digest: TaskResponseDigest,
    non_signers_pub_keys_g1: Vec<BlsG1Point>,
    quorum_apks_g1: Vec<BlsG1Point>,
    signers_apk_g2: BlsG2Point,
    signers_agg_sig_g1: Signature,
    non_signer_quorum_bitmap_indices: Vec<u32>,
    quorum_apk_indices: Vec<u32>,
    total_stake_indices: Vec<u32>,
    non_signer_stake_indices: Vec<Vec<u32>>,
}

#[derive(Debug, Clone)]
pub struct AggregatedOperators {
    signers_apk_g2: BlsG2Point,
    signers_agg_sig_g1: Signature,
    signers_total_stake_per_quorum: HashMap<u8, U256>,
    pub signers_operator_ids_set: HashMap<FixedBytes<32>, bool>,
}

#[derive(Debug, Clone)]
pub struct BlsAggregatorService<A: AvsRegistryService>
where
    A: Clone,
{
    aggregated_response_sender: UnboundedSender<BlsAggregationServiceResponse>,
    pub aggregated_response_receiver: Arc<Mutex<UnboundedReceiver<BlsAggregationServiceResponse>>>,
    signed_task_response:
        Arc<RwLock<HashMap<TaskIndex, UnboundedSender<SignedTaskResponseDigest>>>>,

    avs_registry_service: A,
}

impl<A: AvsRegistryService + Send + Sync + Clone + 'static> BlsAggregatorService<A> {
    pub fn new(avs_registry_service: A) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            aggregated_response_sender: tx,
            aggregated_response_receiver: Arc::new(Mutex::new(rx)),
            signed_task_response: Arc::new(RwLock::new(HashMap::new())),
            avs_registry_service,
        }
    }

    pub async fn initialize_new_task<S: AvsRegistryService>(
        &self,
        task_index: TaskIndex,
        task_created_block: u32,
        quorum_nums: Vec<u8>,
        quorum_threshold_percentages: QuorumThresholdPercentages,
        time_to_expiry: Duration,
    ) {
        let mut task_channel = self.signed_task_response.write();

        if task_channel.contains_key(&task_index) {
            // error
        }

        let (tx, mut rx) = mpsc::unbounded_channel();
        task_channel.insert(task_index, tx);
        let self_clone = self.clone();
        let mut i = 0;
        tokio::spawn(async move {
            while let Some(signed_response) = rx.recv().await {
                dbg!(i);
                i += 1;
                // Process each signed response here
                self_clone
                    .single_task_aggregator(
                        task_index,
                        task_created_block,
                        quorum_nums.clone(),
                        quorum_threshold_percentages.clone(),
                        time_to_expiry,
                        signed_response,
                    )
                    .await;
            }
        });
    }

    /// Processs signatures
    pub async fn process_new_signature(
        &self,
        task_index: TaskIndex,
        task_response_digest: TaskResponseDigest,
        bls_signature: Signature,
        operator_id: FixedBytes<32>,
    ) {
        let task_channel = self.signed_task_response.read();

        if let Some(sender) = task_channel.get(&task_index) {
            dbg!(&task_index);
            let task = SignedTaskResponseDigest {
                task_response_digest,
                bls_signature,
                operator_id,
            };
            let _x = sender.send(task);
        }
    }

    pub async fn single_task_aggregator(
        &self,
        task_index: TaskIndex,
        task_created_block: u32,
        quorum_nums: Vec<u8>,
        quorum_threshold_percentages: QuorumThresholdPercentages,
        time_to_expiry: Duration,
        signed_task_digest: SignedTaskResponseDigest,
    ) {
        let mut quorum_threshold_percentage_map = HashMap::new();

        for (i, quorum_number) in quorum_nums.iter().enumerate() {
            quorum_threshold_percentage_map.insert(*quorum_number, quorum_threshold_percentages[i]);
        }

        let mut operator_state_avs = self
            .avs_registry_service
            .get_operators_avs_state_at_block(task_created_block, quorum_nums.clone().into())
            .await;
        // throw erro if
        let quorums_avs_stake = self
            .avs_registry_service
            .get_quorums_avs_state_at_block(quorum_nums.clone().into(), task_created_block)
            .await;
        // throw erro if != nil
        let mut total_stake_per_quorum = HashMap::new();

        for (quorum_num, quorum_avs_stake) in &quorums_avs_stake {
            total_stake_per_quorum.insert(*quorum_num, quorum_avs_stake.total_stake);
        }
        let mut quorum_apks_g1: Vec<BlsG1Point> = vec![];
        for quorum_number in quorum_nums.iter() {
            if let Some(val) = quorums_avs_stake.get(quorum_number) {
                quorum_apks_g1.push(val.agg_pub_key_g1.clone());
            }
        }

        let task_expired_timer = time::sleep(time_to_expiry);

        tokio::pin!(task_expired_timer);
        let mut aggregated_operators: HashMap<FixedBytes<32>, AggregatedOperators> = HashMap::new();
        self.verify_signature(task_index, &signed_task_digest, &operator_state_avs)
            .await;

        let start = Instant::now();
        loop {
            // tokio::select! {
            //     _ = &mut task_expired_timer => {
            //         // todo tracing
            //         println!("time expired");
            //         break;
            //     },
            // _ = async {} => {
            let now = Instant::now();
            if now - start > time_to_expiry {
                break;
            }
            let mut aggregate_response: AggregatedOperators;
            let response = match aggregated_operators.get(&signed_task_digest.task_response_digest)
            {
                None => {
                    let operator_state = operator_state_avs
                        .get(&signed_task_digest.operator_id)
                        .unwrap();
                    let operator_g2_pubkey: BlsG2Point = operator_state
                        .operator_info
                        .pub_keys
                        .clone()
                        .unwrap()
                        .g2_pub_key;
                    &AggregatedOperators {
                        signers_apk_g2: BlsG2Point::new(
                            (G2Affine::zero() + operator_g2_pubkey.g2()).into(),
                        ),
                        signers_agg_sig_g1: signed_task_digest.bls_signature.clone(),
                        signers_operator_ids_set: HashMap::from([(
                            signed_task_digest.operator_id,
                            true,
                        )]),
                        signers_total_stake_per_quorum: operator_state.stake_per_quorum.clone(),
                    }
                }
                Some(response) => {
                    // TODO: add this
                    // digestAggregatedOperators.signersAggSigG1.Add(signedTaskResponseDigest.BlsSignature)
                    // digestAggregatedOperators.signersApkG2.Add(operatorsAvsStateDict[signedTaskResponseDigest.OperatorId].OperatorInfo.Pubkeys.G2Pubkey)
                    // digestAggregatedOperators.signersOperatorIdsSet[signedTaskResponseDigest.OperatorId] = true
                    // for quorumNum, stake := range operatorsAvsStateDict[signedTaskResponseDigest.OperatorId].StakePerQuorum {
                    //     if _, ok := digestAggregatedOperators.signersTotalStakePerQuorum[quorumNum]; !ok {
                    //         // if we haven't seen this quorum before, initialize its signed stake to 0
                    //         // possible if previous operators who sent us signatures were not part of this quorum
                    //         digestAggregatedOperators.signersTotalStakePerQuorum[quorumNum] = big.NewInt(0)
                    //     }
                    //     digestAggregatedOperators.signersTotalStakePerQuorum[quorumNum].Add(digestAggregatedOperators.signersTotalStakePerQuorum[quorumNum], stake)
                    // }
                    response
                }
            };
            aggregate_response = response.clone();
            if aggregated_operators.contains_key(&signed_task_digest.task_response_digest) {
                aggregate_response.signers_agg_sig_g1 = Signature::new(
                    (aggregate_response
                        .signers_agg_sig_g1
                        .g1_point()
                        .g1()
                        .into_group()
                        + signed_task_digest.bls_signature.g1_point().g1())
                    .into_affine(),
                );

                if let Some(op_avs_state) =
                    operator_state_avs.get_mut(&signed_task_digest.operator_id)
                {
                    if let Some(pub_key) = &op_avs_state.operator_info.pub_keys {
                        aggregate_response.signers_apk_g2 = BlsG2Point::new(
                            (aggregate_response.signers_apk_g2.g2() + pub_key.g2_pub_key.g2())
                                .into(),
                        );
                    }
                }

                aggregate_response
                    .signers_operator_ids_set
                    .insert(FixedBytes(*signed_task_digest.operator_id), true);

                if let Some(state_avs) = operator_state_avs.get(&signed_task_digest.operator_id) {
                    for (quorum_num, stake) in state_avs.stake_per_quorum.clone() {
                        if aggregate_response
                            .signers_total_stake_per_quorum
                            .get(&quorum_num)
                            .is_some()
                        {
                            aggregate_response
                                .signers_total_stake_per_quorum
                                .insert(quorum_num, U256::from(0));
                        }
                        aggregate_response.signers_total_stake_per_quorum.insert(
                            quorum_num,
                            aggregate_response.signers_total_stake_per_quorum[&quorum_num] + stake,
                        );
                    }
                }
            } else {
                dbg!("else");
                let mut operator_id_set = HashMap::new();
                operator_id_set.insert(signed_task_digest.operator_id, true);
                // first operator

                if let Some(avs_state) =
                    operator_state_avs.get(&signed_task_digest.operator_id.clone())
                {
                    aggregate_response = AggregatedOperators {
                        signers_agg_sig_g1: signed_task_digest.bls_signature.clone(),
                        // bls_keypair.public_key_g2
                        signers_apk_g2: bls_keypair.public_key_g2,
                        // BlsG2Point::new(G2Affine::zero()),
                        signers_operator_ids_set: operator_id_set,
                        signers_total_stake_per_quorum: avs_state.stake_per_quorum.clone(),
                    };
                }
            }

            aggregated_operators.insert(
                signed_task_digest.task_response_digest,
                aggregate_response.clone(),
            );

            // check stake threshold
            if self.check_if_stake_thresholds_met(
                aggregate_response.signers_total_stake_per_quorum,
                total_stake_per_quorum.clone(),
                quorum_threshold_percentage_map.clone(),
            ) {
                let mut non_signers_operators_ids: Vec<FixedBytes<32>> = vec![];
                for (i, op_info) in &operator_state_avs {
                    if aggregate_response
                        .signers_operator_ids_set
                        .contains_key(&op_info.operator_id)
                    {
                        non_signers_operators_ids.push(*i);
                    }
                }

                non_signers_operators_ids.sort_by(|a, b| a.cmp(b));

                let mut non_signers_g1_pub_keys: Vec<BlsG1Point> = vec![];
                for operator_id in non_signers_operators_ids.iter() {
                    if let Some(operator) = operator_state_avs.get(operator_id) {
                        if let Some(keys) = &operator.operator_info.pub_keys {
                            non_signers_g1_pub_keys.push(keys.g1_pub_key.clone());
                        }
                    }
                }

                let indices = self
                    .avs_registry_service
                    // .get_avs_registry()
                    .get_check_signatures_indices(
                        task_created_block,
                        quorum_nums.clone(),
                        non_signers_operators_ids,
                    )
                    .await
                    .unwrap();

                let bls_aggregation_service_response = BlsAggregationServiceResponse {
                    task_index,
                    task_response_digest: signed_task_digest.task_response_digest,
                    non_signers_pub_keys_g1: non_signers_g1_pub_keys,
                    quorum_apks_g1: quorum_apks_g1.clone(),
                    signers_apk_g2: aggregate_response.signers_apk_g2,
                    signers_agg_sig_g1: aggregate_response.signers_agg_sig_g1,
                    non_signer_quorum_bitmap_indices: indices.clone().quorumApkIndices,
                    quorum_apk_indices: indices.quorumApkIndices,
                    total_stake_indices: indices.totalStakeIndices,
                    non_signer_stake_indices: indices.nonSignerStakeIndices,
                };

                let _ = self
                    .aggregated_response_sender
                    .send(bls_aggregation_service_response);
                //     }
                // }
            }
        }
    }

    pub async fn verify_signature(
        &self,
        _task_index: TaskIndex,
        signed_task_response_digest: &SignedTaskResponseDigest,
        operator_avs_state: &HashMap<FixedBytes<32>, OperatorAvsState>,
    ) {
        let Some(operator_state) = operator_avs_state.get(&signed_task_response_digest.operator_id)
        else {
            todo!() // throw error operator not found
        };

        let Some(pub_keys) = &operator_state.operator_info.pub_keys else {
            todo!() // throw error operator pub key not found
        };

        let signature_verified = verify_message(
            pub_keys.g2_pub_key.g2(),
            signed_task_response_digest.task_response_digest.as_slice(),
            signed_task_response_digest.bls_signature.g1_point().g1(),
        );

        if !signature_verified {
            todo!() // throw incorrect signature error
        }
    }

    pub fn check_if_stake_thresholds_met(
        &self,
        signed_stake_per_quorum: HashMap<u8, U256>,
        total_stake_per_quorum: HashMap<u8, U256>,
        quorum_threshold_percentages_map: HashMap<u8, QuorumThresholdPercentage>,
    ) -> bool {
        let Some((quorum_num, quorum_threshold_percentage)) =
            quorum_threshold_percentages_map.into_iter().next()
        else {
            return true;
        };

        // TODO: check if quorum num <= u8 max assert
        let Some(signed_stake_by_quorum) = signed_stake_per_quorum.get(&quorum_num) else {
            return false;
        };

        let Some(total_stake_by_quorum) = total_stake_per_quorum.get(&quorum_num) else {
            return false;
        };

        let signed_stake = signed_stake_by_quorum * U256::from(100);
        let threshold_stake = *total_stake_by_quorum * U256::from(quorum_threshold_percentage);

        signed_stake >= threshold_stake
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{B256, U256};
    use core::panic;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_services_avsregistry::fake_avs_registry_service::FakeAvsRegistryService;
    use eigen_types::operator::QuorumThresholdPercentages;
    use eigen_types::{avs::TaskIndex, test::TestOperator};
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::time::Duration;
    use std::vec;

    use super::BlsAggregatorService;

    fn new_bls_key_pair_panics(hex_key: String) -> BlsKeyPair {
        let keypair = BlsKeyPair::new(hex_key);
        match keypair {
            Err(_) => panic!(),
            Ok(keypair) => keypair,
        }
    }

    fn hash(task_response: u64) -> B256 {
        // TODO: add marshalling
        let mut hasher = Sha256::new();
        hasher.update(task_response.to_be_bytes());
        B256::from_slice(hasher.finalize().as_ref())
    }

    #[tokio::test]
    async fn one_quorum_one_operator_one_correct_signature() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };

        let block_number = 1;
        let task_index: TaskIndex = 0;
        let quorum_numbers = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data

        // Compute the TaskResponseDigest as the SHA-256 sum of the TaskResponse (previously converting the taskresponse into a JSON string)
        let task_response_digest = hash(task_response);
        // assert!(task_response_digest.is_ok());
        let bls_signature = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, vec![test_operator_1.clone()]);
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);
        bls_agg_service
            .initialize_new_task::<FakeAvsRegistryService>(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await;

        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature,
                test_operator_1.operator_id,
            )
            .await;

        //     require.Equal(t, wantAggregationServiceResponse, gotAggregationServiceResponse)
        //     require.EqualValues(t, taskIndex, gotAggregationServiceResponse.TaskIndex)
        use crate::bls_agg::BlsAggregationServiceResponse;
        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest,
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1: vec![test_operator_1.bls_keypair.public_key()],
            signers_apk_g2: test_operator_1.bls_keypair.public_key_g2(),
            signers_agg_sig_g1: test_operator_1
                .bls_keypair
                .sign_message(task_response_digest.as_ref()),
            non_signer_quorum_bitmap_indices: vec![],
            quorum_apk_indices: vec![],
            total_stake_indices: vec![],
            non_signer_stake_indices: vec![],
        };

        let response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await;
        dbg!(&expected_agg_service_response);
        dbg!(&response);
        assert_eq!(expected_agg_service_response, response.unwrap());
    }
}
