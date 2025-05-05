#![allow(missing_docs)]
use alloy::{providers::Provider, rpc::types::Filter, sol_types::SolEvent};
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
    ws_url: String,
    task_processor: TP,
}

impl<TP: ChallengerTaskProcessor> Challenger<TP> {
    pub fn new(ws_url: String, task_processor: TP) -> Self {
        Self {
            ws_url,
            task_processor,
        }
    }

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
                    self.task_processor.handle_task_creation(log).await?;
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
}
