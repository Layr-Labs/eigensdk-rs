use eigensdk_client_avsregistry::reader::AvsRegistryChainReader;
use eigensdk_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use std::collections::HashMap;
use eigensdk_types::operator::{self, OperatorAvsState};
use ethers_core::{k256::elliptic_curve::rand_core::block, types::{Bytes, U64}};
pub struct AvsRegistryServiceChainCaller {
    avs_registry: AvsRegistryChainReader,
    operators_info_service : OperatorInfoServiceInMemory
}


impl AvsRegistryServiceChainCaller{

    fn new(avs_registry: AvsRegistryChainReader, operators_info_service : OperatorInfoServiceInMemory) -> Self{

        Self { avs_registry, operators_info_service }

    }

    // pub async fn get_operators_avs_state_at_block(&self,block_num : u32,quorum_nums:Bytes ) {

    //     let operators_avs_state:HashMap<[u8;32],OperatorAvsState> = HashMap::new();

    //     let operators_stakes_in_quorums = self.avs_registry.get_operators_stake_in_quorums_at_block(block_num,quorum_nums.clone()).await.unwrap();

    //     if operators_stakes_in_quorums.len() != quorum_nums.len() {
    //         // throw error 
    //     }

    //     for (quorum_id,quorum_num) in quorum_nums.iter().enumerate(){

    //         for operator in &operators_stakes_in_quorums[quorum_id]{
    //             // let info = self.avs_registr
    //         }

    //     }


    // }

    // pub async fn get_operator_info(&self, operator_id: [u8;32]) {

    //     let operator_addr = self.avs_registry.get_operator_from_id(operator_id).await.unwrap();

    //     let info = self.operators_info_service


    // }



}