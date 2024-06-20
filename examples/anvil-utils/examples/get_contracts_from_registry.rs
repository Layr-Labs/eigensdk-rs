use eigen_testing_utils::anvil_constants::{
    get_delegation_manager_address, get_erc20_mock_strategy, get_operator_state_retriever_address,
    get_registry_coordinator_address, get_service_manager_address, get_strategy_manager_address,
};
use eyre::Result;

#[tokio::main]
pub async fn main() -> Result<()> {
    get_service_manager_address().await;
    get_registry_coordinator_address().await;
    get_operator_state_retriever_address().await;
    get_delegation_manager_address().await;
    get_strategy_manager_address().await;
    get_erc20_mock_strategy().await;
    Ok(())
}
