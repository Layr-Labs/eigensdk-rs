//use eigen_metrics_collectors_rpc_calls::RpcCalls as RpcCallsCollector;
use alloy_provider::{Provider, ProviderBuilder, RootProvider};
use alloy_transport_http::{Client, Http};
use thiserror::Error;
use url::Url;

use crate::client;

pub struct InstrumentedClient {
    client: RootProvider<Http<Client>>,
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
        Ok(InstrumentedClient {
            client,
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

        Ok(InstrumentedClient {
            client: client.clone(),
            net_version,
        })
    }
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
