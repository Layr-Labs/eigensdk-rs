#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]

use std::collections::HashMap;

use alloy_primitives::FixedBytes;
use eigen_client_avsregistry::error::AvsRegistryError;
use eigen_types::operator::{OperatorAvsState, QuorumAvsState};
use eigen_utils::binding::OperatorStateRetriever::CheckSignaturesIndices;
// #![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod chaincaller;
pub mod fake_avs_registry_service;

pub trait AvsRegistryService {
    fn get_operators_avs_state_at_block(
        &self,
        block_num: u32,
        quorum_nums: &Vec<u8>,
    ) -> impl std::future::Future<Output = HashMap<FixedBytes<32>, OperatorAvsState>> + Send;

    fn get_quorums_avs_state_at_block(
        &self,
        quorum_nums: &Vec<u8>,
        block_num: u32,
    ) -> impl std::future::Future<Output = HashMap<u8, QuorumAvsState>> + Send;

    /// Get Signature indices
    fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> impl std::future::Future<Output = Result<CheckSignaturesIndices, AvsRegistryError>> + Send;
}
