use alloy_primitives::{Address, U8};
use alloy_sol_types::{sol, SolCall, SolConstructor, SolInterface};
use eigensdk_client_eth::client::Client;
use eigensdk_logging::logger::Logger;
use alloy_transport_http::Http;
use alloy_sol_types::SolEvent;
use alloy_contract::{private::Network, private::Provider, private::Transport, SolCallBuilder};
use ethers::{prelude::Abigen, providers::Middleware};
use std::path::{Path,PathBuf};
use eigensdk_contracts_bindings::{RegistryCoordinator,OperatorStateRetriever,StakeRegistry};




fn generate_bindings(contract_name: &str,input_path : &str,output_path: &str ) {
    let coontract : String = format!("../../../../crates/contracts/bindings/json/{input_path}").to_string();
    println!("path :{}",coontract);

    match Abigen::new(&contract_name, coontract){
        Ok(v)=>{
            println!("okoik");
           let _=v.generate().expect("failed to abigen").write_to_file(format!("{output_path}/src/{contract_name}.rs"));
        },
        Err(e)=>{
            println!("abigenerr{}",e);
        }
    }

}



#[derive(Debug)]
pub struct AvsRegistryChainReader<M> {
    logger: Logger,
    bls_apk_registry_addr: Address,
    registry_coordinator_addr: Address,
    registry_coordinator:ethers::contract::Contract<M>,
    operator_state_retriever:ethers::contract::Contract<M>,
    stake_registry: ethers::contract::Contract<M>,
    eth_client: Client,
}

trait AvsRegistryReader {
    fn get_quorum_count() -> Result<U8, String>;
}

impl <M: 'static + Middleware>AvsRegistryChainReader<M> {
    fn new(
        logger : Logger,
        registry_coordinator_addr: Address,
        bls_apk_registry_addr: Address,
        registry_coordinate: ethers::contract::Contract<M>,
        operator_state_retriever: ethers::contract::Contract<M>,
        stake_registry:ethers::contract::Contract<M>,
        eth_client: Client,
    ) -> Self {
        AvsRegistryChainReader {
            logger,
            bls_apk_registry_addr,
            registry_coordinator_addr,
            registry_coordinator:registry_coordinate,
            operator_state_retriever,
            stake_registry,
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
            generate_bindings("RegistryCoordinator","RegistryCoordinator.json", "../../../../crates/contracts/bindings/src/RegistryCoordinatorbindings");
        
        // let registry_coordinator = RegistryCoordinator{registry_coordinator_addr};

       

        todo!()
    }

    // fn get_quorum_count(&self) ->Result<U8, String>{
        
    //    let s =  quorumCountCall{};
    // }

}


#[test]
fn test_binding_generation() {
    
    generate_bindings("RegistryCoordinator","RegistryCoordinator.json", "../../../../crates/contracts/bindings");
    generate_bindings("OperatorStateRetriever","OperatorStateRetriever.json", "../../../../crates/contracts/bindings");
    generate_bindings("StakeRegistry","StakeRegistry.json", "../../../../crates/contracts/bindings");


}