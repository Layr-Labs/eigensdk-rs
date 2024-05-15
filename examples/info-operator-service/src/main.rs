use alloy_primitives::address;
use eigensdk_client_avsregistry::{
    reader::AvsRegistryChainReader, subscriber::AvsRegistryChainSubscriber,
};
use eigensdk_services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;
use tokio::time::{self, Duration};

const HOLESKY_PROVIDER: &str = "https://ethereum-holesky.blockpi.network/v1/rpc/public";
const WS_HOLESKY_PROIVIDER: &str = "wss://holesky.drpc.org";

#[tokio::main]
async fn main() {
    let avs_registry_chain_reader = AvsRegistryChainReader::new(
        address!("53012C69A189cfA2D9d29eb6F19B32e0A2EA3490"),
        address!("066cF95c1bf0927124DFB8B02B401bc23A79730D"),
        address!("B4baAfee917fb4449f5ec64804217bccE9f46C67"),
        address!("BDACD5998989Eec814ac7A0f0f6596088AA2a270"),
        HOLESKY_PROVIDER.to_string(),
    );
    let avs_registry_subscriber = AvsRegistryChainSubscriber::new(WS_HOLESKY_PROIVIDER.to_string());

    let operators_info = Arc::new(Mutex::new(
        OperatorInfoServiceInMemory::new(
            avs_registry_subscriber,
            avs_registry_chain_reader,
            WS_HOLESKY_PROIVIDER.to_string(),
        )
        .await,
    ));
    let operators_info_clone = Arc::clone(&operators_info);
    // start the service with a particular block range
    // from block : 1536406
    // to block : 0 means current block , else normal
    task::spawn(async move {
        let operators_info = operators_info_clone.lock().await;
        operators_info.start_service(1536406, 0).await
    });

    // Do whatever in this loop. We are getting the operator info , and re entering after 60 seconds
    // indefinitely. You can always break or run it as per your preference.
    loop {
        println!("entered loop");
        let info = operators_info.lock().await;
        // https://holesky.etherscan.io/tx/0xa5e239184bb8b3340a2ea2d73f6ef394663c76a7313e5b1e8d886f2ae0f71f1f
        let res = info
            .get_operator_info(address!("40d2c07fe83cf73df224f691cefd46257c4e5149"))
            .await;
        match res {
            Some(operator_pub_keys) => {
                println!("operator pub keys : {:?}", operator_pub_keys);
            }
            None => {}
        };
        drop(info); // Explicitly drop the lock to release it
        time::sleep(Duration::from_secs(60)).await; // Adjust the duration as needed
    }
}
