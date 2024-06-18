//! get operator id
use alloy_primitives::{address, Address};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_testing_utils::m2_holesky_constants::{
    BLS_APK_REGISTRY, OPERATOR_STATE_RETRIEVER, REGISTRY_COORDINATOR, STAKE_REGISTRY,
};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let holesky_provider = "https://holesky.drpc.org";
    let avs_registry = AvsRegistryChainReader::new(
        REGISTRY_COORDINATOR,
        BLS_APK_REGISTRY,
        OPERATOR_STATE_RETRIEVER,
        STAKE_REGISTRY,
        holesky_provider.to_string(),
    );
    let operator: Address = address!("1D79000206BAFfaE662fFCdba1C2a6176d14dF48");
    let operator_id = avs_registry.get_operator_id(operator).await.unwrap();

    println!("operator id is  :{:?}", operator_id);
    Ok(())
}
