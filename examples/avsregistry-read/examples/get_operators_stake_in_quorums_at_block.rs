//! get operators stake in quorums at block
use alloy_primitives::{hex::FromHex, Bytes};
use eigen_client_avsregistry::reader::{AvsRegistryChainReader, AvsRegistryReader};
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
    let block_num = 1741955;
    let _operators_state = avs_registry
        .get_operators_stake_in_quorums_at_block(
            block_num,
            Bytes::from_hex("0x00").expect("failed to generate bytes"),
        )
        .await?;

    Ok(())
}
