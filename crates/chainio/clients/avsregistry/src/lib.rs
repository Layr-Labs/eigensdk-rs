//! AvsRegistry methods for reading, writing and subscribing purposes.

#![doc(
    html_logo_url = "https://github.com/supernovahs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/supernovahs/eigen-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

#[allow(dead_code)]
/// Reader module
pub mod reader;
#[allow(dead_code)]
/// Subscriber module
pub mod subscriber;

#[allow(dead_code)]
/// Writer module
pub mod writer;

/// Avs registry error message
pub mod error;
