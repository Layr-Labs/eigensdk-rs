use alloy_primitives::{FixedBytes, U256};
use ark_bn254::G2Affine;
use ark_ec::AffineRepr;
use eigen_crypto_bls::{BlsG1Point, BlsG2Point, Signature};
use eigen_crypto_bn254::utils::verify_message;
use eigen_services_avsregistry::AvsRegistryService;
use eigen_types::{
    avs::{SignatureVerificationError, SignedTaskResponseDigest, TaskIndex, TaskResponseDigest},
    operator::{OperatorAvsState, QuorumThresholdPercentage, QuorumThresholdPercentages},
};
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::{
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        Mutex,
    },
    time::{timeout, Duration},
};

/// The response from the BLS aggregation service
#[allow(unused)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlsAggregationServiceResponse {
    pub task_index: TaskIndex,
    pub task_response_digest: TaskResponseDigest,
    pub non_signers_pub_keys_g1: Vec<BlsG1Point>,
    pub quorum_apks_g1: Vec<BlsG1Point>,
    pub signers_apk_g2: BlsG2Point,
    pub signers_agg_sig_g1: Signature,
    pub non_signer_quorum_bitmap_indices: Vec<u32>,
    pub quorum_apk_indices: Vec<u32>,
    pub total_stake_indices: Vec<u32>,
    pub non_signer_stake_indices: Vec<Vec<u32>>,
}

/// Possible errors raised in BLS aggregation
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum BlsAggregationServiceError {
    #[error("task expired error")]
    TaskExpired,
    #[error("task not found error")]
    TaskNotFound,
    #[error("signature verification error")]
    SignatureVerificationError(SignatureVerificationError),
    #[error("channel was closed")]
    ChannelClosed,
    #[error("error sending to channel")]
    ChannelError,
    #[error("Avs Registry Error")]
    RegistryError,
    #[error("duplicate task index error")]
    DuplicateTaskIndex,
}

/// Contains the aggregated operators signers information
#[derive(Debug, Clone)]
pub struct AggregatedOperators {
    signers_apk_g2: BlsG2Point,
    signers_agg_sig_g1: Signature,
    signers_total_stake_per_quorum: HashMap<u8, U256>,
    pub signers_operator_ids_set: HashMap<FixedBytes<32>, bool>,
}

/// The BLS Aggregator Service main struct
#[derive(Debug)]
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

    ///   Creates a new task meant to process new signed task responses for a task tokio channel.
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
    pub async fn initialize_new_task(
        &self,
        task_index: TaskIndex,
        task_created_block: u32,
        quorum_nums: Vec<u8>,
        quorum_threshold_percentages: QuorumThresholdPercentages,
        time_to_expiry: Duration,
    ) -> Result<(), BlsAggregationServiceError> {
        let mut task_channel = self.signed_task_response.write();

        if task_channel.contains_key(&task_index) {
            return Err(BlsAggregationServiceError::DuplicateTaskIndex);
        }

        let (tx, rx) = mpsc::unbounded_channel();
        task_channel.insert(task_index, tx);

        let avs_registry_service = self.avs_registry_service.clone();
        let aggregated_response_sender = self.aggregated_response_sender.clone();
        tokio::spawn(async move {
            // Process each signed response here
            let _ = BlsAggregatorService::<A>::single_task_aggregator(
                avs_registry_service,
                task_index,
                task_created_block,
                quorum_nums.clone(),
                quorum_threshold_percentages.clone(),
                time_to_expiry,
                aggregated_response_sender,
                rx,
            )
            .await
            .inspect_err(|err| {
                println!("Error: {:?}", err);
            });
        });
        Ok(())
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
    ///
    /// # Errors
    ///
    /// Returns error:
    /// * `TaskNotFound` - If the task is not found.
    /// * `ChannelError` - If there is an error while sending the task through the channel.
    /// * `SignatureVerificationError` - If the signature verification fails.
    pub async fn process_new_signature(
        &self,
        task_index: TaskIndex,
        task_response_digest: TaskResponseDigest,
        bls_signature: Signature,
        operator_id: FixedBytes<32>,
    ) -> Result<(), BlsAggregationServiceError> {
        let (tx, rx) = mpsc::channel(1);
        let task = SignedTaskResponseDigest {
            task_response_digest,
            bls_signature,
            operator_id,
            signature_verification_channel: tx,
        };

        let mut rx = {
            let task_channel = self.signed_task_response.read();

            let sender = task_channel
                .get(&task_index)
                .ok_or(BlsAggregationServiceError::TaskNotFound)?;

            // send the task to the aggregator thread
            sender
                .send(task)
                .map_err(|_| BlsAggregationServiceError::ChannelError)?;
            rx
            // release the lock
        };

        // return the signature verification result
        rx.recv()
            .await
            .ok_or(BlsAggregationServiceError::ChannelClosed)?
            .map_err(BlsAggregationServiceError::SignatureVerificationError)
    }

    /// Adds a new operator to the aggregated operators by aggregating its public key, signature and stake.
    ///
    /// # Arguments
    ///
    /// - `aggregated_operators` - Contains the information of all the aggregated operators.
    /// - `operator_state` - The state of the operator, contains information about its stake.
    /// - `signed_task_digest` - Contains the id and signature of the new operator.
    ///
    /// # Returns
    ///
    /// The given aggregated operators, aggregated with the new operator info.
    fn aggregate_new_operator(
        aggregated_operators: &mut AggregatedOperators,
        operator_state: OperatorAvsState,
        signed_task_digest: SignedTaskResponseDigest,
    ) -> &mut AggregatedOperators {
        aggregated_operators.signers_agg_sig_g1 = Signature::new(
            (aggregated_operators.signers_agg_sig_g1.g1_point().g1()
                + signed_task_digest.bls_signature.g1_point().g1())
            .into(),
        );
        let operator_g2_pubkey = operator_state
            .operator_info
            .pub_keys
            .clone()
            .unwrap()
            .g2_pub_key
            .g2();
        aggregated_operators.signers_apk_g2 =
            BlsG2Point::new((aggregated_operators.signers_apk_g2.g2() + operator_g2_pubkey).into());
        aggregated_operators
            .signers_operator_ids_set
            .insert(signed_task_digest.operator_id, true);
        for (quorum_num, stake) in operator_state.stake_per_quorum.iter() {
            aggregated_operators
                .signers_total_stake_per_quorum
                .entry(*quorum_num)
                .and_modify(|v| *v += stake)
                .or_insert(*stake);
        }
        aggregated_operators
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
    /// * `aggregated_response_sender` - The sender channel for the aggregated responses
    /// * `rx` - The receiver channel for the signed task responses
    #[allow(clippy::too_many_arguments)]
    pub async fn single_task_aggregator(
        avs_registry_service: A,
        task_index: TaskIndex,
        task_created_block: u32,
        quorum_nums: Vec<u8>,
        quorum_threshold_percentages: QuorumThresholdPercentages,
        time_to_expiry: Duration,
        aggregated_response_sender: UnboundedSender<
            Result<BlsAggregationServiceResponse, BlsAggregationServiceError>,
        >,
        mut rx: UnboundedReceiver<SignedTaskResponseDigest>,
    ) -> Result<(), BlsAggregationServiceError> {
        let quorum_threshold_percentage_map: HashMap<u8, u8> = quorum_nums
            .iter()
            .enumerate()
            .map(|(i, quorum_number)| (*quorum_number, quorum_threshold_percentages[i]))
            .collect();

        let operator_state_avs = avs_registry_service
            .get_operators_avs_state_at_block(task_created_block, &quorum_nums)
            .await
            .map_err(|_| BlsAggregationServiceError::RegistryError)?;

        let quorums_avs_state = avs_registry_service
            .get_quorums_avs_state_at_block(&quorum_nums, task_created_block)
            .await
            .map_err(|_| BlsAggregationServiceError::RegistryError)?;
        let total_stake_per_quorum: HashMap<_, _> = quorums_avs_state
            .iter()
            .map(|(k, v)| (*k, v.total_stake))
            .collect();

        let quorum_apks_g1: Vec<BlsG1Point> = quorum_nums
            .iter()
            .filter_map(|quorum_num| quorums_avs_state.get(quorum_num))
            .map(|avs_state| avs_state.agg_pub_key_g1.clone())
            .collect();

        let mut aggregated_operators: HashMap<FixedBytes<32>, AggregatedOperators> = HashMap::new();

        // iterate over the signed task responses receive from the channel, until the time to expiry is reached or the channel is closed
        while let Some(signed_task_digest) = timeout(time_to_expiry, rx.recv())
            .await
            .inspect_err(|_err| {
                // timeout
                println!("expire");
                let _ =
                    aggregated_response_sender.send(Err(BlsAggregationServiceError::TaskExpired));
            })
            .map_err(|_| BlsAggregationServiceError::TaskExpired)?
        {
            // check if the operator has already signed for this digest
            if aggregated_operators
                .get(&signed_task_digest.task_response_digest)
                .map(|operators| {
                    operators
                        .signers_operator_ids_set
                        .contains_key(&signed_task_digest.operator_id)
                })
                .unwrap_or(false)
            {
                signed_task_digest
                    .signature_verification_channel
                    .send(Err(SignatureVerificationError::DuplicateSignature))
                    .await
                    .map_err(|_| BlsAggregationServiceError::ChannelError)?;
                continue;
            }

            let verification_result = BlsAggregatorService::<A>::verify_signature(
                task_index,
                &signed_task_digest,
                &operator_state_avs,
            )
            .await;
            let verification_failed = verification_result.is_err();

            signed_task_digest
                .signature_verification_channel
                .send(verification_result)
                .await
                .map_err(|_| BlsAggregationServiceError::ChannelError)?;

            if verification_failed {
                continue;
            }

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

            let digest_aggregated_operators = aggregated_operators
                .get_mut(&signed_task_digest.task_response_digest)
                .map(|digest_aggregated_operators| {
                    BlsAggregatorService::<A>::aggregate_new_operator(
                        digest_aggregated_operators,
                        operator_state.clone(),
                        signed_task_digest.clone(),
                    )
                    .clone()
                })
                .unwrap_or(AggregatedOperators {
                    signers_apk_g2: BlsG2Point::new((G2Affine::zero() + operator_g2_pubkey).into()),
                    signers_agg_sig_g1: signed_task_digest.bls_signature.clone(),
                    signers_operator_ids_set: HashMap::from([(
                        operator_state.operator_id.into(),
                        true,
                    )]),
                    signers_total_stake_per_quorum: operator_state.stake_per_quorum.clone(),
                });

            aggregated_operators.insert(
                signed_task_digest.task_response_digest,
                digest_aggregated_operators.clone(),
            );

            if !BlsAggregatorService::<A>::check_if_stake_thresholds_met(
                &digest_aggregated_operators.signers_total_stake_per_quorum,
                &total_stake_per_quorum,
                &quorum_threshold_percentage_map,
            ) {
                continue;
            }

            let bls_aggregation_service_response = BlsAggregatorService::build_aggregated_response(
                task_index,
                task_created_block,
                signed_task_digest,
                &operator_state_avs,
                digest_aggregated_operators,
                &avs_registry_service,
                &quorum_apks_g1,
                &quorum_nums,
            )
            .await?;

            aggregated_response_sender
                .send(Ok(bls_aggregation_service_response))
                .map_err(|_| BlsAggregationServiceError::ChannelError)?;
        }
        Err(BlsAggregationServiceError::ChannelClosed)
    }

    /// Builds the aggregated response containing all the aggregation info.
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task.
    /// * `task_created_block` - The block in which the task was created.
    /// * `signed_task_digest` - The signed task.
    /// * `operator_state_avs` - A hashmap with the operator state per operator id.
    /// * `digest_aggregated_operators` - The aggregated operators.
    /// * `avs_registry_service` - The avs registry service.
    /// * `quorum_apks_g1` - The quorum aggregated public keys.
    /// * `quorum_nums` - The quorum numbers.
    ///
    /// # Returns
    ///
    /// The BLS aggregation service response.
    #[allow(clippy::too_many_arguments)]
    async fn build_aggregated_response(
        task_index: TaskIndex,
        task_created_block: u32,
        signed_task_digest: SignedTaskResponseDigest,
        operator_state_avs: &HashMap<FixedBytes<32>, OperatorAvsState>,
        digest_aggregated_operators: AggregatedOperators,
        avs_registry_service: &A,
        quorum_apks_g1: &[BlsG1Point],
        quorum_nums: &[u8],
    ) -> Result<BlsAggregationServiceResponse, BlsAggregationServiceError> {
        let mut non_signers_operators_ids: Vec<FixedBytes<32>> = operator_state_avs
            .keys()
            .filter(|operator_id| {
                !digest_aggregated_operators
                    .signers_operator_ids_set
                    .contains_key(*operator_id)
            })
            .cloned()
            .collect::<Vec<_>>();

        non_signers_operators_ids.sort();

        let non_signers_pub_keys_g1: Vec<BlsG1Point> = non_signers_operators_ids
            .iter()
            .filter_map(|operator_id| operator_state_avs.get(operator_id))
            .filter_map(|operator_avs_state| operator_avs_state.operator_info.pub_keys.clone())
            .map(|pub_keys| pub_keys.g1_pub_key)
            .collect();

        let indices = avs_registry_service
            .get_check_signatures_indices(
                task_created_block,
                quorum_nums.into(),
                non_signers_operators_ids,
            )
            .await
            .map_err(|_err| BlsAggregationServiceError::RegistryError)?;

        Ok(BlsAggregationServiceResponse {
            task_index,
            task_response_digest: signed_task_digest.task_response_digest,
            non_signers_pub_keys_g1,
            quorum_apks_g1: quorum_apks_g1.into(),
            signers_apk_g2: digest_aggregated_operators.signers_apk_g2,
            signers_agg_sig_g1: digest_aggregated_operators.signers_agg_sig_g1,
            non_signer_quorum_bitmap_indices: indices.clone().nonSignerQuorumBitmapIndices,
            quorum_apk_indices: indices.quorumApkIndices,
            total_stake_indices: indices.totalStakeIndices,
            non_signer_stake_indices: indices.nonSignerStakeIndices,
        })
    }

    /// Verifies the signature of the task response given a `operator_avs_state`.
    /// If the signature is correct, it returns `Ok(())`, otherwise it returns an error.
    ///
    /// # Arguments
    ///
    /// * `task_index` - The index of the task
    /// * `signed_task_response_digest` - The signed task response digest
    /// * `operator_avs_state` - A hashmap containing the staked of all the operator indexed by operator_id.
    ///   This is used to get the `operator_state` to obtain the operator public key.
    ///
    /// # Error
    ///
    /// Returns error:
    /// - `SignatureVerificationError::OperatorNotFound` if the operator is not found,
    /// - `SignatureVerificationError::OperatorPublicKeyNotFound` if the operator public key is not found,
    /// - `SignatureVerificationError::IncorrectSignature` if the signature is incorrect.
    pub async fn verify_signature(
        _task_index: TaskIndex,
        signed_task_response_digest: &SignedTaskResponseDigest,
        operator_avs_state: &HashMap<FixedBytes<32>, OperatorAvsState>,
    ) -> Result<(), SignatureVerificationError> {
        let Some(operator_state) = operator_avs_state.get(&signed_task_response_digest.operator_id)
        else {
            return Err(SignatureVerificationError::OperatorNotFound);
        };

        let Some(pub_keys) = &operator_state.operator_info.pub_keys else {
            return Err(SignatureVerificationError::OperatorPublicKeyNotFound);
        };

        verify_message(
            pub_keys.g2_pub_key.g2(),
            signed_task_response_digest.task_response_digest.as_slice(),
            signed_task_response_digest.bls_signature.g1_point().g1(),
        )
        .then_some(())
        .ok_or(SignatureVerificationError::IncorrectSignature)
    }

    /// Checks if the stake thresholds are met for the given set of quorum members.
    ///
    /// # Arguments
    ///
    /// * `signed_stake_per_quorum` - The signed stake per quorum.
    /// * `total_stake_per_quorum` - The total stake per quorum.
    /// * `quorum_threshold_percentages_map` - The quorum threshold percentages map,
    ///   containing the quorum id as a key and its corresponding quorum threshold percentage.
    ///
    /// # Returns
    ///
    /// Returns `true` if the stake thresholds are met for all the members, otherwise `false`.
    pub fn check_if_stake_thresholds_met(
        signed_stake_per_quorum: &HashMap<u8, U256>,
        total_stake_per_quorum: &HashMap<u8, U256>,
        quorum_threshold_percentages_map: &HashMap<u8, QuorumThresholdPercentage>,
    ) -> bool {
        for (quorum_num, quorum_threshold_percentage) in quorum_threshold_percentages_map {
            let (Some(signed_stake_by_quorum), Some(total_stake_by_quorum)) = (
                signed_stake_per_quorum.get(quorum_num),
                total_stake_per_quorum.get(quorum_num),
            ) else {
                return false;
            };

            let signed_stake = signed_stake_by_quorum * U256::from(100);
            let threshold_stake = *total_stake_by_quorum * U256::from(*quorum_threshold_percentage);

            if signed_stake < threshold_stake {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{BlsAggregationServiceError, BlsAggregationServiceResponse, BlsAggregatorService};
    use alloy_primitives::{B256, U256};
    use eigen_crypto_bls::{BlsG1Point, BlsG2Point, BlsKeyPair, Signature};
    use eigen_services_avsregistry::fake_avs_registry_service::FakeAvsRegistryService;
    use eigen_types::avs::SignatureVerificationError::{DuplicateSignature, IncorrectSignature};
    use eigen_types::operator::{QuorumNum, QuorumThresholdPercentages};
    use eigen_types::{avs::TaskIndex, test::TestOperator};
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::time::Duration;
    use std::vec;

    const PRIVATE_KEY_1: &str =
        "13710126902690889134622698668747132666439281256983827313388062967626731803599";
    const PRIVATE_KEY_2: &str =
        "14610126902690889134622698668747132666439281256983827313388062967626731803500";
    const PRIVATE_KEY_3: &str =
        "15610126902690889134622698668747132666439281256983827313388062967626731803501";

    fn hash(task_response: u64) -> B256 {
        let mut hasher = Sha256::new();
        hasher.update(task_response.to_be_bytes());
        B256::from_slice(hasher.finalize().as_ref())
    }

    fn aggregate_g1_public_keys(operators: &[TestOperator]) -> BlsG1Point {
        operators
            .iter()
            .map(|op| op.bls_keypair.public_key().g1())
            .reduce(|a, b| (a + b).into())
            .map(BlsG1Point::new)
            .unwrap()
    }

    fn aggregate_g2_public_keys(operators: &[TestOperator]) -> BlsG2Point {
        operators
            .iter()
            .map(|op| op.bls_keypair.public_key_g2().g2())
            .reduce(|a, b| (a + b).into())
            .map(BlsG2Point::new)
            .unwrap()
    }

    fn aggregate_g1_signatures(signatures: &[Signature]) -> Signature {
        let agg = signatures
            .iter()
            .map(|s| s.g1_point().g1())
            .reduce(|a, b| (a + b).into())
            .unwrap();
        Signature::new(agg)
    }

    #[tokio::test]
    async fn test_1_quorum_1_operator_1_correct_signature() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };

        let block_number = 1;
        let task_index: TaskIndex = 0;
        let quorum_numbers = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data

        let task_response_digest = hash(task_response);
        let bls_signature = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, vec![test_operator_1.clone()]);
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);
        bls_agg_service
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

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
    async fn test_1_quorum_2_operator_2_duplicated_signatures() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
        };
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let task_index: TaskIndex = 0;
        let quorum_numbers = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, test_operators.clone());
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);
        bls_agg_service
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        let bls_signature_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let second_signature_processing_result = bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_1.clone(),
                test_operator_1.operator_id,
            )
            .await;

        assert_eq!(
            second_signature_processing_result,
            Err(BlsAggregationServiceError::SignatureVerificationError(
                DuplicateSignature
            ))
        );

        let bls_signature_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_signature_2.clone(),
                test_operator_2.operator_id,
            )
            .await
            .unwrap();

        let quorum_apks_g1 = aggregate_g1_public_keys(&test_operators);
        let signers_apk_g2 = aggregate_g2_public_keys(&test_operators);
        let signers_agg_sig_g1 = aggregate_g1_signatures(&[bls_signature_1, bls_signature_2]);
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
    async fn test_1_quorum_3_operator_3_correct_signatures() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
        };
        let test_operator_3 = TestOperator {
            operator_id: U256::from(3).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(300)), (1u8, U256::from(100))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_3.into()).unwrap(),
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

        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, test_operators.clone());
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);

        bls_agg_service
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();
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

        let quorum_apks_g1 = aggregate_g1_public_keys(&test_operators);
        let signers_apk_g2 = aggregate_g2_public_keys(&test_operators);
        let signers_agg_sig_g1 =
            aggregate_g1_signatures(&vec![bls_sig_op_1, bls_sig_op_2, bls_sig_op_3]);

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
        };
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100u8, 100u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, test_operators.clone());
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);

        bls_agg_service
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();
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

        let quorum_apks_g1 = aggregate_g1_public_keys(&test_operators);
        let signers_apk_g2 = aggregate_g2_public_keys(&test_operators);
        let signers_agg_sig_g1 = aggregate_g1_signatures(&[bls_sig_op_1, bls_sig_op_2]);

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
        };
        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100u8, 100u8];
        let time_to_expiry = Duration::from_secs(1);

        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, test_operators.clone());
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);

        // initialize 2 concurrent tasks
        let task_1_index = 1;
        let task_1_response = 123; // Initialize with appropriate data
        let task_1_response_digest = hash(task_1_response);
        bls_agg_service
            .initialize_new_task(
                task_1_index,
                block_number as u32,
                quorum_numbers.clone(),
                quorum_threshold_percentages.clone(),
                time_to_expiry,
            )
            .await
            .unwrap();

        let task_2_index = 2;
        let task_2_response = 234; // Initialize with appropriate data
        let task_2_response_digest = hash(task_2_response);
        bls_agg_service
            .initialize_new_task(
                task_2_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        let bls_sig_task_1_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_1_response_digest.as_ref());
        bls_agg_service
            .process_new_signature(
                task_1_index,
                task_1_response_digest,
                bls_sig_task_1_op_1.clone(),
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
                task_1_response_digest,
                bls_sig_task_1_op_2.clone(),
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
                bls_sig_task_2_op_1.clone(),
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
                bls_sig_task_2_op_2.clone(),
                test_operator_2.operator_id,
            )
            .await
            .unwrap();

        let quorum_apks_g1 = aggregate_g1_public_keys(&test_operators);
        let signers_apk_g2 = aggregate_g2_public_keys(&test_operators);
        let signers_agg_sig_g1_task_1 =
            aggregate_g1_signatures(&[bls_sig_task_1_op_1, bls_sig_task_1_op_2]);

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

        let signers_agg_sig_g1_task_2 =
            aggregate_g1_signatures(&[bls_sig_task_2_op_1, bls_sig_task_2_op_2]);

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
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
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
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
    async fn test_1_quorum_2_operator_1_signatures_50_threshold() {
        let test_operator_1 = TestOperator {
            operator_id: U256::from(1).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
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

        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, test_operators.clone());
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);

        bls_agg_service
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();
        bls_agg_service
            .process_new_signature(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            )
            .await
            .unwrap();

        let quorum_apks_g1 = aggregate_g1_public_keys(&test_operators);

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
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
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();
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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            // Note the quorums is [0, 1], but operator id 2 just stake 1.
            stake_per_quorum: HashMap::from([(1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
        };

        let test_operators = vec![test_operator_1.clone(), test_operator_2.clone()];
        let block_number = 1;
        let task_index = 0;
        let quorum_numbers: Vec<QuorumNum> = vec![0, 1];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![100u8, 100u8];
        let time_to_expiry = Duration::from_secs(1);
        let task_response = 123; // Initialize with appropriate data
        let task_response_digest = hash(task_response);

        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, test_operators.clone());
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);

        bls_agg_service
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

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

        let signers_apk_g2 = aggregate_g2_public_keys(&test_operators);
        let signers_agg_sig_g1 = aggregate_g1_signatures(&[bls_sig_op_1, bls_sig_op_2]);

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            // Note the quorums is [0, 1], but operator id 2 just stake 1.
            stake_per_quorum: HashMap::from([(1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
        };

        let test_operator_3 = TestOperator {
            operator_id: U256::from(3).into(),
            // Note the quorums is [0, 1], but operator id 3 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_3.into()).unwrap(),
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

        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, test_operators.clone());
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);

        bls_agg_service
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

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
        let signers_apk_g2 =
            aggregate_g2_public_keys(&vec![test_operator_1.clone(), test_operator_2.clone()]);
        let signers_agg_sig_g1 = aggregate_g1_signatures(&[bls_sig_op_1, bls_sig_op_2]);
        let quorum_apks_g1 = vec![
            aggregate_g1_public_keys(&vec![test_operator_1, test_operator_3.clone()]),
            aggregate_g1_public_keys(&vec![test_operator_2, test_operator_3.clone()]),
        ];

        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest,
            non_signers_pub_keys_g1: vec![test_operator_3.bls_keypair.public_key()],
            quorum_apks_g1,
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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            // Note the quorums is [0, 1], but operator id 2 just stake 1.
            stake_per_quorum: HashMap::from([(1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
        };

        let test_operator_3 = TestOperator {
            operator_id: U256::from(3).into(),
            // Note the quorums is [0, 1], but operator id 3 just stake 0.
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_3.into()).unwrap(),
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
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
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
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            // Note the quorums is [0, 1], but operator id 2 just stake 1.
            stake_per_quorum: HashMap::from([(1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
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
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
        };
        let test_operator_2 = TestOperator {
            operator_id: U256::from(2).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100)), (1u8, U256::from(200))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_2.into()).unwrap(),
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
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

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
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_1.into()).unwrap(),
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
            .sign_message(hash(task_response).as_ref());
        let fake_avs_registry_service =
            FakeAvsRegistryService::new(block_number, vec![test_operator_1.clone()]);
        let bls_agg_service = BlsAggregatorService::new(fake_avs_registry_service);
        bls_agg_service
            .initialize_new_task(
                task_index,
                block_number as u32,
                quorum_numbers,
                quorum_threshold_percentages,
                time_to_expiry,
            )
            .await
            .unwrap();

        let result = bls_agg_service
            .process_new_signature(
                task_index,
                wrong_task_response_digest,
                bls_signature,
                test_operator_1.operator_id,
            )
            .await;

        assert_eq!(
            Err(BlsAggregationServiceError::SignatureVerificationError(
                IncorrectSignature
            )),
            result
        );

        // Also test that the aggregator service is not affected by the invalid signature, so the task should expire
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
}
