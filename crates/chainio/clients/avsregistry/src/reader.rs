use alloy_primitives::{Address,U8};
use alloy_sol_types::{sol,SolCall,SolConstructor,SolInterface};
use eigensdk_client_eth::client::Client;
use eigensdk_logging::logger::Logger;

use alloy_contract::{SolCallBuilder,private::Provider,private::Transport,private::Network};
// use self::{
//     OperatorStateRetriever::OperatorStateRetrieverCalls,
//     RegistryCoordinator::RegistryCoordinatorCalls, StakeRegistry::StakeRegistryCalls,
// };




// sol!{
//     #[derive(Debug)]
//     OperatorStateRetriever,
//     "../../../../crates/contracts/src/OperatorStateRetriever.json"
// }

// sol! {
// #[derive(Debug)] 
// StakeRegistry,
//     "../../../../crates/contracts/src/StakeRegistry.json"
// }

#[derive(Debug)]
pub struct AvsRegistryChainReader {
    logger: Logger,
    bls_apk_registry_addr: Address,
    registry_coordinator_addr: Address,
    // registry_coordinator: RegistryCoordinatorCalls,
    // operator_state_retriever: OperatorStateRetrieverCalls,
    // stake_registry: StakeRegistryCalls,
    eth_client:Client
}

trait AvsRegistryReader {

    fn get_quorum_count() -> Result<U8,String>;

}


impl AvsRegistryChainReader{

    fn new(registry_coordinator_addr: Address,
        bls_apk_registry_addr : Address,
        // registry_coordinator: RegistryCoordinatorCalls,
        // operator_state_retriever: OperatorStateRetrieverCalls,
        // stake_registry: StakeRegistryCalls,
        logger: Logger,
        eth_client : Client 
    ) -> Self{
        AvsRegistryChainReader{
            logger,
            bls_apk_registry_addr,
            registry_coordinator_addr,
            // registry_coordinator,
            // operator_state_retriever,
            // stake_registry,
            eth_client
        }
    }

    fn build_avs_registry_chain_reader
    <N,T,P,C>(
       provider:P,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr : Address,
        eth_client : Client,
        logger : Logger
    ) ->Result<AvsRegistryChainReader,String>
    where 
    N : alloy_contract::private::Network,
    T: alloy_contract::private::Transport + Clone,
    P : alloy_contract::private::Provider<N,T>,
    C:SolCall
    {
        let a = 2;
        // let registry_coordinator = RegistryCoordinator{registry_coordinator_addr};
          
    sol!{
        #[sol(rpc)]
        "../../../../crates/contracts/src/RegistryCoordinator.sol"
        
    }


        todo!()

    }

    



}
