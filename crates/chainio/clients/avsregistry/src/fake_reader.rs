use std::collections::HashMap;

use crate::{error::AvsRegistryError, reader::AvsRegistryReader};
use alloy_primitives::{Address, Bytes, FixedBytes};
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::{operator::OperatorPubKeys, test::TestOperator};
use eigen_utils::binding::OperatorStateRetriever;

struct FakeAvsRegistryReader {
    // operator_address: Address,
    operator_pubkeys: BlsKeyPair,
    operator_id: FixedBytes<32>,
}

impl FakeAvsRegistryReader {
    fn new(operator: TestOperator) -> Self {
        Self {
            operator_id: operator.operator_id,
            operator_pubkeys: operator.bls_keypair,
        }
    }
}

impl AvsRegistryReader for FakeAvsRegistryReader {
    async fn get_operators_stake_in_quorums_at_block(
        &self,
        block_number: u32,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<OperatorStateRetriever::Operator>>, AvsRegistryError> {
        todo!()
    }
    async fn get_check_signatures_indices(
        &self,
        reference_block_number: u32,
        quorum_numbers: Vec<u8>,
        non_signer_operator_ids: Vec<FixedBytes<32>>,
    ) -> Result<OperatorStateRetriever::CheckSignaturesIndices, AvsRegistryError> {
        todo!()
    }
    async fn get_operator_from_id(
        &self,
        operator_id: [u8; 32],
    ) -> Result<Address, AvsRegistryError> {
        todo!()
    }

    async fn query_existing_registered_operator_sockets(
        &self,
        start_block: u64,
        stop_block: u64,
    ) -> Result<HashMap<FixedBytes<32>, String>, AvsRegistryError> {
        todo!()
    }

    async fn query_existing_registered_operator_pub_keys(
        &self,
        start_block: u64,
        mut stop_block: u64,
        ws_url: String,
    ) -> Result<(Vec<Address>, Vec<OperatorPubKeys>), AvsRegistryError> {
        todo!()
    }
}

// func (f *FakeAVSRegistryReader) QueryExistingRegisteredOperatorPubKeys(
// 	ctx context.Context,
// 	startBlock *big.Int,
// 	stopBlock *big.Int,
// 	blockRange *big.Int,
// ) ([]types.OperatorAddr, []types.OperatorPubkeys, error) {
// 	return f.opAddress, f.opPubKeys, f.err
// }

// func (f *FakeAVSRegistryReader) QueryExistingRegisteredOperatorSockets(
// 	ctx context.Context,
// 	startBlock *big.Int,
// 	stopBlock *big.Int,
// 	blockRange *big.Int,
// ) (map[types.OperatorId]types.Socket, error) {
// 	if len(f.opPubKeys) == 0 {
// 		return nil, nil
// 	}

// 	return map[types.OperatorId]types.Socket{
// 		types.OperatorIdFromG1Pubkey(f.opPubKeys[0].G1Pubkey): f.socket,
// 	}, nil
// }

// func (f *FakeAVSRegistryReader) GetOperatorFromId(
// 	opts *bind.CallOpts,
// 	operatorId types.OperatorId,
// ) (common.Address, error) {
// 	return f.opAddress[0], f.err
// }

// func (f *FakeAVSRegistryReader) GetOperatorsStakeInQuorumsAtBlock(
// 	opts *bind.CallOpts,
// 	quorumNumbers types.QuorumNums,
// 	blockNumber types.BlockNum,
// ) ([][]opstateretriever.OperatorStateRetrieverOperator, error) {
// 	return [][]opstateretriever.OperatorStateRetrieverOperator{
// 		{
// 			{
// 				OperatorId: f.operatorId,
// 				Stake:      big.NewInt(123),
// 			},
// 		},
// 	}, nil
// }

// func (f *FakeAVSRegistryReader) GetCheckSignaturesIndices(
// 	opts *bind.CallOpts,
// 	referenceBlockNumber uint32,
// 	quorumNumbers types.QuorumNums,
// 	nonSignerOperatorIds []types.OperatorId,
// ) (opstateretriever.OperatorStateRetrieverCheckSignaturesIndices, error) {
// 	return opstateretriever.OperatorStateRetrieverCheckSignaturesIndices{}, nil
// }
