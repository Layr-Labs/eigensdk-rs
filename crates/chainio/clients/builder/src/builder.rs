//! builder
use eigensdk_client_avsregistry::{
    reader::AvsRegistryChainReader, subscriber::AvsRegistryChainSubscriber,
    writer::AvsRegistryChainWriter,
};
use eigensdk_client_elcontracts::{reader::ELChainReader, writer::ELChainWriter};
use eigensdk_client_eth::client::Client;
use eigensdk_metrics::eigenmetrics::EigenMetrics;
use prometheus_client::registry::Registry;

struct BuildAllConfig {
    EthHttpUrl: String,
    EthWsUrl: String,
    RegistryCoordinatorAddr: String,
    AvsName: String,
    FromMetricsIpPortAddress: String,
}

struct Clients {
    avs_registry_chain_reader: AvsRegistryChainReader,
    avs_registry_chain_subscriber: AvsRegistryChainSubscriber,
    avs_registry_chain_writer: AvsRegistryChainWriter,
    el_chain_reader: ELChainReader,
    el_chain_writer: ELChainWriter,
    eth_http_client: Client,
    eth_ws_client: Client,
    metrics: EigenMetrics,
    prometheus_registry: Registry,
}
