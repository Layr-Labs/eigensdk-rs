//! AVS Registry Service
//!
//! This service is used to get the AVS state of the operators and the quorums.
//!
//! ## Introduction
//!
//! The [`AvsRegistryServiceChainCaller`] allows to get the AVS state of the operators and the quorums at any block.
//! This service is used by the `BLS Aggregator service` and use under the hood the `OperatorInfoService` to get the operator info.
//!
//! ## Main Components
//!
//! ### AvsRegistryServiceChainCaller
//!
//! The main struct of the service, it implements the [`AvsRegistryService`] trait.
//!
//! - `avs_registry`: `AvsRegistryReader` to get the information from the chain
//! - `operators_info_service`: `OperatorInfoService` to get the operator info
//!
//! ### OperatorAvsState
//!
//! Represents the AVS state of an operator.
//!
//! - `operator_id`: Operator ID
//! - `operator_info`: Operator info public key
//! - `stake_per_quorum`: Stake per quorum
//! - `block_num`: Block number
//!
//! ### QuorumAvsState
//!
//! Represents the AVS state of a quorum.
//!
//! - `quorum_num`: Quorum number
//! - `total_stake`: Total stake
//! - `agg_pub_key_g1`: Aggregated G1 public key
//! - `block_num`: Block number
//!
//! ## Usage
//!
//! ### Initialize the service
//!
//! To initialize the service, you need to provide an `AvsRegistryReader` and an `OperatorInfoService` and call the [`new`] method.
//!
//! ```rust
//! let avs_registry_service =
//!     AvsRegistryServiceChainCaller::new(avs_registry, operator_info_service);
//! ```
//!
//! ### Get the AVS state of the operators and the quorums at a specific block
//!
//! To get the state of the operator in a specific quorum at a specific block, you can use the [`get_operators_avs_state_at_block`] method.
//! The list of quorum nums and the list of operators stakes in quorums should have the same length.
//! The method returns a hashmap with the operator ID as the key and the `OperatorAvsState` as the value.
//!
//! ```rust
//! let operator_avs_state = avs_registry_service
//!     .get_operators_avs_state_at_block(block_num, &quorum_nums)
//!     .await
//!     .unwrap();
//! ```
//!
//! To get the state of the quorum at a specific block, you can use the [`get_quorums_avs_state_at_block`] method.
//! The method returns a hashmap with the quorum number as the key and the `QuorumAvsState` as the value.
//!
//! ```rust
//! let quorum_state_per_number = avs_registry_service
//!     .get_quorums_avs_state_at_block(&quorum_nums, block_num)
//!     .await
//!     .unwrap();
//! ```
//!
//! ### Get the signatures indices of quorum members for a specific block
//!
//! To get the signatures indices of quorum members for a specific block, you can use the [`get_check_signatures_indices`] method.
//!
//! ```rust
//! let check_signatures_indices = avs_registry_service
//!     .get_check_signatures_indices(block_num, &quorum_nums, &non_signer_operator_ids)
//!     .await
//!     .unwrap();
//! ```
//!
//! [`AvsRegistryServiceChainCaller`]: chaincaller::AvsRegistryServiceChainCaller
//! [`new`]: chaincaller::AvsRegistryServiceChainCaller::new()
//! [`get_operators_avs_state_at_block`]: chaincaller::AvsRegistryServiceChainCaller::get_operators_avs_state_at_block()
//! [`get_quorums_avs_state_at_block`]: chaincaller::AvsRegistryServiceChainCaller::get_quorums_avs_state_at_block()
//! [`get_check_signatures_indices`]: chaincaller::AvsRegistryServiceChainCaller::get_check_signatures_indices()
//!

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

use std::collections::HashMap;

use alloy::primitives::FixedBytes;
use async_trait::async_trait;
use eigen_client_avsregistry::error::AvsRegistryError;
use eigen_types::avs_state::{OperatorAvsState, QuorumAvsState};
use eigen_utils::slashing::middleware::operatorstateretriever::OperatorStateRetriever::CheckSignaturesIndices;

pub mod chaincaller;
#[doc(hidden)]
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
        block_num: u64,
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
        block_num: u64,
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
        reference_block_number: u64,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<CheckSignaturesIndices, AvsRegistryError>;
}
