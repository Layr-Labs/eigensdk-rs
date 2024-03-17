use alloy_contract::{
    private::{Network, Transport},
    ContractInstance, SolCallBuilder,
};
use alloy_primitives::{U32, U8};
use alloy_sol_types::SolEvent;
use alloy_sol_types::{sol, SolCall, SolConstructor, SolInterface};
use eigensdk_client_eth::client::Client;
use eigensdk_contracts_bindings::{
    OperatorStateRetriever::{
        self, operator_state_retriever, GetOperatorStateCall,CheckSignaturesIndices,
        GetOperatorStateWithRegistryCoordinatorAndOperatorIdCall, Operator,
    },
    RegistryCoordinator::registry_coordinator,
    StakeRegistry::{self, stake_registry},
};
use eigensdk_logging::logger::Logger;
use eigensdk_types::{operator::{bitmap_to_quorum_ids,OperatorPubKeys}};
use ethers::{
    prelude::Abigen,
    providers::Middleware,
    types::{Address, Bytes, H256, U256},
};

use ethers_core::{abi::Abi, k256::elliptic_curve::rand_core::block,types::{Filter,FilterBlockOption,BlockNumber,ValueOrArray,Topic}};
use ethers_providers::{Http, Provider};
use num_bigint::BigInt;
use serde_json;
use std::collections::HashMap;
use std::fs;
const REGISTRY_COORDINATOR_PATH: &str =
    "../../../../crates/contracts/bindings/json/RegistryCoordinator.json";
const STAKE_REGISTRY_PATH: &str = "../../../../crates/contracts/bindings/json/StakeRegistry.json";
const OPERATOR_STATE_RETRIEVER: &str =
    "../../../../crates/contracts/bindings/json/OperatorStateRetriever.json";


    // cast sig-event "NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))"
    const NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE:H256 = H256([
        0xe3,0xfb,0x66,0x13,0xaf,0x2e,0x89,0x30,0xcf,0x85,0xd4,0x7f,0xcf,0x6d,0xb1,0x01,0x92,0x22,0x4a,0x64,0xc6,
        0xcb,0xe8,0x02,0x3e,0x0e,0xee,0x1b,0xa3,0x82,0x80,0x41
        ]);

fn generate_bindings(contract_name: &str, input_path: &str, output_path: &str) {
    let coontract: String =
        format!("../../../../crates/contracts/bindings/json/{input_path}").to_string();
    println!("path :{}", coontract);

    match Abigen::new(&contract_name, coontract) {
        Ok(v) => {
            println!("okoik");
            let _ = v
                .generate()
                .expect("failed to abigen")
                .write_to_file(format!("{output_path}/src/{contract_name}.rs"));
        }
        Err(e) => {
            println!("abigenerr{}", e);
        }
    }
}

#[derive(Debug)]
pub struct AvsRegistryChainReader {
    logger: Logger,
    bls_apk_registry_addr: Address,
    registry_coordinator_addr: Address,
    operator_state_retriever: Address,
    stake_registry_addr: Address,
    // operator_state_retriever:ethers::contract::Contract<M>,
    // stake_registry: ethers::contract::Contract<M>,
    eth_client: Provider<Http>,
}

trait AvsRegistryReader {
    fn get_quorum_count() -> Result<U8, String>;
}

impl AvsRegistryChainReader {
    fn new(
        logger: Logger,
        registry_coordinator_addr: Address,
        bls_apk_registry_addr: Address,
        operator_state_retriever: Address,
        stake_registry_addr: Address,
        // operator_state_retriever: ethers::contract::Contract<M>,
        // stake_registry:ethers::contract::Contract<M>,
        eth_client: Provider<Http>,
    ) -> Self {
        AvsRegistryChainReader {
            logger,
            bls_apk_registry_addr,
            registry_coordinator_addr,
            operator_state_retriever,
            stake_registry_addr,
            // registry_coordinator:registry_coordinate,
            // operator_state_retriever,
            // stake_registry,
            eth_client,
        }
    }

    async fn build_avs_registry_chain_reader(
        &self,
        registry_coordinator_addr: Address,
        operator_state_retriever_addr: Address,
        stake_registry_addr: Address,
        eth_client: Provider<Http>,
        logger: Logger,
    ) -> Result<AvsRegistryChainReader, String> {
        let json_str = fs::read_to_string(REGISTRY_COORDINATOR_PATH).unwrap();
        let registry_coordinator_abi: Abi = serde_json::from_str(&json_str).unwrap();
        let contract_registry_coordinator = ethers::contract::Contract::new(
            registry_coordinator_addr,
            registry_coordinator_abi,
            eth_client.clone().into(),
        );

        let bls_apk_registry_addr = contract_registry_coordinator
            .method::<_, Address>("blsApkRegistry", ())
            .unwrap()
            .call()
            .await
            .expect("fail bls ");

        let stake_json_str = fs::read_to_string(STAKE_REGISTRY_PATH).unwrap();

        let stake_registry_abi: Abi = serde_json::from_str(&stake_json_str).unwrap();
        let contract_stake_registry = ethers::contract::Contract::new(
            stake_registry_addr,
            stake_registry_abi,
            eth_client.clone().into(),
        );

        let operator_json_str = fs::read_to_string(OPERATOR_STATE_RETRIEVER).unwrap();
        let operator_state_retriever_abi: Abi = serde_json::from_str(&operator_json_str).unwrap();

        let contract_operator_state_retriever = ethers::contract::Contract::new(
            operator_state_retriever_addr,
            operator_state_retriever_abi,
            eth_client.clone().into(),
        );

        Ok(AvsRegistryChainReader {
            logger,
            bls_apk_registry_addr,
            registry_coordinator_addr,
            operator_state_retriever: operator_state_retriever_addr,
            stake_registry_addr,
            // contract_registry_coordinator,
            eth_client,
        })
    }

    async fn get_quorum_count(&self) -> Result<u8, String> {
        let json_str = fs::read_to_string(REGISTRY_COORDINATOR_PATH).unwrap();
        let registry_coordinator_abi: Abi = serde_json::from_str(&json_str).unwrap();
        let contract_registry_coordinator = ethers::contract::Contract::new(
            self.registry_coordinator_addr,
            registry_coordinator_abi,
            self.eth_client.clone().into(),
        );

        let quorum_count = contract_registry_coordinator
            .method::<_, u8>("quorumCount", ())
            .unwrap()
            .call()
            .await
            .expect("failed");

        Ok(quorum_count)
    }

    async fn get_operators_stake_in_quorums_at_block(
        &self,
        block_number: u32,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<Operator>>, String> {
        let operator_json_str = fs::read_to_string(OPERATOR_STATE_RETRIEVER).unwrap();
        let operator_state_retriever_abi: Abi = serde_json::from_str(&operator_json_str).unwrap();

        let contract_operator_state_retriever =
            operator_state_retriever::OperatorStateRetriever::new(
                self.operator_state_retriever,
                self.eth_client.clone().into(),
            );
        let res = contract_operator_state_retriever
            .get_operator_state(self.registry_coordinator_addr, quorum_numbers, block_number)
            .call()
            .await
            .unwrap();
        Ok(res)
    }

    async fn get_operators_stake_in_quorums_at_block_operator_id(
        &self,
        block_number: u32,
        operator_id: H256,
    ) -> Result<(U256, Vec<Vec<Operator>>), String> {
        let contract_operator_state_retriever =
            operator_state_retriever::OperatorStateRetriever::new(
                self.operator_state_retriever,
                self.eth_client.clone().into(),
            );
        let s = contract_operator_state_retriever
            .get_operator_state_with_registry_coordinator_and_operator_id(
                self.registry_coordinator_addr,
                operator_id.into(),
                block_number,
            )
            .call()
            .await
            .unwrap();
        Ok(s)
    }

    async fn get_operators_stake_in_quorums_at_current_block(
        &self,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<Operator>>, String> {
        let current_block_number = self
            .eth_client
            .get_block_number()
            .await
            .expect("Failed to get current block number");

        if current_block_number > u32::MAX.into() {
            return Err("block number exceed maximum u32 value".to_string());
        }

        Ok(self
            .get_operators_stake_in_quorums_at_block(
                current_block_number.as_u64() as u32,
                quorum_numbers,
            )
            .await
            .expect("failed to get operaotrs stakr"))
    }

    async fn get_operator_addrs_in_quorums_at_current_block(
        &self,
        quorum_numbers: Bytes,
    ) -> Result<Vec<Vec<Address>>, String> {
        let current_block_number = self
            .eth_client
            .get_block_number()
            .await
            .expect("Failed to get current block number");

        if current_block_number > u32::MAX.into() {
            return Err("block number exceed maximum u32 value".to_string());
        }

        let operator_stakes = self
            .get_operators_stake_in_quorums_at_block(
                current_block_number.as_u64() as u32,
                quorum_numbers,
            )
            .await
            .expect("Failed to get operators stake");

        let mut quorum_operators_addrs: Vec<Vec<Address>> = Vec::new();

        for quorum in operator_stakes.iter() {
            let mut operator_addrs: Vec<Address> = Vec::new();

            for operator in quorum.iter() {
                operator_addrs.push(operator.operator.clone());
            }

            quorum_operators_addrs.push(operator_addrs);
        }

        return Ok(quorum_operators_addrs);
    }

    async fn get_operators_stake_in_quorums_of_operator_at_block(
        &self,
        operator_id: H256,
        block_number: u32,
    ) -> Result<(Vec<u8>, Vec<Vec<Operator>>), String> {
        let (quorum_bitmaps, operator_stakes) = self
            .get_operators_stake_in_quorums_at_block_operator_id(block_number, operator_id)
            .await
            .unwrap();

        let quorums = bitmap_to_quorum_ids(quorum_bitmaps);

        let s = (quorums, operator_stakes);
        return Ok(s);
    }

    async fn get_operators_stake_in_quorums_of_operator_at_current_block(
        &self,
        operator_id: H256,
    ) -> Result<(Vec<u8>, Vec<Vec<Operator>>), String> {
        let current_block_number = self
            .eth_client
            .get_block_number()
            .await
            .expect("failed to get current block number ");
        if current_block_number > u32::MAX.into() {
            return Err("block number exceed maximum u32 value".to_string());
        }

        self.get_operators_stake_in_quorums_of_operator_at_block(
            operator_id,
            current_block_number.as_u64() as u32,
        )
        .await
    }

    async fn get_operator_stake_in_quorums_of_operator_at_current_block(&self, operator_id: H256) ->HashMap<u8, BigInt>{
        let registry_coordinator = registry_coordinator::RegistryCoordinator::new(
            self.registry_coordinator_addr,
            self.eth_client.clone().into(),
        );

        let quorum_bitmap = registry_coordinator
            .get_current_quorum_bitmap(operator_id.into())
            .call()
            .await
            .unwrap();
        let quorums = bitmap_to_quorum_ids(quorum_bitmap);

        let mut quorum_stakes: HashMap<u8, BigInt> = HashMap::new();
        let stake_registry = stake_registry::StakeRegistry::new(
            self.stake_registry_addr,
            self.eth_client.clone().into(),
        );
        for quorum in quorums.iter() {
            let stakes = stake_registry.get_current_stake(operator_id.into(), *quorum).call().await.unwrap();
            quorum_stakes.insert(*quorum,stakes.into());
        }
        quorum_stakes
    }

    async fn get_check_signatures_indices(&self, reference_block_number: u32, quorum_numbers: Vec<u8>,non_signer_operator_ids :Vec<[u8;32]> ) -> Result<CheckSignaturesIndices,String> {

        let mut non_signer_operator_ids_bytes:Vec<[u8;32]> = Vec::with_capacity(non_signer_operator_ids.len());
        let contract_operator_state_retriever = operator_state_retriever::OperatorStateRetriever::new(self.operator_state_retriever,self.eth_client.clone().into());

        let check_signature_indices = contract_operator_state_retriever.get_check_signatures_indices(self.registry_coordinator_addr, reference_block_number, quorum_numbers.into(), non_signer_operator_ids).call().await.expect("Failed to get check signature");

        Ok(check_signature_indices)

    }

    async fn get_operator_id(&self,operator_address : Address) -> Result<[u8;32],String> {
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(self.registry_coordinator_addr,self.eth_client.clone().into());

        let operator_id = contract_registry_coordinator.get_operator_id(operator_address).call().await.unwrap();

        return Ok(operator_id);

    }

    async fn get_operator_from_id(&self , operator_id : H256) -> Result<Address,String>{
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(self.registry_coordinator_addr,self.eth_client.clone().into());

        let operator_address = contract_registry_coordinator.get_operator_from_id(operator_id.into()).call().await.expect("Failed to get operator address");


        Ok(operator_address)

    }

    async fn is_operator_registered(&self,operator_address: Address) -> Result<bool, String> {
        let contract_registry_coordinator = registry_coordinator::RegistryCoordinator::new(self.registry_coordinator_addr,self.eth_client.clone().into());

        let operator_status = contract_registry_coordinator.get_operator_status(operator_address).call().await.unwrap();

        Ok(operator_status == 1) 
    }


    async fn query_existing_registered_operator_pub_keys(&self,start_block : BlockNumber, stop_block : BlockNumber )  {

        let block_option :FilterBlockOption  = FilterBlockOption::Range { from_block:Some(start_block), to_block: Some(stop_block) };

        let query = Filter{
            block_option,
            address: Some(ValueOrArray::Value(self.bls_apk_registry_addr)),
            topics: [Some(Topic::Value(Some(NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE))),None,None,None]
        };

        let logs  = self.eth_client.get_logs(&query).await.unwrap();

        let operator_addresses :Vec<Address> ;
        let operator_pub_keys :Vec<OperatorPubKeys>;

        for (i,v_log) in logs.iter().enumerate(){

            let operator_addr =Address::from_slice(&v_log.topics[i].as_bytes()[12..]);            
            operator_addresses.push(operator_addr);

            


        }

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
    let log = Logger {};
    let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    let instance = AvsRegistryChainReader::new(
        log,
        Address::from_low_u64_be(23),
        Address::from_low_u64_be(544),
        Address::from_low_u64_be(5445),
        Address::from_low_u64_be(34),
        provider.clone(),
    );
    let new_log = Logger {};
    let s = AvsRegistryChainReader::build_avs_registry_chain_reader(
        &instance,
        Address::from_low_u64_be(333),
        Address::from_low_u64_be(87),
        Address::from_low_u64_be(675),
        provider,
        new_log,
    );
}
