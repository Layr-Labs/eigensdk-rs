use alloy_primitives::{U8,U32};
use alloy_sol_types::{sol, SolCall, SolConstructor, SolInterface};
use eigensdk_client_eth::client::Client;
use eigensdk_logging::logger::Logger;
use alloy_sol_types::SolEvent;
use std::fs;
use alloy_contract::{private::{Network, Transport}, ContractInstance, SolCallBuilder};
use ethers::{prelude::Abigen, providers::Middleware,types::{Address,Bytes,H256,U256}};
use eigensdk_types::operator;
use eigensdk_contracts_bindings::{OperatorStateRetriever::{self, operator_state_retriever, GetOperatorStateCall, GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall, Operator}, RegistryCoordinator, StakeRegistry::{self, stake_registry}};
use ethers_providers::{Provider, Http};
use serde_json;
use ethers_core::{abi::Abi, k256::elliptic_curve::rand_core::block};


const REGISTRY_COORDINATOR_PATH:&str = "../../../../crates/contracts/bindings/json/RegistryCoordinator.json";
const STAKE_REGISTRY_PATH:&str =  "../../../../crates/contracts/bindings/json/StakeRegistry.json";
const OPERATOR_STATE_RETRIEVER:&str = "../../../../crates/contracts/bindings/json/OperatorStateRetriever.json";

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
pub struct AvsRegistryChainReader {
    logger: Logger,
    bls_apk_registry_addr: Address,
    registry_coordinator_addr: Address,
    operator_state_retriever : Address,
    // operator_state_retriever:ethers::contract::Contract<M>,
    // stake_registry: ethers::contract::Contract<M>,
    eth_client: Provider<Http>,
}

trait AvsRegistryReader {
    fn get_quorum_count() -> Result<U8, String>;
}

impl AvsRegistryChainReader {
    fn new(
        logger : Logger,
        registry_coordinator_addr: Address,
        bls_apk_registry_addr: Address,
        operator_state_retriever : Address,
        // operator_state_retriever: ethers::contract::Contract<M>,
        // stake_registry:ethers::contract::Contract<M>,
        eth_client: Provider<Http>,
    ) -> Self {
        AvsRegistryChainReader {
            logger,
            bls_apk_registry_addr,
            registry_coordinator_addr,
            operator_state_retriever,
            // registry_coordinator:registry_coordinate,
            // operator_state_retriever,
            // stake_registry,
            eth_client,
        }
    }

    async fn build_avs_registry_chain_reader(&self,registry_coordinator_addr: Address,operator_state_retriever_addr:Address,eth_client: Provider<Http>,logger : Logger) ->Result<AvsRegistryChainReader,String>
    {
        let json_str = fs::read_to_string(REGISTRY_COORDINATOR_PATH).unwrap();
        let registry_coordinator_abi:Abi =  serde_json::from_str(&json_str).unwrap();
        let contract_registry_coordinator = ethers::contract::Contract::new(registry_coordinator_addr,registry_coordinator_abi,eth_client.clone().into());
    

        let bls_apk_registry_addr = contract_registry_coordinator.method::<_,Address>("blsApkRegistry",()).unwrap().call().await.expect("fail bls ");
        let stake_registry_addr = contract_registry_coordinator.method::<_,Address>("StakeRegistry",()).unwrap().call().await.expect("fail stake reader");

        let stake_json_str = fs::read_to_string(STAKE_REGISTRY_PATH).unwrap();

        let stake_registry_abi : Abi = serde_json::from_str(&stake_json_str).unwrap();
        let contract_stake_registry = ethers::contract::Contract::new(stake_registry_addr,stake_registry_abi,eth_client.clone().into());

        let operator_json_str = fs::read_to_string(OPERATOR_STATE_RETRIEVER).unwrap();
        let operator_state_retriever_abi :Abi = serde_json::from_str(&operator_json_str).unwrap();

        let contract_operator_state_retriever = ethers::contract::Contract::new(operator_state_retriever_addr,operator_state_retriever_abi,eth_client.clone().into());

        Ok(AvsRegistryChainReader{
            logger,
            bls_apk_registry_addr,
            registry_coordinator_addr,
            operator_state_retriever: operator_state_retriever_addr,
            // contract_registry_coordinator,
            eth_client

        })
    }

    async fn get_quorum_count(&self) ->Result<u8, String>{
        let json_str = fs::read_to_string(REGISTRY_COORDINATOR_PATH).unwrap();
        let registry_coordinator_abi:Abi =  serde_json::from_str(&json_str).unwrap();
        let contract_registry_coordinator = ethers::contract::Contract::new(self.registry_coordinator_addr,registry_coordinator_abi,self.eth_client.clone().into());
        
        let quorum_count = contract_registry_coordinator.method::<_,u8>("quorumCount",()).unwrap().call().await.expect("failed");

        Ok(quorum_count)

    }

    async fn get_operators_stake_in_quorums_at_block(&self, block_number : u32, quorum_numbers :Bytes) -> Result<Vec<Vec<Operator>>,String>{

        let operator_json_str = fs::read_to_string(OPERATOR_STATE_RETRIEVER).unwrap();
        let operator_state_retriever_abi :Abi = serde_json::from_str(&operator_json_str).unwrap();

        let contract_operator_state_retriever = operator_state_retriever::OperatorStateRetriever::new(self.operator_state_retriever,self.eth_client.clone().into());
        let res = contract_operator_state_retriever.get_operator_state(self.registry_coordinator_addr, quorum_numbers, block_number).call().await.unwrap();
        Ok(res)
    }

    async fn get_operators_stake_in_quorums_at_block_operator_id(&self, block_number : u32 ,operator_id : H256) -> Result<(U256,Vec<Vec<Operator>>),String>{

        let contract_operator_state_retriever = operator_state_retriever::OperatorStateRetriever::new(self.operator_state_retriever,self.eth_client.clone().into());
       let s = contract_operator_state_retriever.get_operator_state_with_registry_coordinator_and_operator_id(self.registry_coordinator_addr, operator_id.into(), block_number).call().await.unwrap();
        Ok(s)

    }

    async fn get_operators_stake_in_quorums_at_current_block(&self, quorum_numbers :Bytes) ->  Result<Vec<Vec<Operator>>,String>{
        let current_block_number = self.eth_client.get_block_number().await.expect("Failed to get current block number");

        if current_block_number > u32::MAX.into(){
             return Err("block number exceed maximum u32 value".to_string());
        }

        Ok(self.get_operators_stake_in_quorums_at_block(current_block_number.as_u64() as u32, quorum_numbers).await.expect("failed to get operaotrs stakr"))
    }


    async fn get_operator_addrs_in_quorums_at_current_block(&self, quorum_numbers :Bytes) -> Result<Vec<Vec<Address>>,String> {

        let current_block_number = self.eth_client.get_block_number().await.expect("Failed to get current block number");

        if current_block_number > u32::MAX.into(){
            return Err("block number exceed maximum u32 value".to_string());
        }

        let operator_stakes = self.get_operators_stake_in_quorums_at_block(current_block_number.as_u64() as u32,quorum_numbers).await.expect("Failed to get operators stake");

        let mut quorum_operators_addrs :Vec<Vec<Address>> = Vec::new();

        for quorum in operator_stakes.iter() {
            let mut operator_addrs : Vec<Address>  = Vec::new();

            for operator in  quorum.iter(){
                operator_addrs.push(operator.operator.clone());
            }

            quorum_operators_addrs.push(operator_addrs);
        }

        return Ok(quorum_operators_addrs)

    }

    async fn get_operators_stake_in_quorums_of_operator_at_block(&self, operator_id : H256, block_number : u32){

        let (quorum_bitmaps, operator_stakes) = self.get_operators_stake_in_quorums_at_block_operator_id(block_number,operator_id).await.unwrap();





    }



}
// #[test]
// fn test_binding_generation() {
    
//     generate_bindings("RegistryCoordinator","RegistryCoordinator.json", "../../../../crates/contracts/bindings");
//     generate_bindings("OperatorStateRetriever","OperatorStateRetriever.json", "../../../../crates/contracts/bindings");
//     generate_bindings("StakeRegistry","StakeRegistry.json", "../../../../crates/contracts/bindings");

// }


#[test]
fn test_build_avs_registry_chain_reader() {
    let log = Logger{};
    let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    let instance = AvsRegistryChainReader::new(log,Address::from_low_u64_be(23),Address::from_low_u64_be(544), Address::from_low_u64_be(34),provider.clone());
    let new_log = Logger{};
    let s =AvsRegistryChainReader::build_avs_registry_chain_reader(&instance,Address::from_low_u64_be(333), Address::from_low_u64_be(675),provider,new_log);
}

