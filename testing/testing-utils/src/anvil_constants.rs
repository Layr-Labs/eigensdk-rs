//! Anvil utilities
use alloy_primitives::{address, Address};
use eigen_utils::{
    contractsregistry::ContractsRegistry::{self, contractsReturn},
    get_provider,
};

/// Local anvil ContractsRegistry which contains a mapping of all locally deployed EL contracts.
pub const CONTRACTS_REGISTRY: Address = address!("5FbDB2315678afecb367f032d93F642f64180aa3");

/// Local anvil rpc http url
pub const ANVIL_HTTP_URL: &str = "http://localhost:8545";

/// Local anvil rpc WS url
pub const ANVIL_WS_URL: &str = "ws://localhost:8545";

#[allow(clippy::type_complexity)]

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
