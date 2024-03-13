use alloy_primitives::Address;
use alloy_sol_types::sol;
use eigensdk_client_eth::client::Client;
use eigensdk_logging::logger::Logger;

use self::{
    OperatorStateRetriever::OperatorStateRetrieverCalls,
    RegistryCoordinator::RegistryCoordinatorCalls, StakeRegistry::StakeRegistryCalls,
};

sol! {
    #[derive(Debug)]
    RegistryCoordinator,
    "../../../../crates/contracts/src/RegistryCoordinator.json"
}

sol! {
    #[derive(Debug)]
    OperatorStateRetriever,
    "../../../../crates/contracts/src/OperatorStateRetriever.json"
}

sol! {
#[derive(Debug)]
StakeRegistry,
    "../../../../crates/contracts/src/StakeRegistry.json"
}

#[derive(Debug)]
pub struct AvsRegistryChainReader {
    logger: Logger,
    bls_apk_registry_addr: Address,
    registry_coordinator_addr: Address,
    registry_coordinator: RegistryCoordinatorCalls,
    operator_state_retriever: OperatorStateRetrieverCalls,
    snake_registry: StakeRegistryCalls,
    eth_client:Client
}

trait AvsRegistryReader {}
