//! Anvil utilities
use alloy::primitives::{address, Address};
use eigen_common::get_provider;
use eigen_utils::rewardsv2::sdk::contractsregistry::ContractsRegistry::{self, contractsReturn};

// Addresses and private keys of accounts generated by anvil

/// Address of the first default account generated by anvil
pub const FIRST_ADDRESS: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
/// Private key of the first default account generated by anvil
pub const FIRST_PRIVATE_KEY: &str =
    "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

/// Address of the second default account generated by anvil
pub const SECOND_ADDRESS: Address = address!("70997970C51812dc3A010C7d01b50e0d17dc79C8");
/// Private key of the second default account generated by anvil
pub const SECOND_PRIVATE_KEY: &str =
    "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

/// Address of the fifth default account generated by anvil
pub const FIFTH_ADDRESS: Address = address!("9965507D1a55bcC2695C58ba16FB37d819B0A4dc");
/// Private key of the fifth default account generated by anvil
pub const FIFTH_PRIVATE_KEY: &str =
    "8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba";

/// Local anvil ContractsRegistry which contains a mapping of all locally deployed EL contracts.
pub const CONTRACTS_REGISTRY: Address = address!("5FbDB2315678afecb367f032d93F642f64180aa3");

/// Local anvil rpc http url
pub const ANVIL_HTTP_URL: &str = "http://localhost:8545";

/// Local anvil rpc WS url
pub const ANVIL_WS_URL: &str = "ws://localhost:8545";

/// Service Manager contract address
pub async fn get_service_manager_address(rpc_url: String) -> Address {
    let provider = get_provider(&rpc_url);
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, provider);

    let val = contracts_registry
        .contracts("mockAvsServiceManager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

/// Registry Coordinator contract address
pub async fn get_registry_coordinator_address(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("mockAvsRegistryCoordinator".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Operator state retriever contract address
pub async fn get_operator_state_retriever_address(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("mockAvsOperatorStateRetriever".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Delegation Manager contract address
pub async fn get_delegation_manager_address(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("delegationManager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Strategy Mananger contract address
pub async fn get_strategy_manager_address(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("strategyManager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Avs Directory contract address
pub async fn get_avs_directory_address(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("avsDirectory".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// erc20 mock strategy contract address
pub async fn get_erc20_mock_strategy(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("erc20MockStrategy".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// proxy admin contract address
pub async fn get_proxy_admin(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("ProxyAdmin".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Rewards contract address
pub async fn get_rewards_coordinator_address(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("rewardsCoordinator".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Allocation Manager contract address
pub async fn get_allocation_manager_address(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("allocationManager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Permission Controller contract address
pub async fn get_permission_controller_address(rpc_url: String) -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, get_provider(&rpc_url));

    let val = contracts_registry
        .contracts("permissionController".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}
