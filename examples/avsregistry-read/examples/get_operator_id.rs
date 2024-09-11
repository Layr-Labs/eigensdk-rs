//! get operator id
use alloy_primitives::{address, Address};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_logging::get_test_logger;
use eigen_testing_utils::m2_holesky_constants::{OPERATOR_STATE_RETRIEVER, REGISTRY_COORDINATOR};
use eyre::Result;

#[tokio::main]
#[allow(clippy::expect_used)]
async fn main() -> Result<()> {
    let holesky_provider = "https://holesky.drpc.org";
    let avs_registry = AvsRegistryChainReader::new(
        get_test_logger().clone(),
        REGISTRY_COORDINATOR,
        OPERATOR_STATE_RETRIEVER,
        holesky_provider.to_string(),
    )
    .await
    .expect("failed to build avs registry chain reader");

    let operator: Address = address!("1D79000206BAFfaE662fFCdba1C2a6176d14dF48");
    let operator_id = avs_registry.get_operator_id(operator).await?;

    println!("operator id is  :{:?}", operator_id);
    Ok(())
}
