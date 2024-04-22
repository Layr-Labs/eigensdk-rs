use eigensdk_client_avsregistry::{
    reader::AvsRegistryChainReader, subscriber::AvsRegistryChainSubscriber,
};
use std::sync::Arc;

use eigensdk_types::operator::OperatorPubKeys;
use ethers_core::types::{Address, U256};
use ethers_providers::{Provider, Ws};
use std::collections::HashMap;

#[derive(Debug)]
pub struct OperatorPubKeysServiceInMemory {
    avs_registry_reader: AvsRegistryChainReader,
    avs_registry_subscriber: AvsRegistryChainSubscriber,
}

impl OperatorPubKeysServiceInMemory {
    pub async fn new(
        avs_registry_subscriber: AvsRegistryChainSubscriber,
        avs_registry_chain_reader: AvsRegistryChainReader,
        web_socket: Arc<Provider<Ws>>,
    ) -> Self {
        let filter = avs_registry_subscriber
            .get_new_pub_key_registration_filter(web_socket)
            .await;

        Self {
            avs_registry_reader: avs_registry_chain_reader,
            avs_registry_subscriber: avs_registry_subscriber,
        }
    }

    pub async fn query_past_registered_operator_events_and_fill_db(
        &self,
    ) -> HashMap<Address, OperatorPubKeys> {
        // Assuming ethers rs fetches data from first block . Have to validate this .
        let (operator_address, operator_pub_keys) = self
            .avs_registry_reader
            .query_existing_registered_operator_pub_keys(None, None)
            .await
            .unwrap();

        let mut map = HashMap::new();
        for (i, address) in operator_address.iter().enumerate() {
            map.insert(*address, operator_pub_keys[i].clone());
        }

        return map;
    }
}
