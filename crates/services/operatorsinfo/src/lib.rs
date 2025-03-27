//! Operators Info Service
//!
//! This crate provides traits and methods to get operator information.
//!
//! # Introduction
//!
//! The Operators Info Service provides functionality to get operators Public Keys and Sockets.
//!
//! The service is designed to be used in conjunction with the `AvsRegistryServiceChainCaller`
//! to get the operators information from the chain.
//!
//! ## Usage
//!
//! ### Initialize the Service
//!
//! When using the [`OperatorInfoServiceInMemory`] struct, you can initialize the service by calling the
//! [`new`] method. This method returns a tuple of the service and a channel to receive errors from the service. Also,
//! it create a background task to process the `OperatorsInfoMessage`.
//!
//! ```rust
//! let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
//!     test_logger.clone(),
//!     avs_registry_chain_reader,
//!     ws_endpoint,
//! )
//! .await
//! .unwrap();
//! ```
//!
//! ### Start the Service
//!
//! Then you can start the service by calling the [`start_service`] method. It will listen to events of [`NEW_PUBKEY_REGISTRATION_EVENT`]
//! and [`OPERATOR_SOCKET_UPDATE`] and save the data in memory.
//!
//! ```rust
//! tokio::spawn(async move {
//!     operators_info_service_in_memory
//!         .start_service(
//!             &token,
//!             0,
//!             get_provider(cloned_http_endpoint.as_str())
//!                 .get_block_number()
//!                 .await
//!                 .unwrap(),
//!         )
//!         .await;
//! });
//! ```
//!
//! ### Query Past Operator Registration Events and Fill the Database
//!
//! To query past operator registration events and fill the database, you can call the [`query_past_registered_operator_events_and_fill_db`] method.
//! This function will send a `OperatorsInfoMessage` to the service channel and store the data in `OperatorState`.
//!
//! ```rust
//! operators_info_service_in_memory
//!     .query_past_registered_operator_events_and_fill_db(0, end_block)
//!     .await;
//! ```
//!
//! ### Retrieve Operator information
//!
//! To retrieve operator information, you can call the [`get_operator_info`] or [`get_operator_socket`] methods.
//!
//! ```rust
//! let operator_info = operators_info_service_in_memory
//!     .get_operator_info(operator_id)
//!     .await
//!     .unwrap();
//!
//! let operator_socket = operators_info_service_in_memory
//!     .get_operator_socket(operator_id)
//!     .await
//!     .unwrap()
//!     .unwrap();
//! ```
//!
//! [`OperatorInfoServiceInMemory`]: operatorsinfo_inmemory::OperatorInfoServiceInMemory
//! [`new`]: operatorsinfo_inmemory::OperatorInfoServiceInMemory::new()
//! [`start_service`]: operatorsinfo_inmemory::OperatorInfoServiceInMemory::start_service()
//! [`query_past_registered_operator_events_and_fill_db`]: operatorsinfo_inmemory::OperatorInfoServiceInMemory::query_past_registered_operator_events_and_fill_db()
//! [`NEW_PUBKEY_REGISTRATION_EVENT`]: eigen_common::NEW_PUBKEY_REGISTRATION_EVENT
//! [`OPERATOR_SOCKET_UPDATE`]: eigen_common::OPERATOR_SOCKET_UPDATE
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
