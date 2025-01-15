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
    use alloy::hex::FromHex;
    use alloy::sol_types::SolValue;
    use alloy_primitives::{address, keccak256, Address, Bytes, FixedBytes, U256, U8};
    use eigen_common::{get_provider, get_signer};
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants::{
        get_allocation_manager_address, get_avs_directory_address, get_delegation_manager_address,
        get_erc20_mock_strategy, get_registry_coordinator_address, get_rewards_coordinator_address,
        get_strategy_manager_address,
    };
    use eigen_utils::{
        core::{
            delegationmanager::DelegationManager,
            irewardscoordinator::{
                IRewardsCoordinator,
                IRewardsCoordinatorTypes::{
                    EarnerTreeMerkleLeaf, RewardsMerkleClaim, TokenTreeMerkleLeaf,
                },
            },
        },
        sdk::mockerc20::MockERC20,
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

    // Using test data taken from
    // https://github.com/Layr-Labs/eigenlayer-contracts/blob/a888a1cd1479438dda4b138245a69177b125a973/src/test/test-data/rewardsCoordinator/processClaimProofs_MaxEarnerAndLeafIndices.json
    pub async fn new_test_claim(http_endpoint: &str) -> (FixedBytes<32>, RewardsMerkleClaim) {
        let el_chain_reader = build_el_chain_reader(http_endpoint.to_string()).await;

        let earner_address = address!("25a1b7322f9796b26a4bec125913b34c292b28d6");
        let claim = RewardsMerkleClaim {
            rootIndex: 0,
            earnerIndex: 7,
            earnerTreeProof: Bytes::from_hex("4bf5e16eaabbc36964f1e1639808669420f55d60e51adb7e9695b77145c479fd6777be59643947bb24d78e69d6605bf369c515b479f3a8967dd68a97c5bb4a4a262b28002eeb6cbbffb7e79e5741bf2be189a6073440a62fabcd8af4dbda94e3").unwrap(),
            earnerLeaf: EarnerTreeMerkleLeaf {
                earner: earner_address,
                earnerTokenRoot: FixedBytes::from_hex(
                    "f8e7e20b32aae1d818dcb593b98982841e9a0ed12c161ad603e3ee3948746cba",
                )
                .unwrap(),
            },
            tokenIndices: vec![7],
            tokenTreeProofs: vec![
                Bytes::from_hex("3cd04e8fc6f23812c570fe12292a30bb9e105e00f5913ac4b4938f23e65d8d10e6b1403d58c9d5450952e7d96c81305dad9fb966e8a27d3a42058e3958a0d30033148e91b455542d05deb81b8305b672e742cd3145f7022a0089bad2e6af9173").unwrap(),
            ],
            tokenLeaves: vec![TokenTreeMerkleLeaf {
                token: address!("7fbfdd1dfd80730385aee232cc9f79b8ae12a654"),
                cumulativeEarnings: U256::from_str("3000000000000000000").unwrap(),
            }],
        };

        let root = FixedBytes::from_hex(
            "37550707c80f3d8907c467999730e52127ab89be3f17a5017a3f1ffb73a1445f",
        )
        .unwrap();

        let key = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
        let rewards_coordinator = IRewardsCoordinator::new(
            get_rewards_coordinator_address(http_endpoint.to_string()).await,
            get_signer(key, http_endpoint),
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

    /// The claim can be submitted from [`ANVIL_FIRST_PRIVATE_KEY`]
    pub async fn new_claim(
        http_endpoint: &str,
        cumulative_earnings: U256,
    ) -> (FixedBytes<32>, RewardsMerkleClaim) {
        let signer = get_signer(ANVIL_FIRST_PRIVATE_KEY, http_endpoint);
        let rewards_coordinator_address =
            get_rewards_coordinator_address(http_endpoint.to_string()).await;

        let el_chain_reader = build_el_chain_reader(http_endpoint.to_string()).await;
        let mock_strategy = get_erc20_mock_strategy(http_endpoint.to_string()).await;
        let (_, token_address) = el_chain_reader
            .get_strategy_and_underlying_token(mock_strategy)
            .await
            .unwrap();

        // Initialize the rewards coordinator bindings
        let rewards_coordinator = IRewardsCoordinator::new(rewards_coordinator_address, &signer);

        // Mint tokens for the rewards coordinator
        let token = MockERC20::new(token_address, &signer);
        let receipt = token
            .mint(rewards_coordinator_address, cumulative_earnings)
            .send()
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        assert!(receipt.status());

        // Generate token tree leaf
        // For the tree structure, see https://github.com/Layr-Labs/eigenlayer-contracts/blob/a888a1cd1479438dda4b138245a69177b125a973/docs/core/RewardsCoordinator.md#rewards-merkle-tree-structure
        let earner_address = ANVIL_FIRST_ADDRESS;
        let token_leaves = vec![TokenTreeMerkleLeaf {
            token: token_address,
            cumulativeEarnings: cumulative_earnings,
        }];
        // Hash token tree leaf to get root
        let encoded_token_leaf = [
            // uint8 internal constant TOKEN_LEAF_SALT = 1;
            U8::from(1).to_be_bytes_vec(),
            token_leaves[0].token.abi_encode_packed(),
            token_leaves[0].cumulativeEarnings.abi_encode_packed(),
        ]
        .concat();
        let earner_token_root = keccak256(encoded_token_leaf);

        // Generate earner tree leaf
        let earner_leaf = EarnerTreeMerkleLeaf {
            earner: earner_address,
            earnerTokenRoot: earner_token_root,
        };
        // Hash earner tree leaf to get root
        let encoded_earner_leaf = [
            // uint8 internal constant EARNER_LEAF_SALT = 0;
            U8::from(0).to_be_bytes_vec(),
            earner_leaf.earner.abi_encode_packed(),
            earner_leaf.earnerTokenRoot.abi_encode_packed(),
        ]
        .concat();
        let earner_tree_root = keccak256(encoded_earner_leaf);

        // Fetch the next root index from contract
        let next_root_index = el_chain_reader
            .get_distribution_roots_length()
            .await
            .unwrap();
        // Construct the claim
        let claim = RewardsMerkleClaim {
            rootIndex: next_root_index.try_into().unwrap(),
            earnerIndex: 0,
            // Empty proof because leaf == root
            earnerTreeProof: vec![].into(),
            earnerLeaf: earner_leaf,
            tokenIndices: vec![0],
            tokenTreeProofs: vec![
                // Empty proof because leaf == root
                vec![].into(),
            ],
            tokenLeaves: token_leaves,
        };

        let root = earner_tree_root;

        // Fetch the current timestamp to increase it
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
