use eigen_aggregator::rpc_server::ProcessSignedTaskResponseClient;
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
    /// Create a new client that connects to the aggregator RPC server
    ///
    /// # Arguments
    ///
    /// * `aggregator_ip_port_address` - The IP and port address of the aggregator
    ///
    /// # Returns
    ///
    /// * `Result<Self, OperatorError>` - The client aggregator
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

    /// Send signed task response to the aggregator with exponential backoff
    ///
    /// # Arguments
    ///
    /// * `signed_task_response` - The signed task response to send
    ///
    /// # Returns
    ///
    /// * `Result<(), OperatorError>` - The result of the operation
    pub async fn send_signed_task_response(
        &self,
        signed_task_response: Vec<u8>,
    ) -> Result<(), OperatorError> {
        let mut delay = Duration::from_secs(1);

        for _ in 0..5 {
            let ctx = tarpc::context::current();
            let response = self
                .client
                .process_signed_task_response(ctx, signed_task_response.clone())
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

        Err(OperatorError::MaxRetryExceeded)
    }
}
