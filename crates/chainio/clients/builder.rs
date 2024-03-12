//! builder
use prometheus_client::registry::Registry;
use avsregistry::{AvsRegistryChainReader,AvsRegistryChainSubscriber,AvsRegistryChainWriter};
use elcontracts::{ELChainReader,ELChainWriter};
use eth::Client;
use eigensdk_metrics::EigenMetrics;


struct BuildAllConfig {
    String EthHttpUrl,
    String EthWsUrl,
    String RegistryCoordinatorAddr,
    String AvsName,
    String FromMetricsIpPortAddress
}


struct Clients {
    avs_registry_chain_reader : AvsRegistryChainReader,
    avs_registry_chain_subscriber : AvsRegistryChainSubscriber,
    avs_registry_chain_writer : AvsRegistryChainWriter,
    el_chain_reader : ELChainReader,
    el_chain_writer : ELChainWriter,
    eth_http_client : Client,
    eth_ws_client : Client,
    metrics : EigenMetrics,
    prometheus_registry: Registry
}