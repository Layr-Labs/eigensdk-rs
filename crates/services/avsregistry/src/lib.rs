#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]

use std::collections::HashMap;

use alloy_primitives::{Bytes, FixedBytes};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_types::operator::{OperatorAvsState, OperatorPubKeys, QuorumAvsState};
// #![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod chaincaller;
pub mod fake_avs_registry_service;

pub trait AvsRegistryService {
    fn get_operators_avs_state_at_block(
        &self,
        block_num: u32,
        quorum_nums: Bytes,
    ) -> impl std::future::Future<Output = HashMap<FixedBytes<32>, OperatorAvsState>> + Send;

    fn get_avs_registry(&self) -> AvsRegistryChainReader;

    fn get_quorums_avs_state_at_block(
        &self,
        quorum_nums: Bytes,
        block_num: u32,
    ) -> impl std::future::Future<Output = HashMap<u8, QuorumAvsState>> + Send;

    fn get_operator_info(
        &self,
        operator_id: [u8; 32],
    ) -> impl std::future::Future<Output = Option<OperatorPubKeys>> + Send;
}
