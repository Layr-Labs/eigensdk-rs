use std::collections::HashMap;

use alloy_primitives::BlockNumber;
use eigen_crypto_bls::OperatorId;
use eigen_types::{
    operator::{OperatorAvsState, OperatorInfo, OperatorPubKeys},
    test::TestOperator,
};

pub struct FakeAvsRegistryService {
    operators: HashMap<BlockNumber, HashMap<OperatorId, OperatorAvsState>>,
}

impl FakeAvsRegistryService {
    pub fn new(block_number: BlockNumber, test_operators: Vec<TestOperator>) -> Self {
        let mut state_per_operator_id: HashMap<OperatorId, OperatorAvsState> = HashMap::new();

        // populate the inner hashmap
        for op in test_operators {
            let state = OperatorAvsState {
                operator_id: op.operator_id.into(),
                operator_info: OperatorInfo {
                    pub_keys: Some(OperatorPubKeys::from(op.bls_keypair)),
                },
                block_num: block_number.into(),
                stake_per_quorum: op.stake_per_quorum,
            };
            state_per_operator_id.insert(op.operator_id, state);
        }

        Self {
            operators: HashMap::from([(block_number, state_per_operator_id)]),
        }
    }
}
