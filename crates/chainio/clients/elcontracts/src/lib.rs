//!Convenience methods for AVSs developers to call Eigen layer contract methods

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

pub mod error;
pub mod reader;
pub mod writer;

#[cfg(test)]
pub(crate) mod test_utils {
    use alloy_primitives::{address, keccak256, Address, FixedBytes, U256, U8};
    use alloy_sol_types::SolValue;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants::{
        get_allocation_manager_address, get_avs_directory_address, get_delegation_manager_address,
        get_erc20_mock_strategy, get_registry_coordinator_address, get_rewards_coordinator_address,
        get_strategy_manager_address,
    };
    use eigen_utils::{
        delegationmanager::DelegationManager,
        get_provider, get_signer,
        irewardscoordinator::{
            IRewardsCoordinator,
            IRewardsCoordinatorTypes::{
                EarnerTreeMerkleLeaf, RewardsMerkleClaim, TokenTreeMerkleLeaf,
            },
        },
    };
    use std::str::FromStr;

    use crate::{reader::ELChainReader, writer::ELChainWriter};

    pub const OPERATOR_ADDRESS: Address = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
    pub const OPERATOR_PRIVATE_KEY: &str =
        "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

    pub const ANVIL_FIRST_ADDRESS: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    pub const ANVIL_FIRST_PRIVATE_KEY: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    pub async fn build_el_chain_reader(http_endpoint: String) -> ELChainReader {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;

        ELChainReader::build(
            get_test_logger().clone(),
            delegation_manager_address,
            avs_directory_address,
            rewards_coordinator,
            &http_endpoint,
        )
        .await
        .unwrap()
    }

    pub async fn new_test_writer(http_endpoint: String, private_key: String) -> ELChainWriter {
        let el_chain_reader = build_el_chain_reader(http_endpoint.clone()).await;
        let strategy_manager = get_strategy_manager_address(http_endpoint.clone()).await;
        let rewards_coordinator = get_rewards_coordinator_address(http_endpoint.clone()).await;
        let delegation_manager = get_delegation_manager_address(http_endpoint.clone()).await;
        let allocation_manager = get_allocation_manager_address(http_endpoint.clone()).await;
        let contract_delegation_manager =
            DelegationManager::new(delegation_manager, get_provider(&http_endpoint));
        let permission_controller = contract_delegation_manager
            .permissionController()
            .call()
            .await
            .unwrap()
            ._0;
        let registry_coordinator = get_registry_coordinator_address(http_endpoint.clone()).await;

        ELChainWriter::new(
            delegation_manager,
            strategy_manager,
            rewards_coordinator,
            permission_controller,
            allocation_manager,
            registry_coordinator,
            el_chain_reader,
            http_endpoint.clone(),
            private_key,
        )
    }

    pub async fn new_claim(http_endpoint: &str) -> (FixedBytes<32>, RewardsMerkleClaim) {
        let el_chain_reader = build_el_chain_reader(http_endpoint.to_string()).await;
        let mock_strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        let (_, token_address) = el_chain_reader
            .get_strategy_and_underlying_token(mock_strategy)
            .await
            .unwrap();

        let earner_address = ANVIL_FIRST_ADDRESS;
        let token_leaves = vec![TokenTreeMerkleLeaf {
            token: token_address,
            cumulativeEarnings: U256::from_str("3000000000000000000").unwrap(),
        }];
        let encoded_token_leaf = [
            // uint8 internal constant TOKEN_LEAF_SALT = 1;
            U8::from(1).to_be_bytes_vec(),
            token_leaves[0].token.abi_encode_packed(),
            token_leaves[0].cumulativeEarnings.abi_encode_packed(),
        ]
        .concat();
        let earner_token_root = dbg!(keccak256(encoded_token_leaf));
        let earner_leaf = EarnerTreeMerkleLeaf {
            earner: earner_address,
            earnerTokenRoot: earner_token_root,
        };
        let encoded_earner_leaf = [
            // uint8 internal constant EARNER_LEAF_SALT = 0;
            U8::from(0).to_be_bytes_vec(),
            earner_leaf.earner.abi_encode_packed(),
            earner_leaf.earnerTokenRoot.abi_encode_packed(),
        ]
        .concat();
        let earner_tree_root = dbg!(keccak256(encoded_earner_leaf));
        let claim = RewardsMerkleClaim {
            rootIndex: 0,
            earnerIndex: 0,
            // Empty because leaf == root
            earnerTreeProof: vec![].into(),
            earnerLeaf: earner_leaf,
            tokenIndices: vec![0],
            tokenTreeProofs: vec![
                // Empty because leaf == root
                vec![].into(),
            ],
            tokenLeaves: token_leaves,
        };

        let root = earner_tree_root;

        let rewards_coordinator = IRewardsCoordinator::new(
            get_rewards_coordinator_address(http_endpoint.to_string()).await,
            get_signer(ANVIL_FIRST_PRIVATE_KEY, http_endpoint),
        );
        let curr_rewards_calculation_end_timestamp = el_chain_reader
            .curr_rewards_calculation_end_timestamp()
            .await
            .unwrap();
        let submit_tx = rewards_coordinator
            .submitRoot(root, curr_rewards_calculation_end_timestamp + 1)
            .send()
            .await
            .unwrap();
        let submit_status = submit_tx.get_receipt().await.unwrap().status();
        assert!(submit_status);

        (root, claim)
    }
}
