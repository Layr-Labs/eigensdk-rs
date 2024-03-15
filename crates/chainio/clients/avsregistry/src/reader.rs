use alloy_primitives::{Address, U8};
use alloy_sol_types::{sol, SolCall, SolConstructor, SolInterface};
use eigensdk_client_eth::client::Client;
use eigensdk_logging::logger::Logger;
use alloy_transport_http::Http;
use alloy_sol_types::SolEvent;
use alloy_contract::{private::Network, private::Provider, private::Transport, SolCallBuilder};
use ethers::{prelude::Abigen};
use std::path::{Path,PathBuf};
use eigensdk_contracts_bindings::registry_coordinator;
// sol! {
//     // #[sol(rpc)]
//     #[derive(Debug)]
//     RegistryCoordinator,
//     "../../../../crates/contracts/src/RegistryCoordinator.json"
//     // "../../../../crates/contracts/src/RegistryCoordinator.sol"
// }



fn generate_bindings(contract_name: &str,input_path : &str,output_path: &str ) {
    let coontract : String = format!("../../../../crates/contracts/bindings/json/{input_path}").to_string();
    println!("path :{}",coontract);

    match Abigen::new(&contract_name, coontract){
        Ok(v)=>{
            println!("okoik");
           let _=v.generate().expect("failed to abigen").write_to_file("../../../../crates/contracts/bindings/src/registry_coordinator.rs");
        },
        Err(e)=>{
            println!("abigenerr{}",e);
        }
    }

}

// use self::{
//     OperatorStateRetriever::OperatorStateRetrieverCalls,
//     RegistryCoordinator::{quorumCountCall, RegistryCoordinatorCalls}, StakeRegistry::StakeRegistryCalls,
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
    eth_client: Client,
}

trait AvsRegistryReader {
    fn get_quorum_count() -> Result<U8, String>;
}

impl AvsRegistryChainReader {
    fn new(
        registry_coordinator_addr: Address,
        bls_apk_registry_addr: Address,
        // registry_coordinator: RegistryCoordinatorCalls,
        // operator_state_retriever: OperatorStateRetrieverCalls,
        // stake_registry: StakeRegistryCalls,
        logger: Logger,
        eth_client: Client,
    ) -> Self {
        AvsRegistryChainReader {
            logger,
            bls_apk_registry_addr,
            registry_coordinator_addr,
            // registry_coordinator,
            // operator_state_retriever,
            // stake_registry,
            eth_client,
        }
    }

    fn build_avs_registry_chain_reader<N: alloy_contract::private::Network,E:SolEvent, P: alloy_contract::private::Provider<N>>(provider: P) -> Result<(), Box<dyn std::error::Error>>
    // where
    //     N: alloy_contract::private::Network,
    //     T: alloy_contract::private::Transport + Clone,
    //     P: alloy_contract::private::Provider<N, Http<T>>,
    //     C: SolCall,
    {
            generate_bindings("RegistryCoordinator","RegistryCoordinator.json", "crates/contracts/src/RegistryCoordinatorbindings");
        
        let a = 2;
        // let registry_coordinator = RegistryCoordinator{registry_coordinator_addr};

       

        todo!()
    }

    // fn get_quorum_count(&self) ->Result<U8, String>{
        
    //    let s =  quorumCountCall{};
    // }

}


#[test]
fn test_binding_generation() {
    
    generate_bindings("RegistryCoordinator","RegistryCoordinator.json", "../../../../crates/contracts/src/bindings");
}