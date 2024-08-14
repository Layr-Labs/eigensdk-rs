use alloy_consensus::TxEnvelope;
use alloy_json_rpc::{RpcParam, RpcReturn};
use alloy_primitives::{Address, BlockHash, BlockNumber, Bytes, ChainId, B256, U256, U64};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_pubsub::{PubSubFrontend, Subscription};
use alloy_rlp::Encodable;
use alloy_rpc_types_eth::{
    Block, BlockNumberOrTag, FeeHistory, Filter, Header, Log, SyncStatus, Transaction,
    TransactionReceipt, TransactionRequest,
};
use alloy_transport::{TransportError, TransportResult};
use alloy_transport_http::{Client, Http};
use alloy_transport_ws::WsConnect;
use eigen_logging::{get_test_logger, logger::Logger};
use eigen_metrics_collectors_rpc_calls::RpcCallsMetrics as RpcCallsCollector;
use hex;
use std::time::Instant;
use thiserror::Error;
use url::Url;

const PENDING_TAG: &str = "pending";

pub struct InstrumentedClient {
    http_client: Option<RootProvider<Http<Client>>>,
    ws_client: Option<RootProvider<PubSubFrontend>>,
    rpc_collector: RpcCallsCollector,
    net_version: u64,
}

/// Possible errors raised in signer creation
#[derive(Error, Debug)]
pub enum InstrumentedClientError {
    #[error("invalid url")]
    InvalidUrl,
    #[error("error getting version")]
    ErrorGettingVersion,
}

impl InstrumentedClient {
    /// Creates a new instance of the InstrumentedClient.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL of the RPC server.
    ///
    /// # Returns
    ///
    /// A new instance of the InstrumentedClient.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid or if there is an error getting the version.
    pub async fn new(url: &str) -> Result<Self, InstrumentedClientError> {
        let url = Url::parse(url).map_err(|_| InstrumentedClientError::InvalidUrl)?;
        let http_client = ProviderBuilder::new().on_http(url);
        let net_version = http_client
            .get_net_version()
            .await
            .map_err(|_| InstrumentedClientError::ErrorGettingVersion)?;

        let rpc_collector = RpcCallsCollector::new(get_test_logger().clone());
        Ok(InstrumentedClient {
            http_client: Some(http_client),
            ws_client: None,
            rpc_collector,
            net_version,
        })
    }

    /// Creates a new instance of the InstrumentedClient that supports ws connection.
    ///
    /// # Arguments
    ///
    /// * `url` - The ws URL of the RPC server .
    ///
    /// # Returns
    ///
    /// A new instance of the InstrumentedClient.
    ///
    /// # Errors
    ///
    /// Returns an error if the URL is invalid or if there is an error getting the version.
    pub async fn new_ws(url: &str) -> Result<Self, InstrumentedClientError> {
        let url = Url::parse(url).map_err(|_| InstrumentedClientError::InvalidUrl)?;
        let ws_connect = WsConnect::new(url);

        let ws_client = ProviderBuilder::new().on_ws(ws_connect).await.unwrap();
        let net_version = ws_client
            .get_net_version()
            .await
            .map_err(|_| InstrumentedClientError::ErrorGettingVersion)?;

        let rpc_collector = RpcCallsCollector::new(get_test_logger().clone());
        Ok(InstrumentedClient {
            http_client: None,
            ws_client: Some(ws_client),
            rpc_collector,
            net_version,
        })
    }

    /// Creates a new instance of the InstrumentedClient from an existing client (`RootProvider`).
    ///
    /// # Arguments
    ///
    /// * `client` - The existing client (`RootProvider`).
    ///
    /// # Returns
    ///
    /// A new instance of the InstrumentedClient.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an error getting the version.
    pub async fn new_from_client(
        client: RootProvider<Http<Client>>,
    ) -> Result<Self, InstrumentedClientError> {
        let net_version = client
            .get_net_version()
            .await
            .map_err(|_| InstrumentedClientError::ErrorGettingVersion)?;

        let rpc_collector = RpcCallsCollector::new(get_test_logger().clone());
        Ok(InstrumentedClient {
            http_client: Some(client),
            ws_client: None,
            rpc_collector,
            net_version,
        })
    }

    /// Returns the chain ID.
    ///
    /// # Returns
    ///
    /// The chain ID.
    pub async fn chain_id(&self) -> TransportResult<ChainId> {
        self.instrument_function("eth_chainId", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get chain id", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Returns the balance of the account at the given block number.
    ///
    /// # Arguments
    ///
    /// * `account` - The account address.
    /// * `block_number` - The block number.
    ///
    /// # Returns
    ///
    /// The balance of the account at the given block number.
    pub async fn balance_at(
        &self,
        account: Address,
        block_number: BlockNumberOrTag,
    ) -> TransportResult<U256> {
        self.instrument_function("eth_getBalance", (account, block_number))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get balance", &[err])
            })
    }

    /// Returns the block having the given block hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The block hash.
    ///
    /// # Returns
    ///
    /// The block having the given block hash.
    pub async fn block_by_hash(&self, hash: BlockHash) -> TransportResult<Option<Block>> {
        self.instrument_function("eth_getBlockByHash", (hash, true))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get block by hash", &[err])
            })
    }

    /// Returns the block having the given block number.
    ///
    /// # Arguments
    ///
    /// * `number` - The block number.
    ///
    /// # Returns
    ///
    /// The block having the given block number.
    pub async fn block_by_number(
        &self,
        number: BlockNumberOrTag,
    ) -> TransportResult<Option<Block>> {
        self.instrument_function("eth_getBlockByNumber", (number, true))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get block by number", &[err])
            })
    }

    /// Returns the latest block number.
    ///
    /// # Returns
    ///
    /// The latest block number.
    pub async fn block_number(&self) -> TransportResult<BlockNumber> {
        self.instrument_function("eth_blockNumber", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get block number", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Executes a message call transaction.
    ///
    /// # Arguments
    ///
    /// * `call` - The message call to be executed
    /// * `block_number` - The block height at which the call runs. *Note:* in case this argument is n
    ///
    /// # Returns
    ///
    /// The returned value of the executed contract.
    pub async fn call_contract(
        &self,
        call: TransactionRequest,
        block_number: BlockNumberOrTag,
    ) -> TransportResult<Bytes> {
        self.instrument_function("eth_call", (call, block_number))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to call contract", &[err])
            })
    }

    /// Returns the compiled bytecode of a smart contract given its address and block number.
    ///
    /// # Arguments
    ///
    /// * `address` - The address of the smart contract.
    /// * `block_number` - The block number.
    ///
    /// # Returns
    ///
    /// The compiled bytecode of the smart contract with the given address and block number.
    pub async fn code_at(
        &self,
        address: Address,
        block_number: BlockNumberOrTag,
    ) -> TransportResult<Bytes> {
        self.instrument_function("eth_getCode", (address, block_number))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get code", &[err])
            })
    }

    /// Estimates the gas needed to execute a specific transaction.
    ///
    /// # Arguments
    ///
    /// * `tx` - The transaction from which the gas consumption is estimated.
    ///
    /// # Returns
    ///
    /// The estimated gas.
    pub async fn estimate_gas(&self, tx: TransactionRequest) -> TransportResult<u64> {
        self.instrument_function("eth_estimateGas", (tx,))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to estimate gas", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Returns a collection of historical gas information.
    ///
    /// # Arguments
    ///
    /// * `block_count` - The number of blocks to include in the collection.
    /// * `last_block` - The last block number to include in the collection.
    /// * `reward_percentiles` - A sorted list of percentage points used to
    /// sample the effective priority fees per gas from each block. The samples are
    /// taken in ascending order and weighted by gas usage. The list is sorted increasingly.
    pub async fn fee_history(
        &self,
        block_count: u64,
        last_block: BlockNumberOrTag,
        reward_percentiles: &[f64],
    ) -> TransportResult<FeeHistory> {
        self.instrument_function(
            "eth_feeHistory",
            (block_count, last_block, reward_percentiles),
        )
        .await
        .inspect_err(|err| {
            self.rpc_collector
                .logger()
                .error("Failed to get fee history", &[err])
        })
    }
    /// Executes a filter query.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter query to be executed.
    ///
    /// # Returns
    ///
    /// A vector of logs.
    pub async fn filter_logs(&self, filter: Filter) -> TransportResult<Vec<Log>> {
        self.instrument_function("eth_getLogs", (filter,))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get filter logs", &[err])
            })
    }

    /// Returns the block header with the given hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The block hash.
    ///
    /// # Returns
    ///
    /// The block header.
    pub async fn header_by_hash(&self, hash: B256) -> TransportResult<Header> {
        let transaction_detail = false;
        self.instrument_function("eth_getBlockByHash", (hash, transaction_detail))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get header by hash", &[err])
            })
    }

    /// Returns a block header with the given block number.
    ///
    /// # Arguments
    ///
    /// * `block_number` - The block number.
    ///
    /// # Returns
    ///
    /// The block header.
    pub async fn header_by_number(
        &self,
        block_number: BlockNumberOrTag,
    ) -> TransportResult<Header> {
        let transaction_detail = false;
        self.instrument_function("eth_getBlockByNumber", (block_number, transaction_detail))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get header by number", &[err])
            })
    }

    /// Returns the nonce of the given account.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the account.
    /// * `block_number` - The block number from where the nonce is taken.
    ///
    /// # Returns
    ///
    /// The nonce of the account.
    pub async fn nonce_at(
        &self,
        account: Address,
        block_number: BlockNumberOrTag,
    ) -> TransportResult<u64> {
        self.instrument_function("eth_getTransactionCount", (account, block_number))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get nonce", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Returns the wei balance of the given account in the pending state.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the account.
    ///
    /// # Returns
    ///
    /// The wei balance of the account.
    pub async fn pending_balance_at(&self, account: Address) -> TransportResult<U256> {
        self.instrument_function("eth_getBalance", (account, PENDING_TAG))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get pending balance", &[err])
            })
    }

    /// Executes a message call transaction using the EVM.
    /// The state seen by the contract call is the pending state.
    ///
    /// # Arguments
    ///
    /// * `call` - The message call to be executed
    ///
    /// # Returns
    ///
    /// The returned value of the executed contract.
    pub async fn pending_call_contract(&self, call: TransactionRequest) -> TransportResult<Bytes> {
        self.call_contract(call, BlockNumberOrTag::Pending).await
    }

    /// Returns the contract code of the given account in the pending state.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the contract.
    ///
    /// # Returns
    ///
    /// The contract code.
    pub async fn pending_code_at(&self, account: Address) -> TransportResult<Bytes> {
        self.instrument_function("eth_getCode", (account, PENDING_TAG))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get pending code", &[err])
            })
    }

    /// Returns the account nonce of the given account in the pending state.
    /// This is the nonce that should be used for the next transaction.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the account.
    ///
    /// # Returns
    ///
    /// * The nonce of the account in the pending state.
    pub async fn pending_nonce_at(&self, account: Address) -> TransportResult<u64> {
        self.instrument_function("eth_getTransactionCount", (account, PENDING_TAG))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get pending nonce", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Returns the value of key in the contract storage of the given account in the pending state.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the contract.
    /// * `key` - The position in the storage.
    ///
    /// # Returns
    ///
    /// The value of the storage position at the provided address.
    pub async fn pending_storage_at(&self, account: Address, key: U256) -> TransportResult<U256> {
        self.instrument_function("eth_getStorageAt", (account, key, PENDING_TAG))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get pending storage", &[err])
            })
    }

    /// Returns the total number of transactions in the pending state.
    ///
    /// # Returns
    ///
    /// The number of pending transactions.
    pub async fn pending_transaction_count(&self) -> TransportResult<u64> {
        self.instrument_function("eth_getBlockTransactionCountByNumber", (PENDING_TAG,))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get transaction count", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Sends a signed transaction into the pending pool for execution.
    ///
    /// # Arguments
    ///
    /// * `tx` - The transaction to be executed.
    ///
    /// # Returns
    ///
    /// The hash of the given transaction.
    pub async fn send_transaction(&self, tx: TxEnvelope) -> TransportResult<B256> {
        let mut encoded_tx = Vec::new();
        tx.encode(&mut encoded_tx);
        self.instrument_function("eth_sendRawTransaction", (hex::encode(encoded_tx),))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to send transaction", &[err])
            })
    }

    /// Returns the value of key in the contract storage of the given account.
    ///
    /// # Arguments
    ///
    /// * `account` - The address of the contract.
    /// * `key` - The position in the storage.
    /// * `block_number` - The block number from which the storage is taken.
    ///
    /// # Returns
    ///
    /// The value of the storage position at the provided address.
    pub async fn storage_at(
        &self,
        account: Address,
        key: U256,
        block_number: U256,
    ) -> TransportResult<U256> {
        self.instrument_function("eth_getStorageAt", (account, key, block_number))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get storage", &[err])
            })
    }

    /// Subscribes to the results of a streaming filter query.
    /// *Note:* this method fails if the InstrumentedClient does not use a web socket client.
    /// # Arguments
    ///
    /// * `filter` - A filter query.
    ///
    /// # Returns
    ///
    /// The subscription.
    ///
    /// # Errors
    ///
    /// * If ws_client is `None`.
    pub async fn subscribe_filter_logs<R: RpcReturn>(
        &self,
        filter: Filter,
    ) -> TransportResult<Subscription<R>> {
        let id: U256 = self
            .instrument_function("eth_subscribe", ("logs", filter))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get logs subscription id", &[err])
            })?;
        if let Some(ws_client) = self.ws_client.as_ref() {
            ws_client.get_subscription(id).await
        } else {
            Err(TransportError::UnsupportedFeature(
                "http client does not support eth_subscribe calls.",
            ))
        }
    }

    /// Subscribes to notifications about the current blockchain head.
    /// *Note:* this method fails if the InstrumentedClient does not use a web socket client.
    ///
    /// # Returns
    ///
    /// The subscription.
    ///
    /// # Errors
    ///
    /// * If ws_client is `None`.
    pub async fn subscribe_new_head<R: RpcReturn>(&self) -> TransportResult<Subscription<R>> {
        let id: U256 = self
            .instrument_function("eth_subscribe", ("newHeads",))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to subscribe new head", &[err])
            })?;
        if let Some(ws_client) = self.ws_client.as_ref() {
            ws_client.get_subscription(id).await
        } else {
            Err(TransportError::UnsupportedFeature(
                "http client does not support eth_subscribe calls.",
            ))
        }
    }

    /// Retrieves the currently suggested gas price.
    ///
    /// # Returns
    ///
    /// The currently suggested gas price.
    pub async fn suggest_gas_price(&self) -> TransportResult<u64> {
        self.instrument_function("eth_gasPrice", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to suggest gas price", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Retrieves the currently suggested gas tip cap after EIP1559.
    ///
    /// # Returns
    ///
    /// The currently suggested gas price.
    pub async fn suggest_gas_tip_cap(&self) -> TransportResult<u64> {
        self.instrument_function("eth_maxPriorityFeePerGas", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to suggest gas tip cap", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Retrieves the current progress of the sync algorithm.
    /// If there's no sync currently running, it returns None.
    ///
    /// # Returns
    ///
    /// The current progress of the sync algorithm.
    pub async fn sync_progress(&self) -> TransportResult<SyncStatus> {
        self.instrument_function("eth_syncing", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get sync progress", &[err])
            })
    }

    /// Returns the transaction with the given hash.
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The transaction hash.
    ///
    /// # Returns
    ///
    /// The transaction with the given hash.
    pub async fn transaction_by_hash(&self, tx_hash: B256) -> TransportResult<Transaction> {
        self.instrument_function("eth_getTransactionByHash", (tx_hash,))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get transaction by hash", &[err])
            })
    }

    /// Returns the total number of transactions in the given block.
    ///
    /// # Arguments
    ///
    /// * `block_hash` - The block hash.
    ///
    /// # Returns
    ///
    /// The number of transactions in the given block.
    pub async fn transaction_count(&self, block_hash: B256) -> TransportResult<u64> {
        self.instrument_function("eth_getBlockTransactionCountByHash", (block_hash,))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get transaction count", &[err])
            })
            .map(|result: U64| result.to())
    }

    /// Returns a single transaction at index in the given block.
    ///
    /// # Arguments
    ///
    /// * `block_hash` - The block hash.
    /// * `index` - The index of the transaction in the block.
    ///
    /// # Returns
    ///
    /// The transaction at index in the given block.
    pub async fn transaction_in_block(
        &self,
        block_hash: B256,
        index: u64,
    ) -> TransportResult<Transaction> {
        self.instrument_function("eth_getTransactionByBlockHashAndIndex", (block_hash, index))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get transaction", &[err])
            })
    }

    /// Returns the receipt of a transaction by transaction hash.
    /// *Note:* the receipt is not available for pending transactions.
    ///
    /// # Arguments
    ///
    /// * `tx_hash` - The hash of the transaction.
    ///
    /// # Returns
    ///
    /// The transaction receipt.
    pub async fn transaction_receipt(&self, tx_hash: B256) -> TransportResult<TransactionReceipt> {
        self.instrument_function("eth_getTransactionReceipt", (tx_hash,))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get receipt", &[err])
            })
    }

    /// Instrument a function call with the given method name and parameters.
    ///
    /// This function will measure the duration of the call and report it to the metrics collector.
    ///
    /// # Arguments
    ///
    /// * `rpc_method_name` - The name of the RPC method being called.
    /// * `params` - The parameters to pass to the RPC method.
    ///
    /// # Returns
    ///
    /// The result of the RPC call.
    async fn instrument_function<'async_trait, P, R>(
        &self,
        rpc_method_name: &str,
        params: P,
    ) -> TransportResult<R>
    where
        P: RpcParam + 'async_trait,
        R: RpcReturn + 'async_trait,
    {
        let start = Instant::now();
        let method_string = String::from(rpc_method_name);

        // send the request with the provided client (http or ws)
        let result = match (self.http_client.as_ref(), self.ws_client.as_ref()) {
            (Some(http_client), _) => http_client.raw_request(method_string.into(), params).await,
            (_, Some(ws_client)) => ws_client.raw_request(method_string.into(), params).await,
            (_, _) => unreachable!(),
        };

        let rpc_request_duration = start.elapsed();

        // we only observe the duration of successful calls (even though this is not well defined in the spec)
        self.rpc_collector.set_rpc_request_duration_seconds(
            rpc_method_name,
            self.net_version.to_string().as_str(),
            rpc_request_duration.as_secs_f64(),
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_consensus::{SignableTransaction, TxLegacy};
    use alloy_node_bindings::Anvil;
    use alloy_primitives::{bytes, TxKind::Call, U256};
    use alloy_provider::network::TxSignerSync;
    use alloy_rpc_types_eth::{
        pubsub::SubscriptionResult, BlockId, BlockNumberOrTag, BlockTransactionsKind,
    };
    use eigen_signer::signer::Config;
    use eigen_testing_utils::anvil_constants::ANVIL_RPC_URL;
    use serial_test::serial;
    use std::{thread, time};
    use tokio;

    const ANVIL_HTTP_URL: &str = "http://localhost:8545";
    const ANVIL_WS_URL: &str = "ws://localhost:8545";

    #[tokio::test]
    #[serial]
    async fn test_suggest_gas_tip_cap() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let fee_per_gas = instrumented_client.suggest_gas_tip_cap().await.unwrap();
        let expected_fee_per_gas = ANVIL_RPC_URL
            .clone()
            .get_max_priority_fee_per_gas()
            .await
            .unwrap();
        assert_eq!(expected_fee_per_gas as u64, fee_per_gas);
    }

    #[tokio::test]
    #[serial]
    async fn test_gas_price() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let gas_price = instrumented_client.suggest_gas_price().await.unwrap();
        let expected_gas_price = ANVIL_RPC_URL.clone().get_gas_price().await.unwrap();
        assert_eq!(gas_price, expected_gas_price as u64);
    }

    #[tokio::test]
    #[serial]
    async fn test_sync_status() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let sync_status = instrumented_client.sync_progress().await.unwrap();
        let expected_sync_status = ANVIL_RPC_URL.clone().syncing().await.unwrap();
        assert_eq!(expected_sync_status, sync_status);
    }

    #[tokio::test]
    #[serial]
    async fn test_chain_id() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();

        let expected_chain_id = ANVIL_RPC_URL.clone().get_chain_id().await.unwrap();
        let chain_id = instrumented_client.chain_id().await.unwrap();

        assert_eq!(expected_chain_id, chain_id);
    }

    #[tokio::test]
    #[serial]
    async fn test_balance_at() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let address = ANVIL_RPC_URL.get_accounts().await.unwrap()[0];

        let expected_balance_at = ANVIL_RPC_URL.get_balance(address).await.unwrap();
        let balance_at = instrumented_client
            .balance_at(address, BlockNumberOrTag::Latest)
            .await
            .unwrap();

        assert_eq!(expected_balance_at, balance_at);
    }

    #[tokio::test]
    #[serial]
    async fn test_subscribe_new_head() {
        let instrumented_client = InstrumentedClient::new_ws(ANVIL_WS_URL).await.unwrap();
        let subscription: TransportResult<Subscription<SubscriptionResult>> =
            instrumented_client.subscribe_new_head().await;
        assert!(subscription.is_ok())
    }

    #[tokio::test]
    #[serial]
    async fn test_subscribe_filter_logs() {
        let instrumented_client = InstrumentedClient::new_ws(ANVIL_WS_URL).await.unwrap();
        let address = ANVIL_RPC_URL.clone().get_accounts().await.unwrap()[0];
        let filter = Filter::new().address(address.to_string().parse::<Address>().unwrap());

        let subscription: TransportResult<Subscription<SubscriptionResult>> =
            instrumented_client.subscribe_filter_logs(filter).await;

        assert!(subscription.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_block_by_hash() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();

        // get the hash from the last block
        let hash = ANVIL_RPC_URL
            .get_block(BlockId::latest(), BlockTransactionsKind::Hashes)
            .await
            .unwrap()
            .unwrap()
            .header
            .hash
            .unwrap();

        let expected_block = ANVIL_RPC_URL
            .get_block_by_hash(hash, BlockTransactionsKind::Full)
            .await
            .unwrap();
        let block = instrumented_client.block_by_hash(hash).await.unwrap();

        assert_eq!(expected_block, block);
    }

    #[tokio::test]
    #[serial]
    async fn test_block_by_number() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let block_number = 1;

        let expected_block = ANVIL_RPC_URL
            .get_block_by_number(block_number.into(), true)
            .await
            .unwrap();
        let block = instrumented_client
            .block_by_number(block_number.into())
            .await
            .unwrap();

        assert_eq!(expected_block, block);
    }

    #[tokio::test]
    #[serial]
    async fn test_transaction_count() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();

        let block = ANVIL_RPC_URL
            .get_block(BlockId::latest(), BlockTransactionsKind::Hashes)
            .await
            .unwrap()
            .unwrap();

        let block_hash = block.header.hash.unwrap();
        let tx_count = instrumented_client
            .transaction_count(B256::from_slice(block_hash.as_slice()))
            .await
            .unwrap();
        let expected_tx_count = block.transactions.len();

        assert_eq!(tx_count, expected_tx_count as u64);
    }

    /// This test tests the following methods
    /// * `send_transaction`
    /// * `transaction_by_hash`
    /// * `transaction_receipt`
    /// * `transaction_in_block`
    #[tokio::test]
    #[serial]
    async fn test_transaction_methods() {
        // create a new anvil instance because otherwise it fails with "nonce too low" error.
        let anvil = Anvil::new().try_spawn().unwrap();
        let rpc_url: String = anvil.endpoint().parse().unwrap();

        let instrumented_client = InstrumentedClient::new(&rpc_url).await.unwrap();

        let addresses = anvil.addresses().to_vec();
        let private_key = anvil.keys().first().unwrap();
        let to = addresses.first().unwrap();
        // build the transaction
        let mut tx = TxLegacy {
            to: Call(*to),
            value: U256::from(0),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(31337),
        };

        let private_key_hex = hex::encode(private_key.to_bytes());
        let config = Config::PrivateKey(private_key_hex);
        let signer = Config::signer_from_config(config).unwrap();
        let signature = signer.sign_transaction_sync(&mut tx).unwrap();
        let signed_tx = tx.into_signed(signature);
        let tx: TxEnvelope = signed_tx.into();

        // test send_transaction
        let tx_hash = instrumented_client.send_transaction(tx).await;
        assert!(tx_hash.is_ok());
        let tx_hash = B256::from_slice(tx_hash.unwrap().as_ref());

        // test transaction_by_hash
        let tx_by_hash = instrumented_client.transaction_by_hash(tx_hash).await;
        assert!(tx_by_hash.is_ok());

        // this sleep is needed because we have to wait since the transaction is not ready yet
        thread::sleep(time::Duration::from_secs(1));

        // test transaction_receipt
        let receipt = instrumented_client.transaction_receipt(tx_hash).await;
        assert!(receipt.is_ok());
        let receipt = receipt.unwrap();

        // test transaction_in_block
        let tx_by_block = instrumented_client
            .transaction_in_block(
                receipt.block_hash.unwrap(),
                receipt.transaction_index.unwrap(),
            )
            .await;
        assert!(tx_by_block.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_estimate_gas() {
        let instrumented_client = InstrumentedClient::new("http:/localhost:8545")
            .await
            .unwrap();
        let anvil = ANVIL_RPC_URL.clone();
        let accounts = anvil.get_accounts().await.unwrap();
        let from = accounts.first().unwrap();
        let to = accounts.get(1).unwrap();

        // build the transaction
        let tx = TxLegacy {
            to: Call(*to),
            value: U256::from(0),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 1_000_000,
            input: bytes!(),
            chain_id: Some(31337),
        };
        let tx_request: TransactionRequest = tx.clone().into();
        let tx_request = tx_request.from(*from);

        let expected_estimated_gas = anvil.clone().estimate_gas(&tx_request).await.unwrap();
        let estimated_gas = instrumented_client.estimate_gas(tx_request).await.unwrap();
        assert_eq!(expected_estimated_gas, estimated_gas.into());
    }

    #[tokio::test]
    #[serial]
    async fn test_call_contract_and_pending_call_contract() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();

        let anvil = ANVIL_RPC_URL.clone();
        let accounts = anvil.get_accounts().await.unwrap();
        let from = accounts.first().unwrap();
        let to = accounts.get(1).unwrap();

        let nonce = instrumented_client
            .nonce_at(*from, BlockNumberOrTag::Latest)
            .await
            .unwrap();

        // build the transaction
        let tx = TxLegacy {
            to: Call(*to),
            value: U256::from(0),
            gas_limit: 1_000_000,
            nonce,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(31337),
        };
        let tx_request: TransactionRequest = tx.clone().into();
        let tx_request = tx_request.from(*from);

        // test call_contract
        let expected_bytes = anvil.call(&tx_request).await.unwrap();
        let bytes = instrumented_client
            .call_contract(tx_request.clone(), BlockNumberOrTag::Earliest)
            .await
            .unwrap();
        assert_eq!(expected_bytes, bytes);

        // test pending_call_contract
        let bytes = instrumented_client.pending_call_contract(tx_request).await;
        assert!(bytes.is_ok());
    }

    #[tokio::test]
    #[serial]
    async fn test_filter_logs() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let address = ANVIL_RPC_URL.clone().get_accounts().await.unwrap()[0];
        let filter = Filter::new().address(address.to_string().parse::<Address>().unwrap());

        let expected_logs = ANVIL_RPC_URL.clone().get_logs(&filter).await.unwrap();
        let logs = instrumented_client.filter_logs(filter).await.unwrap();

        assert_eq!(expected_logs, logs);
    }

    #[tokio::test]
    #[serial]
    async fn test_storage_at() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();

        let account = ANVIL_RPC_URL.clone().get_accounts().await.unwrap()[0];
        let expected_storage = ANVIL_RPC_URL
            .clone()
            .get_storage_at(account, U256::ZERO)
            .await
            .unwrap();

        let storage = instrumented_client
            .storage_at(account, U256::ZERO, U256::ZERO)
            .await
            .unwrap();

        assert_eq!(expected_storage, storage);
    }

    #[tokio::test]
    #[serial]
    async fn test_block_number() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();

        let expected_block_number = ANVIL_RPC_URL.clone().get_block_number().await.unwrap();
        let block_number = instrumented_client.block_number().await.unwrap();

        assert_eq!(expected_block_number, block_number);
    }

    #[tokio::test]
    #[serial]
    async fn test_code_at() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let address = ANVIL_RPC_URL.get_accounts().await.unwrap()[0];

        let expected_code = ANVIL_RPC_URL.get_code_at(address).await.unwrap();
        let code = instrumented_client
            .code_at(address, BlockNumberOrTag::Latest)
            .await
            .unwrap();

        assert_eq!(expected_code, code);
    }

    #[tokio::test]
    #[serial]
    async fn test_fee_history() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let block_count = 4;
        let last_block = BlockNumberOrTag::Latest;
        let reward_percentiles = [0.2, 0.3];

        let expected_fee_history = ANVIL_RPC_URL
            .get_fee_history(block_count, last_block, &reward_percentiles)
            .await
            .unwrap();
        let fee_history = instrumented_client
            .fee_history(block_count, last_block, &reward_percentiles)
            .await
            .unwrap();

        assert_eq!(expected_fee_history, fee_history);
    }

    #[tokio::test]
    #[serial]
    async fn test_header_by_hash() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let hash = ANVIL_RPC_URL
            .get_block(BlockId::latest(), BlockTransactionsKind::Hashes)
            .await
            .unwrap()
            .unwrap()
            .header
            .hash
            .unwrap();

        let expected_header = ANVIL_RPC_URL
            .get_block_by_hash(hash, BlockTransactionsKind::Hashes)
            .await
            .unwrap()
            .unwrap()
            .header;
        let header = instrumented_client.header_by_hash(hash).await.unwrap();

        assert_eq!(expected_header, header);
    }

    #[tokio::test]
    #[serial]
    async fn test_header_by_number() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let block_number = BlockNumberOrTag::Earliest;

        let header = instrumented_client
            .header_by_number(block_number)
            .await
            .unwrap();

        let expected_header = ANVIL_RPC_URL
            .get_block_by_number(block_number, false)
            .await
            .unwrap()
            .unwrap()
            .header;

        assert_eq!(expected_header, header);
    }

    #[tokio::test]
    #[serial]
    async fn test_nonce_at() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let address = ANVIL_RPC_URL.get_accounts().await.unwrap()[0];

        let expected_nonce = ANVIL_RPC_URL.get_transaction_count(address).await.unwrap();
        let nonce = instrumented_client
            .nonce_at(address, BlockNumberOrTag::Latest)
            .await
            .unwrap();

        assert_eq!(expected_nonce, nonce);
    }

    #[tokio::test]
    #[serial]
    async fn test_pending_balance_at() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let address = ANVIL_RPC_URL.get_accounts().await.unwrap()[0];

        // TODO: currently comparing "pending" balance with "latest" balance. Check for pending transactions?
        let expected_balance = ANVIL_RPC_URL.get_balance(address).await.unwrap();
        let balance = instrumented_client
            .pending_balance_at(address)
            .await
            .unwrap();

        assert_eq!(expected_balance, balance);
    }

    #[tokio::test]
    #[serial]
    async fn test_pending_code_at() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let address = ANVIL_RPC_URL.get_accounts().await.unwrap()[0];

        // TODO: currently comparing "pending" with "latest". Check for pending transactions?
        let expected_code = ANVIL_RPC_URL.get_code_at(address).await.unwrap();
        let code = instrumented_client.pending_code_at(address).await.unwrap();

        assert_eq!(expected_code, code);
    }

    #[tokio::test]
    #[serial]
    async fn test_pending_nonce_at() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let address = ANVIL_RPC_URL.get_accounts().await.unwrap()[0];

        // TODO: currently comparing "pending" with "latest". Check for pending transactions?
        let expected_pending_nonce_at = ANVIL_RPC_URL.get_transaction_count(address).await.unwrap();
        let pending_nonce_at = instrumented_client.pending_nonce_at(address).await.unwrap();

        assert_eq!(expected_pending_nonce_at, pending_nonce_at);
    }

    #[tokio::test]
    #[serial]
    async fn test_pending_storage_at() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();
        let address = ANVIL_RPC_URL.get_accounts().await.unwrap()[0];
        let key = U256::from(10);

        // TODO: currently comparing "pending" with "latest". Check for pending transactions?
        // TODO: set storage and check change
        let expected_pending_storage_at = ANVIL_RPC_URL.get_storage_at(address, key).await.unwrap();
        let pending_storage_at = instrumented_client
            .pending_storage_at(address, key)
            .await
            .unwrap();

        assert_eq!(expected_pending_storage_at, pending_storage_at);
    }

    #[tokio::test]
    #[serial]
    async fn test_pending_transaction_count() {
        let instrumented_client = InstrumentedClient::new(ANVIL_HTTP_URL).await.unwrap();

        let expected_transaction_count: u64 = ANVIL_RPC_URL
            .get_block_by_number(BlockNumberOrTag::Pending, false)
            .await
            .unwrap()
            .unwrap()
            .transactions
            .len() as u64;

        let transaction_count = instrumented_client.pending_transaction_count().await;

        assert_eq!(expected_transaction_count, transaction_count.unwrap());
    }
}
