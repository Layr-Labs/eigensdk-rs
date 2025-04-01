//! BLS Agreggation Service crate.
//! # BLS Aggregation Service
//!
//! ## Introduction
//!
//! The BLS Aggregation Service provides functionality to aggregate BLS signatures from multiple operators into a single aggregated signature. This is used by the Aggregator to verify the signatures of the operators and send the aggregated response once the quorum is reached or time expires. AVS developers can use it to define and manage tasks, set quorum requirements and integrate aggregated results into their smart contracts.
//!
//! ## Key Features
//!
//! - **BLS Signature Aggregation**: Combines multiple individual signatures into a single verifiable aggregated signature.
//! - **Quorum Verification**: Ensures that the required participation threshold is reached for each quorum.
//! - **Task Management**: Allows initializing tasks and processing signatures for those tasks.
//! - **Configurable Time Window**: Allows defining waiting periods for signature collection.
//!
//! ## Main Components
//!
//! - [`TaskMetadata`]: Defines task parameters, including quorums and thresholds.
//! - [`TaskSignature`]: Contains an individual BLS signature for a specific task.
//! - [`ServiceHandle`]: Interface for sending tasks and signatures to the service.
//! - [`AggregateReceiver`]: Channel for receiving aggregated responses.
//! - [`BlsAggregatorService`]: The main service that manages tasks and aggregates signatures.
//!
//! ## Usage
//!
//! When you initialize the BLS Aggregation Service, it returns a tuple of [`ServiceHandle`] and [`AggregateReceiver`]. The `ServiceHandle` is used to interact with the service. The available messages to send to the service are:
//!
//! - [`initialize_task(metadata: TaskMetadata)`]: Initializes a new task. If you want to set a time to expiry for the task, you can use the [`with_window_duration()`] method in a builder pattern.
//! - [`process_signature(task_signature: TaskSignature)`]: Processes a signature for a task
//!
//! The `AggregateReceiver` is used to receive aggregated responses from the service. To get the aggregated response, you can use the [`receive_aggregated_response()`] method.
//!
//! Once a task is initialized, the service will start processing the task in a loop in the background. The service will wait for the quorum to be reached or the time to expire. Once the quorum is reached or the time expires, the service will aggregate the signatures and send the aggregated response to the `AggregateReceiver`.
//!
//! ### Initialize the Service
//!
//! ```rust,no_run
//! # use eigen_services_blsaggregation::bls_agg::{
//! #     AggregateReceiver, BlsAggregatorService, TaskMetadata, TaskSignature
//! # };
//! # use eigen_client_avsregistry::{
//! #     reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter,
//! # };
//! # use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
//! # use eigen_services_avsregistry::AvsRegistryService;
//! # use eigen_logging::get_test_logger;
//! # use eigen_testing_utils::{
//! #     anvil_constants::{
//! #         get_operator_state_retriever_address, get_registry_coordinator_address,
//! #     },
//! # };
//! # use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
//! # #[tokio::main]
//! # async fn main() {
//! #     let http_endpoint = "http://localhost:8545";
//! #     let ws_endpoint = "ws://localhost:8546";
//! #     let logger = get_test_logger();
//! #     let registry_coordinator_address =
//! #         get_registry_coordinator_address(http_endpoint.to_string()).await;
//! #     let operator_state_retriever_address =
//! #         get_operator_state_retriever_address(http_endpoint.to_string()).await;
//! #     
//! #     // Create avs clients to interact with contracts deployed on anvil
//! #     let avs_registry_reader = AvsRegistryChainReader::new(
//! #         get_test_logger(),
//! #         registry_coordinator_address,
//! #         operator_state_retriever_address,
//! #         http_endpoint.to_string(),
//! #     )
//! #     .await
//! #     .unwrap();
//! #
//! #     // Create operators info service
//! #     let operators_info = OperatorInfoServiceInMemory::new(
//! #         get_test_logger(),
//! #         avs_registry_reader.clone(),
//! #         ws_endpoint.to_string(),
//! #     )
//! #     .await
//! #     .unwrap()
//! #     .0;
//! #
//! #     // Create avs registry service chain caller
//! #     let avs_registry_service =
//! #         AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);
//!     let (service_handle, mut aggregate_receiver) =
//!         BlsAggregatorService::new(avs_registry_service, logger).start();
//! # }
//! ```
//!
//! ### Initialize a Task
//!
//! ```rust,no_run
//! # use eigen_services_blsaggregation::bls_agg::{
//! #     AggregateReceiver, BlsAggregatorService, TaskMetadata, TaskSignature
//! # };
//! # use eigen_client_avsregistry::{
//! #     reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter,
//! # };
//! # use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
//! # use eigen_services_avsregistry::AvsRegistryService;
//! # use eigen_logging::get_test_logger;
//! # use eigen_testing_utils::{
//! #     anvil_constants::{
//! #         get_operator_state_retriever_address, get_registry_coordinator_address,
//! #     },
//! # };
//! # use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
//! # use std::time::Duration;
//! # use eigen_testing_utils::anvil_constants::{ANVIL_HTTP_URL, ANVIL_WS_URL};
//! # #[tokio::main]
//! # async fn main() {
//! #     let http_endpoint = ANVIL_HTTP_URL;
//! #     let ws_endpoint = ANVIL_WS_URL;
//! #     let logger = get_test_logger();
//! #     let registry_coordinator_address =
//! #         get_registry_coordinator_address(http_endpoint.to_string()).await;
//! #     let operator_state_retriever_address =
//! #         get_operator_state_retriever_address(http_endpoint.to_string()).await;
//! #     
//! #     // Create avs clients to interact with contracts deployed on anvil
//! #     let avs_registry_reader = AvsRegistryChainReader::new(
//! #         get_test_logger(),
//! #         registry_coordinator_address,
//! #         operator_state_retriever_address,
//! #         http_endpoint.to_string(),
//! #     )
//! #     .await
//! #     .unwrap();
//! #
//! #     // Create operators info service
//! #     let operators_info = OperatorInfoServiceInMemory::new(
//! #         get_test_logger(),
//! #         avs_registry_reader.clone(),
//! #         ws_endpoint.to_string(),
//! #     )
//! #     .await
//! #     .unwrap()
//! #     .0;
//! #
//! #     // Create avs registry service chain caller
//! #     let avs_registry_service =
//! #         AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);
//! #    let (service_handle, mut aggregate_receiver) =
//! #        BlsAggregatorService::new(avs_registry_service, logger).start();
//! #
//! #    let task_index = 0;
//! #    let block_number = 1;
//! #    let quorum_numbers = vec![0];
//! #    let quorum_threshold_percentages = vec![100];
//! #    let time_to_expiry = Duration::from_secs(60);
//! #
//!     let metadata = TaskMetadata::new(
//!         task_index,
//!         block_number,
//!         quorum_numbers,
//!         quorum_threshold_percentages,
//!         time_to_expiry,
//!     );
//!     service_handle.initialize_task(metadata).await.unwrap();
//! # }
//! ```
//!
//! ### Process a Signature
//!
//! ```rust,no_run
//! # use eigen_services_blsaggregation::bls_agg::{
//! #     BlsAggregatorService, TaskSignature
//! # };
//! # use eigen_client_avsregistry::{
//! #     reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter,
//! # };
//! # use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
//! # use eigen_services_avsregistry::AvsRegistryService;
//! # use eigen_logging::get_test_logger;
//! # use eigen_testing_utils::{
//! #     anvil_constants::{
//! #         get_operator_state_retriever_address, get_registry_coordinator_address,
//! #     },
//! # };
//! # use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
//! # use sha2::{Digest, Sha256};
//! # use alloy::primitives::{B256, FixedBytes};
//! # use eigen_crypto_bls::BlsKeyPair;
//! # use eigen_testing_utils::anvil_constants::{ANVIL_HTTP_URL, ANVIL_WS_URL, OPERATOR_BLS_KEY};
//! # #[tokio::main]
//! # async fn main() {
//! #     let http_endpoint = ANVIL_HTTP_URL;
//! #     let ws_endpoint = ANVIL_WS_URL;
//! #     let logger = get_test_logger();
//! #     let registry_coordinator_address =
//! #         get_registry_coordinator_address(http_endpoint.to_string()).await;
//! #     let operator_state_retriever_address =
//! #         get_operator_state_retriever_address(http_endpoint.to_string()).await;
//! #     
//! #     // Create avs clients to interact with contracts deployed on anvil
//! #     let avs_registry_reader = AvsRegistryChainReader::new(
//! #         get_test_logger(),
//! #         registry_coordinator_address,
//! #         operator_state_retriever_address,
//! #         http_endpoint.to_string(),
//! #     )
//! #     .await
//! #     .unwrap();
//! #
//! #     // Create operators info service
//! #     let operators_info = OperatorInfoServiceInMemory::new(
//! #         get_test_logger(),
//! #         avs_registry_reader.clone(),
//! #         ws_endpoint.to_string(),
//! #     )
//! #     .await
//! #     .unwrap()
//! #     .0;
//! #
//! #     // Create avs registry service chain caller
//! #     let avs_registry_service =
//! #         AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);
//! #    let (service_handle, mut aggregate_receiver) =
//! #        BlsAggregatorService::new(avs_registry_service, logger).start();
//! #
//! #    let task_index = 0;
//! #    let task_response: u64 = 123;
//! #    let mut hasher = Sha256::new();
//! #    hasher.update(task_response.to_be_bytes());
//! #    let task_response_digest = B256::from_slice(hasher.finalize().as_ref());
//! #    let bls_key_pair = BlsKeyPair::new(OPERATOR_BLS_KEY.to_string()).unwrap();
//! #    let bls_signature = bls_key_pair.sign_message(task_response_digest.as_ref());
//! #    let operator_id = FixedBytes::from_slice(&[1]);
//! #
//!     let task_signature = TaskSignature::new(
//!         task_index,
//!         task_response_digest,
//!         bls_signature,
//!         operator_id,
//!     );
//!     service_handle.process_signature(task_signature).await.unwrap();
//! # }
//! ```
//!
//! ### Receive an Aggregated Response
//!
//! ```rust,no_run
//! # use eigen_services_blsaggregation::bls_agg::BlsAggregatorService;
//! # use eigen_client_avsregistry::{
//! #     reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter,
//! # };
//! # use eigen_services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
//! # use eigen_services_avsregistry::AvsRegistryService;
//! # use eigen_logging::get_test_logger;
//! # use eigen_testing_utils::{
//! #     anvil_constants::{
//! #         get_operator_state_retriever_address, get_registry_coordinator_address,
//! #     },
//! # };
//! # use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
//! # use eigen_testing_utils::anvil_constants::{ANVIL_HTTP_URL, ANVIL_WS_URL};
//! # #[tokio::main]
//! # async fn main() {
//! #     let http_endpoint = ANVIL_HTTP_URL;
//! #     let ws_endpoint = ANVIL_WS_URL;
//! #     let logger = get_test_logger();
//! #     let registry_coordinator_address =
//! #         get_registry_coordinator_address(http_endpoint.to_string()).await;
//! #     let operator_state_retriever_address =
//! #         get_operator_state_retriever_address(http_endpoint.to_string()).await;
//! #     
//! #     // Create avs clients to interact with contracts deployed on anvil
//! #     let avs_registry_reader = AvsRegistryChainReader::new(
//! #         get_test_logger(),
//! #         registry_coordinator_address,
//! #         operator_state_retriever_address,
//! #         http_endpoint.to_string(),
//! #     )
//! #     .await
//! #     .unwrap();
//! #
//! #     // Create operators info service
//! #     let operators_info = OperatorInfoServiceInMemory::new(
//! #         get_test_logger(),
//! #         avs_registry_reader.clone(),
//! #         ws_endpoint.to_string(),
//! #     )
//! #     .await
//! #     .unwrap()
//! #     .0;
//! #
//! #     // Create avs registry service chain caller
//! #     let avs_registry_service =
//! #         AvsRegistryServiceChainCaller::new(avs_registry_reader.clone(), operators_info);
//! #    let (service_handle, mut aggregate_receiver) =
//! #        BlsAggregatorService::new(avs_registry_service, logger).start();
//! #
//!     match aggregate_receiver.receive_aggregated_response().await {
//!         Ok(aggregated_response) => {
//!             // Handle the aggregated response
//!         }
//!         Err(e) => {
//!             // Handle the error
//!         }
//!     }
//! # }
//! ```
//!
//! ## Example Diagram
//!
//! The following diagram shows the sequence of events when a user creates the BLS Aggregator Service, starts the service, and then initializes a task. The service processes two signatures from the operators and then aggregates them into a single aggregated signature.
//!
//! <pre>
#![cfg_attr(doc, doc = simple_mermaid::mermaid!("../diagram/sequence-bls.mmd"))]
//! </pre>
//!
//! ## Testing
//!
//! To run the [integration tests](https://github.com/Layr-Labs/eigensdk-rs/blob/dev/crates/services/bls_aggregation/src/bls_agg_test.rs), you can use the following command:
//!
//! ```sh
//! cargo test --package eigen-services-blsaggregation --lib -- bls_agg_test::integration_test --show-output
//! ```
//! [`new()`]: bls_agg::BlsAggregatorService::new()
//! [`start()`]: bls_agg::BlsAggregatorService::start()
//! [`initialize_task(metadata: TaskMetadata)`]: bls_agg::ServiceHandle::initialize_task()
//! [`process_signature(task_signature: TaskSignature)`]: bls_agg::ServiceHandle::process_signature()
//! [`receive_aggregated_response()`]: bls_agg::AggregateReceiver::receive_aggregated_response()
//! [`with_window_duration()`]: bls_agg::TaskMetadata::with_window_duration()
//! [`ServiceHandle`]: bls_agg::ServiceHandle
//! [`AggregateReceiver`]: bls_agg::AggregateReceiver
//! [`TaskMetadata`]: bls_agg::TaskMetadata
//! [`TaskSignature`]: bls_agg::TaskSignature
//! [`BlsAggregatorService`]: bls_agg::BlsAggregatorService
#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]

pub mod bls_agg;
mod bls_agg_test;
pub mod bls_aggregation_service_error;
pub mod bls_aggregation_service_response;
