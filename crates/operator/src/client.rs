use eigen_aggregator::rpc_server::ProcessSignedTaskResponseClient;
use serde::Serialize;
use tarpc::tokio_serde::formats::Json;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use crate::error::OperatorError;

/// Client Aggregator
#[derive(Debug, Clone)]
pub struct ClientAggregator {
    /// TARPC client to send requests to aggregator
    pub client: ProcessSignedTaskResponseClient,
}

impl ClientAggregator {
    /// new
    pub async fn new(aggregator_ip_port_address: String) -> Result<Self, OperatorError> {
        let transport =
            tarpc::serde_transport::tcp::connect(aggregator_ip_port_address, Json::default)
                .await
                .map_err(|_| OperatorError::TransportError)?;
        let client =
            ProcessSignedTaskResponseClient::new(tarpc::client::Config::default(), transport)
                .spawn();

        Ok(Self { client })
    }

    /// Send signed task response
    pub async fn send_signed_task_response(
        &self,
        signed_task_response: impl Serialize,
    ) -> Result<(), OperatorError> {
        let mut delay = Duration::from_secs(1);

        for _ in 0..5 {
            let signed_task_string = serde_json::to_string(&signed_task_response)?;
            let ctx = tarpc::context::current();
            let response = self
                .client
                .process_signed_task_response(ctx, signed_task_string)
                .await?;

            if response.is_ok() {
                info!("Signed task response sent to aggregator");
                return Ok(());
            }

            // Exponential backoff
            info!("Retrying in {} seconds...", delay.as_secs());
            sleep(delay).await;
            delay *= 2; // Double the delay for the next retry
        }
        error!("Could not send signed task response to aggregator. Tried 5 times.");
        // TODO: return error indicating that the task response could not be sent
        Ok(())
    }
}
