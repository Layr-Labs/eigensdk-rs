#![allow(missing_docs)]
use alloy::{providers::Provider, rpc::types::Filter, sol_types::SolEvent};
use challenger::ChallengerTaskProcessor;
use eigen_common::get_ws_provider;
use error::ChallengerError;
use futures_util::StreamExt;
use tracing::info;

pub mod challenger;
pub mod error;

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

    pub async fn start_challenger(&mut self) -> Result<(), ChallengerError> {
        info!("challenger crate launched");

        let ws_provider = get_ws_provider(&self.ws_url).await?;

        // Subscribe to NewTaskEvent
        let task_filter = Filter::new().event_signature(
            <<TP as ChallengerTaskProcessor>::NewTaskEvent as SolEvent>::SIGNATURE_HASH,
        );
        let mut task_stream = ws_provider
            .subscribe_logs(&task_filter)
            .await?
            .into_stream();

        // Subscribe to TaskResponseEvent
        let responded_filter = Filter::new().event_signature(
            <<TP as ChallengerTaskProcessor>::TaskResponseEvent as SolEvent>::SIGNATURE_HASH,
        );
        let mut respond_stream = ws_provider
            .subscribe_logs(&responded_filter)
            .await?
            .into_stream();

        loop {
            tokio::select! {
                Some(log) = task_stream.next() => {
                    let decode = log.log_decode::<TP::NewTaskEvent>().ok();
                    if let Some(decoded) = decode {
                        self.task_processor.handle_task_creation(decoded);
                    }
                },
                Some(log) = respond_stream.next() => {
                    let decode = log.log_decode::<TP::TaskResponseEvent>().ok();
                    if let Some(decoded) = decode {
                        self.task_processor.handle_task_response(decoded);
                    }
                },
                else => {
                    // If both streams are exhausted, break the loop.
                    info!("challenger:No more logs to process, exiting loop.");
                    break;
                }
            }
        }

        Ok(())
    }
}
