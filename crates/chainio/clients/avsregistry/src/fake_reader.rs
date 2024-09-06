use std::collections::HashMap;

use crate::{error::AvsRegistryError, reader::AvsRegistryReader};
use alloy_primitives::{Address, Bytes, FixedBytes, U256};
use async_trait::async_trait;
use eigen_crypto_bls::{BlsKeyPair, OperatorId};
use eigen_types::{
    operator::{OperatorPubKeys, QuorumNum},
    test::TestOperator,
};
use eigen_utils::binding::OperatorStateRetriever;

const OPERATOR_STAKE: u128 = 123;

/// This struct is used to test AvsRegistryServiceChainCaller methods.
#[derive(Debug)]
pub struct FakeAvsRegistryReader {
    operator_address: Address,
    operator_pubkeys: OperatorPubKeys,
    operator_id: FixedBytes<32>,
}

#[derive(Debug, Clone)]
pub struct FakeOperator {
    pub operator_id: OperatorId,
    pub pubkeys: OperatorPubKeys,
    pub stake_per_quorum: HashMap<QuorumNum, U256>,
}

impl FakeAvsRegistryReader {
    /// Creates a FakeAvsRegistryReader
    ///
    /// # Arguments
    ///
    /// * `operator` - A TestOperator.
    /// * `operator_address` - The operator address.
    ///
    /// # Returns
    ///
    /// A FakeAvsRegistryReader
    pub fn new(operator: FakeOperator, operator_address: Address) -> Self {
        Self {
            operator_address,
            operator_id: operator.operator_id,
            operator_pubkeys: operator.pubkeys,
        }
    }
}

#[async_trait]
impl AvsRegistryReader for FakeAvsRegistryReader {
    async fn get_operators_stake_in_quorums_at_block(
        &self,
        _block_number: u32,
        _quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<OperatorStateRetriever::Operator>>, AvsRegistryError> {
        Ok(vec![vec![OperatorStateRetriever::Operator {
            operator: self.operator_address,
            operatorId: self.operator_id,
            stake: OPERATOR_STAKE,
        }]])
    }

    async fn get_check_signatures_indices(
        &self,
        _reference_block_number: u32,
        _quorum_numbers: Vec<u8>,
        _non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<OperatorStateRetriever::CheckSignaturesIndices, AvsRegistryError> {
        unimplemented!()
    }

    async fn get_operator_from_id(
        &self,
        _operator_id: [u8; 32],
    ) -> Result<Address, AvsRegistryError> {
        Ok(self.operator_address)
    }
}
