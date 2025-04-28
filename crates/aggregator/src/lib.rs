//! Aggregator crate

/// Aggregator Config
pub mod config;
/// Aggregator error
pub mod error;
/// RPC server
pub mod rpc_server;
/// Signed Task Response
pub mod signed_task_response;
/// Traits
pub mod traits;

use alloy::dyn_abi::abi::decode_params;
use alloy::dyn_abi::Decoder;
use alloy::primitives::Bytes;
use alloy::providers::Provider;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use alloy::sol_types::{SolEvent, SolValue};
use alloy_rlp::{decode_exact, Decodable};
use alloy_rlp::{RlpDecodable, RlpEncodable};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_common::get_ws_provider;
use eigen_logging::get_logger;
use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigen_services_blsaggregation::bls_agg::{
    AggregateReceiver, BlsAggregatorService, ServiceHandle,
};
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use futures_util::{future, StreamExt};
use rpc_server::{ProcessSignedTaskResponse, ProcessSignedTaskResponseServer};
use std::{net::SocketAddr, sync::Arc};
use tarpc::server::{self, Channel};
use tarpc::tokio_serde::formats::Json;
use tokio::sync::Mutex;
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
    ws_rpc_url: String,
}

impl<TP: TaskProcessor + Send + Sync + 'static + Clone> Aggregator<TP> {
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
            ws_rpc_url: config.ws_rpc_url,
        })
    }

    /// Starts the aggregator service
    ///
    /// Creates the following tasks:
    /// - start_server: Starts the server that receives signatures
    /// - process_tasks: receives tasks from the log and processes them
    /// - process_aggregated_signatures: processes the aggregated signatures
    ///
    /// # Returns
    ///
    /// * `Result<(), AggregatorError>` - The result of the operation
    pub async fn start(self) -> Result<(), AggregatorError> {
        info!("Starting aggregator");

        let task_processor = self.task_processor.clone();
        let service_handle = self.service_handle.clone();
        let port_address = self.port_address.clone();

        // Spawn three tasks: one for the server that receives signature, one for processing tasks, and another to process aggregated signatures
        let server_handle = tokio::spawn(Self::start_server(
            port_address,
            task_processor.clone(),
            service_handle.clone(),
        ));

        let process_handle = tokio::spawn(Self::process_tasks(
            self.ws_rpc_url,
            task_processor.clone(),
            service_handle,
        ));
        let aggregate_handle = tokio::spawn(Self::process_aggregated_signatures(
            task_processor,
            self.aggregated_response_receiver,
        ));

        // Wait for the tasks to complete and handle potential errors
        let (server_result, process_result, aggregate_result) =
            tokio::try_join!(server_handle, process_handle, aggregate_handle)
                .map_err(|_| AggregatorError::JoinError)?;

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
    ) -> Result<(), AggregatorError> {
        let addr: SocketAddr = port_address.parse().map_err(|e| {
            AggregatorError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;

        let task_processor_clone = task_processor.clone();
        let service_handle_clone = service_handle.clone();

        let mut listener = tarpc::serde_transport::tcp::listen(&addr, Json::default).await?;
        info!("Server running at {}", addr);

        listener.config_mut().max_frame_length(usize::MAX);
        listener
            .filter_map(|r| future::ready(r.ok()))
            .map(server::BaseChannel::with_defaults)
            .for_each_concurrent(None, |channel| {
                let task_processor = task_processor_clone.clone();
                let service_handle = service_handle_clone.clone();
                async move {
                    let server =
                        ProcessSignedTaskResponseServer::new(task_processor, service_handle);
                    channel
                        .execute(server.serve())
                        .for_each(|response| async move {
                            tokio::spawn(response);
                        })
                        .await;
                }
            })
            .await;

        Ok(())
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
        // .and_then(|log| log.log_decode().ok())
        // .map(|v| v.inner.data)
        {
            // task_index viene del topic (si es indexed)
            let task_index = u32::from_be_bytes(event.topics()[1][28..32].try_into().unwrap());

            // Campos del data
            let task_created_block = u32::abi_decode(&mut decoder, false).unwrap();
            let quorum_numbers = Bytes::abi_decode(&mut decoder, false).unwrap();
            let quorum_threshold_percentage = u8::abi_decode(&mut decoder, false).unwrap();
            let input = <TP::Input>::abi_decode_params(&mut decoder, false).unwrap();

            // let metadata = task_processor
            //     .lock()
            //     .await
            //     .process_new_task(event)
            //     .await
            //     .map_err(AggregatorError::TaskProcessorError)?;
            // service_handle.initialize_task(metadata).await?;
        }

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

#[derive(Debug, RlpEncodable, RlpDecodable)]
struct GenericEvent<Input>
where
    Input: Clone + Decodable,
{
    task_index: u32,
    task: Task<Input>,
}

#[derive(Debug, RlpEncodable, RlpDecodable)]
pub struct Task<Input>
where
    Input: Clone + Decodable,
{
    #[allow(missing_docs)]
    pub input: Input,
    #[allow(missing_docs)]
    pub task_created_block: u32,
    #[allow(missing_docs)]
    pub quorum_numbers: Bytes,
    #[allow(missing_docs)]
    pub quorum_threshold_percentage: u8,
}
