use eigensdk_contracts_bindings::{
    AVSDirectory,
    DelegationManager::{self, delegation_manager},
    ISlasher, StrategyManager,
};
use eigensdk_logging::logger::Logger;
use ethers_providers::{Http, Provider};
pub struct ELChainReader {
    logger: Logger,
    slasher: ISlasher::ISlasherCalls,
    delegation_manager: DelegationManager::DelegationManagerCalls,
    strategy_manager: StrategyManager::StrategyManagerCalls,
    avs_directory: AVSDirectory::AVSDirectoryCalls,
    client: Provider<Http>,
}

pub struct ELReader {}

impl ELChainReader {
    fn new(
        slasher: ISlasher::ISlasherCalls,
        delegation_manager: DelegationManager::DelegationManagerCalls,
        strategy_manager: StrategyManager::StrategyManagerCalls,
        avs_directory: AVSDirectory::AVSDirectoryCalls,
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
}
