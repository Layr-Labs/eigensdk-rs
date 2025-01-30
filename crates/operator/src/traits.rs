use super::error::OperatorError;
use crate::client::ClientAggregator;
use alloy::{primitives::Address, providers::WsConnect, rpc::types::Filter, sol_types::SolEvent};
use alloy_provider::{Provider, ProviderBuilder};
use eigen_aggregator::rpc_server::SignedTaskResponse;
use eigen_aggregator::traits::TaskResponse;
use eigen_client_avsregistry::reader::AvsRegistryChainReader;
use eigen_crypto_bls::BlsKeyPair;
use eigen_types::operator::OperatorId;
use futures_util::StreamExt;
use std::sync::Arc;
use tracing::info;

/// Operator methods
pub trait Operator {
    /// Task response
    type TaskResponse: TaskResponse + Send;
    /// New task event
    type NewTaskEvent: SolEvent + Send;

    /// Processes new task
    fn process_new_task(new_task_created: Self::NewTaskEvent) -> Self::TaskResponse;

    /// Start the operator
    fn start_operator(
        avs_registry_reader: &AvsRegistryChainReader,
        key_pair: &BlsKeyPair,
        operator_id: &OperatorId,
        operator_address: Address,
        operator_name: &str,
        client_aggregator: &ClientAggregator,
        ws_rpc_url: &str,
    ) -> impl std::future::Future<Output = Result<(), OperatorError>> + Send {
        async move {
            let is_registered = avs_registry_reader
                .is_operator_registered(operator_address)
                .await
                .map_err(|_| OperatorError::RegistrationError)?;
            info!("is {} registered {}", operator_name, is_registered);
            let arc_client = Arc::new(client_aggregator);
            if !is_registered {
                return Err(OperatorError::RegistrationError);
            }

            info!("Starting operator");

            let ws = WsConnect::new(ws_rpc_url);
            let provider = ProviderBuilder::new()
                .on_ws(ws)
                .await
                .map_err(|_| OperatorError::TransportError)?;

            let filter = Filter::new().event_signature(Self::NewTaskEvent::SIGNATURE_HASH);
            let sub = provider
                .subscribe_logs(&filter)
                .await
                .map_err(|_| OperatorError::SubscribeLogsError)?;
            let mut stream = sub.into_stream();

            // TODO: this loop could be a separate function
            while let Some(log) = stream.next().await {
                // TODO: check error handling (maybe use continue)
                let data: Self::NewTaskEvent = log
                    .log_decode()
                    .map_err(|_| OperatorError::SubscribeLogsError)?
                    .inner
                    .data;
                info!("{} picked up a new task", operator_name);
                let task_response = Self::process_new_task(data);
                let signed_task_response =
                    Self::sign_task_response(key_pair, operator_id, task_response)?;
                let _ = arc_client
                    .send_signed_task_response(signed_task_response)
                    .await;
            }
            Ok(())
        }
    }

    /// Sign the task response
    fn sign_task_response(
        key_pair: &BlsKeyPair,
        operator_id: &OperatorId,
        task_response: Self::TaskResponse,
    ) -> Result<SignedTaskResponse<Self::TaskResponse>, OperatorError> {
        let hash_msg = task_response.digest();
        let signed_msg = key_pair.sign_message(&hash_msg);
        let signed_task_response = SignedTaskResponse::new(task_response, signed_msg, *operator_id);
        Ok(signed_task_response)
    }
}
