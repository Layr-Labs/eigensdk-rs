use std::{collections::HashMap, ops::Add};

use crate::{error::AvsRegistryError, reader::AvsRegistryReader};
use alloy_primitives::{Address, Bytes, FixedBytes};
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::{operator::OperatorPubKeys, test::TestOperator};
use eigen_utils::binding::OperatorStateRetriever;

pub struct FakeAvsRegistryReader {
    operator_address: Address,
    operator_pubkeys: BlsKeyPair,
    operator_id: FixedBytes<32>,
}

impl FakeAvsRegistryReader {
    pub fn new(operator: TestOperator, operator_address: Address) -> Self {
        Self {
            operator_address,
            operator_id: operator.operator_id,
            operator_pubkeys: operator.bls_keypair,
        }
    }
}

impl AvsRegistryReader for FakeAvsRegistryReader {
    async fn get_operators_stake_in_quorums_at_block(
        &self,
        _block_number: u32,
        _quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<OperatorStateRetriever::Operator>>, AvsRegistryError> {
        todo!()
    }
    async fn get_check_signatures_indices(
        &self,
        _reference_block_number: u32,
        _quorum_numbers: Vec<u8>,
        _non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<OperatorStateRetriever::CheckSignaturesIndices, AvsRegistryError> {
        todo!()
    }
    async fn get_operator_from_id(
        &self,
        _operator_id: [u8; 32],
    ) -> Result<Address, AvsRegistryError> {
        Ok(self.operator_address)
    }

    async fn query_existing_registered_operator_sockets(
        &self,
        _start_block: u64,
        _stop_block: u64,
    ) -> Result<HashMap<FixedBytes<32>, String>, AvsRegistryError> {
        todo!()
    }

    async fn query_existing_registered_operator_pub_keys(
        &self,
        _start_block: u64,
        mut _stop_block: u64,
        _ws_url: String,
    ) -> Result<(Vec<Address>, Vec<OperatorPubKeys>), AvsRegistryError> {
        todo!()
    }
}
