//! Anvil utilities
use alloy_network::Ethereum;
use alloy_primitives::{address, Address};
use alloy_provider::{
    fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    RootProvider,
};
use alloy_transport_http::{Client, Http};
use eigen_utils::{
    binding::ContractsRegistry::{self, contractsReturn},
    get_provider,
};
use once_cell::sync::Lazy;

/// Local anvil ContractsRegistry which contains a mapping of all locally deployed EL contracts.
pub const CONTRACTS_REGISTRY: Address = address!("5FbDB2315678afecb367f032d93F642f64180aa3");

/// Local anvil rpc http url
pub const ANVIL_HTTP_URL: &str = "http://localhost:8545";

/// Local anvil rpc WS url
pub const ANVIL_WS_URL: &str = "ws://localhost:8545";

#[allow(clippy::type_complexity)]
/// Local anvil rpc url alloy instance
pub static ANVIL_RPC_URL: Lazy<
    FillProvider<
        JoinFill<
            JoinFill<JoinFill<alloy_provider::Identity, GasFiller>, NonceFiller>,
            ChainIdFiller,
        >,
        RootProvider<Http<Client>>,
        Http<Client>,
        Ethereum,
    >,
> = Lazy::new(|| get_provider(ANVIL_HTTP_URL));

/// Service Manager contract address
pub async fn get_service_manager_address() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("mockAvsServiceManager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;
    address
}

/// Registry Coordinator contract address
pub async fn get_registry_coordinator_address() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("mockAvsRegistryCoordinator".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Operator state retriever contract address
pub async fn get_operator_state_retriever_address() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("mockAvsOperatorStateRetriever".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Delegation Manager contract address
pub async fn get_delegation_manager_address() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("delegationManager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Strategy Mananger contract address
pub async fn get_strategy_manager_address() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("strategyManager".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// Avs Directory contract address
pub async fn get_avs_directory_address() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("avsDirectory".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// erc20 mock strategy contract address
pub async fn get_erc20_mock_strategy() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("erc20MockStrategy".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}

/// proxy admin contract address
pub async fn get_proxy_admin() -> Address {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contracts("ProxyAdmin".to_string())
        .call()
        .await
        .unwrap();

    let contractsReturn { _0: address } = val;

    address
}
