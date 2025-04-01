//! BLS Agreggation Service crate.
//! # BLS Aggregation Service
//!
//! ## Introduction
//!
//! The BLS Aggregation Service provides functionality to aggregate BLS signatures from multiple operators into a single aggregated signature. This is used by the Aggregator to verify the signatures of the operators and send the aggregated response once the quorum is reached or time expires. AVS developers can use it to define and manage tasks, set quorum requirements and integrate aggregated results into their smart contracts.
//!
//! [Example of a aggregation service in action](https://github.com/Layr-Labs/incredible-squaring-avs-rs/blob/dev/crates/aggregator/src/lib.rs).
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
//! ### TaskMetadata
//!
//! Defines the metadata for a task.
//!
//! - `task_index`: Unique identifier for the task
//! - `task_created_block`: Block in which the task was created
//! - `quorum_numbers`: Quorum numbers that should respond to the task
//! - `quorum_threshold_percentages`: Threshold percentages for each quorum
//! - `time_to_expiry`: Time before the task response aggregation expires
//! - `window_duration`: Duration of the window to wait for signatures after quorum is reached
//!
//! ### TaskSignature
//!
//! Represents an individual signature for a task.
//!
//! - `task_index`: Index of the task
//! - `task_response_digest`: Digest of the task response
//! - `bls_signature`: BLS signature of the task response
//! - `operator_id`: ID of the operator that signed the response
//!
//! ### ServiceHandle
//!
//! Represents a handle to interact with the BLS Aggregation Service.
//!
//! - `msg_sender`: UnboundedSender to send messages to the BLS Aggregation Service
//!
//! ### AggregateReceiver
//!
//! Represents a receiver to receive aggregated responses from the BLS Aggregation Service.
//!
//! - `aggregate_receiver`: UnboundedReceiver to receive aggregated responses from the service
//!
//! ### BlsAggregatorService
//!
//! The main service that coordinates signature aggregation:
//!
//! - `new()`: Creates a new instance of the service
//! - `start()`: Starts the BLS Aggregator Service running the main loop in background
//!
//! ## Usage
//!
//! When you initialize the BLS Aggregation Service, it returns a tuple of `ServiceHandle` and `AggregateReceiver`. The `ServiceHandle` is used to interact with the service. The available messages to send to the service are:
//!
//! - `initialize_task(metadata: TaskMetadata)`: Initializes a new task. If you want to set a time to expiry for the task, you can use the `with_time_to_expiry()` method in a builder pattern.
//! - `process_signature(task_signature: TaskSignature)`: Processes a signature for a task
//!
//! The `AggregateReceiver` is used to receive aggregated responses from the service. To get the aggregated response, you can use the `receive_aggregated_response()` method.
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
//! #    let (service_handle, mut aggregate_receiver) =
//! #        BlsAggregatorService::new(avs_registry_service, logger).start();
//! #
//! #    let task_index = 0;
//! #    let task_response: u64 = 123;
//! #    let mut hasher = Sha256::new();
//! #    hasher.update(task_response.to_be_bytes());
//! #    let task_response_digest = B256::from_slice(hasher.finalize().as_ref());
//! #    let bls_key_pair = BlsKeyPair::new(
//! #        "12248929636257230549931416853095037629726205319386239410403476017439825112537"
//! #            .to_string(),
//! #    )
//! #    .unwrap();
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
//!
#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]

pub mod bls_agg;
mod bls_agg_test;
pub mod bls_aggregation_service_error;
pub mod bls_aggregation_service_response;
