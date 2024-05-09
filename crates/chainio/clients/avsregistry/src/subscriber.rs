use alloy_sol_types::sol;
sol!(
    #[allow(missing_docs)]
    #[derive(Debug)]
    #[sol(rpc)]
    BLSApkRegistry,
    "../../../../crates/contracts/bindings/utils/json/BLSApkRegistry.json"
);
use alloy_network::Ethereum;
use alloy_primitives::Address;
use alloy_provider::{
    fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
    RootProvider,
};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::Filter;
use alloy_transport::BoxTransport;
use BLSApkRegistry::{BLSApkRegistryEvents, BLSApkRegistryInstance};
/// AvsRegistry Chain Subscriber struct
#[derive(Debug)]
pub struct AvsRegistryChainSubscriber {
    bls_apk_registry: BLSApkRegistryEvents,
    provider: String,
}

impl AvsRegistryChainSubscriber {
    fn new(bls_apk_registry: BLSApkRegistryEvents, provider: String) -> Self {
        return AvsRegistryChainSubscriber {
            bls_apk_registry: bls_apk_registry,
            provider,
        };
    }

    async fn build_avs_registry_chain_reader(
        &self,
        bls_apk_registry_addr: Address,
        provider: String,
    ) -> Result<
        BLSApkRegistryInstance<
            BoxTransport,
            FillProvider<
                JoinFill<
                    JoinFill<JoinFill<alloy_provider::Identity, GasFiller>, NonceFiller>,
                    ChainIdFiller,
                >,
                RootProvider<BoxTransport>,
                BoxTransport,
                Ethereum,
            >,
        >,
        Box<dyn std::error::Error>,
    > {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_builtin(&self.provider)
            .await?;
        let bls_apk_reg = BLSApkRegistry::new(bls_apk_registry_addr, provider);

        return Ok(bls_apk_reg);
    }

    pub async fn get_new_pub_key_registration_filter<'a>(&self) -> Filter {
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .on_builtin(&self.provider)
            .await?;
        let current_block_number = provider.get_block_number().await.unwrap();

        let filter = Filter::new()
            .event_signature(
                "NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))".into(),
            )
            .from_block(current_block_number);
        filter
    }
}
