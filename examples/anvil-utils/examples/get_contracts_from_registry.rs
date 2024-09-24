//! Example to showcase testing EL contracts using anvil
use eigen_testing_utils::{
    anvil::start_anvil_container,
    anvil_constants::{
        get_delegation_manager_address, get_erc20_mock_strategy,
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_service_manager_address, get_strategy_manager_address,
    },
};
use eyre::Result;

/// Calling ContractsRegistry contract to extract the El contract addresses on anvil
#[tokio::main]
pub async fn main() -> Result<()> {
    let (_container, http_endpoint, _ws_endpoint) = start_anvil_container().await;

    get_service_manager_address(http_endpoint.clone()).await;
    get_registry_coordinator_address(http_endpoint.clone()).await;
    get_operator_state_retriever_address(http_endpoint.clone()).await;
    get_delegation_manager_address(http_endpoint.clone()).await;
    get_strategy_manager_address(http_endpoint.clone()).await;
    get_erc20_mock_strategy(http_endpoint).await;
    Ok(())
}
