use eigensdk_client_avsregistry::reader::AvsRegistryChainReader;
use eigensdk_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use std::collections::HashMap;
use eigensdk_types::operator::OperatorAvsState;
pub struct AvsRegistryServiceChainCaller {
    avs_registry: AvsRegistryChainReader,
    operators_info_service : OperatorInfoServiceInMemory
}


impl AvsRegistryServiceChainCaller{

    fn new(avs_registry: AvsRegistryChainReader, operators_info_service : OperatorInfoServiceInMemory) -> Self{

        Self { avs_registry, operators_info_service }

    }

    pub async fn get_operators_avs_state_at_block() {

        let operators_avs_state:HashMap<[u8;32],OperatorAvsState> = HashMap::new();


    }


}