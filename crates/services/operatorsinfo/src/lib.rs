//! Operators Info Service
//!
//! This crate provides traits and methods to get operator information.
//!
//! # Introduction
//!
//! The Operators Info Service provides functionality to get operators Public Keys and Sockets.
//!
//! The service is designed to be used in conjunction with the [`AvsRegistryServiceChainCaller`]
//! to get the operators information from the chain.
//!
//! ## Usage
//!
//! When using the [`OperatorInfoServiceInMemory`] struct, you can initialize the service by calling the
//! [`new`] method. This method returns a tuple of the service and a channel to receive errors from the service. Also,
//! it create a background task to process the [`OperatorsInfoMessage`].
//!
//! ```rust
//! let operator_info_service = OperatorInfoServiceInMemory::new(logger, avs_registry_reader, ws).await.unwrap();
//! ```
//!
//! Then you can start the service by calling the [`start_service`] method. It will listen to events of [`NEW_PUBKEY_REGISTRATION_EVENT`]
//! and [`OPERATOR_SOCKET_UPDATE`] and save the data in memory.
//!
//! ```rust
//! tokio::spawn(async move {
//!     let _ = clone_operators_info
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
//! To query past operator registration events and fill the database, you can call the [`query_past_registered_operator_events_and_fill_db`] method.
//! This function will send a [`OperatorsInfoMessage`] to the service channel and store the data in [`OperatorState`].
//!
//! ```rust
//! let _ = operators_info_service_in_memory
//!     .query_past_registered_operator_events_and_fill_db(0, end_block)
//!     .await;
//! ```

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

#[doc(hidden)]
pub mod fake_operator_info;
pub mod operator_info;
pub mod operatorsinfo_inmemory;
