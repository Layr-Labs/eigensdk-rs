//! get operator from id
use alloy_primitives::FixedBytes;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_testing_utils::m2_holesky_constants::{
    BLS_APK_REGISTRY, OPERATOR_STATE_RETRIEVER, REGISTRY_COORDINATOR, STAKE_REGISTRY,
};
use eyre::Result;
use std::str::FromStr;

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
    let operator_id =
        FixedBytes::from_str("0xb31102e4cf235efcb84545cb656b039782755994835365d1cd11764ccb4f2fdd")
            .expect("invalid operator id ");
    let operator_address = avs_registry
        .get_operator_from_id(*operator_id)
        .await
        .unwrap();

    println!("operator address is  :{:?}", operator_address);
    Ok(())
}
