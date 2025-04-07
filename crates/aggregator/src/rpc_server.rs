use std::sync::Arc;

use eigen_services_blsaggregation::bls_agg::{ServiceHandle, TaskSignature};
use tarpc::{context::Context, ServerError};
use tokio::sync::Mutex;
use tracing::info;

use crate::{AggregatorError, SignedTaskResponse, TaskProcessor, TaskResponse};

#[tarpc::service]
/// This is the service definition. It defines one RPC, [`process_signed_task_response`].
/// This is the RPC that the aggregator will use to process the signed task response.
pub trait ProcessSignedTaskResponse {
    /// Processes the signed task response
    ///
    /// # Arguments
    ///
    /// * `signed_task_response` - The signed task response
    ///
    /// # Returns
    ///
    /// * `Result<bool, ServerError>` - The result of the operation
    async fn process_signed_task_response(
        signed_task_response: String,
    ) -> Result<bool, ServerError>;
}

#[derive(Debug, Clone)]
/// Server for the ProcessSignedTaskResponse RPC
pub struct ProcessSignedTaskResponseServer<TP>
where
    TP: Clone,
{
    task_processor: Arc<Mutex<TP>>,
    service_handle: ServiceHandle,
}

/// Implementation of the ProcessSignedTaskResponse trait for the ProcessSignedTaskResponseServer
/// The async method serves the RPC request and processes the signed task response
impl<TP: TaskProcessor + std::clone::Clone> ProcessSignedTaskResponse
    for ProcessSignedTaskResponseServer<TP>
{
    async fn process_signed_task_response(
        self,
        _ctx: Context,
        signed_task_response: String,
    ) -> Result<bool, ServerError> {
        let task_processor_clone = self.task_processor.clone();
        let service_handle = &self.service_handle;
        let parsed: SignedTaskResponse<TP::TaskResponse> =
            serde_json::from_str(&signed_task_response).map_err(|_| {
                ServerError::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid signed task response".to_string(),
                )
            })?;

        Self::process_signed_task_response(task_processor_clone, service_handle, parsed)
            .await
            .map_err(|_| {
                ServerError::new(
                    std::io::ErrorKind::Other,
                    "Error processing signed task response".to_string(),
                )
            })?;
        Ok(true)
    }
}

impl<TP: TaskProcessor + std::clone::Clone> ProcessSignedTaskResponseServer<TP> {
    /// Creates a new [`ProcessSignedTaskResponseServer`]
    ///
    /// # Arguments
    ///
    /// * `task_processor` - The task processor
    /// * `service_handle` - The service handle
    ///
    /// # Returns
    ///
    /// * `Self` - The [`ProcessSignedTaskResponseServer`]
    pub fn new(task_processor: Arc<Mutex<TP>>, service_handle: ServiceHandle) -> Self {
        Self {
            task_processor,
            service_handle,
        }
    }

    /// Processes the signed task response
    ///
    /// # Arguments
    ///
    /// * `task_processor` - The task processor
    /// * `service_handle` - The service handle
    /// * [`SignedTaskResponse`] - The signed task response
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    async fn process_signed_task_response(
        task_processor: Arc<Mutex<TP>>,
        service_handle: &ServiceHandle,
        signed_task_response: SignedTaskResponse<TP::TaskResponse>,
    ) -> Result<(), AggregatorError> {
        let SignedTaskResponse {
            task_response,
            signature,
            operator_id,
        } = signed_task_response;
        let task_index = task_response.task_index();

        let task_response_digest = task_processor
            .lock()
            .await
            .process_task_response(task_response)
            .await
            .map_err(AggregatorError::TaskProcessorError)?;

        let task_signature =
            TaskSignature::new(task_index, task_response_digest, signature, operator_id);

        service_handle.process_signature(task_signature).await?;
        info!("processed signature for index {:?}", task_index);

        Ok(())
    }
}
