//! Anvil utilities
use alloy_network::Ethereum;
use alloy_primitives::{address, Address, U256};
use alloy_provider::{
    fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    RootProvider,
};
use alloy_transport_http::{Client, Http};
use eigen_utils::{
    binding::ContractsRegistry::{self, contractNamesReturn},
    get_provider,
};
use once_cell::sync::Lazy;

/// Local anvil ContractsRegistry which contains a mapping of all locally deployed EL contracts.
pub const CONTRACTS_REGISTRY: Address = address!("5FbDB2315678afecb367f032d93F642f64180aa3");

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
> = Lazy::new(|| get_provider("http://localhost:8545"));

/// Service Manager contract address
pub async fn get_service_manager_address() {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contractNames(U256::from(0))
        .call()
        .await
        .unwrap();

    let contractNamesReturn { _0: first_name } = val;

    println!("contract name  : {:?}", first_name);
}

/// Registry Coordinator contract address
pub async fn get_registry_coordinator_address() {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contractNames(U256::from(1))
        .call()
        .await
        .unwrap();

    let contractNamesReturn { _0: first_name } = val;

    println!("contract name  : {:?}", first_name);
}

/// Operator state retriever contract address
pub async fn get_operator_state_retriever_address() {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contractNames(U256::from(2))
        .call()
        .await
        .unwrap();

    let contractNamesReturn { _0: first_name } = val;

    println!("contract name  : {:?}", first_name);
}

/// Delegation Manager contract address
pub async fn get_delegation_manager_address() {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contractNames(U256::from(3))
        .call()
        .await
        .unwrap();

    let contractNamesReturn { _0: first_name } = val;

    println!("contract name  : {:?}", first_name);
}

/// Strategy Mananger contract address
pub async fn get_strategy_manager_address() {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contractNames(U256::from(4))
        .call()
        .await
        .unwrap();

    let contractNamesReturn { _0: first_name } = val;

    println!("contract name  : {:?}", first_name);
}

pub async fn get_erc20_mock_strategy() {
    let contracts_registry = ContractsRegistry::new(CONTRACTS_REGISTRY, (*ANVIL_RPC_URL).clone());

    let val = contracts_registry
        .contractNames(U256::from(5))
        .call()
        .await
        .unwrap();

    let contractNamesReturn { _0: first_name } = val;

    println!("contract name  : {:?}", first_name);
}
