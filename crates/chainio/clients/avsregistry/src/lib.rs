//! AvsRegistry methods for reading, writing and subscribing purposes.

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

/// Reader module
pub mod reader;

#[allow(dead_code)]
/// Writer module
pub mod writer;

/// Avs registry error message
pub mod error;

#[allow(dead_code)]
/// Fake avs registry module
pub mod fake_reader;
