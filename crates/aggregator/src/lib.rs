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
use jsonrpc_core::serde_json;
use jsonrpc_core::{Error, IoHandler, Params, Value};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;
use traits::{task_processor::TaskProcessor, task_response::TaskResponse};

use config::AggregatorConfig;
pub use error::AggregatorError;
pub use signed_task_response::SignedTaskResponse;

/// Aggregator
#[derive(Debug)]
pub struct Aggregator<TP> {
    port_address: String,
    tp: TP,
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
    pub async fn new(config: AggregatorConfig, tp: TP) -> Result<Self, AggregatorError> {
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
            tp,
            service_handle,
            aggregated_response_receiver,
        })
    }

    /// Starts the aggregator service
    pub async fn start(self, ws_rpc_url: String) -> Result<(), AggregatorError> {
        info!("Starting aggregator");

        let tp = Arc::new(self.tp);
        let service_handle = self.service_handle.clone();
        let port_address = self.port_address.clone();

        // Spawn three tasks: one for the server that receives signature, one for processing tasks, and another to process aggregated signatures
        let server_handle = tokio::spawn(Self::start_server(
            port_address,
            tp.clone(),
            service_handle.clone(),
        ));
        let process_handle =
            tokio::spawn(Self::process_tasks(ws_rpc_url, tp.clone(), service_handle));
        let aggregate_handle = tokio::spawn(Self::process_aggregated_signatures(
            tp,
            self.aggregated_response_receiver,
        ));

        // Wait for both tasks to complete and handle potential errors
        match tokio::try_join!(server_handle, process_handle, aggregate_handle) {
            Ok((server_result, process_result, aggregate_result)) => {
                server_result?;
                process_result?;
                aggregate_result?;
            }
            Err(_e) => {
                return Err(AggregatorError::JoinError);
            }
        }

        Ok(())
    }

    /// Starts the RPC server
    ///
    /// # Arguments
    ///
    /// * `port_address` - The server address
    /// * `tp` - The task processor
    /// * `service_handle` - The service handle
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    async fn start_server(
        port_address: String,
        tp: Arc<TP>,
        service_handle: ServiceHandle,
    ) -> Result<(), AggregatorError> {
        let mut io = IoHandler::new();
        io.add_method("process_signed_task_response", move |params: Params| {
            let tp = tp.clone();
            let service_handle = service_handle.clone();
            async move {
                let Params::Map(map) = params else {
                    return Err(Error::invalid_params("Expected a map"));
                };
                let params = map
                    .get("params")
                    .ok_or(Error::invalid_params("Expected params"))?;
                let signed_task_response: SignedTaskResponse<TP::TaskResponse> =
                    serde_json::from_value(params.clone())
                        .map_err(|err| Error::invalid_params(err.to_string()))?;

                let result =
                    Self::process_signed_task_response(&tp, &service_handle, signed_task_response)
                        .await;

                match result {
                    Ok(_) => Ok(Value::Bool(true)),
                    Err(_) => Err(Error::invalid_params("Invalid task response")),
                }
            }
        });

        let socket: SocketAddr = port_address.parse().map_err(|e| {
            AggregatorError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;

        let server = ServerBuilder::new(io)
            .cors(DomainsValidation::AllowOnly(vec![
                AccessControlAllowOrigin::Any,
            ]))
            .start_http(&socket)?;

        // TODO: move to an async library
        tokio::task::spawn_blocking(move || server.wait());

        info!("Server running at {socket}");

        Ok(())
    }

    /// Processes the tasks
    ///
    /// # Arguments
    ///
    /// * `ws_rpc_url` - The websocket RPC URL
    /// * `tp` - The task processor
    /// * `service_handle` - The service handle
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    async fn process_tasks(
        ws_rpc_url: String,
        tp: Arc<TP>,
        service_handle: ServiceHandle,
    ) -> Result<(), AggregatorError> {
        let ws = WsConnect::new(ws_rpc_url.clone());
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        let filter = Filter::new().event_signature(TP::NewTaskEvent::SIGNATURE_HASH);
        let sub = provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            let event: TP::NewTaskEvent = log.log_decode()?.inner.data;

            let info = tp.process_new_task(event).await?;

            service_handle.initialize_task(info).await?;
        }

        Ok(())
    }

    /// Processes the signed task response
    ///
    /// # Arguments
    ///
    /// * `tp` - The task processor
    /// * `service_handle` - The service handle
    /// * [`SignedTaskResponse`] - The signed task response
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    async fn process_signed_task_response(
        tp: &TP,
        service_handle: &ServiceHandle,
        signed_task_response: SignedTaskResponse<TP::TaskResponse>,
    ) -> Result<(), AggregatorError> {
        let SignedTaskResponse {
            task_response,
            signature,
            operator_id,
        } = signed_task_response;
        let task_index = task_response.task_index();

        let task_response_digest = tp.process_task_response(task_response).await?;

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
    /// * `tp` - The task processor
    /// * `aggregated_response_receiver` - The aggregated response receiver
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    async fn process_aggregated_signatures(
        tp: Arc<TP>,
        mut aggregated_response_receiver: AggregateReceiver,
    ) -> Result<(), AggregatorError> {
        loop {
            let service_response = aggregated_response_receiver
                .receive_aggregated_response()
                .await?;

            tp.process_aggregated_response(service_response).await?;
        }
    }
}
