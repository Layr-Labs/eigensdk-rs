//! Operators Info Service
//!
//! This crate provides traits and methods to get operator information.
//!
//! ## Introduction
//!
//! The Operators Info Service provides functionality to get operators Public Keys and Sockets.
//!
//! The service is designed to be used in conjunction with the `AvsRegistryServiceChainCaller`
//! to get the operators information from the chain.
//!
//! ## Main Components
//!
//! ### OperatorInfoServiceInMemory
//!
//! The main struct of the service, it implements the [`OperatorInfoService`] trait.
//! It fetches and stores operators info (addresses and public key) in memory.
//!
//! - `avs_registry_reader`: `AvsRegistryChainReader` to get the operators information from the chain
//!
//! ### OperatorSocket
//!
//! Represents an operator with the ID and the socket.
//!
//! - `id`: Operator ID
//! - `socket`: Operator socket
//!
//! ### OperatorPubKeys
//!
//! Represents the operator public keys.
//!
//! - `g1_pub_key`: Operator G1 public key
//! - `g2_pub_key`: Operator G2 public key
//!
//! ## Usage
//!
//! ### Initialize the Service
//!
//! When using the [`OperatorInfoServiceInMemory`] struct, you can initialize the service by calling the
//! [`new`] method. This method returns a tuple of the service and a channel to receive errors from the service. Also,
//! it creates a background task to process the `OperatorsInfoMessage`. The messages that the service will process are:
//! - `InsertOperatorInfo`: Save the operator info in memory.
//! - `Remove`: Remove the operator info from state.
//! - `GetPubKeys`: Get the operator public keys from memory.
//! - `GetSockets`: Get the operator socket from memory.
//!
//! ```rust,no_run
//!# use eigen_testing_utils::anvil_constants::{
//!#     ANVIL_HTTP_URL, ANVIL_WS_URL,
//!#     get_operator_state_retriever_address, get_registry_coordinator_address,
//!# };
//!# use eigen_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
//!# use eigen_logging::get_test_logger;
//!# use eigen_client_avsregistry::reader::AvsRegistryChainReader;
//!# async fn example () {
//!#     let logger = get_test_logger();
//!#     let http_endpoint = ANVIL_HTTP_URL;
//!#     let ws_endpoint = ANVIL_WS_URL;
//!#
//!#     let registry_coordinator_address = get_registry_coordinator_address(http_endpoint.to_string()).await;
//!#     let operator_state_retriever_address = get_operator_state_retriever_address(http_endpoint.to_string()).await;
//!#
//!#     let avs_registry_chain_reader = AvsRegistryChainReader::new(
//!#         logger.clone(),
//!#         registry_coordinator_address,
//!#         operator_state_retriever_address,
//!#         http_endpoint.to_string(),
//!#     )
//!#     .await
//!#     .unwrap();
//!#
//!     let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
//!         logger.clone(),
//!         avs_registry_chain_reader,
//!         ws_endpoint.to_string(),
//!     )
//!     .await
//!     .unwrap();
//!# }
//! ```
//!
//! ### Start the Service
//!
//! Then you can start the service by calling the [`start_service`] method. It will listen to events of `NEW_PUBKEY_REGISTRATION_EVENT`
//! and `OPERATOR_SOCKET_UPDATE` and save the data in memory. To stop the service, you can use the `CancellationToken`.
//!
//! ```rust,no_run
//!# use eigen_logging::get_test_logger;
//!# use eigen_testing_utils::anvil_constants::{
//!#     ANVIL_HTTP_URL, ANVIL_WS_URL,
//!#     get_operator_state_retriever_address, get_registry_coordinator_address,
//!# };
//!# use eigen_services_operatorsinfo::{
//!#     operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceInMemory,
//!# };
//!# use eigen_client_avsregistry::reader::AvsRegistryChainReader;
//!# async fn example () {
//!#     let logger = get_test_logger();
//!#     let http_endpoint = ANVIL_HTTP_URL;
//!#     let ws_endpoint = ANVIL_WS_URL;
//!#
//!#     let registry_coordinator_address = get_registry_coordinator_address(http_endpoint.to_string()).await;
//!#     let operator_state_retriever_address = get_operator_state_retriever_address(http_endpoint.to_string()).await;
//!#
//!#     let avs_registry_chain_reader = AvsRegistryChainReader::new(
//!#         logger.clone(),
//!#         registry_coordinator_address,
//!#         operator_state_retriever_address,
//!#         http_endpoint.to_string(),
//!#     )
//!#     .await
//!#     .unwrap();
//!#
//!#     let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
//!#         logger.clone(),
//!#         avs_registry_chain_reader,
//!#         ws_endpoint.to_string(),
//!#     )
//!#     .await
//!#     .unwrap()
//!#     .0;
//!#
//!#     let clone_operators_info = operators_info_service_in_memory.clone();
//!#     let cancellation_token = tokio_util::sync::CancellationToken::new();
//!#     let cloned_token = cancellation_token.clone();
//!#     let cloned_http_endpoint = http_endpoint.clone();
//!#     let end_block = 100;
//!#
//!     tokio::spawn(async move {
//!         let _ = clone_operators_info
//!             .start_service(
//!                 &cloned_token,
//!                 0,
//!                 end_block,
//!             )
//!             .await;
//!     });
//!# }
//! ```
//!
//! ### Query Past Operator Registration Events and Fill the Database
//!
//! To query past operator registration events and fill the database, you can call the [`query_past_registered_operator_events_and_fill_db`] method.
//! This function will send a `OperatorsInfoMessage` with the `InsertOperatorInfo` action to the service channel and store the data in `OperatorState`.
//!
//! ```rust,no_run
//!# use eigen_testing_utils::anvil_constants::{
//!#     ANVIL_HTTP_URL, ANVIL_WS_URL,
//!#     get_operator_state_retriever_address, get_registry_coordinator_address,
//!# };
//!# use eigen_services_operatorsinfo::{
//!#     operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceInMemory,
//!# };
//!# use eigen_logging::get_test_logger;
//!# use eigen_client_avsregistry::reader::AvsRegistryChainReader;
//!# async fn example () {
//!#     let logger = get_test_logger();
//!#     let http_endpoint = ANVIL_HTTP_URL;
//!#     let ws_endpoint = ANVIL_WS_URL;
//!#
//!#     let registry_coordinator_address = get_registry_coordinator_address(http_endpoint.to_string()).await;
//!#     let operator_state_retriever_address = get_operator_state_retriever_address(http_endpoint.to_string()).await;
//!#
//!#     let avs_registry_chain_reader = AvsRegistryChainReader::new(
//!#         logger.clone(),
//!#         registry_coordinator_address,
//!#         operator_state_retriever_address,
//!#         http_endpoint.to_string(),
//!#     )
//!#     .await
//!#     .unwrap();
//!#
//!#     let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
//!#         logger,
//!#         avs_registry_chain_reader,
//!#         ws_endpoint.to_string(),
//!#     )
//!#     .await
//!#     .unwrap()
//!#     .0;
//!#
//!#     let end_block = 100;
//!     operators_info_service_in_memory
//!         .query_past_registered_operator_events_and_fill_db(0, end_block)
//!         .await;
//!# }
//! ```
//!
//! ### Retrieve Operator information
//!
//! To retrieve operator information, you can call the [`get_operator_info`] or [`get_operator_socket`] methods.
//!
//! ```rust,no_run
//!# use eigen_logging::get_test_logger;
//!# use alloy::primitives::{Address};
//!# use eigen_testing_utils::anvil_constants::{
//!#     ANVIL_HTTP_URL, ANVIL_WS_URL, FIRST_ADDRESS,
//!#     get_operator_state_retriever_address, get_registry_coordinator_address,
//!# };
//!# use eigen_services_operatorsinfo::{
//!#     operator_info::OperatorInfoService, operatorsinfo_inmemory::OperatorInfoServiceInMemory,
//!# };
//!# use eigen_client_avsregistry::reader::AvsRegistryChainReader;
//!# async fn example () {
//!#     let logger = get_test_logger();
//!#     let http_endpoint = ANVIL_HTTP_URL;
//!#     let ws_endpoint = ANVIL_WS_URL;
//!#
//!#     let registry_coordinator_address = get_registry_coordinator_address(http_endpoint.to_string()).await;
//!#     let operator_state_retriever_address = get_operator_state_retriever_address(http_endpoint.to_string()).await;
//!#
//!#     let avs_registry_chain_reader = AvsRegistryChainReader::new(
//!#         logger.clone(),
//!#         registry_coordinator_address,
//!#         operator_state_retriever_address,
//!#         http_endpoint.to_string(),
//!#     )
//!#     .await
//!#     .unwrap();
//!#
//!#     let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
//!#         logger.clone(),
//!#         avs_registry_chain_reader,
//!#         ws_endpoint.to_string(),
//!#     )
//!#     .await
//!#     .unwrap()
//!#     .0;
//!#
//!#     let operator_address = Address::from(FIRST_ADDRESS);
//!#
//!     let operator_info = operators_info_service_in_memory
//!         .get_operator_info(operator_address)
//!         .await
//!         .unwrap()
//!         .unwrap();
//!#
//!     let operator_socket = operators_info_service_in_memory
//!         .get_operator_socket(operator_address)
//!         .await
//!     .unwrap()
//!     .unwrap();
//!# }
//! ```
//!
//! [`OperatorInfoServiceInMemory`]: operatorsinfo_inmemory::OperatorInfoServiceInMemory
//! [`OperatorInfoService`]: operator_info::OperatorInfoService
//! [`new`]: operatorsinfo_inmemory::OperatorInfoServiceInMemory::new()
//! [`start_service`]: operatorsinfo_inmemory::OperatorInfoServiceInMemory::start_service()
//! [`query_past_registered_operator_events_and_fill_db`]: operatorsinfo_inmemory::OperatorInfoServiceInMemory::query_past_registered_operator_events_and_fill_db()
//! [`get_operator_info`]: operator_info::OperatorInfoService::get_operator_info()
//! [`get_operator_socket`]: operator_info::OperatorInfoService::get_operator_socket()

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

#[doc(hidden)]
pub mod fake_operator_info;
pub mod operator_info;
pub mod operatorsinfo_inmemory;
pub mod operatorsinfo_onchain;
