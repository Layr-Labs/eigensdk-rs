//use eigen_metrics_collectors_rpc_calls::RpcCalls as RpcCallsCollector;
use alloy_json_rpc::{RpcParam, RpcReturn};
use alloy_primitives::{Address, BlockHash, BlockNumber, Bytes, ChainId, U256};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_rpc_types_eth::{Block, FeeHistory, SyncStatus, Transaction, TransactionReceipt};
use alloy_transport::TransportResult;
use alloy_transport_http::{reqwest::Method, Client, Http};
use eigen_logging::get_test_logger;
use eigen_logging::logger::Logger;
use eigen_metrics_collectors_rpc_calls::RpcCallsMetrics as RpcCallsCollector;
use std::time::Instant;
use thiserror::Error;
use url::Url;

pub struct InstrumentedClient {
    client: RootProvider<Http<Client>>,
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
    pub async fn new(url: &str) -> Result<Self, InstrumentedClientError> {
        let url = Url::parse(url).map_err(|_| InstrumentedClientError::InvalidUrl)?;
        let client = ProviderBuilder::new().on_http(url);

        let net_version = client
            .get_net_version()
            .await
            .map_err(|_| InstrumentedClientError::ErrorGettingVersion)?;

        let rpc_collector = RpcCallsCollector::new(get_test_logger().clone());
        Ok(InstrumentedClient {
            client,
            rpc_collector,
            net_version,
        })
    }

    pub async fn new_from_client(
        client: RootProvider<Http<Client>>,
    ) -> Result<Self, InstrumentedClientError> {
        let net_version = client
            .get_net_version()
            .await
            .map_err(|_| InstrumentedClientError::ErrorGettingVersion)?;

        let rpc_collector = RpcCallsCollector::new(get_test_logger().clone());
        Ok(InstrumentedClient {
            client,
            rpc_collector,
            net_version,
        })
    }

    pub async fn chain_id(&self) -> TransportResult<ChainId> {
        self.instrument_function("eth_chainId", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get chain id", &[err])
            })
    }

    pub async fn balance_at(
        &self,
        account: Address,
        block_number: BlockNumber,
    ) -> TransportResult<U256> {
        self.instrument_function("eth_getBalance", (account, block_number))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get balance", &[err])
            })
    }

    pub async fn block_by_hash(&self, hash: BlockHash) -> TransportResult<Option<Block>> {
        self.instrument_function("eth_getBlockByHash", (hash, true))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get block by hash", &[err])
            })
    }

    pub async fn block_by_number(&self, number: BlockNumber) -> TransportResult<Option<Block>> {
        self.instrument_function("eth_getBlockByNumber", (number, true))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get block by number", &[err])
            })
    }

    pub async fn block_number(&self) -> TransportResult<BlockNumber> {
        self.instrument_function("eth_blockNumber", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get block number", &[err])
            })
    }

    pub async fn code_at(
        &self,
        address: Address,
        block_number: BlockNumber,
    ) -> TransportResult<Bytes> {
        self.instrument_function("eth_getCode", (address, block_number))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get code", &[err])
            })
    }

    pub async fn fee_history(
        &self,
        block_count: u64,
        last_block: BlockNumber,
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

    pub async fn subscribe_new_head(&self) -> TransportResult<u128> {
        self.instrument_function("eth_subscribe", "newHeads")
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to subscribe new head", &[err])
            })
    }

    pub async fn suggest_gas_price(&self) -> TransportResult<u64> {
        self.instrument_function("eth_gasPrice", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to suggest gas price", &[err])
            })
    }

    // Check if this method is properly named
    pub async fn suggest_gas_tip_cap(&self) -> TransportResult<u64> {
        self.instrument_function("eth_maxPriorityFeePerGas", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to suggest gas tip cap", &[err])
            })
    }

    pub async fn sync_progress(&self) -> TransportResult<SyncStatus> {
        self.instrument_function("eth_syncing", ())
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get sync progress", &[err])
            })
    }

    pub async fn transaction_in_block(
        &self,
        block_hash: [u8; 32],
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

    pub async fn transaction_count(&self, block_hash: Address) -> TransportResult<u64> {
        self.instrument_function("eth_getBlockTransactionCountByHash", block_hash)
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get transaction count", &[err])
            })
    }

    pub async fn transaction_receipt(
        &self,
        tx_hash: [u8; 32],
    ) -> TransportResult<TransactionReceipt> {
        self.instrument_function("eth_getTransactionReceipt", tx_hash)
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
        let result = self.client.raw_request(method_string.into(), params).await;
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
