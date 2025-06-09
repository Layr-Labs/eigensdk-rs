#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
//! Signer module for EigenSDK.
//!
//! This module provides a set of traits and implementations for signing transactions.
//!
//! * [`signer`] - Legacy signer module.
//! * [`signer_v2`] - New signer module that supports multiple signers. The difference between this
//!   and the legacy signer is that this module is designed to be used in a more flexible way
//!   and has a more intuitive interface.
//! * [`web3_signer`] - Web3 signer module.

pub mod signer;
pub mod signer_v2;
pub mod web3_signer;
