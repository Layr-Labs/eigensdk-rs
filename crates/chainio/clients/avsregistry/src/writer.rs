use eigensdk_client_elcontracts::reader::ELReader;
use eigensdk_logging::logger::Logger;
use eigensdk_txmgr::TxManager;
use ethers_core::types::Address;
use ethers_providers::{Http, Middleware, Provider};
use std::sync::Arc;
use eigensdk_contracts_bindings::{RegistryCoordinator::registry_coordinator::{self}, OperatorStateRetriever::operator_state_retriever::{self}};

pub struct AvsRegistryChainWriter {
    service_manager_addr: Address,
    registry_coordinator_addr: Address,
    operator_state_retriever_addr: Address,
    stake_registry_addr: Address,
    bls_apk_registry_addr: Address,
    el_reader: ELReader,
    logger: Logger,
    client: Provider<Http>,
    tx_mgr: TxManager,
}

trait AvsRegistryWriter {}

impl AvsRegistryChainWriter {
    async fn new_avs_registry_chain_writer(
        service_manager_addr: Address,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        stake_registry_addr: Address,
        bls_apk_registry_addr: Address,
        el_reader: ELReader,
        logger: Logger,
        client: Provider<Http>,
        tx_mgr: TxManager,
    ) -> Self {
        AvsRegistryChainWriter {
            service_manager_addr,
            registry_coordinator_addr,
            operator_state_retriever_addr,
            stake_registry_addr,
            bls_apk_registry_addr,
            el_reader,
            logger,
            client,
            tx_mgr,
        }
    }

    async fn build_avs_registry_chain_writer(
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        logger: Logger,
        client: Provider<Http>,
        tx_mgr: TxManager,
    ) {
        let provider = Arc::new(client);
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(registry_coordinator_addr,provider.clone());

        let contract_operator_state_retriever = operator_state_retriever::OperatorStateRetriever::new(operator_state_retriever_addr,provider);


        let service_manager_addr = contract_registry_coordinator.service_manager().call().await.unwrap();
        
        

    }
}
