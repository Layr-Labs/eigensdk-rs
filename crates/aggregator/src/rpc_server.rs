use std::sync::Arc;

use eigen_services_blsaggregation::bls_agg::{ServiceHandle, TaskSignature};
use tarpc::{context::Context, ServerError};
use tokio::sync::Mutex;
use tracing::info;

use crate::{AggregatorError, SignedTaskResponse, TaskProcessor, TaskResponse};

#[tarpc::service]
pub trait ProcessSignedTaskResponse {
    async fn process_signed_task_response(
        signed_task_response: String,
    ) -> Result<bool, ServerError>;
}

#[derive(Clone)]
pub struct ProcessSignedTaskResponseServer<TP>
where
    TP: Clone,
{
    task_processor: Arc<Mutex<TP>>,
    service_handle: ServiceHandle,
}

impl<TP: TaskProcessor + std::clone::Clone> ProcessSignedTaskResponse
    for ProcessSignedTaskResponseServer<TP>
{
    // Each defined rpc generates an async fn that serves the RPC
    async fn process_signed_task_response(
        self,
        _ctx: Context,
        signed_task_response: String,
    ) -> Result<bool, ServerError> {
        dbg!("RECIBI REQUEST");
        let task_processor_clone = self.task_processor.clone();
        let service_handle = &self.service_handle;

        dbg!("POR PARSEAR");
        let parsed: SignedTaskResponse<TP::TaskResponse> =
            serde_json::from_str(&signed_task_response).unwrap();
        dbg!("PARSED");

        Self::process_signed_task_response(task_processor_clone, service_handle, parsed)
            .await
            .unwrap();
        Ok(true)
    }
}

impl<TP: TaskProcessor + std::clone::Clone> ProcessSignedTaskResponseServer<TP> {
    pub fn new(task_processor: Arc<Mutex<TP>>, service_handle: ServiceHandle) -> Self {
        Self {
            task_processor,
            service_handle,
        }
    }

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
