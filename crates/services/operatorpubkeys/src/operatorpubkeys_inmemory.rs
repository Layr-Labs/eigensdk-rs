use eigensdk_client_avsregistry::{
    reader::AvsRegistryChainReader, subscriber::AvsRegistryChainSubscriber,
};
use eigensdk_contract_bindings::BLSApkRegistry::BLSApkRegistryEvents;
use ethers::contract::EthLogDecode;
use eigensdk_crypto_bls::attestation::{G1Point,G2Point};
use std::sync::Arc;
use ethers_providers::StreamExt;
use eigensdk_types::operator::OperatorPubKeys;
use ethers_core::{types::{Address},abi::RawLog};
use ethers_providers::{Middleware, Provider, Ws};
use std::collections::HashMap;
use eigensdk_crypto_bn254::utils::u256_to_bigint256;
#[derive(Debug)]
pub struct OperatorPubKeysServiceInMemory {
    avs_registry_reader: AvsRegistryChainReader,
    avs_registry_subscriber: AvsRegistryChainSubscriber,
    web_socket: Arc<Provider<Ws>>,
}

impl OperatorPubKeysServiceInMemory {
    pub async fn new(
        avs_registry_subscriber: AvsRegistryChainSubscriber,
        avs_registry_chain_reader: AvsRegistryChainReader,
        web_socket: Arc<Provider<Ws>>,
    ) -> Self {
        

        Self {
            avs_registry_reader: avs_registry_chain_reader,
            avs_registry_subscriber: avs_registry_subscriber,
            web_socket
        }
    }

    pub async fn start_service(&self) {
        let  mut map:HashMap<Address,(OperatorPubKeys)> = HashMap::new();
        // query past operator registratinos and store in memory 
       map = self.query_past_registered_operator_events_and_fill_db(map).await;

        let filter = self.avs_registry_subscriber
        .get_new_pub_key_registration_filter(self.web_socket.clone())
        .await;

        let mut subcription_new_operator_registration_stream = self.web_socket.subscribe_logs(&filter).await.unwrap();

        while let Some(log) = subcription_new_operator_registration_stream.next().await{

            let data = BLSApkRegistryEvents::decode_log(&RawLog::from(log)).unwrap();
            
            match data {
                BLSApkRegistryEvents::NewPubkeyRegistrationFilter(pubkeyreg)=>{
                    let operator_pub_key = OperatorPubKeys {
                        g1_pub_key: G1Point::new(
                            u256_to_bigint256(pubkeyreg.pubkey_g1.x),
                            u256_to_bigint256(pubkeyreg.pubkey_g1.y),
                        ),
                        g2_pub_key: G2Point::new(
                            (
                                u256_to_bigint256(pubkeyreg.pubkey_g2.x[0]),
                                u256_to_bigint256(pubkeyreg.pubkey_g2.x[1]),
                            ),
                            (
                                u256_to_bigint256(pubkeyreg.pubkey_g2.y[0]),
                                u256_to_bigint256(pubkeyreg.pubkey_g2.y[1]),
                            ),
                        ),
                    };
                    map.insert(pubkeyreg.operator,operator_pub_key);
                },
                _ =>{
                    
                }
            }
            
        }

    }


    pub async fn query_past_registered_operator_events_and_fill_db(
        &self,
        mut map:HashMap<Address,OperatorPubKeys>
    ) -> HashMap<Address, OperatorPubKeys> {
        // Assuming ethers rs fetches data from first block . Have to validate this .
        let (operator_address, operator_pub_keys) = self
            .avs_registry_reader
            .query_existing_registered_operator_pub_keys(None, None)
            .await
            .unwrap();

        for (i, address) in operator_address.iter().enumerate() {
            map.insert(*address, operator_pub_keys[i].clone());
        }

        return map;
    }
}
