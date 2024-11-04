use alloy::providers::Provider;
use alloy::rpc::types::Filter;
use alloy_primitives::{Address, FixedBytes};
use async_trait::async_trait;
use eigen_client_avsregistry::{error::AvsRegistryError, reader::AvsRegistryChainReader};
use eigen_crypto_bls::{
    alloy_registry_g1_point_to_g1_affine, alloy_registry_g2_point_to_g2_affine, BlsG1Point,
    BlsG2Point,
};
use eigen_logging::logger::SharedLogger;
use eigen_types::operator::{
    operator_id_from_g1_pub_key, OperatorId, OperatorPubKeys, OperatorTypesError,
};
use eigen_utils::{
    blsapkregistry::{
        BLSApkRegistry,
        BN254::{G1Point, G2Point},
    },
    get_ws_provider,
    registrycoordinator::RegistryCoordinator,
    NEW_PUBKEY_REGISTRATION_EVENT, OPERATOR_SOCKET_UPDATE,
};
use eyre::Result;
use futures_util::StreamExt;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::{
    mpsc::{self, UnboundedSender},
    oneshot::{self, Sender},
    RwLock,
};
use tokio_util::sync::CancellationToken;

use crate::operator_info::OperatorInfoService;

/// Fetches operator information from the registry.
/// Loads and stores operators info (addresses and public key) in memory.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct OperatorInfoServiceInMemory {
    logger: SharedLogger,
    pub avs_registry_reader: AvsRegistryChainReader,
    ws: String,
    pub_keys: UnboundedSender<OperatorsInfoMessage>,
}

/// State of the operator info service.
#[derive(Debug, Clone)]
struct OperatorState {
    operator_info_data: Arc<RwLock<HashMap<Address, OperatorPubKeys>>>,
    operator_addr_to_id: Arc<RwLock<HashMap<Address, OperatorId>>>,
    socket_dict: Arc<RwLock<HashMap<OperatorId, String>>>,
}

/// Error type for the operator info service.
#[derive(Error, Debug)]
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
    #[error("AVS Registry Error")]
    AvsRegistryReader(#[from] AvsRegistryError),
    #[error("Alloy Transport Error")]
    AlloyError(#[from] alloy::transports::TransportError),
    #[error("Socket not found")]
    SocketNotFound,
    #[error("Conversion from pubkey to id  error")]
    OperatorTypes(#[from] OperatorTypesError),
}

#[derive(Debug)]
pub struct OperatorSocket {
    pub id: OperatorId,
    pub socket: String,
}

#[derive(Debug)]
enum OperatorsInfoMessage {
    InsertOperatorInfo(
        Option<Address>,
        Option<Box<OperatorPubKeys>>,
        Option<OperatorSocket>,
    ),
    #[allow(dead_code)]
    Remove(Address),
    GetPubKeys(Address, Sender<Option<OperatorPubKeys>>),
    GetSockets(Address, Sender<Option<String>>),
}

#[async_trait]
impl OperatorInfoService for OperatorInfoServiceInMemory {
    async fn get_operator_info(
        &self,
        address: Address,
    ) -> Result<Option<OperatorPubKeys>, OperatorInfoServiceError> {
        let (responder_tx, responder_rx) = oneshot::channel();

        let _ = self
            .pub_keys
            .send(OperatorsInfoMessage::GetPubKeys(address, responder_tx))
            .map_err(|_| OperatorInfoServiceError::ChannelClosed)?;
        Ok(responder_rx
            .await
            .map_err(|_| OperatorInfoServiceError::ChannelClosed)?)
    }

    async fn get_operator_socket(
        &self,
        address: Address,
    ) -> Result<Option<String>, OperatorInfoServiceError> {
        let (responder_tx, responder_rx) = oneshot::channel();

        let _ = self
            .pub_keys
            .send(OperatorsInfoMessage::GetSockets(address, responder_tx))
            .map_err(|_| OperatorInfoServiceError::ChannelClosed)?;
        Ok(responder_rx
            .await
            .map_err(|_| OperatorInfoServiceError::ChannelClosed)?)
    }
}

impl OperatorInfoServiceInMemory {
    /// Creates a new operator info service given a logger, an avs registry chain reader and a web socket.
    ///
    /// # Arguments
    ///
    /// * `logger` - A shared logger.
    /// * `avs_registry_chain_reader` - An avs registry chain reader.
    /// * `web_socket` - A web socket.
    ///
    /// # Returns
    ///
    /// A new operator info service.
    pub async fn new(
        logger: SharedLogger,
        avs_registry_chain_reader: AvsRegistryChainReader,
        web_socket: String,
    ) -> Result<(Self, mpsc::UnboundedReceiver<OperatorInfoServiceError>), OperatorInfoServiceError>
    {
        let (pubkeys_tx, mut pubkeys_rx) = mpsc::unbounded_channel();
        let (error_tx, error_rx) = mpsc::unbounded_channel();

        let operator_state = OperatorState {
            operator_info_data: Arc::new(RwLock::new(HashMap::new())),
            operator_addr_to_id: Arc::new(RwLock::new(HashMap::new())),
            socket_dict: Arc::new(RwLock::new(HashMap::new())),
        };

        // Spawn a detached task for processing commands
        tokio::spawn({
            let operator_state = operator_state.clone();
            let error_tx = error_tx.clone();
            async move {
                while let Some(cmd) = pubkeys_rx.recv().await {
                    if let Err(e) = async {
                        match cmd {
                            OperatorsInfoMessage::InsertOperatorInfo(addr, keys, socket_info) => {
                                if let (Some(addr), Some(keys)) = (addr, keys) {
                                    let mut data = operator_state.operator_info_data.write().await;
                                    data.insert(addr, *keys.clone());

                                    let operator_id = operator_id_from_g1_pub_key(keys.g1_pub_key)?; // Use ? to propagate error

                                    let mut id_map =
                                        operator_state.operator_addr_to_id.write().await;
                                    id_map.insert(addr, alloy_primitives::FixedBytes(operator_id));
                                }
                                let mut socket_data = operator_state.socket_dict.write().await;
                                if let Some(socket) = socket_info {
                                    socket_data.insert(FixedBytes(*socket.id), socket.socket);
                                }
                            }
                            OperatorsInfoMessage::Remove(addr) => {
                                let mut data = operator_state.operator_info_data.write().await;
                                data.remove(&addr);
                            }
                            OperatorsInfoMessage::GetPubKeys(addr, responder) => {
                                let data = operator_state.operator_info_data.read().await;
                                let result = data.get(&addr).cloned();
                                let _ = responder.send(result);
                            }
                            OperatorsInfoMessage::GetSockets(addr, responder) => {
                                let operator_id = operator_state
                                    .operator_addr_to_id
                                    .read()
                                    .await
                                    .get(&addr)
                                    .cloned();
                                if let Some(id) = operator_id {
                                    let socket =
                                        operator_state.socket_dict.read().await.get(&id).cloned();
                                    let _ = responder.send(socket);
                                }
                            }
                        }
                        Ok::<(), OperatorInfoServiceError>(())
                    }
                    .await
                    {
                        // Send the error to the error channel
                        let _ = error_tx.send(e);
                    }
                }
            }
        });

        Ok((
            Self {
                logger,
                avs_registry_reader: avs_registry_chain_reader,
                ws: web_socket,
                pub_keys: pubkeys_tx,
            },
            error_rx,
        ))
    }

    /// Starts the operator info service.
    ///
    /// # Arguments
    ///
    /// * `cancellation_token` - A cancellation token than can be used to stop the service.
    /// * `start_block` - The start block to query for past operator registrations.
    /// * `end_block` - The end block to query for past operator registrations.
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, otherwise an error.
    pub async fn start_service(
        &self,
        cancellation_token: &CancellationToken,
        start_block: u64,
        end_block: u64,
    ) -> Result<(), OperatorInfoServiceError> {
        // Query past operator registrations
        self.query_past_registered_operator_events_and_fill_db(start_block, end_block)
            .await?;

        let provider = get_ws_provider(&self.ws).await?;
        let current_block_number = provider.get_block_number().await?;

        // Subscribe to new pubkey registration events
        let new_pubkey_registration_filter = Filter::new()
            .event(NEW_PUBKEY_REGISTRATION_EVENT)
            .from_block(current_block_number);

        let operator_socket_update_filter = Filter::new()
            .event(OPERATOR_SOCKET_UPDATE)
            .from_block(current_block_number);

        let subcription_new_operator_registration_stream = provider
            .subscribe_logs(&new_pubkey_registration_filter)
            .await?;
        let subscription_operator_socket_update_filter = provider
            .subscribe_logs(&operator_socket_update_filter)
            .await?;

        let mut new_operator_registration_stream = subcription_new_operator_registration_stream
            .into_stream()
            .fuse();
        let mut operator_socket_update_stream = subscription_operator_socket_update_filter
            .into_stream()
            .fuse();
        let pub_keys = self.pub_keys.clone();
        let self_clone = self.clone();

        loop {
            tokio::select! {
                _ = cancellation_token.cancelled() => {
                    self.logger.info("Cancellation signal received, stopping the stream.", "eigen-services-operatorsinfo.start_service");
                    break;
                },
                log = new_operator_registration_stream.next() => {
                    match log {
                        Some(log) => {

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
                                    Some(event_data.operator),
                                    Some(Box::new(operator_pub_key)),
                                    None

                                ));
                            }
                        },
                        None => {
                            break;
                        }
                    }
                },

                log =operator_socket_update_stream.next() =>{

                    match log {
                        Some(log) => {

                            let data = log
                                .log_decode::<RegistryCoordinator::OperatorSocketUpdate>()
                                .ok();

                            if let Some(operator_socket_update_event) = data {
                                let event_data = operator_socket_update_event.data();
                                let operator_socket = OperatorSocket {
                                    id: event_data.operatorId,
                                    socket:event_data.socket.clone()
                                };
                                // Send message

                                self_clone.logger.debug(
                                    &format!(
                                        "Received new socket registration event  operator_id : {:?} , socket : {:?}",
                                        event_data.operatorId, event_data.socket
                                    ),
                                    "eigen-services-operatorsinfo.start_service",
                                );

                                let _ = pub_keys.send(OperatorsInfoMessage::InsertOperatorInfo(
                                    None,
                                    None,
                                    Some(OperatorSocket{socket:operator_socket.socket , id:operator_socket.id })

                                ));
                            }
                        },
                        None => {
                            break;
                        }
                    }

                }
            }
        }

        Ok(())
    }

    /// Queries past operator registration events and fills the database by sending messages
    /// to the operator info service channel.
    /// This information is then stored in OperatorState`.
    ///
    /// # Arguments
    ///
    /// * `start_block` - The start block to query for past operator registrations.
    /// * `end_block` - The end block to query for past operator registrations.
    ///
    /// # Returns
    ///
    /// Ok(()) if successful, otherwise an error.
    pub async fn query_past_registered_operator_events_and_fill_db(
        &self,
        start_block: u64,
        end_block: u64,
    ) -> Result<(), OperatorInfoServiceError> {
        // let (operator_address, operator_pub_keys);
        let handle_1 = self
            .avs_registry_reader
            .query_existing_registered_operator_pub_keys(start_block, end_block, self.ws.clone());

        let handle_2 = self
            .avs_registry_reader
            .query_existing_registered_operator_sockets(start_block, end_block);
        let mut operator_address: Vec<Address> = vec![];
        let mut socket_map: HashMap<FixedBytes<32>, String> = HashMap::new();
        let mut operator_pub_keys: Vec<OperatorPubKeys> = vec![];
        if let Ok(res) = futures::future::try_join(handle_1, handle_2).await {
            let (pub_keys, operator_sockets) = res;
            (operator_address, operator_pub_keys) = pub_keys;
            socket_map = operator_sockets;
        }

        for (i, address) in operator_address.iter().enumerate() {
            let operator_id = FixedBytes(operator_id_from_g1_pub_key(
                operator_pub_keys[i].g1_pub_key.clone(),
            )?);
            if let Some(socket) = socket_map.get(&operator_id) {
                let message = OperatorsInfoMessage::InsertOperatorInfo(
                    Some(*address),
                    Some(Box::new(operator_pub_keys[i].clone())),
                    Some(OperatorSocket {
                        id: operator_id,
                        socket: socket.to_string(),
                    }),
                );
                self.logger.debug(
                    &format!(
                        "New pub key found  operator_address : {:?} , operator_pub_keys : {:?}",
                        operator_address, operator_pub_keys
                    ),
                    "eigen-services-operatorsinfo.query_past_registered_operator_events_and_fill_db",
                );
                let _ = self.pub_keys.send(message);
            } else {
                return Err(OperatorInfoServiceError::SocketNotFound);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{address, Bytes, U256};
    use alloy_signer_local::PrivateKeySigner;
    use eigen_client_avsregistry::writer::AvsRegistryChainWriter;
    use eigen_client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
    use eigen_crypto_bls::BlsKeyPair;
    use eigen_logging::{get_logger, get_test_logger, init_logger};
    use eigen_testing_utils::anvil::start_anvil_container;
    use eigen_testing_utils::anvil_constants::{
        get_avs_directory_address, get_delegation_manager_address,
        get_operator_state_retriever_address, get_registry_coordinator_address,
        get_rewards_coordinator_address, get_strategy_manager_address,
    };
    use eigen_types::operator::Operator;
    use eigen_utils::get_provider;
    use std::str::FromStr;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[tokio::test]
    async fn test_query_past_registered_operator_events_and_fill_db() {
        let (_container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        init_logger(eigen_logging::log_level::LogLevel::Debug);
        let test_logger = get_logger();
        register_operator(
            http_endpoint.clone(),
            "0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6",
            "202646553755999769005569871314544341631930435075911377994162443131009480062",
        )
        .await;

        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            test_logger.clone(),
            get_registry_coordinator_address(http_endpoint.clone()).await,
            get_operator_state_retriever_address(http_endpoint.clone()).await,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
            test_logger.clone(),
            avs_registry_chain_reader,
            ws_endpoint,
        )
        .await
        .unwrap()
        .0;

        let end_block = get_provider(http_endpoint.as_str())
            .get_block_number()
            .await
            .unwrap();
        let _ = operators_info_service_in_memory
            .query_past_registered_operator_events_and_fill_db(0, end_block)
            .await;

        let address = address!("90f79bf6eb2c4f870365e785982e1f101e93b906");
        let operator_info = operators_info_service_in_memory
            .get_operator_info(address)
            .await;
        let operator_socket = operators_info_service_in_memory
            .get_operator_socket(address)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(operator_socket, "socket");
        assert!(operator_info.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_start_service_1_operator_register() {
        // start anvil in a container
        let (_container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        let test_logger = get_test_logger();
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            test_logger.clone(),
            get_registry_coordinator_address(http_endpoint.clone()).await,
            get_operator_state_retriever_address(http_endpoint.clone()).await,
            http_endpoint.clone(),
        )
        .await
        .unwrap();

        let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
            test_logger.clone(),
            avs_registry_chain_reader,
            ws_endpoint,
        )
        .await
        .unwrap()
        .0;
        let clone_operators_info = operators_info_service_in_memory.clone();

        let token = tokio_util::sync::CancellationToken::new().clone();
        let cancel_token = token.clone();
        let cloned_http_endpoint = http_endpoint.clone();

        tokio::spawn(async move {
            let _ = clone_operators_info
                .start_service(
                    &token,
                    0,
                    get_provider(cloned_http_endpoint.as_str())
                        .get_block_number()
                        .await
                        .unwrap(),
                )
                .await;
        });

        register_operator(
            http_endpoint,
            "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80",
            "12248929636257230549931416853095037629726205319386239410403476017439825112537",
        )
        .await;

        // need to wait at least 3 seconds to get the event processed
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

        cancel_token.clone().cancel();

        let address = address!("f39fd6e51aad88f6f4ce6ab8827279cfffb92266");
        let operator_info = operators_info_service_in_memory
            .get_operator_info(address)
            .await;
        assert!(operator_info.unwrap().is_some());
        let operator_socket = operators_info_service_in_memory
            .get_operator_socket(address)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(operator_socket, "socket");
    }

    #[tokio::test]
    async fn test_start_service_2_operator_register() {
        let (_container, http_endpoint, ws_endpoint) = start_anvil_container().await;
        let test_logger = get_test_logger();
        let avs_registry_chain_reader = AvsRegistryChainReader::new(
            test_logger.clone(),
            get_registry_coordinator_address(http_endpoint.clone()).await,
            get_operator_state_retriever_address(http_endpoint.clone()).await,
            http_endpoint.clone(),
        )
        .await
        .unwrap();
        let operators_info_service_in_memory = OperatorInfoServiceInMemory::new(
            test_logger.clone(),
            avs_registry_chain_reader,
            ws_endpoint,
        )
        .await
        .unwrap()
        .0;
        let clone_operators_info = operators_info_service_in_memory.clone();

        let token = tokio_util::sync::CancellationToken::new().clone();
        let cancel_token = token.clone();
        let cloned_http_endpoint = http_endpoint.clone();
        tokio::spawn(async move {
            let _ = clone_operators_info
                .start_service(
                    &token,
                    0,
                    get_provider(cloned_http_endpoint.as_str())
                        .get_block_number()
                        .await
                        .unwrap(),
                )
                .await;
        });
        register_operator(
            http_endpoint.clone(),
            "0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
            "1328790040692576325258580129229001772890358018148159309458854770206210226319",
        )
        .await;
        register_operator(
            http_endpoint,
            "0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a",
            "8949062771264691130193054363356855357736539613420316273398900351143637925935",
        )
        .await;
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await; // need to wait atleast 3 second to get the event processed

        cancel_token.clone().cancel();

        let address = address!("70997970c51812dc3a010c7d01b50e0d17dc79c8");
        let operator_info = operators_info_service_in_memory
            .get_operator_info(address)
            .await;
        assert!(operator_info.unwrap().is_some());

        let address_2 = address!("3c44cdddb6a900fa2b585dd299e03d12fa4293bc");
        let operator_info_2 = operators_info_service_in_memory
            .get_operator_info(address_2)
            .await;
        assert!(operator_info_2.unwrap().is_some());
        let operator_socket = operators_info_service_in_memory
            .get_operator_socket(address)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(operator_socket, "socket");
        let operator_socket = operators_info_service_in_memory
            .get_operator_socket(address_2)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(operator_socket, "socket");
    }

    pub async fn register_operator(http_endpoint: String, pvt_key: &str, bls_key: &str) {
        let delegation_manager_address =
            get_delegation_manager_address(http_endpoint.clone()).await;
        let avs_directory_address = get_avs_directory_address(http_endpoint.clone()).await;
        let strategy_manager_address = get_strategy_manager_address(http_endpoint.clone()).await;
        let rewards_coordinator_address =
            get_rewards_coordinator_address(http_endpoint.clone()).await;
        let el_chain_reader = ELChainReader::new(
            get_test_logger(),
            Address::ZERO,
            delegation_manager_address,
            avs_directory_address,
            http_endpoint.to_string(),
        );
        let signer = PrivateKeySigner::from_str(pvt_key).unwrap();

        let el_chain_writer = ELChainWriter::new(
            delegation_manager_address,
            strategy_manager_address,
            rewards_coordinator_address,
            el_chain_reader,
            http_endpoint.to_string(),
            pvt_key.to_string(),
        );

        let operator_details = Operator {
            address: signer.address(),
            earnings_receiver_address: signer.address(),
            delegation_approver_address: signer.address(),
            staker_opt_out_window_blocks: 3,
            metadata_url: Some("eigensdk-rs".to_string()),
        };

        el_chain_writer
            .register_as_operator(operator_details)
            .await
            .unwrap();

        let avs_registry_writer = AvsRegistryChainWriter::build_avs_registry_chain_writer(
            get_test_logger(),
            http_endpoint.to_string(),
            pvt_key.to_string(),
            get_registry_coordinator_address(http_endpoint.clone()).await,
            get_operator_state_retriever_address(http_endpoint.clone()).await,
        )
        .await
        .unwrap();

        let bls_key_pair = BlsKeyPair::new(bls_key.to_string()).unwrap();
        let salt: FixedBytes<32> = FixedBytes::from([0x02; 32]);
        let now = SystemTime::now();

        // Convert SystemTime to a Duration since the UNIX epoch
        let duration_since_epoch = now
            .duration_since(UNIX_EPOCH)
            .inspect_err(|_| println!("System time seems to be before the UNIX epoch."))
            .unwrap();
        // Convert the duration to seconds
        let seconds = duration_since_epoch.as_secs(); // Returns a u64

        // Convert seconds to U256
        let expiry = U256::from(seconds) + U256::from(10000);

        let quorum_numbers = Bytes::from_str("0x00").unwrap();
        let socket = "socket";

        let _ = avs_registry_writer
            .register_operator_in_quorum_with_avs_registry_coordinator(
                bls_key_pair,
                salt,
                expiry,
                quorum_numbers,
                socket.to_string(),
            )
            .await
            .unwrap();
    }
}
