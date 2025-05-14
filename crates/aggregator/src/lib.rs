//! Aggregator crate

/// Aggregator Config
pub mod config;
/// Aggregator error
pub mod error;
/// RPC server
pub mod rpc_server;
/// Signed Task Response
pub mod signed_task_response;
/// Task Processor
pub mod task_processor;

use alloy::dyn_abi::SolType;
use alloy::providers::Provider;
use alloy::providers::{ProviderBuilder, WsConnect};
use alloy::rpc::types::Filter;
use alloy::sol_types::SolValue;
use ark_ec::AffineRepr;
pub use config::AggregatorConfig;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_common::get_ws_provider;
use eigen_crypto_bls::error::BlsError;
use eigen_crypto_bls::{convert_to_g1_point, convert_to_g2_point};
use eigen_logging::get_logger;
use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
use eigen_services_blsaggregation::bls_agg::{
    AggregateReceiver, BlsAggregatorService, ServiceHandle,
};
pub use eigen_services_blsaggregation::{
    bls_agg::TaskMetadata, bls_aggregation_service_response::BlsAggregationServiceResponse,
};
use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use eigen_task_manager::event_decoder::decode_new_task;
use eigen_utils::slashing::middleware::{
    iblssignaturechecker::IBLSSignatureCheckerTypes::NonSignerStakesAndSignature,
    iblssignaturechecker::BN254::{G1Point, G2Point},
};
pub use error::AggregatorError;
use futures_util::{future, StreamExt};
use rpc_server::{ProcessSignedTaskResponse, ProcessSignedTaskResponseServer};
pub use signed_task_response::SignedTaskResponse;
use std::fmt::Debug;
use std::net::SocketAddr;
use tarpc::server::{self, Channel};
use tarpc::tokio_serde::formats::Json;
use task_processor::TaskProcessor;
use tracing::info;

/// Aggregator
#[derive(Debug)]
pub struct Aggregator<TP> {
    port_address: String,
    task_processor: TP,
    service_handle: ServiceHandle,
    aggregated_response_receiver: AggregateReceiver,
    ws_rpc_url: String,
}

impl<TP> Aggregator<TP>
where
    TP: TaskProcessor + Debug + Send + Sync + 'static + Clone,
    TP::Input: From<<<TP::Input as SolValue>::SolType as SolType>::RustType>,
    TP::Output: From<<<TP::Output as SolValue>::SolType as SolType>::RustType>,
{
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
            task_processor,
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

        let service_handle = self.service_handle.clone();
        let port_address = self.port_address.clone();

        // Spawn three tasks: one for the server that receives signature, one for processing tasks, and another to process aggregated signatures
        let server_handle = tokio::spawn(Self::start_server(
            port_address,
            self.task_processor.clone(),
            service_handle.clone(),
        ));

        let process_handle = tokio::spawn(Self::process_tasks(
            self.ws_rpc_url,
            self.task_processor.clone(),
            service_handle,
        ));
        let aggregate_handle = tokio::spawn(Self::process_aggregated_signatures(
            self.task_processor,
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
        task_processor: TP,
        service_handle: ServiceHandle,
    ) -> Result<(), AggregatorError> {
        let addr: SocketAddr = port_address.parse().map_err(|e| {
            AggregatorError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))
        })?;

        let service_handle_clone = service_handle.clone();

        let mut listener = tarpc::serde_transport::tcp::listen(&addr, Json::default).await?;
        info!("Server running at {}", addr);

        listener.config_mut().max_frame_length(usize::MAX);
        listener
            .filter_map(|r| future::ready(r.ok()))
            .map(server::BaseChannel::with_defaults)
            .for_each_concurrent(None, |channel| {
                let service_handle = service_handle_clone.clone();
                let task_processor = task_processor.clone();
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
        mut task_processor: TP,
        service_handle: ServiceHandle,
    ) -> Result<(), AggregatorError> {
        let ws = WsConnect::new(ws_rpc_url.clone());
        let filter = Filter::new().event_signature(TP::NEW_TASK_EVENT_SELECTOR);
        let provider = ProviderBuilder::new().on_ws(ws).await?;

        while let Some(log) = provider
            .subscribe_logs(&filter)
            .await?
            .into_stream()
            .next()
            .await
        {
            let (task_index, task) = decode_new_task::<TP::Input>(&log)?;
            let task_metadata = task_processor.process_new_task(task_index, task).await?;
            service_handle.initialize_task(task_metadata).await?;
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
        task_processor: TP,
        mut aggregated_response_receiver: AggregateReceiver,
    ) -> Result<(), AggregatorError> {
        loop {
            let service_response = aggregated_response_receiver
                .receive_aggregated_response()
                .await?;

            let non_signing_operator_pubkeys =
                get_non_signing_operator_pubkeys(service_response.clone())?;

            task_processor
                .process_aggregated_response(
                    service_response.task_index,
                    service_response.task_response_digest,
                    non_signing_operator_pubkeys,
                )
                .await?;
        }
    }
}

/// Build the [`NonSignerStakesAndSignature`] struct from the [`BlsAggregationServiceResponse`]
///
/// # Arguments
///
/// * `response` - The response of the BLS aggregation service
///
/// # Returns
///
/// * `Result<NonSignerStakesAndSignature, AggregatorError>` - The non-signing operator pub keys
fn get_non_signing_operator_pubkeys(
    response: BlsAggregationServiceResponse,
) -> Result<NonSignerStakesAndSignature, BlsError> {
    let mut non_signer_pub_keys = Vec::<G1Point>::new();
    for pub_key in response.non_signers_pub_keys_g1.iter() {
        if pub_key.g1().x().is_some() {
            let g1 = convert_to_g1_point(pub_key.g1())?;
            non_signer_pub_keys.push(G1Point { X: g1.X, Y: g1.Y })
        } else {
            info!(
                "Zero non_signers for the task index :{:?}",
                response.task_index
            );
        }
    }

    let mut quorum_apks = Vec::<G1Point>::new();
    for pub_key in response.quorum_apks_g1.iter() {
        let g1 = convert_to_g1_point(pub_key.g1())?;
        quorum_apks.push(G1Point { X: g1.X, Y: g1.Y })
    }

    let apk_g2 = convert_to_g2_point(response.signers_apk_g2.g2())?;
    let sigma = convert_to_g1_point(response.signers_agg_sig_g1.g1_point().g1())?;

    Ok(NonSignerStakesAndSignature {
        nonSignerPubkeys: non_signer_pub_keys,
        nonSignerQuorumBitmapIndices: response.non_signer_quorum_bitmap_indices,
        quorumApks: quorum_apks,
        apkG2: G2Point {
            X: apk_g2.X,
            Y: apk_g2.Y,
        },
        sigma: G1Point {
            X: sigma.X,
            Y: sigma.Y,
        },
        quorumApkIndices: response.quorum_apk_indices,
        totalStakeIndices: response.total_stake_indices,
        nonSignerStakeIndices: response.non_signer_stake_indices,
    })
}
