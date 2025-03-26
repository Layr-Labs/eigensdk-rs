use alloy::{
    primitives::{keccak256, FixedBytes, U256, U8},
    sol_types::SolValue,
};
use eigen_client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
use eigen_common::{get_provider, get_signer};
use eigen_logging::get_test_logger;

use eigen_client_avsregistry::{reader::AvsRegistryChainReader, writer::AvsRegistryChainWriter};
use eigen_utils::slashing::{
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

use crate::anvil_constants::{
    get_allocation_manager_address, get_avs_directory_address, get_delegation_manager_address,
    get_erc20_mock_strategy, get_operator_state_retriever_address,
    get_registry_coordinator_address, get_rewards_coordinator_address, get_service_manager_address,
    get_strategy_manager_address, FIRST_ADDRESS, FIRST_PRIVATE_KEY,
};

/// Creates an avs registry chain writer from a given http endpoint and a private key
pub async fn build_avs_registry_chain_writer(
    http_endpoint: String,
    private_key: String,
) -> AvsRegistryChainWriter {
    let registry_coordinator_address =
        get_registry_coordinator_address(http_endpoint.clone()).await;
    let service_manager_addr = get_service_manager_address(http_endpoint.clone()).await;
    AvsRegistryChainWriter::build_avs_registry_chain_writer(
        get_test_logger(),
        http_endpoint,
        private_key,
        registry_coordinator_address,
        service_manager_addr,
    )
    .await
    .unwrap()
}

/// Creates an avs registry chain reader from a given http endpoint
pub async fn build_avs_registry_chain_reader(http_endpoint: String) -> AvsRegistryChainReader {
    let registry_coordinator_addr = get_registry_coordinator_address(http_endpoint.clone()).await;
    let operator_state_retriever_address =
        get_operator_state_retriever_address(http_endpoint.clone()).await;

    AvsRegistryChainReader::new(
        get_test_logger(),
        registry_coordinator_addr,
        operator_state_retriever_address,
        http_endpoint.to_string(),
    )
    .await
    .unwrap()
}

/// Creates an eigen layer chain reader from a given http endpoint
pub async fn build_el_chain_reader(http_endpoint: String) -> ELChainReader {
    let delegation_manager_address = get_delegation_manager_address(http_endpoint.clone()).await;
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

/// Creates an eigen layer chain writer from a given http endpoint
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
        Some(permission_controller),
        Some(allocation_manager),
        registry_coordinator,
        el_chain_reader,
        http_endpoint.clone(),
        private_key,
    )
}

/// Creates a new claim of amount <cumulative_earnings> in the anvil on the address defined in http_endpoint.
/// The claim can be submitted from [`FIRST_PRIVATE_KEY`].
pub async fn new_claim(
    http_endpoint: &str,
    cumulative_earnings: U256,
) -> (FixedBytes<32>, RewardsMerkleClaim) {
    let signer = get_signer(FIRST_PRIVATE_KEY, http_endpoint);
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
    let earner_address = FIRST_ADDRESS;
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
