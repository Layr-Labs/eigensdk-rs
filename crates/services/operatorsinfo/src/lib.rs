//! This crate provides traits and methods to get operator information.

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod fake_operator_info;
pub mod operator_info;
pub mod operatorsinfo_inmemory;
pub struct OperatorPubKeysService {}
