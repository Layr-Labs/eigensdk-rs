//! get operators stake in quorums at block
use alloy_primitives::{hex::FromHex, Bytes};
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
    let block_num = 1741955;
    let operators_state = avs_registry
        .get_operators_stake_in_quorums_at_block(
            block_num,
            Bytes::from_hex("0x00").expect("failed to generate bytes"),
        )
        .await
        .unwrap();

    println!(
        "operator state at block : {:?} is {:?}",
        block_num, operators_state
    );

    Ok(())
}
