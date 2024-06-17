//! register operator in quorum with avs registry coordinator
use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
use eigen_testing_utils::m2_holesky_constants::{
    BLS_APK_REGISTRY, OPERATOR_STATE_RETRIEVER, REGISTRY_COORDINATOR, STAKE_REGISTRY,SERVICE_MANAGER_ADDRESS,SLASHER_ADDRESS,DELEGATION_MANAGER_ADDRESS,AVS_DIRECTORY_ADDRESS
};
use eigen_client_elcontracts::reader::ELChainReader;

#[tokio::main]
async fn main() {

  let holesky_provider = "https://holesky.drpc.org";
  let el_chain_reader:ELChainReader = ELChainReader::new(SLASHER_ADDRESS,DELEGATION_MANAGER_ADDRESS,AVS_DIRECTORY_ADDRESS,holesky_provider.to_string());
  let avs_registry_writer = AvsRegistryChainWriter::new(SERVICE_MANAGER_ADDRESS,REGISTRY_COORDINATOR,OPERATOR_STATE_RETRIEVER,STAKE_REGISTRY,BLS_APK_REGISTRY,el_chain_reader,holesky_provider.to_string());

    


}
