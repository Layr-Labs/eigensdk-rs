//use eigen_metrics_collectors_rpc_calls::RpcCalls as RpcCallsCollector;
use alloy_json_rpc::{RpcParam, RpcReturn};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_transport::TransportResult;
use alloy_transport_http::{reqwest::Method, Client, Http};
use eigen_logging::get_test_logger;
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

    /*
    func (iec *InstrumentedClient) BlockByHash(ctx context.Context, hash common.Hash) (*types.Block, error) {
        blockByHash := func() (*types.Block, error) { return iec.client.BlockByHash(ctx, hash) }
        block, err := instrumentFunction[*types.Block](blockByHash, "eth_getBlockByHash", iec)
        if err != nil {
            return nil, err
        }
        return block, nil
    }
    */
    pub async fn block_by_hash(&self, hash: String) -> TransportResult<()> {
        // WIP TODO!!!
        //let block_by_hash = || self.client.get_block_by_hash(hash);
        //self.instrument_function("eth_getBlockByHash", block_by_hash)
        //    .await
        todo!()
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
