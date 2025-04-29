use std::fmt::Debug;

use crate::{AggregatorError, SignedTaskResponse};
use alloy::{
    contract::private::{Provider, Transport},
    network::Network,
};
use eigen_services_blsaggregation::bls_agg::{ServiceHandle, TaskSignature};
use eigen_task_processor::{
    task_manager::TaskManagerContract, task_response::TaskResponse, IndexingTaskProcessor,
};
use tarpc::{context::Context, ServerError};
use tracing::info;

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
pub struct ProcessSignedTaskResponseServer<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Debug + Send + Sync + 'static + Clone,
    T: Transport + Clone + Send + Sync + 'static,
    P: Provider<T, N> + Clone + Send + Sync + 'static,
    N: Network,
{
    task_processor: IndexingTaskProcessor<TM, T, P, N>,
    service_handle: ServiceHandle,
}

/// Implementation of the ProcessSignedTaskResponse trait for the ProcessSignedTaskResponseServer
/// The async method serves the RPC request and processes the signed task response
impl<TM, T, P, N> ProcessSignedTaskResponse for ProcessSignedTaskResponseServer<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Debug + Send + Sync + 'static + Clone,
    T: Transport + Clone + Send + Sync + 'static,
    P: Provider<T, N> + Clone + Send + Sync + 'static,
    N: Network,
{
    async fn process_signed_task_response(
        mut self,
        _ctx: Context,
        signed_task_response: String,
    ) -> Result<bool, ServerError> {
        let service_handle = &self.service_handle;
        let parsed: SignedTaskResponse<TaskResponse<TM::Output>> =
            serde_json::from_str(&signed_task_response).map_err(|_| {
                ServerError::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid signed task response".to_string(),
                )
            })?;

        Self::process_signed_task_response(&mut self.task_processor, service_handle, parsed)
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

impl<TM, T, P, N> ProcessSignedTaskResponseServer<TM, T, P, N>
where
    TM: TaskManagerContract<T, P, N> + Debug + Send + Sync + 'static + Clone,
    T: Transport + Clone + Send + Sync + 'static,
    P: Provider<T, N> + Clone + Send + Sync + 'static,
    N: Network,
{
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
    pub fn new(
        task_processor: IndexingTaskProcessor<TM, T, P, N>,
        service_handle: ServiceHandle,
    ) -> Self {
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
        task_processor: &mut IndexingTaskProcessor<TM, T, P, N>,
        service_handle: &ServiceHandle,
        signed_task_response: SignedTaskResponse<TaskResponse<TM::Output>>,
    ) -> Result<(), AggregatorError> {
        let SignedTaskResponse {
            task_response,
            signature,
            operator_id,
        } = signed_task_response;
        let task_index = task_response.task_index;

        let task_response_digest = task_processor.process_task_response(task_response).await;

        let task_signature =
            TaskSignature::new(task_index, task_response_digest, signature, operator_id);

        service_handle.process_signature(task_signature).await?;
        info!("processed signature for index {}", task_index);

        Ok(())
    }
}
