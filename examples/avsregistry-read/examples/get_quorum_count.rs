//! get_quorum_count
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

    let quorum_count = avs_registry.get_quorum_count().await.unwrap();

    println!("quorum count is :{:?}", quorum_count);
    Ok(())
}
