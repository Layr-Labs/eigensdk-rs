//! get operators stake in quorums at current block
use alloy_primitives::FixedBytes;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
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
    let operators_state = avs_registry
        .get_operator_stake_in_quorums_of_operator_at_current_block(
            FixedBytes::from_str(
                "0x9bdde6f82077712c6e1c9aa8e7fca47529effb948faafa1fa21aebd343fc7fec",
            )
            .expect("wrong operator id"),
        )
        .await?;

    println!("operator state at current block is {:?}", operators_state);

    Ok(())
}
