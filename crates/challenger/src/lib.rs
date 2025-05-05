#![allow(missing_docs)]
use alloy::{
    dyn_abi::SolType,
    primitives::Bytes,
    providers::Provider,
    rpc::types::{Filter, Log},
    sol_types::{SolEvent, SolValue},
};
use challenger::ChallengerTaskProcessor;
use eigen_common::get_ws_provider;
use eigen_task_processor::{task::Task, task_response::TaskResponse};
use error::ChallengerError;
use futures_util::StreamExt;
use tracing::info;

pub mod challenger;
pub mod challenger_processor;
pub mod error;
pub mod task_manager;

/// Main Challenger struct
#[derive(Debug)]
pub struct Challenger<TP: ChallengerTaskProcessor> {
    /// The websocket url
    ws_url: String,
    /// The task processor
    task_processor: TP,
}

impl<TP: ChallengerTaskProcessor> Challenger<TP>
where
    TP::Input: From<<<TP::Input as SolValue>::SolType as SolType>::RustType>,
{
    /// Create a new challenger
    ///
    /// # Arguments
    ///
    /// * `ws_url` - The websocket url
    /// * `task_processor` - The task processor
    ///
    /// # Returns
    ///
    /// * `Self` - The challenger
    pub fn new(ws_url: String, task_processor: TP) -> Self {
        Self {
            ws_url,
            task_processor,
        }
    }

    /// Start the service and start listening for new tasks and task responses events
    /// It also checks if the response is correct, if not it raises a challenge.
    ///
    /// # Arguments
    ///
    /// * `is_response_correct` - The logic to check if the response is correct
    ///
    /// # Returns
    pub async fn start_challenger<F>(
        &mut self,
        is_response_correct: F,
    ) -> Result<(), ChallengerError>
    where
        F: Fn(Task<TP::Input>, TaskResponse<TP::Output>) -> Result<bool, ChallengerError>
            + Send
            + Sync,
    {
        info!("challenger crate launched");

        let ws_provider = get_ws_provider(&self.ws_url).await?;

        // Subscribe to NewTaskEvent
        let task_filter = Filter::new().event_signature(TP::NewTaskEvent::SIGNATURE_HASH);
        let mut task_stream = ws_provider
            .subscribe_logs(&task_filter)
            .await?
            .into_stream();

        // Subscribe to TaskResponseEvent
        let responded_filter = Filter::new().event_signature(TP::TaskResponseEvent::SIGNATURE_HASH);
        let mut responded_stream = ws_provider
            .subscribe_logs(&responded_filter)
            .await?
            .into_stream();

        loop {
            tokio::select! {
                Some(log) = task_stream.next() => {
                    let (task_index, task) = self.decode_task_creation_event(log)?;
                    self.task_processor.handle_task_creation(task_index, task).await?;
                },
                Some(log) = responded_stream.next() => {
                    self.task_processor
                        .handle_task_response(log, &is_response_correct)
                        .await?;
                },
                else => {
                    // If both streams are exhausted, break the loop.
                    info!("challenger: No more logs to process, exiting loop.");
                    break;
                }
            }
        }

        Ok(())
    }

    fn decode_task_creation_event(
        &self,
        log: Log,
    ) -> Result<(u32, Task<TP::Input>), ChallengerError> {
        // event NewTaskCreated(uint32 indexed taskIndex, Task task);
        // Since taskIndex is indexed type, it is present in the topics array
        // The first element of the topic is the event hash signature, the second is the taskIndex
        let bytes = log
            .topics()
            .get(1)
            .ok_or(ChallengerError::TaskIndexMissingInTopics)?
            .0;

        // u32 values are stored in the last 4 bytes of a 32 bytes array (left-padded).
        let task_index_bytes: [u8; 4] = bytes[28..32]
            .try_into()
            .map_err(|_| ChallengerError::InvalidTaskIndexConversion)?;
        let task_index = u32::from_be_bytes(task_index_bytes);

        // Skip the first 32 bytes of the ABI-encoded data (the dynamic offset pointer)
        let data = log
            .inner
            .data
            .data
            .0
            .get(32..)
            .ok_or(ChallengerError::EmptyDecodedData)?;

        // Decode Task<TM::Input>
        let (input, task_created_block, quorum_numbers, quorum_threshold_percentage) =
            <(
                <TP::Input as SolValue>::SolType,
                <u32 as SolValue>::SolType,
                <Bytes as SolValue>::SolType,
                <u32 as SolValue>::SolType,
            )>::abi_decode_params(data, false)?;

        let task = Task::<TP::Input> {
            input: input.into(),
            task_created_block,
            quorum_numbers,
            quorum_threshold_percentage,
        };

        Ok((task_index, task))
    }
}
