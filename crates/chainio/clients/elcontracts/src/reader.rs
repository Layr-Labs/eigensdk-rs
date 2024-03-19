use eigensdk_contracts_bindings::{
    AVSDirectory,
    DelegationManager::{self, delegation_manager},
    ISlasher, StrategyManager,
};
use eigensdk_logging::logger::Logger;
use ethers_core::types::Address;
use ethers_providers::{Http, Provider};
use std::sync::Arc;
pub struct ELChainReader {
    logger: Logger,
    slasher: Address,
    delegation_manager: Address,
    strategy_manager: Address,
    avs_directory: Address,
    client: Provider<Http>,
}

pub struct ELReader {}

impl ELChainReader {
    fn new(
        slasher: Address,
        delegation_manager: Address,
        strategy_manager: Address,
        avs_directory: Address,
        logger: Logger,
        client: Provider<Http>,
    ) -> Self {
        ELChainReader {
            slasher,
            delegation_manager,
            strategy_manager,
            avs_directory,
            logger,
            client,
        }
    }

    pub async fn build(
        delegation_manager: Address,
        avs_directory: Address,
        logger: Logger,
        client: Provider<Http>,
    ) -> Self {
        let provider = Arc::new(client.clone());
        let contract_delegation_manager =
            delegation_manager::DelegationManager::new(delegation_manager, provider);
        let slasher_addr = contract_delegation_manager.slasher().call().await.unwrap();

        let strategy_manager_addr = contract_delegation_manager
            .strategy_manager()
            .call()
            .await
            .unwrap();

        Self {
            avs_directory,
            slasher: slasher_addr,
            delegation_manager,
            strategy_manager: strategy_manager_addr,
            logger,
            client,
        }
    }
}
