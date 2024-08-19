use alloy_primitives::{FixedBytes, U256};
use ark_bn254::G2Affine;
use ark_ec::AffineRepr;
use eigen_crypto_bls::BlsG1Point;
use eigen_crypto_bls::{BlsG2Point, Signature};
use eigen_crypto_bn254::utils::verify_message;
use eigen_services_avsregistry::AvsRegistryService;
use eigen_types::{
    avs::{SignedTaskResponseDigest, TaskIndex, TaskResponseDigest},
    operator::{OperatorAvsState, QuorumThresholdPercentage, QuorumThresholdPercentages},
};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, UnboundedReceiver, UnboundedSender},
    Mutex,
};
use tokio::time::{timeout, Duration};

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

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BlsAggregationServiceError {
    #[error("task expired error")]
    TaskExpired,
    #[error("task not found error")]
    TaskNotFound,
    #[error("incorrect signature error")]
    IncorrectSignature,
    #[error("operator public key not found")]
    OperatorPublicKeyNotFound,
    #[error("operator not found")]
    OperatorNotFound,
    #[error("channel was closed")]
    ChannelClosed,
    #[error("error sendinq to channel")]
    ChannelError,
    #[error("Avs Registry Error")]
    RegistryError,
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
    aggregated_response_sender:
        UnboundedSender<Result<BlsAggregationServiceResponse, BlsAggregationServiceError>>,
    pub aggregated_response_receiver: Arc<
        Mutex<UnboundedReceiver<Result<BlsAggregationServiceResponse, BlsAggregationServiceError>>>,
    >,
    signed_task_response:
        Arc<RwLock<HashMap<TaskIndex, UnboundedSender<SignedTaskResponseDigest>>>>,

    avs_registry_service: A,
}

impl<A: AvsRegistryService + Send + Sync + Clone + 'static> BlsAggregatorService<A> {
    /// Creates a new instance of the BlsAggregatorService with the given AVS registry service
    ///
    /// Creates a tokio unbounded_channel to send and received aggregated responses.
    ///
    /// # Arguments
    ///
    /// * `avs_registry_service` - The AVS registry service
    pub fn new(avs_registry_service: A) -> Self {
        let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
        Self {
            aggregated_response_sender: tx,
            aggregated_response_receiver: Arc::new(Mutex::new(rx)),
            signed_task_response: Arc::new(RwLock::new(HashMap::new())),
            avs_registry_service,
        }
    }

    ///  Creates a new task meant to process new signed task responses for a task tokio channel.
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task
    /// * `task_created_block` - The block number at which the task was created
    /// * `quorum_nums` - The quorum numbers for the task
    /// * `quorum_threshold_percentages` - The quorum threshold percentages for the task
    /// * `time_to_expiry` - The timemetout for the task reader to expire
    ///
    /// # Error
    ///
    /// Returns error if the task index already exists
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

        let (tx, rx) = mpsc::unbounded_channel();
        task_channel.insert(task_index, tx);
        let self_clone = self.clone();
        tokio::spawn(async move {
            // Process each signed response here
            let _ = self_clone
                .single_task_aggregator(
                    task_index,
                    task_created_block,
                    quorum_nums.clone(),
                    quorum_threshold_percentages.clone(),
                    time_to_expiry,
                    rx,
                )
                .await
                .inspect_err(|err| {
                    println!("Error: {:?}", err);
                });
        });
    }

    /// Processs signatures received from the channel and sends
    /// the signed task response to the task channel.
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task
    /// * `task_response_digest` - The digest of the task response
    /// * `bls_signature` - The BLS signature of the task response
    /// * `operator_id` - The operator ID of the operator that signed the task response
    pub async fn process_new_signature(
        &self,
        task_index: TaskIndex,
        task_response_digest: TaskResponseDigest,
        bls_signature: Signature,
        operator_id: FixedBytes<32>,
    ) -> Result<(), BlsAggregationServiceError> {
        let task_channel = self.signed_task_response.read();

        let sender = task_channel
            .get(&task_index)
            .ok_or(BlsAggregationServiceError::TaskNotFound)?;

        let task = SignedTaskResponseDigest {
            task_response_digest,
            bls_signature,
            operator_id,
        };
        let _x = sender.send(task);
        Ok(())
    }

    /// Processes each signed task responses given a task_index for a single task.
    ///
    /// It reads the signed task responses from the receiver channel and aggregates them.
    /// * If the quorum threshold is met, it sends the aggregated response to the aggregated response sender.
    /// * If the time to expiry is reached, it sends a task expired error to the aggregated response sender.
    /// * If the signature is incorrect, it sends an incorrect signature error to error channel.
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task
    /// * `task_created_block` - The block number at which the task was created
    /// * `quorum_nums` - The quorum numbers for the task
    /// * `quorum_threshold_percentages` - The quorum threshold percentages for the task
    /// * `time_to_expiry` - The timeout for the task reader to expire
    /// * `rx` - The receiver channel for the signed task responses
    pub async fn single_task_aggregator(
        &self,
        task_index: TaskIndex,
        task_created_block: u32,
        quorum_nums: Vec<u8>,
        quorum_threshold_percentages: QuorumThresholdPercentages,
        time_to_expiry: Duration,
        mut rx: UnboundedReceiver<SignedTaskResponseDigest>,
    ) -> Result<(), BlsAggregationServiceError> {
        let mut quorum_threshold_percentage_map = HashMap::new();

        for (i, quorum_number) in quorum_nums.iter().enumerate() {
            quorum_threshold_percentage_map.insert(*quorum_number, quorum_threshold_percentages[i]);
        }

        let operator_state_avs = self
            .avs_registry_service
            .get_operators_avs_state_at_block(task_created_block, quorum_nums.clone().into())
            .await;
        // TODO: throw erro if
        let quorums_avs_stake = self
            .avs_registry_service
            .get_quorums_avs_state_at_block(quorum_nums.clone().into(), task_created_block)
            .await;
        // TODO: throw erro if != nil

        let total_stake_per_quorum: HashMap<_, _> = quorums_avs_stake
            .iter()
            .map(|(k, v)| (*k, v.total_stake))
            .collect();

        let mut quorum_apks_g1: Vec<BlsG1Point> = vec![];
        for quorum_number in quorum_nums.iter() {
            if let Some(val) = quorums_avs_stake.get(quorum_number) {
                quorum_apks_g1.push(val.agg_pub_key_g1.clone());
            }
        }
        let mut aggregated_operators: HashMap<FixedBytes<32>, AggregatedOperators> = HashMap::new();

        loop {
            let signed_task_digest = timeout(time_to_expiry, rx.recv())
                .await
                .inspect_err(|_err| {
                    // timeout
                    println!("expire");
                    let _ = self
                        .aggregated_response_sender
                        .send(Err(BlsAggregationServiceError::TaskExpired));
                })
                .map_err(|_| BlsAggregationServiceError::TaskExpired)?
                .ok_or(BlsAggregationServiceError::ChannelClosed)?;

            // TODO: handle error
            self.verify_signature(task_index, &signed_task_digest, &operator_state_avs)
                .await;

            let operator_state = operator_state_avs
                .get(&signed_task_digest.operator_id)
                .unwrap();
            let operator_g2_pubkey = operator_state
                .operator_info
                .pub_keys
                .clone()
                .unwrap()
                .g2_pub_key
                .g2();

            let response = match aggregated_operators
                .get_mut(&signed_task_digest.task_response_digest)
            {
                None => &AggregatedOperators {
                    signers_apk_g2: BlsG2Point::new((G2Affine::zero() + operator_g2_pubkey).into()),
                    signers_agg_sig_g1: signed_task_digest.bls_signature.clone(),
                    signers_operator_ids_set: HashMap::from([(
                        operator_state.operator_id.into(),
                        true,
                    )]),
                    signers_total_stake_per_quorum: operator_state.stake_per_quorum.clone(),
                },
                Some(digest_aggregated_operators) => {
                    digest_aggregated_operators.signers_agg_sig_g1 = Signature::new(
                        (digest_aggregated_operators
                            .signers_agg_sig_g1
                            .g1_point()
                            .g1()
                            + signed_task_digest.bls_signature.g1_point().g1())
                        .into(),
                    );
                    digest_aggregated_operators.signers_apk_g2 = BlsG2Point::new(
                        (digest_aggregated_operators.signers_apk_g2.g2() + operator_g2_pubkey)
                            .into(),
                    );
                    digest_aggregated_operators
                        .signers_operator_ids_set
                        .insert(signed_task_digest.operator_id, true);
                    for (quorum_num, stake) in operator_state.stake_per_quorum.iter() {
                        digest_aggregated_operators
                            .signers_total_stake_per_quorum
                            .entry(*quorum_num)
                            .and_modify(|v| *v += stake)
                            .or_insert(*stake);
                    }
                    digest_aggregated_operators
                }
            };

            let digest_aggregated_operators = response.clone();
            aggregated_operators.insert(
                signed_task_digest.task_response_digest,
                digest_aggregated_operators.clone(),
            );

            if !self.check_if_stake_thresholds_met(
                digest_aggregated_operators.signers_total_stake_per_quorum,
                total_stake_per_quorum.clone(),
                quorum_threshold_percentage_map.clone(),
            ) {
                continue;
            }

            let mut non_signers_operators_ids: Vec<FixedBytes<32>> = operator_state_avs
                .keys()
                .filter(|operator_id| {
                    digest_aggregated_operators
                        .signers_operator_ids_set
                        .contains_key(*operator_id)
                })
                .cloned()
                .collect::<Vec<_>>();

            non_signers_operators_ids.sort();

            let mut non_signers_pub_keys_g1: Vec<BlsG1Point> = vec![];
            for operator_id in non_signers_operators_ids.iter() {
                if let Some(operator) = operator_state_avs.get(operator_id) {
                    if let Some(keys) = &operator.operator_info.pub_keys {
                        non_signers_pub_keys_g1.push(keys.g1_pub_key.clone());
                    }
                }
            }

            let indices = self
                .avs_registry_service
                .get_check_signatures_indices(
                    task_created_block,
                    quorum_nums.clone(),
                    non_signers_operators_ids,
                )
                .await
                .map_err(|_err| BlsAggregationServiceError::RegistryError)?;

            let bls_aggregation_service_response = BlsAggregationServiceResponse {
                task_index,
                task_response_digest: signed_task_digest.task_response_digest,
                non_signers_pub_keys_g1,
                quorum_apks_g1: quorum_apks_g1.clone(),
                signers_apk_g2: digest_aggregated_operators.signers_apk_g2,
                signers_agg_sig_g1: digest_aggregated_operators.signers_agg_sig_g1,
                non_signer_quorum_bitmap_indices: indices.clone().quorumApkIndices,
                quorum_apk_indices: indices.quorumApkIndices,
                total_stake_indices: indices.totalStakeIndices,
                non_signer_stake_indices: indices.nonSignerStakeIndices,
            };

            self.aggregated_response_sender
                .send(Ok(bls_aggregation_service_response))
                .map_err(|_| BlsAggregationServiceError::ChannelError)?;
        }
    }

    /// Verifies the signature of the task response given a `operator_avs_state`.
    /// If the signature is correct, it returns `Ok(())`, otherwise it returns an error.
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task
    /// * `signed_task_response_digest` - The signed task response digest
    /// * `operator_avs_state` - A hashmap containing the staked of all the operator indexed by operator_id.
    ///  This is used to get the `operator_state` to obtain the operator public key.
    ///
    /// # Error
    ///
    /// Returns error:
    /// - `BlsAggregationServiceError::OperatorNotFound` if the operator is not found,
    /// - `BlsAggregationServiceError::OperatorPublicKeyNotFound` if the operator public key is not found,
    /// - `BlsAggregationServiceError::IncorrectSignature` if the signature is incorrect.
    pub async fn verify_signature(
        &self,
        _task_index: TaskIndex,
        signed_task_response_digest: &SignedTaskResponseDigest,
        operator_avs_state: &HashMap<FixedBytes<32>, OperatorAvsState>,
    ) -> Result<(), BlsAggregationServiceError> {
        let Some(operator_state) = operator_avs_state.get(&signed_task_response_digest.operator_id)
        else {
            return Err(BlsAggregationServiceError::OperatorNotFound);
        };

        let Some(pub_keys) = &operator_state.operator_info.pub_keys else {
            return Err(BlsAggregationServiceError::OperatorPublicKeyNotFound);
        };

        verify_message(
            pub_keys.g2_pub_key.g2(),
            signed_task_response_digest.task_response_digest.as_slice(),
            signed_task_response_digest.bls_signature.g1_point().g1(),
        )
        .then(|| ())
        .ok_or(BlsAggregationServiceError::IncorrectSignature)
    }

    /// Checks if the stake thresholds are met for the given set of quorum members.
    ///
    /// # Arguments
    ///
    /// * `signed_stake_per_quorum` - The signed stake per quorum.
    /// * `total_stake_per_quorum` - The total stake per quorum.
    /// * `quorum_threshold_percentages_map` - The quorum threshold percentages map,
    /// containing the quorum id as a key and its corresponding quorum threshold percentage.
    ///
    /// # Returns
    ///
    /// Returns `true` if the stake thresholds are met for all the members, otherwise `false`.
    pub fn check_if_stake_thresholds_met(
        &self,
        signed_stake_per_quorum: HashMap<u8, U256>,
        total_stake_per_quorum: HashMap<u8, U256>,
        quorum_threshold_percentages_map: HashMap<u8, QuorumThresholdPercentage>,
    ) -> bool {
        for (quorum_num, quorum_threshold_percentage) in quorum_threshold_percentages_map {
            let (Some(signed_stake_by_quorum), Some(total_stake_by_quorum)) = (
                signed_stake_per_quorum.get(&quorum_num),
                total_stake_per_quorum.get(&quorum_num),
            ) else {
                return false;
            };

            let signed_stake = signed_stake_by_quorum * U256::from(100);
            let threshold_stake = *total_stake_by_quorum * U256::from(quorum_threshold_percentage);

            if signed_stake < threshold_stake {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use alloy_primitives::{B256, U256};
    use ark_bn254::G1Affine;
    use eigen_crypto_bls::{BlsG1Point, BlsG2Point, BlsKeyPair, Signature};
    use eigen_services_avsregistry::fake_avs_registry_service::FakeAvsRegistryService;
    use eigen_types::operator::{QuorumNum, QuorumThresholdPercentages};
    use eigen_types::{avs::TaskIndex, test::TestOperator};
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::time::Duration;
    use std::vec;

    use super::{BlsAggregationServiceError, BlsAggregationServiceResponse, BlsAggregatorService};

    fn new_bls_key_pair_panics(hex_key: String) -> BlsKeyPair {
        BlsKeyPair::new(hex_key).unwrap()
    }

    fn hash(task_response: u64) -> B256 {
        // TODO: add marshalling
        let mut hasher = Sha256::new();
        hasher.update(task_response.to_be_bytes());
        B256::from_slice(hasher.finalize().as_ref())
    }

    #[tokio::test]
    async fn test_1_quorum_1_operator_1_correct_signature() {
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
            .await
            .unwrap();

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

        assert_eq!(
            expected_agg_service_response,
            response.clone().unwrap().unwrap()
        );
        assert_eq!(task_index, response.unwrap().unwrap().task_index);
    }

    #[tokio::test]
    async fn test_1_quorum_3_operator_3_correct_signatures() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };
        let test_operator_3 = TestOperator {
            operator_id: U256::from(3).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(300)), (1u8, U256::from(100))]),
            bls_keypair: new_bls_key_pair_panics(
                "15610126902690889134622698668747132666439281256983827313388062967626731803501"
                    .into(),
            ),
        };
        let test_operators = vec![
            test_operator_1.clone(),
            test_operator_2.clone(),
            test_operator_3.clone(),
        ];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_op_3 = test_operator_3
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_3.clone(),
                test_operator_3.operator_id,
            )
            .await
            .unwrap();
        let quorum_apks_g1 = BlsG1Point::new(
            (test_operator_1.bls_keypair.public_key().g1()
                + test_operator_2.bls_keypair.public_key().g1()
                + test_operator_3.bls_keypair.public_key().g1())
            .into(),
        );

        let signers_apk_g2: BlsG2Point = BlsG2Point::new(
            (test_operator_1.bls_keypair.public_key_g2().g2()
                + test_operator_2.bls_keypair.public_key_g2().g2()
                + test_operator_3.bls_keypair.public_key_g2().g2())
            .into(),
        );

        let signers_agg_sig_g1 = Signature::new(
            (bls_sig_op_1.g1_point().g1()
                + bls_sig_op_2.g1_point().g1()
                + bls_sig_op_3.g1_point().g1())
            .into(),
        );

        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest,
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1: vec![quorum_apks_g1],
            signers_apk_g2,
            signers_agg_sig_g1,
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

        assert_eq!(
            expected_agg_service_response,
            response.clone().unwrap().unwrap()
        );
        assert_eq!(task_index, response.unwrap().unwrap().task_index);
    }

    #[tokio::test]
    async fn test_2_quorum_2_operator_2_correct_signatures() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100u8, 100u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1,
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_2,
                test_operator_2.operator_id,
            )
            .await
            .unwrap();

        let quorum_apks_g1: BlsG1Point = BlsG1Point::new(
            (G1Affine::identity()
                + test_operator_1.bls_keypair.public_key().g1()
                + test_operator_2.bls_keypair.public_key().g1())
            .into(),
        );
        let signers_apk_g2 = BlsG2Point::new(
            (test_operator_1.bls_keypair.public_key_g2().g2()
                + test_operator_2.bls_keypair.public_key_g2().g2())
            .into(),
        );
        let signers_agg_sig_g1 = Signature::new(
            (test_operator_1
                .bls_keypair
                .sign_message(task_response_digest.as_ref())
                .g1_point()
                .g1()
                + test_operator_2
                    .bls_keypair
                    .sign_message(task_response_digest.as_ref())
                    .g1_point()
                    .g1())
            .into(),
        );
        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest,
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1: vec![quorum_apks_g1.clone(), quorum_apks_g1],
            signers_apk_g2,
            signers_agg_sig_g1,
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

        assert_eq!(expected_agg_service_response, response.unwrap().unwrap());
    }

    #[tokio::test]
    async fn test_2_concurrent_tasks_2_quorum_2_operator_2_correct_signatures() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100u8, 100u8];
        let time_to_expiry = Duration::from_secs(1);

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);

        // initialize 2 concurrent tasks
        let task_1_index = 1;
        let task_1_response = 123; // Initialize with appropriate data
        let task_1_response_digest = hash(task_1_response);
        bls_agg_service
            .initialize_new_task::<FakeAvsRegistryService>(
                task_1_index,
                block_number as u32,
                quorum_numbers.clone(),
                quorum_threshold_percentages.clone(),
                time_to_expiry.clone(),
            )
            .await;

        let task_2_index = 2;
        let task_2_response = 234; // Initialize with appropriate data
        let task_2_response_digest = hash(task_2_response);
        bls_agg_service
            .initialize_new_task::<FakeAvsRegistryService>(
                task_2_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await;

        let bls_sig_task_1_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_1_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_1_index,
                task_1_response_digest,
                bls_sig_task_1_op_1,
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_task_1_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_1_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_1_index,
                task_1_response_digest.clone(),
                bls_sig_task_1_op_2,
                test_operator_2.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_task_2_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_2_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_2_index,
                task_2_response_digest,
                bls_sig_task_2_op_1,
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_task_2_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_2_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_2_index,
                task_2_response_digest,
                bls_sig_task_2_op_2,
                test_operator_2.operator_id,
            )
            .await
            .unwrap();

        let quorum_apks_g1: BlsG1Point = BlsG1Point::new(
            (G1Affine::identity()
                + test_operator_1.bls_keypair.public_key().g1()
                + test_operator_2.bls_keypair.public_key().g1())
            .into(),
        );
        let signers_apk_g2 = BlsG2Point::new(
            (test_operator_1.bls_keypair.public_key_g2().g2()
                + test_operator_2.bls_keypair.public_key_g2().g2())
            .into(),
        );
        let signers_agg_sig_g1_task_1 = Signature::new(
            (test_operator_1
                .bls_keypair
                .sign_message(task_1_response_digest.as_ref())
                .g1_point()
                .g1()
                + test_operator_2
                    .bls_keypair
                    .sign_message(task_1_response_digest.as_ref())
                    .g1_point()
                    .g1())
            .into(),
        );
        let expected_response_task_1 = BlsAggregationServiceResponse {
            task_index: task_1_index,
            task_response_digest: task_1_response_digest,
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1: vec![quorum_apks_g1.clone(), quorum_apks_g1.clone()],
            signers_apk_g2: signers_apk_g2.clone(),
            signers_agg_sig_g1: signers_agg_sig_g1_task_1,
            non_signer_quorum_bitmap_indices: vec![],
            quorum_apk_indices: vec![],
            total_stake_indices: vec![],
            non_signer_stake_indices: vec![],
        };

        let signers_agg_sig_g1_task_2 = Signature::new(
            (test_operator_1
                .bls_keypair
                .sign_message(task_2_response_digest.as_ref())
                .g1_point()
                .g1()
                + test_operator_2
                    .bls_keypair
                    .sign_message(task_2_response_digest.as_ref())
                    .g1_point()
                    .g1())
            .into(),
        );
        let expected_response_task_2 = BlsAggregationServiceResponse {
            task_index: task_2_index,
            task_response_digest: task_2_response_digest,
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1: vec![quorum_apks_g1.clone(), quorum_apks_g1.clone()],
            signers_apk_g2,
            signers_agg_sig_g1: signers_agg_sig_g1_task_2,
            non_signer_quorum_bitmap_indices: vec![],
            quorum_apk_indices: vec![],
            total_stake_indices: vec![],
            non_signer_stake_indices: vec![],
        };

        let first_response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
            .unwrap();
        let second_response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await
            .unwrap();

        let (task_1_response, task_2_response) = if first_response.clone().unwrap().task_index == 1
        {
            (first_response, second_response)
        } else {
            (second_response, first_response)
        };

        assert_eq!(expected_response_task_1, task_1_response.unwrap());
        assert_eq!(expected_response_task_2, task_2_response.unwrap());
    }

    #[tokio::test]
    async fn test_1_quorum_1_operator_0_signatures_task_expired() {
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
        let _task_response = 123; // Initialize with appropriate data

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

        let response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await;

        assert_eq!(
            Err(BlsAggregationServiceError::TaskExpired),
            response.unwrap()
        );
    }

    #[tokio::test]
    async fn test_1_quorum_2_operator_1_signatures_50_threshold() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![50u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let quorum_apks_g1 = BlsG1Point::new(
            (test_operator_1.bls_keypair.public_key().g1()
                + test_operator_2.bls_keypair.public_key().g1())
            .into(),
        );

        let signers_apk_g2: BlsG2Point = test_operator_1.bls_keypair.public_key_g2();

        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest,
            non_signers_pub_keys_g1: vec![test_operator_2.bls_keypair.public_key()], //
            quorum_apks_g1: vec![quorum_apks_g1],
            signers_apk_g2,
            signers_agg_sig_g1: bls_sig_op_1,
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

        assert_eq!(
            expected_agg_service_response,
            response.clone().unwrap().unwrap()
        );
        assert_eq!(task_index, response.unwrap().unwrap().task_index);
    }

    #[tokio::test]
    async fn test_1_quorum_2_operator_1_signatures_60_threshold() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![60u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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
                bls_sig_op_1,
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await;

        assert_eq!(
            Err(BlsAggregationServiceError::TaskExpired),
            response.unwrap()
        );
    }

    #[tokio::test]
    async fn test_2_quorums_2_operators_which_just_take_1_quorum_2_correct_signatures() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            // Note the quorums is [0, 1], but operator id 1 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            // Note the quorums is [0, 1], but operator id 2 just stake 1.
            stake_per_quorum: HashMap::from([(1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };

        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100u8, 100u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            )
            .await
            .unwrap();
        let signers_apk_g2 = BlsG2Point::new(
            (test_operator_1.bls_keypair.public_key_g2().g2()
                + test_operator_2.bls_keypair.public_key_g2().g2())
            .into(),
        );
        let signers_agg_sig_g1 =
            Signature::new((bls_sig_op_1.g1_point().g1() + bls_sig_op_2.g1_point().g1()).into());
        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest,
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1: vec![
                test_operator_1.bls_keypair.public_key(),
                test_operator_2.bls_keypair.public_key(),
            ],
            signers_apk_g2,
            signers_agg_sig_g1,
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

        assert_eq!(
            expected_agg_service_response,
            response.clone().unwrap().unwrap()
        );
        assert_eq!(task_index, response.unwrap().unwrap().task_index);
    }

    #[tokio::test]
    async fn test_2_quorums_3_operators_which_just_stake_1_quorum_50_threshold() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            // Note the quorums is [0, 1], but operator id 1 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            // Note the quorums is [0, 1], but operator id 2 just stake 1.
            stake_per_quorum: HashMap::from([(1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };

        let test_operator_3 = TestOperator {
            operator_id: U256::from(3).into(),
            // Note the quorums is [0, 1], but operator id 3 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "15710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };

        let test_operators = vec![
            test_operator_1.clone(),
            test_operator_2.clone(),
            test_operator_3.clone(),
        ];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![50u8, 50u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            )
            .await
            .unwrap();
        let signers_apk_g2 = BlsG2Point::new(
            (test_operator_1.bls_keypair.public_key_g2().g2()
                + test_operator_2.bls_keypair.public_key_g2().g2())
            .into(),
        );
        let signers_agg_sig_g1 =
            Signature::new((bls_sig_op_1.g1_point().g1() + bls_sig_op_2.g1_point().g1()).into());
        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest,
            non_signers_pub_keys_g1: vec![test_operator_3.bls_keypair.public_key()],
            quorum_apks_g1: vec![
                // TODO: in Go it adds each public key to G1Point::zero()
                BlsG1Point::new(
                    (test_operator_1.bls_keypair.public_key().g1()
                        + test_operator_3.bls_keypair.public_key().g1())
                    .into(),
                ),
                BlsG1Point::new(
                    (test_operator_2.bls_keypair.public_key().g1()
                        + test_operator_3.bls_keypair.public_key().g1())
                    .into(),
                ),
            ],
            signers_apk_g2,
            signers_agg_sig_g1,
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

        assert_eq!(
            expected_agg_service_response,
            response.clone().unwrap().unwrap()
        );
        assert_eq!(task_index, response.unwrap().unwrap().task_index);
    }

    #[tokio::test]
    async fn test_2_quorums_3_operators_which_just_stake_1_quorum_60_threshold() {
        // results in `task expired`
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            // Note the quorums is [0, 1], but operator id 1 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            // Note the quorums is [0, 1], but operator id 2 just stake 1.
            stake_per_quorum: HashMap::from([(1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };

        let test_operator_3 = TestOperator {
            operator_id: U256::from(3).into(),
            // Note the quorums is [0, 1], but operator id 3 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "15710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };

        let test_operators = vec![
            test_operator_1.clone(),
            test_operator_2.clone(),
            test_operator_3.clone(),
        ];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![60u8, 60u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());

        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            )
            .await
            .unwrap();

        let response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await;

        assert_eq!(
            Err(BlsAggregationServiceError::TaskExpired),
            response.unwrap()
        );
    }

    #[tokio::test]
    async fn test_2_quorums_1_operator_which_just_take_1_quorum_1_signature_task_expired() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            // Note the quorums is [0, 1], but operator id 1 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };

        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

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

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());

        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await;
        assert_eq!(
            Err(BlsAggregationServiceError::TaskExpired),
            response.unwrap()
        );
    }

    #[tokio::test]
    async fn test_2_quorums_2_operators_where_1_operator_just_take_1_quorum_1_signature_task_expired(
    ) {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            // Note the quorums is [0, 1], but operator id 1 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            // Note the quorums is [0, 1], but operator id 2 just stake 1.
            stake_per_quorum: HashMap::from([(1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };

        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100, 100];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];

        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1,
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await;

        assert_eq!(
            Err(BlsAggregationServiceError::TaskExpired),
            response.unwrap()
        );
    }

    #[tokio::test]
    async fn send_signature_of_task_not_initialized() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            // Note the quorums is [0, 1], but operator id 1 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };

        let block_number = 1;
        let task_index = 0;
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, vec![test_operator_1.clone()]);
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());

        let result = bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await;

        assert_eq!(Err(BlsAggregationServiceError::TaskNotFound), result);
    }

    #[tokio::test]
    async fn test_1_quorum_2_operator_2_signatures_on_2_different_msgs() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "14610126902690889134622698668747132666439281256983827313388062967626731803500"
                    .into(),
            ),
        };
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100u8];
        let time_to_expiry = Duration::from_secs(1);
        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
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

        let task_response_1 = 123; // Initialize with appropriate data
        let task_response_1_digest = hash(task_response_1);
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_1_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_1_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let task_response_2 = 456; // Initialize with appropriate data
        let task_response_2_digest = hash(task_response_2);
        let bls_sig_op_2 = test_operator_1
            .bls_keypair
            .sign_message(task_response_2_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_2_digest,
                bls_sig_op_2.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let response = bls_agg_service
            .aggregated_response_receiver
            .lock()
            .await
            .recv()
            .await;

        assert_eq!(
            Err(BlsAggregationServiceError::TaskExpired),
            response.unwrap()
        );
    }

    #[tokio::test]
    async fn test_1_quorum_1_operator_1_invalid_signature() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: new_bls_key_pair_panics(
                "13710126902690889134622698668747132666439281256983827313388062967626731803599"
                    .into(),
            ),
        };

        let block_number = 1;
        let task_index = 0;
        let quorum_numbers = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data

        let wrong_task_response_digest = hash(task_response + 1);
        let bls_signature = test_operator_1
            .bls_keypair
            .sign_message(wrong_task_response_digest.as_ref());
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

        let result = bls_agg_service
            .process_new_signature(
                task_index,
                wrong_task_response_digest,
                bls_signature,
                test_operator_1.operator_id,
            )
            .await;

        assert_eq!(Err(BlsAggregationServiceError::TaskExpired), result);
    }

    #[tokio::test]
    async fn test() {}
}
