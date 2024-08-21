use alloy_primitives::{Address, FixedBytes};
use alloy_provider::{Provider, ProviderBuilder, WsConnect};
use alloy_rpc_types::Filter;
use anyhow::{Context, Result};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::{
    alloy_registry_g1_point_to_g1_affine, alloy_registry_g2_point_to_g2_affine, BlsG1Point,
    BlsG2Point,
};
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::{operator_id_from_g1_pub_key, OperatorPubKeys};
use eigen_utils::{
    binding::BLSApkRegistry::{self, G1Point, G2Point},
    get_ws_provider, NEW_PUBKEY_REGISTRATION_EVENT,
};
use futures_util::StreamExt;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::{
    mpsc,
    mpsc::UnboundedSender,
    oneshot::{self, Sender},
    RwLock,
};
#[derive(Debug, Clone)]
pub struct OperatorInfoServiceInMemory {
    logger: SharedLogger,
    avs_registry_reader: AvsRegistryChainReader,
    ws: String,
    pub_keys: UnboundedSender<OperatorsInfoMessage>,
}

#[derive(Debug, Clone)]
struct OperatorState {
    operator_info_data: Arc<RwLock<HashMap<Address, OperatorPubKeys>>>,
    operator_addr_to_id: Arc<RwLock<HashMap<Address, FixedBytes<32>>>>,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum OperatorInfoServiceError {
    #[error("failed to retrieve operator info")]
    OperatorInfoRetrievalError,
    #[error("operator not found")]
    OperatorNotFound,
    #[error("channel was closed")]
    ChannelClosed,
    #[error("error sending to channel")]
    ChannelError,
    #[error("websocket connection failed")]
    WebSocketConnectionError,
}

#[derive(Debug)]
enum OperatorsInfoMessage {
    InsertOperatorInfo(Address, Box<OperatorPubKeys>),
    Remove(Address),
    Get(
        Address,
        Sender<Result<Option<OperatorPubKeys>, OperatorInfoServiceError>>,
    ),
}

impl OperatorInfoServiceInMemory {
    pub async fn new(
        logger: SharedLogger,
        avs_registry_chain_reader: AvsRegistryChainReader,
        web_socket: String,
    ) -> Self {
        let (pubkeys_tx, mut pubkeys_rx) = mpsc::unbounded_channel();
        let operator_state = OperatorState {
            operator_info_data: Arc::new(RwLock::new(HashMap::new())),
            operator_addr_to_id: Arc::new(RwLock::new(HashMap::new())),
        };

        tokio::spawn({
            let operator_state = operator_state.clone();
            async move {
                while let Some(cmd) = pubkeys_rx.recv().await {
                    match cmd {
                        OperatorsInfoMessage::InsertOperatorInfo(addr, keys) => {
                            let mut data = operator_state.operator_info_data.write().await;
                            data.insert(addr, *keys.clone());
                            let operator_id = operator_id_from_g1_pub_key(keys.g1_pub_key);
                            let mut id_map = operator_state.operator_addr_to_id.write().await;
                            id_map.insert(addr, alloy_primitives::FixedBytes(operator_id));
                        }
                        OperatorsInfoMessage::Remove(addr) => {
                            let mut data = operator_state.operator_info_data.write().await;
                            data.remove(&addr);
                        }
                        OperatorsInfoMessage::Get(addr, responder) => {
                            let data = operator_state.operator_info_data.read().await;
                            let result = data.get(&addr).cloned();
                            let _ = responder.send(Ok(result)).expect("Failed to send response");
                        }
                    }
                }
            }
        });

        Self {
            logger,
            avs_registry_reader: avs_registry_chain_reader,
            ws: web_socket,
            pub_keys: pubkeys_tx,
        }
    }

    pub async fn start_service(
        &self,
        start_block: u64,
        end_block: u64,
        shutdown_rx: tokio::sync::watch::Receiver<()>,
    ) -> Result<()> {
        // Query past operator registrations
        self.query_past_registered_operator_events_and_fill_db(start_block, end_block)
            .await?;

        let provider = get_ws_provider(&self.ws).await?;
        let current_block_number = provider.get_block_number().await?;
        let filter = Filter::new()
            .event(NEW_PUBKEY_REGISTRATION_EVENT)
            .from_block(current_block_number);

        let subcription_new_operator_registration_stream = provider.subscribe_logs(&filter).await?;
        let mut stream = subcription_new_operator_registration_stream.into_stream();

        let shutdown_rx = shutdown_rx.clone();
        let pub_keys = self.pub_keys.clone();
        let self_clone = self.clone();
        tokio::spawn(async move {
            while let Some(log) = stream.next().await {
                if shutdown_rx.has_changed().unwrap_or(false) {
                    self_clone.logger.info(
                        "shutdown of operators info service ",
                        "eigen-services-operatorsinfo.start_service",
                    );
                    break;
                }

                let data = log
                    .log_decode::<BLSApkRegistry::NewPubkeyRegistration>()
                    .ok();

                if let Some(new_pub_key_event) = data {
                    let event_data = new_pub_key_event.data();
                    let operator_pub_key = OperatorPubKeys {
                        g1_pub_key: BlsG1Point::new(alloy_registry_g1_point_to_g1_affine(
                            G1Point {
                                X: event_data.pubkeyG1.X,
                                Y: event_data.pubkeyG1.Y,
                            },
                        )),
                        g2_pub_key: BlsG2Point::new(alloy_registry_g2_point_to_g2_affine(
                            G2Point {
                                X: event_data.pubkeyG2.X,
                                Y: event_data.pubkeyG2.Y,
                            },
                        )),
                    };
                    // Send message

                    self_clone.logger.debug(
                        &format!(
                            "New pub key found  operator_address : {:?} , operator_pub_keys : {:?}",
                            event_data.operator, operator_pub_key
                        ),
                        "eigen-services-operatorsinfo.start_service",
                    );

                    let _ = pub_keys.send(OperatorsInfoMessage::InsertOperatorInfo(
                        event_data.operator,
                        Box::new(operator_pub_key),
                    ));
                }
            }
        });

        Ok(())
    }

    pub async fn get_operator_info(
        &self,
        address: Address,
    ) -> Result<Option<OperatorPubKeys>, OperatorInfoServiceError> {
        let (responder_tx, responder_rx) = oneshot::channel();
        let _ = self
            .pub_keys
            .send(OperatorsInfoMessage::Get(address, responder_tx))
            .map_err(|_| OperatorInfoServiceError::ChannelClosed)?;

        responder_rx
            .await
            .map_err(|_| OperatorInfoServiceError::ChannelClosed)?
    }

    pub async fn query_past_registered_operator_events_and_fill_db(
        &self,
        start_block: u64,
        end_block: u64,
    ) -> Result<()> {
        println!("eee");
        let (operator_address, operator_pub_keys) = self
            .avs_registry_reader
            .query_existing_registered_operator_pub_keys(start_block, end_block, self.ws.clone())
            .await?;
        for (i, address) in operator_address.iter().enumerate() {
            let message = OperatorsInfoMessage::InsertOperatorInfo(
                *address,
                Box::new(operator_pub_keys[i].clone()),
            );
            self.logger.debug(
                &format!(
                    "New pub key found  operator_address : {:?} , operator_pub_keys : {:?}",
                    operator_address, operator_pub_keys
                ),
                "eigen-services-operatorsinfo.query_past_registered_operator_events_and_fill_db",
            );
            let _ = self.pub_keys.send(message);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use alloy_primitives::address;
    use eigen_logging::get_test_logger;
    use eigen_testing_utils::anvil_constants::{
        get_operator_state_retriever_address, get_registry_coordinator_address,
    };
    use eigen_testing_utils::m2_holesky_constants::{
        OPERATOR_STATE_RETRIEVER, REGISTRY_COORDINATOR,
    };
    use std::env;
    use tokio::sync::watch;

    #[tokio::test]
    async fn test_query_past_registered_operator_events_and_fill_db() {
        let websocket_url_holesky = env::var("HOLESKY_WS_URL").expect("HOLEESKY_WS_URL not set");
        let http_url_holesky = env::var("HOLESKY_HTTP_URL").expect("HOLESKY_HTTP_URL not set");
        let anvil_ws_url = "ws://localhost:8545";
        let anvil_http_url = "http://localhost:8545";
        let test_logger = get_test_logger();
        println!("rrr");

        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            test_logger.clone(),
            get_registry_coordinator_address().await,
            get_operator_state_retriever_address().await,
            anvil_http_url.to_string(),
        )
        .await
        .unwrap();

        let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
            test_logger.clone(),
            avs_registry_chain_reader,
            anvil_ws_url.to_string(),
        )
        .await;

        let _ = operators_info_service_in_memory
            .query_past_registered_operator_events_and_fill_db(0, 100)
            .await;
        // Give some time for the background task to process the messages
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // let address = address!("385b3b04126b221b09ad68fd55dee74965e9be8b");
        // let operator_info = operators_info_service_in_memory
        //     .get_operator_info(address)
        //     .await;
        // assert!(operator_info.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_start_service() {
        let websocket_url_holesky = env::var("HOLESKY_WS_URL").expect("HOLEESKY_WS_URL not set");
        let http_url_holesky = env::var("HOLESKY_HTTP_URL").expect("HOLESKY_HTTP_URL not set");
        let test_logger = get_test_logger();
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            test_logger.clone(),
            REGISTRY_COORDINATOR,
            OPERATOR_STATE_RETRIEVER,
            http_url_holesky.clone(),
        )
        .await
        .unwrap();
        let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
            test_logger.clone(),
            avs_registry_chain_reader,
            websocket_url_holesky.clone(),
        )
        .await;

        let (shutdown_tx, shutdown_rx) = watch::channel(());

        // Use a timeout to ensure the test does not run indefinitely
        let _ = tokio::spawn({
            async move {
                let shutdown_rx = shutdown_rx.clone();
                operators_info_service_in_memory
                    .start_service(2019065, 2039045, shutdown_rx)
                    .await
                    .unwrap();
            }
        });

        // Wait some time to simulate some operations
        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

        // // Send the shutdown signal
        let _ = shutdown_tx.send(());

        // TODO (supernova): Call Register Operator to emit the NewPubkeyRegistration event , so the subscriber catches it.
        // Blocked : https://github.com/Layr-Labs/eigensdk-rs/issues/49
        // // Wait for the service to finish
        // let _ = service_handle.await;
    }
}
