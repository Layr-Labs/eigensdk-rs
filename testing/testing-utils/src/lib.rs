//! testing utilities for eigenlayer

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

/// EigenLayer constants compatible with mainnet.
pub mod mainnet_constants;

///  Holesky EigenLayer contract m2 compatible constants.
pub mod m2_holesky_constants;

/// Anvil constants
#[allow(clippy::unwrap_used)]
pub mod anvil_constants;

/// Test data read from JSON files, used for compliance testing.
pub mod test_data;

/// Transaction utilities for testing.
pub mod transaction;

/// Anvil utilities for testing.
#[allow(clippy::unwrap_used)]
#[allow(clippy::expect_used)]
pub mod anvil;
