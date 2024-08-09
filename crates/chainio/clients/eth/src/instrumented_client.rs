//use eigen_metrics_collectors_rpc_calls::RpcCalls as RpcCallsCollector;
use alloy_json_rpc::{RpcParam, RpcReturn};
use alloy_primitives::{
    Address, BlockHash, BlockNumber, Bytes, ChainId, Uint, B256, U128, U256, U64,
};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_rpc_types_eth::{
    Block, FeeHistory, Header, SyncStatus, Transaction, TransactionReceipt, TransactionRequest,
};
use alloy_transport::TransportResult;
use alloy_transport_http::{Client, Http};
use eigen_logging::get_test_logger;
use eigen_logging::logger::Logger;
use eigen_metrics_collectors_rpc_calls::RpcCallsMetrics as RpcCallsCollector;
use std::time::Instant;
use thiserror::Error;
use url::Url;

const PENDING_TAG: &str = "pending";
const HEX_RADIX: u32 = 16;
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

    pub async fn filter_logs(&self) {
        todo!()
    }

    pub async fn pending_call_contract(&self) {
        todo!()
    }

    pub async fn subscribe_filter_logs(&self) {
        todo!()
    }

    pub async fn transaction_by_hash(&self) {
        todo!()
    }

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
            .map(|result: U64| result.to())
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

    pub async fn header_by_number(&self, block_number: BlockNumber) -> TransportResult<Header> {
        let transaction_detail = false;
        self.instrument_function("eth_getBlockByNumber", (block_number, transaction_detail))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get header by number", &[err])
            })
    }

    pub async fn nonce_at(&self, account: Address, block_number: U256) -> TransportResult<u64> {
        self.instrument_function("eth_getTransactionCount", (account, block_number))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get nonce", &[err])
            })
    }

    pub async fn pending_balance_at(&self, account: Address) -> TransportResult<U256> {
        self.instrument_function("eth_getBalance", (account, PENDING_TAG))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get pending balance", &[err])
            })
    }

    pub async fn pending_code_at(&self, account: Address) -> TransportResult<Bytes> {
        self.instrument_function("eth_getCode", (account, PENDING_TAG))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get pending code", &[err])
            })
    }

    pub async fn pending_nonce_at(&self, account: Address) -> TransportResult<u64> {
        self.instrument_function("eth_getTransactionCount", (account, PENDING_TAG))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get pending nonce", &[err])
            })
    }

    pub async fn pending_storage_at(&self, account: Address, key: B256) -> TransportResult<U256> {
        self.instrument_function("eth_getStorageAt", (account, key, PENDING_TAG))
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get pending storage", &[err])
            })
    }

    pub async fn pending_transaction_count(&self) -> TransportResult<u64> {
        self.instrument_function("eth_getBlockTransactionCountByNumber", PENDING_TAG)
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to get transaction count", &[err])
            })
    }

    pub async fn send_transaction(&self, tx: TransactionRequest) -> TransportResult<B256> {
        // TODO: encode the transaction
        self.instrument_function("eth_sendRawTransaction", tx)
            .await
            .inspect_err(|err| {
                self.rpc_collector
                    .logger()
                    .error("Failed to send transaction", &[err])
            })
    }

    pub async fn storage_at(
        &self,
        account: Address,
        key: [u8; 32],
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
            .map(|result: U64| result.to())
    }

    // TODO: Check if this method is properly named
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{bytes, TxKind::Call, U256};
    use alloy_rpc_types_eth::{BlockNumberOrTag, TransactionRequest};
    use eigen_logging::{log_level::LogLevel, logger::Logger, tracing_logger::TracingLogger};
    use eigen_testing_utils::anvil_constants::ANVIL_RPC_URL;
    use once_cell::sync::OnceCell;
    use tokio;

    static TEST_LOGGER: OnceCell<TracingLogger> = OnceCell::new();
    const PRIVATE_KEY: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";

    #[tokio::test]
    async fn test_send_transaction_from_legacy() {
        // TEST_LOGGER.get_or_init(|| {
        //     TracingLogger::new_text_logger(false, String::from(""), LogLevel::Debug, false)
        // });

        // Spin up a local Anvil node.
        // Ensure `anvil` is available in $PATH.
        // let anvil = Anvil::new().try_spawn().unwrap();
        // let rpc_url: String = anvil.endpoint().parse().unwrap();

        // Create a provider.
        // let logger = TEST_LOGGER.get().unwrap();

        // Create two users, Alice and Bob.
        // let _alice = anvil.addresses()[0];
        // let _bob = anvil.addresses()[1];
        let instrumented_client = InstrumentedClient::new("http://localhost:8545")
            .await
            .unwrap();

        let gas_price = instrumented_client.suggest_gas_price().await;
        assert!(gas_price.is_ok());

        let fee_per_gas = instrumented_client.suggest_gas_tip_cap().await;
        assert!(fee_per_gas.is_ok());

        let sync_status = instrumented_client.sync_progress().await;
        assert!(sync_status.is_ok());
    }

    #[tokio::test]
    async fn test_block_number() {
        let instrumented_client = InstrumentedClient::new("http://localhost:8545")
            .await
            .unwrap();

        let expected_block_number = ANVIL_RPC_URL.clone().get_block_number().await.unwrap();
        let block_number = instrumented_client.block_number().await.unwrap();

        assert_eq!(expected_block_number, block_number);
    }

    #[tokio::test]
    async fn test_suggest_gas_tip_cap() {
        let instrumented_client = InstrumentedClient::new("http://localhost:8545")
            .await
            .unwrap();

        let expected_chain_id = ANVIL_RPC_URL.clone().get_chain_id().await.unwrap();
        let chain_id = instrumented_client.chain_id().await.unwrap();

        assert_eq!(expected_chain_id, chain_id);
    }
}
