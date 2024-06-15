use alloy_primitives::Address;
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_types::Filter;
use alloy_transport_ws::WsConnect;
use anyhow::Result;
use eigen_client_avsregistry::{
    reader::AvsRegistryChainReader, subscriber::AvsRegistryChainSubscriber,
};
use eigen_types::operator::{operator_id_from_g1_pub_key, OperatorPubKeys};
use eigen_utils::binding::BLSApkRegistry::{self, G1Point, G2Point};
use futures_util::StreamExt;
use std::collections::HashMap;
use tokio::sync::{
    mpsc,
    mpsc::UnboundedSender,
    oneshot::{self, Sender},
};
#[derive(Debug)]
pub struct OperatorInfoServiceInMemory {
    avs_registry_reader: AvsRegistryChainReader,
    avs_registry_subscriber: AvsRegistryChainSubscriber,
    ws: String,
    pub_keys: UnboundedSender<OperatorsInfoMessage>,
}

#[derive(Debug)]
enum OperatorsInfoMessage {
    InsertOperatorInfo(Address, OperatorPubKeys),
    Remove(Address),
    Get(Address, Sender<Option<OperatorPubKeys>>),
}

impl OperatorInfoServiceInMemory {
    pub async fn new(
        avs_registry_subscriber: AvsRegistryChainSubscriber,
        avs_registry_chain_reader: AvsRegistryChainReader,
        web_socket: String,
    ) -> Self {
        let (pubkeys_tx, mut pubkeys_rx) = mpsc::unbounded_channel();
        let mut operator_info_data = HashMap::new();

        let mut operator_addr_to_id = HashMap::new();

        tokio::spawn(async move {
            while let Some(cmd) = pubkeys_rx.recv().await {
                match cmd {
                    OperatorsInfoMessage::InsertOperatorInfo(addr, keys) => {
                        operator_info_data.insert(addr, keys.clone());
                        let operator_id = operator_id_from_g1_pub_key(keys.g1_pub_key);
                        operator_addr_to_id.insert(addr, operator_id);
                    }
                    OperatorsInfoMessage::Remove(addr) => {
                        operator_info_data.remove(&addr);
                    }
                    OperatorsInfoMessage::Get(addr, responder) => {
                        let result = operator_info_data.get(&addr).cloned();
                        let _ = responder.send(result);
                    }
                }
            }
        });

        Self {
            avs_registry_reader: avs_registry_chain_reader,
            avs_registry_subscriber: avs_registry_subscriber,
            ws: web_socket,
            pub_keys: pubkeys_tx,
        }
    }

    pub async fn start_service(&self, start_block: u64, end_block: u64) -> Result<()> {
        // query past operator registrations
        self.query_past_registered_operator_events_and_fill_db(start_block, end_block)
            .await;

        let ws = WsConnect::new(&self.ws);
        let provider = ProviderBuilder::new().on_ws(ws).await?;
        let current_block_number = provider.get_block_number().await?;
        let filter = Filter::new()
            .event("NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))")
            .from_block(current_block_number);

        let subcription_new_operator_registration_stream = provider.subscribe_logs(&filter).await?;
        let mut stream = subcription_new_operator_registration_stream.into_stream();
        while let Some(log) = stream.next().await {
            let data = log
                .log_decode::<BLSApkRegistry::NewPubkeyRegistration>()
                .ok();

            if let Some(new_pub_key_event) = data {
                let event_data = new_pub_key_event.data();
                let operator_pub_key = OperatorPubKeys {
                    g1_pub_key: G1Point {
                        X: event_data.pubkeyG1.X,
                        Y: event_data.pubkeyG1.Y,
                    },
                    g2_pub_key: G2Point {
                        X: event_data.pubkeyG2.X,
                        Y: event_data.pubkeyG2.Y,
                    },
                };
                // send message
                let _ = self.pub_keys.send(OperatorsInfoMessage::InsertOperatorInfo(
                    event_data.operator,
                    operator_pub_key,
                ));
            }
        }
        Ok(())
    }

    pub async fn get_operator_info(&self, address: Address) -> Option<OperatorPubKeys> {
        let (responder_tx, responder_rx) = oneshot::channel();
        let _ = self
            .pub_keys
            .send(OperatorsInfoMessage::Get(address, responder_tx));
        responder_rx.await.unwrap_or(None)
    }

    pub async fn query_past_registered_operator_events_and_fill_db(
        &self,
        start_block: u64,
        end_block: u64,
    ) {
        let (operator_address, operator_pub_keys) = self
            .avs_registry_reader
            .query_existing_registered_operator_pub_keys(start_block, end_block)
            .await
            .unwrap();
        for (i, address) in operator_address.iter().enumerate() {
            let message =
                OperatorsInfoMessage::InsertOperatorInfo(*address, operator_pub_keys[i].clone());
            let _ = self.pub_keys.send(message);
        }
    }
}
