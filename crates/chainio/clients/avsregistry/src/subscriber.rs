use crate::{error::AvsRegistryError, NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE};
use eigensdk_contract_bindings::BLSApkRegistry::bls_apk_registry::{
    self, BLSApkRegistry, BLSApkRegistryEvents,
};
use ethers_core::types::{Address, Filter};
use ethers_providers::{
    Http, Middleware, Provider, ProviderError, StreamExt, SubscriptionStream, Ws,
};
use futures::Future;
use std::pin::Pin;
use std::sync::Arc;

type MyStream<'a, T> =
    Pin<Box<dyn Future<Output = Result<SubscriptionStream<'static, Ws, T>, ProviderError>> + Send>>;

/// AvsRegistry Chain Subscriber struct
#[derive(Debug, Clone)]
pub struct AvsRegistryChainSubscriber {
    bls_apk_registry: BLSApkRegistryEvents,
}

impl AvsRegistryChainSubscriber {
    fn new(bls_apk_registry: BLSApkRegistryEvents) -> Self {
        return AvsRegistryChainSubscriber {
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

    pub async fn get_new_pub_key_registration_filter<'a>(
        &self,
        client: Arc<Provider<Ws>>,
    ) -> Filter {
        let current_block_number = client.get_block_number().await.unwrap();

        let filter = Filter::new()
            .topic0(NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE)
            .from_block(current_block_number);
        filter
    }
}
