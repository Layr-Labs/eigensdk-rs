//! get operator from id
use alloy_primitives::FixedBytes;
use eigen_client_avsregistry::reader::{AvsRegistryChainReader, AvsRegistryReader};
use eigen_logging::get_test_logger;
use eigen_testing_utils::m2_holesky_constants::{OPERATOR_STATE_RETRIEVER, REGISTRY_COORDINATOR};
use eyre::Result;
use std::str::FromStr;

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
    let operator_id =
        FixedBytes::from_str("0xb31102e4cf235efcb84545cb656b039782755994835365d1cd11764ccb4f2fdd")
            .expect("invalid operator id ");
    let operator_address = avs_registry.get_operator_from_id(*operator_id).await?;

    println!("operator address is  :{:?}", operator_address);
    Ok(())
}
