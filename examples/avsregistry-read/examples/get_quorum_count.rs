//! get_quorum_count
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_logging::init_logger;
use eigen_testing_utils::m2_holesky_constants::{OPERATOR_STATE_RETRIEVER, REGISTRY_COORDINATOR};
use eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let holesky_provider = "https://holesky.drpc.org";
    let avs_registry = AvsRegistryChainReader::new(
        init_logger().clone(),
        REGISTRY_COORDINATOR,
        OPERATOR_STATE_RETRIEVER,
        holesky_provider.to_string(),
    )
    .await
    .expect("failed to build avs registry chain reader");

    let quorum_count = avs_registry.get_quorum_count().await.unwrap();

    println!("quorum count is :{:?}", quorum_count);
    Ok(())
}
