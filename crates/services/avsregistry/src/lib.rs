#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use std::collections::HashMap;

use alloy_primitives::FixedBytes;
use async_trait::async_trait;
use eigen_client_avsregistry::error::AvsRegistryError;
use eigen_types::operator::{OperatorAvsState, QuorumAvsState};
use eigen_utils::operatorstateretriever::OperatorStateRetriever::CheckSignaturesIndices;

pub mod chaincaller;
pub mod fake_avs_registry_service;

#[async_trait]
pub trait AvsRegistryService {
    /// Get the operators AVS state at a specific block number
    ///
    /// # Arguments
    ///
    /// * `block_num` - The block number to get the AVS state at
    /// * `quorum_nums` - The list of quorum numbers
    ///
    /// # Returns
    ///
    /// A hashmap containing the operator ID and the operator AVS state
    async fn get_operators_avs_state_at_block(
        &self,
        block_num: u32,
        quorum_nums: &[u8],
    ) -> Result<HashMap<FixedBytes<32>, OperatorAvsState>, AvsRegistryError>;

    /// Get the quorum AVS state at a specific block
    ///
    /// # Arguments
    ///
    /// * `quorum_nums` - The list of quorum numbers
    /// * `block_num` - The block number
    ///
    /// # Returns
    ///
    /// A hashmap containing the quorum number and the quorum AVS state.
    async fn get_quorums_avs_state_at_block(
        &self,
        quorum_nums: &[u8],
        block_num: u32,
    ) -> Result<HashMap<u8, QuorumAvsState>, AvsRegistryError>;

    /// Get the signatures indices of quorum members for a specific block and checks
    /// if the indices are valid
    ///
    /// # Arguments
    ///
    /// * `reference_block_number` - The reference block number
    /// * `quorum_numbers` - The list of quorum numbers
    /// * `non_signer_operator_ids` - The list of non-signer operator ids
    ///
    /// # Returns
    ///
    /// A struct containing the indices of the quorum members that signed,
    /// and the ones that didn't
    async fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<CheckSignaturesIndices, AvsRegistryError>;
}
