use super::bls_aggregation_service_error::BlsAggregationServiceError;
use super::bls_aggregation_service_response::BlsAggregationServiceResponse;
use alloy::primitives::{FixedBytes, Uint, U256};
use ark_bn254::{G1Affine, G2Affine};
use ark_ec::AffineRepr;
use eigen_crypto_bls::{BlsG1Point, BlsG2Point, Signature};
use eigen_crypto_bn254::utils::verify_message;
use eigen_logging::logger::SharedLogger;
use eigen_services_avsregistry::AvsRegistryService;
use eigen_types::avs_state::OperatorAvsState;
use eigen_types::{
    avs::{SignatureVerificationError, TaskIndex, TaskResponseDigest},
    operator::{QuorumThresholdPercentage, QuorumThresholdPercentages},
};
use std::collections::HashMap;
use tokio::{
    sync::{
        mpsc::{self, UnboundedReceiver, UnboundedSender},
        oneshot,
    },
    time::Duration,
};

/// Contains the aggregated operators signers information
#[derive(Debug, Clone)]
pub struct AggregatedOperators {
    signers_apk_g2: BlsG2Point,
    signers_agg_sig_g1: Signature,
    signers_total_stake_per_quorum: HashMap<u8, U256>,
    pub signers_operator_ids_set: HashMap<FixedBytes<32>, bool>,
}

/// Contains the metadata required to initialize a new task.
#[derive(Clone)]
pub struct TaskMetadata {
    /// Index of the task
    task_index: TaskIndex,
    /// Block the task was created
    task_created_block: u64,
    /// Quorum numbers which should respond to the task
    quorum_numbers: Vec<u8>,
    /// Thresholds for each quorum
    quorum_threshold_percentages: QuorumThresholdPercentages,
    /// Time before expiry of the task response aggregation
    time_to_expiry: Duration,
    // Duration of the window to wait for signatures after quorum is reached
    window_duration: Duration,
}

impl TaskMetadata {
    /// Creates a new instance of [`TaskMetadata`]
    ///
    /// # Arguments
    ///
    /// * `task_index` - index of the task
    /// * `task_created_block` - block number at which the task was created
    /// * `quorum_numbers` - quorum numbers which should respond to the task
    /// * `quorum_threshold_percentages` - threshold percentages for each quorum
    /// * `time_to_expiry` - time until the task expires
    ///
    /// Use [`with_window_duration`](Self::with_window_duration) to set the window duration.
    /// If the window duration is not set, it will default to [`Duration::ZERO`].
    ///
    /// # Returns
    ///
    /// A new instance of [`TaskMetadata`]
    pub fn new(
        task_index: TaskIndex,
        task_created_block: u64,
        quorum_numbers: Vec<u8>,
        quorum_threshold_percentages: QuorumThresholdPercentages,
        time_to_expiry: Duration,
    ) -> Self {
        Self {
            task_index,
            task_created_block,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
            window_duration: Duration::ZERO,
        }
    }

    /// Sets the window duration for the task
    ///
    /// # Arguments
    /// * `window_duration` - The duration of the window to wait for signatures after quorum is reached
    ///
    /// # Returns
    ///
    /// An instance of [`TaskMetadata`] with the window duration set
    pub fn with_window_duration(mut self, window_duration: Duration) -> Self {
        self.window_duration = window_duration;
        self
    }
}

/// Contains the information of a signed task response
#[derive(Clone)]
pub struct TaskSignature {
    // Index of the task
    task_index: TaskIndex,
    // Digest of the task response
    task_response_digest: TaskResponseDigest,
    // BLS signature of the task response
    bls_signature: Signature,
    // Operator ID of the operator that signed the task response
    operator_id: FixedBytes<32>,
}

impl TaskSignature {
    /// Creates a new instance of [`TaskSignature``]
    ///
    /// # Arguments
    /// * `task_index` - index of the task
    /// * `task_response_digest` - digest of the task response
    /// * `bls_signature` - bls signature of the task response
    /// * `operator_id` - operator ID of the operator that signed the task response
    ///
    /// # Returns
    ///
    /// [`TaskSignature`] instance
    pub fn new(
        task_index: TaskIndex,
        task_response_digest: TaskResponseDigest,
        bls_signature: Signature,
        operator_id: FixedBytes<32>,
    ) -> Self {
        Self {
            task_index,
            task_response_digest,
            bls_signature,
            operator_id,
        }
    }
}

/// Valid messages to interact with the BLS Aggregator Service
pub enum AggregationMessage {
    InitializeTask(
        TaskMetadata,
        oneshot::Sender<Result<(), BlsAggregationServiceError>>,
    ),
    ProcessSignature(
        TaskSignature,
        oneshot::Sender<Result<(), BlsAggregationServiceError>>,
    ),
}

/// Handler to interact with the BLS Aggregator Service
#[derive(Debug, Clone)]
pub struct ServiceHandle {
    /// Channel to send messages to the BLS Aggregator Service
    msg_sender: UnboundedSender<AggregationMessage>,
}

impl ServiceHandle {
    /// Sends a message to the BLS Aggregator Service to initialize a new task.
    ///
    /// # Arguments
    ///
    /// * `metadata` - The metadata of the task to initialize
    ///
    /// # Returns
    ///
    /// Returns error if the task index already exists
    pub async fn initialize_task(
        &self,
        metadata: TaskMetadata,
    ) -> Result<(), BlsAggregationServiceError> {
        let (tx, rx) = oneshot::channel();
        self.msg_sender
            .send(AggregationMessage::InitializeTask(metadata, tx))
            .map_err(|_| BlsAggregationServiceError::SenderError)?;

        rx.await
            .map_err(|_| BlsAggregationServiceError::ReceiverError)?
    }

    /// Sends a message to the BLS Aggregator Service to process a signature.
    ///
    /// # Arguments
    ///
    /// * `task_signature` - The signed task response
    ///
    /// # Returns error:
    ///
    /// * `TaskNotFound` - If the task is not found
    /// * `ChannelError` - If there is an error while sending the task through the channel
    /// * `SignatureVerificationError` - If the signature verification fails
    pub async fn process_signature(
        &self,
        task_signature: TaskSignature,
    ) -> Result<(), BlsAggregationServiceError> {
        let (tx, rx) = oneshot::channel();
        self.msg_sender
            .send(AggregationMessage::ProcessSignature(task_signature, tx))
            .map_err(|_| BlsAggregationServiceError::SenderError)?;

        rx.await
            .map_err(|_| BlsAggregationServiceError::ReceiverError)?
    }
}

/// Receiver to receive the aggregated responses from the BLS Aggregator Service.
#[derive(Debug)]
pub struct AggregateReceiver {
    /// Channel to receive the aggregated responses from the BLS Aggregator Service
    aggregate_receiver:
        UnboundedReceiver<Result<BlsAggregationServiceResponse, BlsAggregationServiceError>>,
}

impl AggregateReceiver {
    /// Receives the aggregated response from the BLS Aggregator Service.
    ///
    /// # Returns
    ///
    /// Returns the aggregated response or an error if the channel is closed.
    pub async fn receive_aggregated_response(
        &mut self,
    ) -> Result<BlsAggregationServiceResponse, BlsAggregationServiceError> {
        self.aggregate_receiver
            .recv()
            .await
            .ok_or(BlsAggregationServiceError::ReceiverError)?
    }
}

/// The BLS Aggregator Service main struct
#[derive(Debug)]
pub struct BlsAggregatorService<A: AvsRegistryService>
where
    A: Clone,
{
    logger: SharedLogger,
    avs_registry_service: A,
}

/// Represents a signed task response digest
#[derive(Debug)]
struct SignedTaskResponseDigest {
    task_response_digest: TaskResponseDigest,

    bls_signature: Signature,

    operator_id: FixedBytes<32>,

    result_channel: oneshot::Sender<Result<(), BlsAggregationServiceError>>,
}

impl<A: AvsRegistryService + Send + Sync + Clone + 'static> BlsAggregatorService<A> {
    /// Creates a new instance of the BlsAggregatorService with the given AVS registry service
    ///
    /// Creates a tokio unbounded_channel to send and received aggregated responses.
    ///
    /// # Arguments
    ///
    /// * `avs_registry_service` - The AVS registry service
    /// * `logger` - Logger to log messages
    pub fn new(avs_registry_service: A, logger: SharedLogger) -> Self {
        Self {
            logger,
            avs_registry_service,
        }
    }

    /// Starts the BLS Aggregator Service running the main loop in background.
    ///
    /// # Returns
    ///
    /// Returns a tuple with the [`ServiceHandle`] and [`AggregateReceiver`] to interact with the service
    pub fn start(self) -> (ServiceHandle, AggregateReceiver) {
        let (msg_tx, msg_rx) = mpsc::unbounded_channel();
        let (agg_tx, agg_rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            self.run(msg_rx, agg_tx).await;
        });

        // Create service handler and aggregate receiver to user can interact with the service
        let service_handler = ServiceHandle { msg_sender: msg_tx };
        let aggregate_receiver = AggregateReceiver {
            aggregate_receiver: agg_rx,
        };

        (service_handler, aggregate_receiver)
    }

    /// Runs the main loop of the BLS Aggregator Service.
    ///
    /// This function continuously processes messages from `msg_receiver` and handles:
    /// * [`InitializeTask`]: Initializes a new aggregation task
    /// * [`ProcessSignature`]: Forwards a signature to the appropriate task aggregator and relays the verification result.
    ///
    /// The final aggregated response is sent through the `aggregate_sender` channel. In addition, each
    /// message (both [`InitializeTask`] and [`ProcessSignature`]) uses its own channel to return specific errors or results.
    ///
    /// # Arguments
    ///
    /// * `msg_receiver` - The receiver channel to receive the valid messages
    /// * `aggregate_sender` - The sender channel to send the aggregated responses
    async fn run(
        self,
        mut msg_receiver: UnboundedReceiver<AggregationMessage>,
        aggregate_sender: UnboundedSender<
            Result<BlsAggregationServiceResponse, BlsAggregationServiceError>,
        >,
    ) {
        let mut task_channels: HashMap<TaskIndex, UnboundedSender<SignedTaskResponseDigest>> =
            HashMap::new();

        while let Some(message) = msg_receiver.recv().await {
            match message {
                AggregationMessage::InitializeTask(metadata, result_sender) => {
                    let task_index = metadata.task_index;
                    if task_channels.contains_key(&task_index) {
                        // Task already exists - return error
                        result_sender
                            .send(Err(BlsAggregationServiceError::DuplicateTaskIndex))
                            .ok();
                        continue;
                    }

                    // Create a new channel to receive the signed task responses
                    let (signature_tx, signature_rx) =
                        mpsc::unbounded_channel::<SignedTaskResponseDigest>();
                    task_channels.insert(task_index, signature_tx);

                    let avs_registry_service = self.avs_registry_service.clone();
                    let aggregated_response_sender = aggregate_sender.clone();
                    let logger = self.logger.clone();

                    tokio::spawn(async move {
                        let _ = BlsAggregatorService::<A>::single_task_aggregator(
                            avs_registry_service,
                            metadata,
                            aggregated_response_sender,
                            signature_rx,
                            logger,
                        )
                        .await
                        .inspect_err(|err| {
                            println!("Error with single_task_aggregator: {:?}", err);
                        });
                    });

                    let _ = result_sender.send(Ok(()));
                }
                AggregationMessage::ProcessSignature(task_signature, result_sender) => {
                    if let Some(sig_sender) = task_channels.get_mut(&task_signature.task_index) {
                        // Send the signed task response to the task aggregator
                        let signed_digest = SignedTaskResponseDigest {
                            task_response_digest: task_signature.task_response_digest,
                            bls_signature: task_signature.bls_signature,
                            operator_id: task_signature.operator_id,
                            result_channel: result_sender,
                        };

                        if let Err(send_error) = sig_sender.send(signed_digest) {
                            let _ = send_error
                                .0
                                .result_channel
                                .send(Err(BlsAggregationServiceError::SenderError));
                        }
                    } else {
                        result_sender
                            .send(Err(BlsAggregationServiceError::TaskNotFound))
                            .ok();
                    }
                }
            }
        }
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
    /// * `metadata` - task metadata
    /// * `aggregated_response_sender` - The sender channel for the aggregated responses
    /// * `signatures_rx` - The receiver channel for the signed task responses
    /// * `logger` - The logger to log messages.
    async fn single_task_aggregator(
        avs_registry_service: A,
        metadata: TaskMetadata,
        aggregated_response_sender: UnboundedSender<
            Result<BlsAggregationServiceResponse, BlsAggregationServiceError>,
        >,
        signatures_rx: UnboundedReceiver<SignedTaskResponseDigest>,
        logger: SharedLogger,
    ) -> Result<(), BlsAggregationServiceError> {
        let quorum_threshold_percentage_map: HashMap<u8, u8> = metadata
            .quorum_numbers
            .iter()
            .enumerate()
            .map(|(i, quorum_number)| (*quorum_number, metadata.quorum_threshold_percentages[i]))
            .collect();
        let operator_state_avs = avs_registry_service
            .get_operators_avs_state_at_block(metadata.task_created_block, &metadata.quorum_numbers)
            .await
            .map_err(|_| BlsAggregationServiceError::RegistryError)?;
        let quorums_avs_state = avs_registry_service
            .get_quorums_avs_state_at_block(&metadata.quorum_numbers, metadata.task_created_block)
            .await
            .map_err(|_| BlsAggregationServiceError::RegistryError)?;
        let total_stake_per_quorum: HashMap<_, _> = quorums_avs_state
            .iter()
            .map(|(k, v)| (*k, v.total_stake))
            .collect();

        let quorum_apks_g1: Vec<BlsG1Point> = metadata
            .quorum_numbers
            .iter()
            .filter_map(|quorum_num| quorums_avs_state.get(quorum_num))
            .map(|avs_state| avs_state.agg_pub_key_g1.clone())
            .collect();

        Self::loop_task_aggregator(
            avs_registry_service,
            metadata.task_index,
            metadata.task_created_block,
            metadata.time_to_expiry,
            aggregated_response_sender,
            signatures_rx,
            operator_state_avs,
            total_stake_per_quorum,
            quorum_threshold_percentage_map,
            quorum_apks_g1,
            metadata.quorum_numbers,
            metadata.window_duration,
            logger,
        )
        .await
    }

    #[allow(clippy::too_many_arguments)]
    async fn loop_task_aggregator(
        avs_registry_service: A,
        task_index: TaskIndex,
        task_created_block: u64,
        time_to_expiry: Duration,
        aggregated_response_sender: UnboundedSender<
            Result<BlsAggregationServiceResponse, BlsAggregationServiceError>,
        >,
        mut signatures_rx: UnboundedReceiver<SignedTaskResponseDigest>,
        operator_state_avs: HashMap<FixedBytes<32>, OperatorAvsState>,
        total_stake_per_quorum: HashMap<u8, Uint<256, 4>>,
        quorum_threshold_percentage_map: HashMap<u8, u8>,
        quorum_apks_g1: Vec<BlsG1Point>,
        quorum_nums: Vec<u8>,
        window_duration: Duration,
        logger: SharedLogger,
    ) -> Result<(), BlsAggregationServiceError> {
        let mut aggregated_operators: HashMap<FixedBytes<32>, AggregatedOperators> = HashMap::new();
        let mut open_window = false;
        let mut current_aggregated_response: Option<BlsAggregationServiceResponse> = None;
        let (window_tx, mut window_rx) = tokio::sync::mpsc::unbounded_channel::<bool>();
        let task_expired_timer = tokio::time::sleep(time_to_expiry);
        tokio::pin!(task_expired_timer);

        loop {
            tokio::select! {
                _ = &mut task_expired_timer => {
                    // If the task is expired, send the aggregated response
                    Self::handle_task_expired(
                        &logger,
                        &aggregated_response_sender,
                        task_index,
                        open_window,
                        &current_aggregated_response,
                    )?;
                    return Ok(());
                },
                _ = window_rx.recv() => {
                    // If the window is finished, send the aggregated response
                    Self::handle_window_finished(
                        &logger,
                        &aggregated_response_sender,
                        task_index,
                        &current_aggregated_response,
                    )?;
                    return Ok(());
                },
                signed_task_digest = signatures_rx.recv() => {
                    // If a new signature is received, handle it
                    Self::handle_new_signature(
                        &logger,
                        &avs_registry_service,
                        &mut aggregated_operators,
                        &mut open_window,
                        &mut current_aggregated_response,
                        &window_tx,
                        task_index,
                        task_created_block,
                        &operator_state_avs,
                        &total_stake_per_quorum,
                        &quorum_threshold_percentage_map,
                        &quorum_apks_g1,
                        &quorum_nums,
                        window_duration,
                        signed_task_digest,
                    ).await?;
                }
            }
        }
    }

    /// Handles a new signature in the [`loop_task_aggregator`] function.
    ///
    /// # Arguments
    ///
    /// * `logger` - The logger to log messages.
    /// * `avs_registry_service` - The avs registry service.
    /// * `aggregated_operators` - The aggregated operators.
    /// * `open_window` - Whether the window is open.
    /// * `current_aggregated_response` - The current aggregated response.
    /// * `window_tx` - The window tx.
    /// * `task_index` - The task index.
    /// * `task_created_block` - The task created block.
    /// * `operator_state_avs` - The operator state avs.
    /// * `total_stake_per_quorum` - The total stake per quorum.
    /// * `quorum_threshold_percentage_map` - The quorum threshold percentage map.
    /// * `quorum_apks_g1` - The quorum apks g1.
    /// * `quorum_nums` - The quorum numbers.
    /// * `window_duration` - The window duration.
    /// * `signed_task_digest` - The signed task digest.
    #[allow(clippy::too_many_arguments)]
    async fn handle_new_signature(
        logger: &SharedLogger,
        avs_registry_service: &A,
        aggregated_operators: &mut HashMap<FixedBytes<32>, AggregatedOperators>,
        open_window: &mut bool,
        current_aggregated_response: &mut Option<BlsAggregationServiceResponse>,
        window_tx: &UnboundedSender<bool>,
        task_index: TaskIndex,
        task_created_block: u64,
        operator_state_avs: &HashMap<FixedBytes<32>, OperatorAvsState>,
        total_stake_per_quorum: &HashMap<u8, Uint<256, 4>>,
        quorum_threshold_percentage_map: &HashMap<u8, u8>,
        quorum_apks_g1: &[BlsG1Point],
        quorum_nums: &[u8],
        window_duration: Duration,
        signed_task_digest: Option<SignedTaskResponseDigest>,
    ) -> Result<(), BlsAggregationServiceError> {
        logger.debug(
            &format!("New signature received for task index: {}", task_index),
            "eigen-services-blsaggregation.bls_agg.handle_new_signature",
        );

        let signed_digest =
            signed_task_digest.ok_or(BlsAggregationServiceError::SignaturesChannelClosed)?;

        // Verify if the operator has already signed for this digest
        if Self::is_duplicate_signature(aggregated_operators, &signed_digest) {
            signed_digest
                .result_channel
                .send(Err(BlsAggregationServiceError::SignatureVerificationError(
                    SignatureVerificationError::DuplicateSignature,
                )))
                .map_err(|_| BlsAggregationServiceError::SenderError)?;
            return Ok(());
        }

        // Verify the signature
        let verification_result = verify_signature(
            task_index,
            &signed_digest,
            operator_state_avs,
            logger.clone(),
        )
        .await
        .map_err(BlsAggregationServiceError::SignatureVerificationError);

        let verification_has_error = verification_result.is_err();

        // Send the verification result to the result channel
        signed_digest
            .result_channel
            .send(verification_result)
            .map_err(|_| BlsAggregationServiceError::SenderError)?;

        // If the signature is incorrect, return
        if verification_has_error {
            return Ok(());
        }

        let operator_state = operator_state_avs.get(&signed_digest.operator_id).unwrap();

        // Update the aggregated operators with the new operator info
        let updated_aggregated = update_aggregated_operators(
            aggregated_operators,
            operator_state,
            signed_digest.task_response_digest,
            signed_digest.bls_signature,
            signed_digest.operator_id,
            logger.clone(),
        );
        aggregated_operators.insert(
            signed_digest.task_response_digest,
            updated_aggregated.clone(),
        );

        // Check if the stake thresholds are met. If not, return
        if !Self::check_if_stake_thresholds_met(
            &updated_aggregated.signers_total_stake_per_quorum,
            total_stake_per_quorum,
            quorum_threshold_percentage_map,
        ) {
            return Ok(());
        }

        logger.debug(
            &format!("Signature threshold is met for task index: {}", task_index),
            "eigen-services-blsaggregation.bls_agg.handle_new_signature",
        );

        // If the window is not open, open it
        if !*open_window {
            *open_window = true;
            Self::start_window(window_tx, window_duration, task_index, logger.clone());
        }

        *current_aggregated_response = Some(
            Self::build_aggregated_response(
                task_index,
                task_created_block,
                signed_digest.task_response_digest,
                operator_state_avs,
                updated_aggregated,
                avs_registry_service,
                quorum_apks_g1,
                quorum_nums,
                logger.clone(),
            )
            .await?,
        );

        Ok(())
    }

    /// Handles when the task expired in the [`loop_task_aggregator`] function.
    /// If the window is open, send the aggregated response. Else, send the error.
    ///
    /// # Arguments
    ///
    /// * `logger` - The logger to log messages.
    /// * `aggregated_response_sender` - The aggregated response sender.
    /// * `task_index` - The task index.
    /// * `open_window` - Whether the window is open.
    /// * `current_aggregated_response` - The current aggregated response.
    fn handle_task_expired(
        logger: &SharedLogger,
        aggregated_response_sender: &UnboundedSender<
            Result<BlsAggregationServiceResponse, BlsAggregationServiceError>,
        >,
        task_index: TaskIndex,
        open_window: bool,
        current_aggregated_response: &Option<BlsAggregationServiceResponse>,
    ) -> Result<(), BlsAggregationServiceError> {
        if open_window {
            logger.debug(
                &format!(
                    "task_expired_timer while in the waiting window for task index: {}",
                    task_index
                ),
                "eigen-services-blsaggregation.bls_agg.handle_task_expired",
            );
            aggregated_response_sender
                .send(Ok(current_aggregated_response.clone().unwrap()))
                .map_err(|_| BlsAggregationServiceError::SenderError)?;
        } else {
            logger.debug(
                &format!(
                    "task_expired_timer NOT in the waiting window for task index: {}",
                    task_index
                ),
                "eigen-services-blsaggregation.bls_agg.handle_task_expired",
            );

            let _ = aggregated_response_sender.send(Err(BlsAggregationServiceError::TaskExpired));
        }
        Ok(())
    }

    /// Handles when the window is finished in the [`loop_task_aggregator`] function.
    ///
    ///
    /// # Arguments
    ///
    /// * `logger` - The logger to log messages.
    /// * `aggregated_response_sender` - The aggregated response sender.
    /// * `task_index` - The task index.
    /// * `current_aggregated_response` - The current aggregated response.
    fn handle_window_finished(
        logger: &SharedLogger,
        aggregated_response_sender: &UnboundedSender<
            Result<BlsAggregationServiceResponse, BlsAggregationServiceError>,
        >,
        task_index: TaskIndex,
        current_aggregated_response: &Option<BlsAggregationServiceResponse>,
    ) -> Result<(), BlsAggregationServiceError> {
        logger.debug(
            &format!(
                "Window finished. Send aggregated response for task index: {}",
                task_index
            ),
            "eigen-services-blsaggregation.bls_agg.handle_window_finished",
        );

        aggregated_response_sender
            .send(Ok(current_aggregated_response.clone().unwrap()))
            .map_err(|_| BlsAggregationServiceError::SenderError)?;
        Ok(())
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
    /// * `logger` - The logger to log messages.
    ///
    /// # Returns
    ///
    /// The BLS aggregation service response.
    #[allow(clippy::too_many_arguments)]
    async fn build_aggregated_response(
        task_index: TaskIndex,
        task_created_block: u64,
        task_response_digest: FixedBytes<32>,
        operator_state_avs: &HashMap<FixedBytes<32>, OperatorAvsState>,
        digest_aggregated_operators: AggregatedOperators,
        avs_registry_service: &A,
        quorum_apks_g1: &[BlsG1Point],
        quorum_nums: &[u8],
        logger: SharedLogger,
    ) -> Result<BlsAggregationServiceResponse, BlsAggregationServiceError> {
        logger.debug(
            &format!("Build aggregated response for task index: {}", task_index),
            "eigen-services-blsaggregation.bls_agg.build_aggregated_response",
        );

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
            task_response_digest,
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
    fn check_if_stake_thresholds_met(
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

    /// Checks if the signature is a duplicate.
    ///
    /// # Arguments
    ///
    /// * `aggregated_operators` - The aggregated operators.
    /// * `signed_digest` - The signed task response digest.
    ///
    /// # Returns
    ///
    /// Returns `true` if the signature is a duplicate, otherwise `false`.
    fn is_duplicate_signature(
        aggregated_operators: &HashMap<FixedBytes<32>, AggregatedOperators>,
        signed_digest: &SignedTaskResponseDigest,
    ) -> bool {
        aggregated_operators
            .get(&signed_digest.task_response_digest)
            .map(|ops| {
                ops.signers_operator_ids_set
                    .contains_key(&signed_digest.operator_id)
            })
            .unwrap_or(false)
    }

    /// Starts the window to wait for new signatures.
    ///
    /// # Arguments
    ///
    /// * `window_tx` - The unbounded sender to send the window signal.
    /// * `window_duration` - The duration of the window.
    /// * `task_index` - The task index.
    /// * `logger` - The logger to log messages.
    fn start_window(
        window_tx: &UnboundedSender<bool>,
        window_duration: Duration,
        task_index: TaskIndex,
        logger: SharedLogger,
    ) {
        let sender = window_tx.clone();
        logger.debug(
            &format!(
                "Create window to wait for new signatures for task index: {}",
                task_index
            ),
            "eigen-services-blsaggregation.bls_agg.start_window",
        );
        tokio::spawn(async move {
            tokio::time::sleep(window_duration).await;
            let _ = sender.send(true);
        });
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
///   This is used to get the `operator_state` to obtain the operator public key.
/// * `logger` - The logger to log messages.
///
/// # Error
///
/// Returns error:
/// - `SignatureVerificationError::OperatorNotFound` if the operator is not found,
/// - `SignatureVerificationError::OperatorPublicKeyNotFound` if the operator public key is not found,
/// - `SignatureVerificationError::IncorrectSignature` if the signature is incorrect.
async fn verify_signature(
    task_index: TaskIndex,
    signed_task_response_digest: &SignedTaskResponseDigest,
    operator_avs_state: &HashMap<FixedBytes<32>, OperatorAvsState>,
    logger: SharedLogger,
) -> Result<(), SignatureVerificationError> {
    let Some(operator_state) = operator_avs_state.get(&signed_task_response_digest.operator_id)
    else {
        logger.error(
            &format!("Operator Not Found for task index: {}", task_index),
            "eigen-services-blsaggregation.bls_agg.verify_signature",
        );
        return Err(SignatureVerificationError::OperatorNotFound);
    };

    let Some(pub_keys) = &operator_state.operator_info.pub_keys else {
        logger.error(
            &format!(
                "Operator Public Key Not Found for task index: {}",
                task_index
            ),
            "eigen-services-blsaggregation.bls_agg.verify_signature",
        );
        return Err(SignatureVerificationError::OperatorPublicKeyNotFound);
    };

    let message = signed_task_response_digest
        .task_response_digest
        .as_slice()
        .try_into()
        .map_err(|_| SignatureVerificationError::IncorrectSignature)?;

    verify_message(
        pub_keys.g2_pub_key.g2(),
        message,
        signed_task_response_digest.bls_signature.g1_point().g1(),
    )
    .then_some(())
    .ok_or(SignatureVerificationError::IncorrectSignature)
    .inspect(|_| {
        logger.debug(
            &format!(
                "Signature verification successful for task index: {}",
                task_index
            ),
            "eigen-services-blsaggregation.bls_agg.verify_signature",
        );
    })
    .inspect_err(|_| {
        logger.error(
            &format!(
                "Signature verification failed for task index: {}",
                task_index
            ),
            "eigen-services-blsaggregation.bls_agg.verify_signature",
        );
    })
}

/// Updates the aggregated operators with the new operator info.
///
/// # Arguments
///
/// * `aggregated_operators` - The aggregated operators.
/// * `operator_state` - The operator state.
/// * `task_response_digest` - The task response digest.
/// * `bls_signature` - The BLS signature.
/// * `operator_id` - The operator id.
/// * `logger` - The logger to log messages.
///
/// # Returns
///
/// The updated aggregated operators.
fn update_aggregated_operators(
    aggregated_operators: &mut HashMap<FixedBytes<32>, AggregatedOperators>,
    operator_state: &OperatorAvsState,
    task_response_digest: FixedBytes<32>,
    bls_signature: Signature,
    operator_id: FixedBytes<32>,
    logger: SharedLogger,
) -> AggregatedOperators {
    logger.debug(
        "Update aggregated operators",
        "eigen-services-blsaggregation.bls_agg.update_aggregated_operators",
    );

    let bls_signature_g1_point = bls_signature.g1_point().g1();

    if let Some(existing) = aggregated_operators.get_mut(&task_response_digest) {
        // If the operator is already in the aggregated operators, aggregate the new operator
        let updated = aggregate_new_operator(
            existing,
            operator_state.clone(),
            operator_id,
            bls_signature_g1_point,
            logger,
        );
        updated.clone()
    } else {
        // If the operator is not in the aggregated operators, create a new aggregated operator
        let operator_g2_pubkey = operator_state
            .operator_info
            .pub_keys
            .clone()
            .unwrap()
            .g2_pub_key
            .g2();
        let mut signers_apk_g2 = BlsG2Point::new(G2Affine::zero());
        let mut signers_agg_sig_g1 = Signature::new(G1Affine::zero());
        for _ in 0..operator_state.stake_per_quorum.len() {
            signers_apk_g2 = BlsG2Point::new((signers_apk_g2.g2() + operator_g2_pubkey).into());
            signers_agg_sig_g1 = Signature::new(
                (signers_agg_sig_g1.g1_point().g1() + bls_signature_g1_point).into(),
            );
        }
        AggregatedOperators {
            signers_apk_g2,
            signers_agg_sig_g1,
            signers_operator_ids_set: HashMap::from([(operator_state.operator_id, true)]),
            signers_total_stake_per_quorum: operator_state.stake_per_quorum.clone(),
        }
    }
}

/// Adds a new operator to the aggregated operators by aggregating its public key, signature and stake.
///
/// # Arguments
///
/// - `aggregated_operators` - Contains the information of all the aggregated operators.
/// - `operator_state` - The state of the operator, contains information about its stake.
/// - `signed_task_digest` - Contains the id and signature of the new operator.
/// - `logger` - The logger to log messages.
///
/// # Returns
///
/// The given aggregated operators, aggregated with the new operator info.
fn aggregate_new_operator(
    aggregated_operators: &mut AggregatedOperators,
    operator_state: OperatorAvsState,
    operator_id: FixedBytes<32>,
    signature_g1_point: G1Affine,
    logger: SharedLogger,
) -> &mut AggregatedOperators {
    let operator_g2_pubkey = operator_state
        .operator_info
        .pub_keys
        .clone()
        .unwrap()
        .g2_pub_key
        .g2();
    aggregated_operators
        .signers_operator_ids_set
        .insert(operator_id, true);

    logger.debug(
        &format!(
            "operator {} inserted in signers_operator_ids_set",
            operator_id
        ),
        "eigen-services-blsaggregation.bls_agg.aggregate_new_operator",
    );

    for (quorum_num, stake) in operator_state.stake_per_quorum.iter() {
        // For each quorum the operator has stake in, we aggregate the signature and update the stake
        aggregated_operators.signers_agg_sig_g1 = Signature::new(
            (aggregated_operators.signers_agg_sig_g1.g1_point().g1() + signature_g1_point).into(),
        );
        aggregated_operators.signers_apk_g2 =
            BlsG2Point::new((aggregated_operators.signers_apk_g2.g2() + operator_g2_pubkey).into());
        aggregated_operators
            .signers_total_stake_per_quorum
            .entry(*quorum_num)
            .and_modify(|v| *v += stake)
            .or_insert(*stake);
    }
    aggregated_operators
}

#[cfg(test)]
mod tests {
    use super::{BlsAggregationServiceError, BlsAggregationServiceResponse, BlsAggregatorService};
    use crate::bls_agg::{TaskMetadata, TaskSignature};
    use alloy::primitives::{B256, U256};
    use eigen_crypto_bls::{BlsG1Point, BlsG2Point, BlsKeyPair, Signature};
    use eigen_logging::get_test_logger;
    use eigen_services_avsregistry::fake_avs_registry_service::FakeAvsRegistryService;
    use eigen_types::avs::SignatureVerificationError::{DuplicateSignature, IncorrectSignature};
    use eigen_types::operator::{QuorumNum, QuorumThresholdPercentages};
    use eigen_types::{avs::TaskIndex, test::TestOperator};
    use sha2::{Digest, Sha256};
    use std::collections::HashMap;
    use std::time::Duration;
    use std::vec;
    use tokio::time::{sleep, Instant};

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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());
        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature,
                test_operator_1.operator_id,
            ))
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

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());
        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();
        let bls_signature_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let second_signature_processing_result = handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_1.clone(),
                test_operator_1.operator_id,
            ))
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

        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_signature_2.clone(),
                test_operator_2.operator_id,
            ))
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

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_op_3 = test_operator_3
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_3.clone(),
                test_operator_3.operator_id,
            ))
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

        let response = aggregator_response.receive_aggregated_response().await;
        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            ))
            .await
            .unwrap();

        let quorum_apks_g1 = aggregate_g1_public_keys(&test_operators);
        let signers_apk_g2 =
            aggregate_g2_public_keys(&[test_operators.clone(), test_operators].concat());
        let signers_agg_sig_g1 = aggregate_g1_signatures(&[
            bls_sig_op_1.clone(),
            bls_sig_op_1,
            bls_sig_op_2.clone(),
            bls_sig_op_2,
        ]);

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

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(expected_agg_service_response, response.unwrap());
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        // initialize 2 concurrent tasks
        let task_1_index = 1;
        let task_1_response = 123; // Initialize with appropriate data
        let task_1_response_digest = hash(task_1_response);
        let metadata1 = TaskMetadata::new(
            task_1_index,
            block_number,
            quorum_numbers.clone(),
            quorum_threshold_percentages.clone(),
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata1).await.unwrap();

        let task_2_index = 2;
        let task_2_response = 234; // Initialize with appropriate data
        let task_2_response_digest = hash(task_2_response);
        let metadata2 = TaskMetadata::new(
            task_2_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        handle.initialize_task(metadata2).await.unwrap();

        let bls_sig_task_1_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_1_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_1_index,
                task_1_response_digest,
                bls_sig_task_1_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_task_1_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_1_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_1_index,
                task_1_response_digest,
                bls_sig_task_1_op_2.clone(),
                test_operator_2.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_task_2_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_2_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_2_index,
                task_2_response_digest,
                bls_sig_task_2_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_task_2_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_2_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_2_index,
                task_2_response_digest,
                bls_sig_task_2_op_2.clone(),
                test_operator_2.operator_id,
            ))
            .await
            .unwrap();

        let quorum_apks_g1 = aggregate_g1_public_keys(&test_operators);
        let signers_apk_g2 =
            aggregate_g2_public_keys(&[test_operators.clone(), test_operators].concat());
        let signers_agg_sig_g1_task_1 = aggregate_g1_signatures(&[
            bls_sig_task_1_op_1.clone(),
            bls_sig_task_1_op_1,
            bls_sig_task_1_op_2.clone(),
            bls_sig_task_1_op_2,
        ]);

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

        let signers_agg_sig_g1_task_2 = aggregate_g1_signatures(&[
            bls_sig_task_2_op_1.clone(),
            bls_sig_task_2_op_1,
            bls_sig_task_2_op_2.clone(),
            bls_sig_task_2_op_2,
        ]);

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

        let first_response = aggregator_response.receive_aggregated_response().await;
        let second_response = aggregator_response.receive_aggregated_response().await;

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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());
        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(Err(BlsAggregationServiceError::TaskExpired), response);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
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

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1,
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(Err(BlsAggregationServiceError::TaskExpired), response);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            ))
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

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            ))
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

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1,
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_digest.as_ref());

        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_2,
                test_operator_2.operator_id,
            ))
            .await
            .unwrap();

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(Err(BlsAggregationServiceError::TaskExpired), response);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());

        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1,
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(Err(BlsAggregationServiceError::TaskExpired), response);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1,
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(Err(BlsAggregationServiceError::TaskExpired), response);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_digest.as_ref());
        let (handle, _) = bls_agg_service.start();
        let result = handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_digest,
                bls_sig_op_1,
                test_operator_1.operator_id,
            ))
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let task_response_1 = 123; // Initialize with appropriate data
        let task_response_1_digest = hash(task_response_1);
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_1_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_1_digest,
                bls_sig_op_1,
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let task_response_2 = 456; // Initialize with appropriate data
        let task_response_2_digest = hash(task_response_2);
        let bls_sig_op_2 = test_operator_1
            .bls_keypair
            .sign_message(task_response_2_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_2_digest,
                bls_sig_op_2,
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(Err(BlsAggregationServiceError::TaskExpired), response);
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
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());
        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        );
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let result = handle
            .process_signature(TaskSignature::new(
                task_index,
                wrong_task_response_digest,
                bls_signature.clone(),
                test_operator_1.operator_id,
            ))
            .await;

        assert_eq!(
            Err(BlsAggregationServiceError::SignatureVerificationError(
                IncorrectSignature
            )),
            result
        );

        // Also test that the aggregator service is not affected by the invalid signature, so the task should expire
        let response = aggregator_response.receive_aggregated_response().await;

        assert_eq!(Err(BlsAggregationServiceError::TaskExpired), response);
    }

    #[tokio::test]
    async fn test_signatures_are_processed_during_window_after_quorum() {
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
        let test_operator_3 = TestOperator {
            operator_id: U256::from(3).into(),
            stake_per_quorum: HashMap::from([(0u8, U256::from(100))]),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_3.into()).unwrap(),
        };
        let test_operators = vec![
            test_operator_1.clone(),
            test_operator_2.clone(),
            test_operator_3.clone(),
        ];
        let block_number = 1;
        let task_index = 0;
        let task_response = 123;
        let quorum_numbers: Vec<QuorumNum> = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![50_u8];
        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let time_to_expiry = Duration::from_secs(5);
        let window_duration = Duration::from_secs(1);

        let start = Instant::now();
        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        )
        .with_window_duration(window_duration);

        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let task_response_1_digest = hash(task_response);
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_1_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_1_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        let task_response_2_digest = hash(task_response);
        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_2_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_2_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            ))
            .await
            .unwrap();

        // quorum reached here, window should be open receiving signatures for 1 second
        sleep(Duration::from_millis(500)).await;
        let task_response_3_digest = hash(task_response);
        let bls_sig_op_3 = test_operator_3
            .bls_keypair
            .sign_message(task_response_3_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_3_digest,
                bls_sig_op_3.clone(),
                test_operator_3.operator_id,
            ))
            .await
            .unwrap();

        let signers_apk_g2 = aggregate_g2_public_keys(&vec![
            test_operator_1.clone(),
            test_operator_2.clone(),
            test_operator_3.clone(),
        ]);
        let signers_agg_sig_g1 =
            aggregate_g1_signatures(&[bls_sig_op_1, bls_sig_op_2, bls_sig_op_3]);
        let quorum_apks_g1 = vec![aggregate_g1_public_keys(&vec![
            test_operator_1,
            test_operator_2,
            test_operator_3,
        ])];

        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest: task_response_3_digest,
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1,
            signers_apk_g2,
            signers_agg_sig_g1,
            non_signer_quorum_bitmap_indices: vec![],
            quorum_apk_indices: vec![],
            total_stake_indices: vec![],
            non_signer_stake_indices: vec![],
        };

        let response = aggregator_response.receive_aggregated_response().await;

        let elapsed = start.elapsed();
        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
        assert!(elapsed < time_to_expiry);
        assert!(elapsed >= window_duration);
    }

    #[tokio::test]
    async fn test_if_quorum_has_been_reached_and_the_task_expires_during_window_the_response_is_sent(
    ) {
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
        let task_index = 0;
        let task_response = 123;
        let quorum_numbers: Vec<QuorumNum> = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![40_u8];
        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let time_to_expiry = Duration::from_secs(2);
        let window_duration = Duration::from_secs(10);

        let start = Instant::now();
        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        )
        .with_window_duration(window_duration);
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let task_response_1_digest = hash(task_response);
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_1_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_1_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        // quorum reached here, window should be open receiving signatures

        let task_response_2_digest = hash(task_response);
        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_2_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_2_digest,
                bls_sig_op_2.clone(),
                test_operator_2.operator_id,
            ))
            .await
            .unwrap();

        let signers_apk_g2 =
            aggregate_g2_public_keys(&vec![test_operator_1.clone(), test_operator_2.clone()]);
        let signers_agg_sig_g1 = aggregate_g1_signatures(&[bls_sig_op_1, bls_sig_op_2]);
        let quorum_apks_g1 = vec![aggregate_g1_public_keys(&vec![
            test_operator_1,
            test_operator_2,
        ])];

        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest: task_response_2_digest,
            non_signers_pub_keys_g1: vec![],
            quorum_apks_g1,
            signers_apk_g2,
            signers_agg_sig_g1,
            non_signer_quorum_bitmap_indices: vec![],
            quorum_apk_indices: vec![],
            total_stake_indices: vec![],
            non_signer_stake_indices: vec![],
        };

        let response = aggregator_response.receive_aggregated_response().await;

        let elapsed = start.elapsed();
        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
        assert!(elapsed >= time_to_expiry);
        assert!(elapsed < window_duration);
    }

    #[tokio::test]
    async fn test_if_window_duration_is_zero_no_signatures_are_aggregated_after_reaching_quorum() {
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
        let task_index = 0;
        let task_response = 123;
        let quorum_numbers: Vec<QuorumNum> = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![40_u8];
        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let time_to_expiry = Duration::from_secs(2);
        let window_duration = Duration::ZERO;

        let start = Instant::now();
        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        )
        .with_window_duration(window_duration);
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let task_response_1_digest = hash(task_response);
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_1_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_1_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        // quorum reached here but window duration is zero, so no more signatures should be aggregated
        sleep(Duration::from_millis(1)).await;

        let task_response_2_digest = hash(task_response);
        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_2_digest.as_ref());
        let process_signature_result = handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_2_digest,
                bls_sig_op_2,
                test_operator_2.operator_id,
            ))
            .await;
        assert_eq!(
            Err(BlsAggregationServiceError::SenderError),
            process_signature_result
        );

        let signers_apk_g2 = aggregate_g2_public_keys(&[test_operator_1.clone()]);
        let signers_agg_sig_g1 = aggregate_g1_signatures(&[bls_sig_op_1]);
        let quorum_apks_g1 = vec![aggregate_g1_public_keys(&vec![
            test_operator_1,
            test_operator_2.clone(),
        ])];

        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest: task_response_1_digest,
            non_signers_pub_keys_g1: vec![test_operator_2.bls_keypair.public_key()],
            quorum_apks_g1,
            signers_apk_g2,
            signers_agg_sig_g1,
            non_signer_quorum_bitmap_indices: vec![],
            quorum_apk_indices: vec![],
            total_stake_indices: vec![],
            non_signer_stake_indices: vec![],
        };

        let response = aggregator_response.receive_aggregated_response().await;

        let elapsed = start.elapsed();
        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
        assert!(elapsed < time_to_expiry);
    }

    #[tokio::test]
    async fn test_no_signatures_are_aggregated_after_window() {
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
        let task_index = 0;
        let task_response = 123;
        let quorum_numbers: Vec<QuorumNum> = vec![0];
        let quorum_threshold_percentages: QuorumThresholdPercentages = vec![40_u8];
        let fake_avs_registry_service = FakeAvsRegistryService::new(block_number, test_operators);
        let bls_agg_service =
            BlsAggregatorService::new(fake_avs_registry_service, get_test_logger());

        let time_to_expiry = Duration::from_secs(5);
        let window_duration = Duration::from_secs(1);

        let start = Instant::now();
        let metadata = TaskMetadata::new(
            task_index,
            block_number,
            quorum_numbers,
            quorum_threshold_percentages,
            time_to_expiry,
        )
        .with_window_duration(window_duration);
        let (handle, mut aggregator_response) = bls_agg_service.start();
        handle.initialize_task(metadata).await.unwrap();

        let task_response_1_digest = hash(task_response);
        let bls_sig_op_1 = test_operator_1
            .bls_keypair
            .sign_message(task_response_1_digest.as_ref());
        handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_1_digest,
                bls_sig_op_1.clone(),
                test_operator_1.operator_id,
            ))
            .await
            .unwrap();

        // quorum reached here, window should be open for 1 second
        sleep(Duration::from_secs(2)).await;

        let task_response_2_digest = hash(task_response);
        let bls_sig_op_2 = test_operator_2
            .bls_keypair
            .sign_message(task_response_2_digest.as_ref());
        let process_signature_result = handle
            .process_signature(TaskSignature::new(
                task_index,
                task_response_2_digest,
                bls_sig_op_2,
                test_operator_2.operator_id,
            ))
            .await;
        assert_eq!(
            Err(BlsAggregationServiceError::SenderError),
            process_signature_result
        );

        let signers_apk_g2 = aggregate_g2_public_keys(&[test_operator_1.clone()]);
        let signers_agg_sig_g1 = aggregate_g1_signatures(&[bls_sig_op_1]);
        let quorum_apks_g1 = vec![aggregate_g1_public_keys(&vec![
            test_operator_1,
            test_operator_2.clone(),
        ])];

        let expected_agg_service_response = BlsAggregationServiceResponse {
            task_index,
            task_response_digest: task_response_1_digest,
            non_signers_pub_keys_g1: vec![test_operator_2.bls_keypair.public_key()],
            quorum_apks_g1,
            signers_apk_g2,
            signers_agg_sig_g1,
            non_signer_quorum_bitmap_indices: vec![],
            quorum_apk_indices: vec![],
            total_stake_indices: vec![],
            non_signer_stake_indices: vec![],
        };

        let response = aggregator_response.receive_aggregated_response().await;

        let elapsed = start.elapsed();
        assert_eq!(expected_agg_service_response, response.clone().unwrap());
        assert_eq!(task_index, response.unwrap().task_index);
        assert!(elapsed < time_to_expiry);
    }
}
