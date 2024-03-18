use crate::NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE;
use eigensdk_contracts_bindings::BLSApkRegistry::bls_apk_registry::{
    self, BLSApkRegistry, BLSApkRegistryEvents, NewPubkeyRegistrationFilter,
};
use eigensdk_logging::logger::Logger;
use ethers::contract::Contract;
use ethers_core::types::{Address, Filter};
use ethers_providers::{Http, Middleware, Provider, StreamExt, Ws};

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
        let current_block_number = client.clone().get_block_number().await.unwrap();
        // let filter = Contract::event_of_type::<NewPubkeyRegistrationFilter>(client.into()).from_block(current_block_number);

        let filter = Filter::new()
            .topic0(NEW_BLS_APK_REGISTRATION_EVENT_SIGNATURE)
            .from_block(current_block_number);

        let mut stream = client.subscribe_logs(&filter).await.unwrap();
    }
}
