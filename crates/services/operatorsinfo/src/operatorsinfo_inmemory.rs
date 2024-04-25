use eigensdk_client_avsregistry::{
    reader::AvsRegistryChainReader, subscriber::AvsRegistryChainSubscriber,
};
use eigensdk_contract_bindings::BLSApkRegistry::BLSApkRegistryEvents;
use ethers::contract::EthLogDecode;
use std::sync::Arc;
use ethers_providers::StreamExt;
use eigensdk_types::operator::{OperatorPubKeys,operator_id_from_g1_pub_key};
use ethers_core::{types::Address,abi::RawLog};
use ethers_providers::{Middleware, Provider, Ws};
use std::collections::HashMap;
use tokio::sync::{mpsc, mpsc::UnboundedSender, oneshot::{Sender,self}};

#[derive(Debug)]
pub struct OperatorInfoServiceInMemory {
    avs_registry_reader: AvsRegistryChainReader,
    avs_registry_subscriber: AvsRegistryChainSubscriber,
    web_socket: Arc<Provider<Ws>>,
    pub_keys: UnboundedSender<OperatorsInfoMessage>,
}

#[derive(Debug)]
enum OperatorsInfoMessage {
    InsertOperatorInfo(Address,OperatorPubKeys),
    Remove(Address),
    Get(Address,Sender<Option<OperatorPubKeys>>)
}

impl OperatorInfoServiceInMemory {
    pub async fn new(
        avs_registry_subscriber: AvsRegistryChainSubscriber,
        avs_registry_chain_reader: AvsRegistryChainReader,
        web_socket: Arc<Provider<Ws>>,
    ) -> Self {
        let (pubkeys_tx ,mut pubkeys_rx ) = mpsc::unbounded_channel();

        let mut operator_info_data = HashMap::new();

        let mut operator_addr_to_id = HashMap::new();

        tokio::spawn( async move{

            while let Some(cmd) = pubkeys_rx.recv().await {
                match cmd {
                    OperatorsInfoMessage::InsertOperatorInfo(addr, keys) => {
                        operator_info_data.insert(addr, keys.clone());
                        let operator_id =operator_id_from_g1_pub_key( keys.g1_pub_key  );
                        operator_addr_to_id.insert(addr,operator_id);
                    },
                    OperatorsInfoMessage::Remove(addr) => {
                        operator_info_data.remove(&addr);
                    },
                    OperatorsInfoMessage::Get(addr,responder) => {
                        let result = operator_info_data.get(&addr).cloned();
                        let _ = responder.send(result );
                    }
                }
            }

        });


        Self {
            avs_registry_reader: avs_registry_chain_reader,
            avs_registry_subscriber: avs_registry_subscriber,
            web_socket,
            pub_keys:pubkeys_tx,
            // operator_addr_to_id:operator_addr_to_id_tx,
            // socket_dict:socket_dict_tx 
        }
    }

    pub async fn start_service(&self) {
      
        // query past operator registrations
        self.query_past_registered_operator_events_and_fill_db().await;

        let filter = self.avs_registry_subscriber
        .get_new_pub_key_registration_filter(self.web_socket.clone())
        .await;

        let mut subcription_new_operator_registration_stream = self.web_socket.subscribe_logs(&filter).await.unwrap();

        while let Some(log) = subcription_new_operator_registration_stream.next().await{

            let data = BLSApkRegistryEvents::decode_log(&RawLog::from(log)).unwrap();
            
            match data {
                BLSApkRegistryEvents::NewPubkeyRegistrationFilter(pubkeyreg)=>{
                    let operator_pub_key = OperatorPubKeys {
                        g1_pub_key:pubkeyreg.pubkey_g1,
                        g2_pub_key: pubkeyreg.pubkey_g2
                    };
                    // send message 
                    let _ = self.pub_keys.send(OperatorsInfoMessage::InsertOperatorInfo(pubkeyreg.operator,operator_pub_key));
                },
                _ =>{
                    
                }
            }
            
        }

    }

    pub async fn get_operator_info(&self, address: Address) -> Option<OperatorPubKeys> {
        let (responder_tx, responder_rx) = oneshot::channel();
        let _ = self.pub_keys.send(OperatorsInfoMessage::Get(address, responder_tx));
        responder_rx.await.unwrap_or(None)
    }

    pub async fn query_past_registered_operator_events_and_fill_db(
        &self,
    ) {
        // Assuming ethers rs fetches data from first block . Have to validate this .
        let (operator_address, operator_pub_keys) = self
            .avs_registry_reader
            .query_existing_registered_operator_pub_keys(None, None)
            .await
            .unwrap();

        for (i, address) in operator_address.iter().enumerate() {
            // let mut pub_keys  = map.lock().unwrap();
            let message = OperatorsInfoMessage::InsertOperatorInfo(*address,operator_pub_keys[i].clone());
            let _ = self.pub_keys.send(message);
        }

    }


}
 