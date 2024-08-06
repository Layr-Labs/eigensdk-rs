//use eigen_metrics_collectors_rpc_calls::RpcCalls as RpcCallsCollector;
use alloy_json_rpc::{RpcParam, RpcReturn};
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_transport::TransportResult;
use alloy_transport_http::{reqwest::Method, Client, Http};
use eigen_metrics_collectors_rpc_calls::RpcCalls;
use std::time::Instant;
use thiserror::Error;
use url::Url;

pub struct InstrumentedClient {
    client: RootProvider<Http<Client>>,
    rpc_collector: RpcCalls,
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
        let rpc_collector = RpcCalls::new();
        Ok(InstrumentedClient {
            client,
            rpc_collector,
            net_version,
        })
    }

    pub async fn new_from_client(
        client: &RootProvider<Http<Client>>,
    ) -> Result<Self, InstrumentedClientError> {
        let net_version = client
            .get_net_version()
            .await
            .map_err(|_| InstrumentedClientError::ErrorGettingVersion)?;

        let rpc_collector = RpcCalls::new();
        Ok(InstrumentedClient {
            client: client.clone(),
            rpc_collector,
            net_version,
        })
    }

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

    /*
    // Generic function used to instrument all the eth calls that we make below
    func instrumentFunction[T any](
        rpcCall func() (T, error),
        rpcMethodName string,
        iec *InstrumentedClient,
    ) (value T, err error) {
        start := time.Now()
        result, err := rpcCall()
        // we count both successful and erroring calls (even though this is not well defined in the spec)
        iec.rpcCallsCollector.AddRPCRequestTotal(rpcMethodName, iec.clientAndVersion)
        if err != nil {
            return value, err
        }
        rpcRequestDuration := time.Since(start)
        // we only observe the duration of successful calls (even though this is not well defined in the spec)
        iec.rpcCallsCollector.ObserveRPCRequestDurationSeconds(
            float64(rpcRequestDuration),
            rpcMethodName,
            iec.clientAndVersion,
        )
        return result, nil
    }


         */
}

// https://docs.rs/alloy-provider/0.1.4/alloy_provider/trait.Provider.html#method.raw_request
/*
// No parameters: `()`
let block_number = provider.raw_request("eth_blockNumber".into(), ()).await?;

// One parameter: `(param,)` or `[param]`
let block = provider.raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Latest,)).await?;

// Two or more parameters: `(param1, param2, ...)` or `[param1, param2, ...]`
let full_block = provider.raw_request("eth_getBlockByNumber".into(), (BlockNumberOrTag::Latest, true)).await?;
*/
