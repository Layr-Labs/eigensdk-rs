//! Aggregator crate

/// Aggregator Config
pub mod config;
/// Aggregator error
pub mod error;
/// Signed Task Response
pub mod signed_task_response;
/// Traits
pub mod traits;

use alloy::providers::Provider;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use alloy::sol_types::SolEvent;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_common::get_ws_provider;
use eigen_logging::get_logger;
use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigen_services_blsaggregation::bls_agg::{
    AggregateReceiver, BlsAggregatorService, ServiceHandle, TaskSignature,
};
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use futures_util::StreamExt;
use jsonrpsee::server::Server;
use jsonrpsee::types::ErrorObject;
use jsonrpsee::RpcModule;
use signed_task_response::RpcRequest;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use tracing::info;

pub use config::AggregatorConfig;
pub use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
pub use error::AggregatorError;
pub use signed_task_response::SignedTaskResponse;
pub use traits::{
    task_processor::{TaskProcessor, TaskProcessorError},
    task_response::TaskResponse,
};

/// Aggregator
#[derive(Debug)]
pub struct Aggregator<TP> {
    port_address: String,
    task_processor: Arc<Mutex<TP>>,
    service_handle: ServiceHandle,
    aggregated_response_receiver: AggregateReceiver,
}

impl<TP: TaskProcessor + Send + Sync + 'static> Aggregator<TP> {
    /// Creates a new aggregator
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the aggregator
    ///
    /// # Returns
    ///
    /// * `Self` - The aggregator
    pub async fn new(
        config: AggregatorConfig,
        task_processor: TP,
    ) -> Result<Self, AggregatorError> {
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            get_logger(),
            config.registry_coordinator,
            config.operator_state_retriever,
            config.http_rpc_url,
        )
        .await?;

        let operators_info_service = OperatorInfoServiceInMemory::new(
            get_logger(),
            avs_registry_chain_reader.clone(),
            config.ws_rpc_url.clone(),
        )
        .await?
        .0;
        let token = tokio_util::sync::CancellationToken::new();
        let avs_registry_service_chaincaller = AvsRegistryServiceChainCaller::new(
            avs_registry_chain_reader,
            operators_info_service.clone(),
        );
        let provider = get_ws_provider(config.ws_rpc_url.as_str()).await?;

        let current_block_number = provider.get_block_number().await?;
        tokio::spawn(async move {
            let _ = operators_info_service
                .start_service(&token, 0, current_block_number)
                .await;
        });

        let (service_handle, aggregated_response_receiver) =
            BlsAggregatorService::new(avs_registry_service_chaincaller, get_logger()).start();

        Ok(Self {
            port_address: config.server_address,
            task_processor: Arc::new(Mutex::new(task_processor)),
            service_handle,
            aggregated_response_receiver,
        })
    }

    /// Starts the aggregator service
    ///
    /// Creates the following tasks:
    /// - start_server: Starts the server that receives signatures
    /// - process_tasks: receives tasks from the log and processes them
    /// - process_aggregated_signatures: processes the aggregated signatures
    ///
    /// # Arguments
    ///
    /// * `ws_rpc_url` - The websocket RPC URL
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    pub async fn start(self, ws_rpc_url: String) -> Result<(), AggregatorError> {
        info!("Starting aggregator");

        let task_processor = self.task_processor.clone();
        let service_handle = self.service_handle.clone();
        let port_address = self.port_address.clone();

        // Spawn three tasks: one for the server that receives signature, one for processing tasks, and another to process aggregated signatures
        let server_handle = tokio::spawn(Self::start_server(
            port_address,
            task_processor,
            service_handle.clone(),
        ));
        let task_processor = self.task_processor.clone();

        let process_handle = tokio::spawn(Self::process_tasks(
            ws_rpc_url,
            task_processor.clone(),
            service_handle,
        ));
        let aggregate_handle = tokio::spawn(Self::process_aggregated_signatures(
            task_processor,
            self.aggregated_response_receiver,
        ));

        // Wait for both tasks to complete and handle potential errors
        let (server_result, process_result, aggregate_result) =
            tokio::try_join!(server_handle, process_handle, aggregate_handle)
                .map_err(|_e| AggregatorError::JoinError)?;

        server_result?;
        process_result?;
        aggregate_result?;

        Ok(())
    }

    /// Starts the RPC server
    ///
    /// # Arguments
    ///
    /// * `port_address` - The server address
    /// * `task_processor` - The task processor
    /// * `service_handle` - The service handle
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    async fn start_server(
        port_address: String,
        task_processor: Arc<Mutex<TP>>,
        service_handle: ServiceHandle,
    ) -> Result<JoinHandle<()>, AggregatorError> {
        // See https://github.com/paritytech/jsonrpsee/blob/42461391fee47c94d42c4a7303355525291df9f6/examples/examples/cors_server.rs
        let mut module = RpcModule::new((service_handle, task_processor));
        module
            .register_async_method(
                "process_signed_task_response",
                |params, ctx, _| async move {
                    let (service_handle, task_processor) = ctx.as_ref();
                    let signed_task_response = params
                        .parse::<RpcRequest<TP::TaskResponse>>()
                        .map_err(|err| ErrorObject::owned(0, err.to_string(), None::<()>))?
                        .params;

                    let task_processor_clone = task_processor.clone();

                    let result = Self::process_signed_task_response(
                        task_processor_clone,
                        service_handle,
                        signed_task_response,
                    )
                    .await;

                    // TODO: Check if we can do map_err and map
                    match result {
                        Ok(()) => Ok(true),
                        Err(err) => Err(ErrorObject::owned(0, err.to_string(), None::<()>)),
                    }
                },
            )
            .map_err(|_| AggregatorError::RpcError)?;

        let socket: SocketAddr = port_address.parse().map_err(|e| {
            AggregatorError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;
        let middleware = tower::ServiceBuilder::new();
        let server = Server::builder()
            .set_http_middleware(middleware)
            .build(socket)
            .await?;

        let handle = server.start(module);

        info!("Server running at {socket}");

        Ok(tokio::spawn(handle.stopped()))
    }

    /// Processes the tasks
    ///
    /// # Arguments
    ///
    /// * `ws_rpc_url` - The websocket RPC URL
    /// * `task_processor` - The task processor
    /// * `service_handle` - The service handle
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    async fn process_tasks(
        ws_rpc_url: String,
        task_processor: Arc<Mutex<TP>>,
        service_handle: ServiceHandle,
    ) -> Result<(), AggregatorError> {
        let ws = WsConnect::new(ws_rpc_url.clone());
        let filter = Filter::new().event_signature(TP::NewTaskEvent::SIGNATURE_HASH);
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        while let Some(event) = provider
            .subscribe_logs(&filter)
            .await?
            .into_stream()
            .next()
            .await
            .and_then(|log| log.log_decode().ok())
            .map(|v| v.inner.data)
        {
            let metadata = task_processor
                .lock()
                .await
                .process_new_task(event)
                .await
                .map_err(AggregatorError::TaskProcessorError)?;
            service_handle.initialize_task(metadata).await?;
        }

        Ok(())
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

    /// Processes aggregated signatures
    ///
    /// # Arguments
    ///
    /// * `task_processor` - The task processor
    /// * `aggregated_response_receiver` - The aggregated response receiver
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    async fn process_aggregated_signatures(
        task_processor: Arc<Mutex<TP>>,
        mut aggregated_response_receiver: AggregateReceiver,
    ) -> Result<(), AggregatorError> {
        loop {
            let service_response = aggregated_response_receiver
                .receive_aggregated_response()
                .await?;

            task_processor
                .lock()
                .await
                .process_aggregated_response(service_response)
                .await
                .map_err(AggregatorError::TaskProcessorError)?;
        }
    }
}
