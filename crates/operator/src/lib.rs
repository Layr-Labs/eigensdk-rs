//! Operator common functions.

use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
};
use client::ClientAggregator;
use eigen_aggregator::{SignedTaskResponse, TaskResponse};
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::operator::OperatorId;
use error::OperatorError;
use futures_util::StreamExt;
use operator_task_processor::OperatorTaskProcessor;
use tracing::info;

/// Tarpc Client
pub mod client;
/// Error
pub mod error;
/// Operator trait
pub mod operator_task_processor;

/// Operator struct to handle the operator logic of processing new tasks
/// and sending signed task responses to the aggregator.
#[derive(Debug)]
pub struct Operator<TP: OperatorTaskProcessor> {
    operator_id: OperatorId,
    operator_name: String,
    client_aggregator: ClientAggregator,
    ws_rpc_url: String,
    key_pair: BlsKeyPair,
    task_processor: TP,
}

impl<TP: OperatorTaskProcessor> Operator<TP> {
    /// Initialize a new operator.
    /// This method does not register the operator.
    ///
    /// # Arguments
    ///
    /// * `avs_registry_reader` - The AVS registry reader.
    /// * `key_pair` - The key pair of the operator.
    /// * `operator_address` - The address of the operator.
    /// * `operator_name` - The name of the operator.
    /// * `client_aggregator` - The client aggregator.
    /// * `ws_rpc_url` - The URL of the WebSocket RPC.
    /// * `task_processor` - The Operator task processor.
    ///
    /// # Returns
    ///
    /// * `Result<Self, OperatorError>` - The operator.
    pub async fn new(
        avs_registry_reader: &AvsRegistryChainReader,
        key_pair: &BlsKeyPair,
        operator_address: Address,
        operator_name: &str,
        client_aggregator: &ClientAggregator,
        ws_rpc_url: &str,
        task_processor: TP,
    ) -> Result<Self, OperatorError> {
        let is_registered = avs_registry_reader
            .is_operator_registered(operator_address)
            .await
            .map_err(|_| OperatorError::RegistrationError)?;
        info!("{} registered: {}", operator_name, is_registered);

        if !is_registered {
            return Err(OperatorError::RegistrationError);
        }

        let operator_id = avs_registry_reader
            .get_operator_id(operator_address)
            .await
            .map_err(|_| OperatorError::OperatorIdError)?;

        Ok(Self {
            operator_id,
            operator_name: operator_name.to_string(),
            ws_rpc_url: ws_rpc_url.to_string(),
            client_aggregator: client_aggregator.clone(),
            key_pair: key_pair.clone(),
            task_processor,
        })
    }

    /// Start listening for new task events. Operator subscribe to the event signature of the task processor.
    /// When a new task is created, the operator will process it and send the signed task response to the aggregator.
    ///
    /// # Arguments
    ///
    /// * `self` - The operator.
    ///
    /// # Returns
    ///
    /// * `Result<(), OperatorError>` - The result of the operation.
    pub async fn start(&self) -> Result<(), OperatorError> {
        let ws = WsConnect::new(&self.ws_rpc_url);
        let provider = ProviderBuilder::new()
            .on_ws(ws)
            .await
            .map_err(|_| OperatorError::TransportError)?;

        let filter = Filter::new().event_signature(self.task_processor.get_event_signature());
        let sub = provider
            .subscribe_logs(&filter)
            .await
            .map_err(|_| OperatorError::SubscribeLogsError)?;
        let mut stream = sub.into_stream();

        while let Some(log) = stream.next().await {
            let data: TP::NewTaskEvent = log
                .log_decode()
                .map_err(|_| OperatorError::SubscribeLogsError)?
                .inner
                .data;

            info!("{} picked up a new task", self.operator_name);

            let task_response = self.task_processor.process_new_task(data);
            let signed_task_response =
                Self::sign_task_response(&self.key_pair, &self.operator_id, task_response)?;
            // TODO: handle the error
            let _ = self
                .client_aggregator
                .send_signed_task_response(signed_task_response)
                .await;
        }

        Ok(())
    }

    /// Sign the task response for the aggregator.
    ///
    /// # Arguments
    ///
    /// * `key_pair` - The key pair of the operator.
    /// * `operator_id` - The id of the operator.
    /// * `task_response` - The task response to sign.
    ///
    /// # Returns
    ///
    /// * `Result<SignedTaskResponse<TP::TaskResponse>, OperatorError>` - The signed task response.
    fn sign_task_response(
        key_pair: &BlsKeyPair,
        operator_id: &OperatorId,
        task_response: TP::TaskResponse,
    ) -> Result<SignedTaskResponse<TP::TaskResponse>, OperatorError> {
        let hash_msg = task_response.digest();
        let signed_msg = key_pair.sign_message(&hash_msg);
        let signed_task_response = SignedTaskResponse::new(task_response, signed_msg, *operator_id);
        info!("Operator signed task response");
        Ok(signed_task_response)
    }
}
