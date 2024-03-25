use crate::NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE;
use eigensdk_contracts_bindings::BLSApkRegistry::bls_apk_registry::{
    self, BLSApkRegistry, BLSApkRegistryEvents,
};
use eigensdk_logging::logger::Logger;
use ethers_core::types::{Address, Filter};
use ethers_providers::{Http, Middleware, Provider, SubscriptionStream, Ws};
use std::sync::Arc;
/// AvsRegistry Chain Subscriber struct
#[derive(Debug)]
pub struct AvsRegistryChainSubscriber {
    logger: Logger,
    bls_apk_registry: BLSApkRegistryEvents,
}

impl AvsRegistryChainSubscriber {
    fn new(bls_apk_registry: BLSApkRegistryEvents, logger: Logger) -> Self {
        return AvsRegistryChainSubscriber {
            logger: logger,
            bls_apk_registry: bls_apk_registry,
        };
    }

    fn build_avs_registry_chain_reader(
        bls_apk_registry_addr: Address,
        client: Provider<Http>,
    ) -> BLSApkRegistry<ethers_providers::Provider<Http>> {
        let bls_apk_reg =
            bls_apk_registry::BLSApkRegistry::new(bls_apk_registry_addr, client.into());

        return bls_apk_reg;
    }

    async fn subscribe_to_new_pub_key_registrations(&self, client: Provider<Ws>) {
        let provider = Arc::new(client.clone());
        let current_block_number = provider.clone().get_block_number().await.unwrap();

        let filter = Filter::new()
            .topic0(NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE)
            .from_block(current_block_number);

        let _ = (client.subscribe_logs(&filter).await.unwrap());
    }
}
