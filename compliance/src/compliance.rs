#[cfg(test)]
pub mod test {
    use alloy_primitives::{Address, B256, U256};
    use eigen_client_avsregistry::fake_reader::FakeAvsRegistryReader;
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_services_avsregistry::{
        chaincaller::AvsRegistryServiceChainCaller, AvsRegistryService,
    };
    use eigen_services_operatorsinfo::fake_operator_info::FakeOperatorInfoService;
    use eigen_types::operator::{OperatorAvsState, OperatorInfo, OperatorPubKeys, QuorumNum};
    use eigen_types::test::TestOperator;
    use serde::Deserialize;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufReader;
    use std::str::FromStr;

    const PRIVATE_KEY_DECIMAL: &str =
        "13710126902690889134622698668747132666439281256983827313388062967626731803599";
    const OPERATOR_ID: &str = "48beccce16ccdf8000c13d5af5f91c7c3dac6c47b339d993d229af1500dbe4a9";
    const OPERATOR_ADDRESS: &str = "0xa0Ee7A142d267C1f36714E4a8F75612F20a79720";

    #[derive(Deserialize, Debug)]
    struct Input {
        quorum_numbers: Vec<QuorumNum>,
        block_num: u32,
    }

    #[derive(Deserialize, Debug)]
    struct TestData {
        input: Input,
    }

    #[tokio::test]
    async fn test_get_operators_avs_state() {
        let test_data_path = std::env::var("TEST_DATA_PATH").unwrap();
        let file = File::open(test_data_path).unwrap();
        let reader = BufReader::new(file);
        let data: TestData = serde_json::from_reader(reader).unwrap();

        let block_num = data.input.block_num;
        let quorum_nums = data.input.quorum_numbers.as_slice();
        let test_operator = TestOperator {
            operator_id: B256::from_str(OPERATOR_ID).unwrap(),
            bls_keypair: BlsKeyPair::new(PRIVATE_KEY_DECIMAL.into()).unwrap(),
            stake_per_quorum: HashMap::from([(1u8, U256::from(123))]),
        };

        let operator_address = Address::from_str(OPERATOR_ADDRESS).unwrap();
        let avs_registry = FakeAvsRegistryReader::new(test_operator.clone(), operator_address);
        let operator_info_service = FakeOperatorInfoService::new(test_operator.bls_keypair.clone());
        let service = AvsRegistryServiceChainCaller::new(avs_registry, operator_info_service);

        let operator_avs_state = service
            .get_operators_avs_state_at_block(block_num, quorum_nums)
            .await
            .unwrap();

        let expected_operator_avs_state = OperatorAvsState {
            operator_id: test_operator.operator_id.into(),
            operator_info: OperatorInfo {
                pub_keys: Some(OperatorPubKeys::from(test_operator.bls_keypair)),
            },
            stake_per_quorum: HashMap::from([(data.input.quorum_numbers[0], U256::from(123))]),
            block_num: block_num.into(),
        };
        let operator_state = operator_avs_state.get(&test_operator.operator_id).unwrap();
        assert_eq!(expected_operator_avs_state, *operator_state);
    }
}
