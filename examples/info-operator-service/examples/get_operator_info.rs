use alloy_primitives::address;
use eigen_client_avsregistry::{
    reader::AvsRegistryChainReader, subscriber::AvsRegistryChainSubscriber,
};
use eigen_services_operatorsinfo::operatorsinfo_inmemory::{OperatorInfoServiceInMemory};
use tokio_util::sync::CancellationToken;
use tokio::time::Duration;
use tokio::task;
const HOLESKY_PROVIDER: &str = "https:/localhost:8545";
const WS_HOLESKY_PROIVIDER: &str = "ws://localhost:8545";
use eigen_logging::get_test_logger;

#[tokio::main]
async fn main() {
    let avs_registry_chain_reader = AvsRegistryChainReader::new(
        get_test_logger().clone(),
        address!("53012C69A189cfA2D9d29eb6F19B32e0A2EA3490"),
        address!("B4baAfee917fb4449f5ec64804217bccE9f46C67"),
        HOLESKY_PROVIDER.to_string(),
    )
    .await
    .expect("failed to build avs registry chain reader");

    let operators_info = 
        OperatorInfoServiceInMemory::new(
            get_test_logger(),
            avs_registry_chain_reader,
            WS_HOLESKY_PROIVIDER.to_string(),
        )
        .await;

    let operators_info_clone = operators_info.clone();
    let cancellation_token :CancellationToken = CancellationToken::new();
    let token_clone = cancellation_token.clone();
    // start the service with a particular block range
    // from block : 0
    // to block : 0 means current block 
    task::spawn(async move {
        operators_info_clone.start_service(&token_clone,0, 0).await
    });


    register_operator(
        "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
        "12248929636257230549931416853095037629726205319386239410403476017439825112537",
    )
    .await;


    tokio::time::sleep(Duration::from_secs(2)).await;
    // send cancel token to stop the service
    cancellation_token.cancel();
    
    // query any operator info from their address 
    let res = operators_info
    .get_operator_info(address!("40d2c07fe83cf73df224f691cefd46257c4e5149"))
    .await.unwrap();

}
