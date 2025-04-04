use std::sync::Arc;

use eigen_services_blsaggregation::bls_agg::{ServiceHandle, TaskSignature};
use tarpc::{client::RpcError, context::Context, ServerError};
use tokio::sync::Mutex;
use tracing::info;

use crate::{AggregatorError, SignedTaskResponse, TaskProcessor, TaskResponse};

#[tarpc::service]
pub trait ProcessTaskSignedResponse {
    async fn process_signed_task_response(signed_task_response: String) -> Result<(), ServerError>;
}

#[derive(Clone)]
pub struct ProcessTaskSignedResponseServer<TP> {
    task_processor: Arc<Mutex<TP>>,
    service_handle: ServiceHandle,
}

impl<TP: TaskProcessor> ProcessTaskSignedResponse for ProcessTaskSignedResponseServer<TP> {
    // Each defined rpc generates an async fn that serves the RPC
    async fn process_signed_task_response(
        self,
        _ctx: Context,
        signed_task_response: String,
    ) -> Result<(), ServerError> {
        // Acá usás los campos de la struct
        let task_processor_clone = self.task_processor.clone();
        let service_handle = &self.service_handle;

        // Parseás el signed_task_response (si es JSON, por ejemplo)
        let parsed: SignedTaskResponse<TP::TaskResponse> =
            serde_json::from_str(&signed_task_response).unwrap();

        Self::process_signed_task_response(task_processor_clone, service_handle, parsed)
            .await
            .unwrap();
        Ok(())
    }
}

impl<TP: TaskProcessor> ProcessTaskSignedResponseServer<TP> {
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
