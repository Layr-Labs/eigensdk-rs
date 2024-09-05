use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use std::{net::SocketAddr, time::Duration};

#[allow(unused)]
pub fn init_registry(socket_addr: SocketAddr) {
    PrometheusBuilder::new()
        .with_http_listener(socket_addr)
        .idle_timeout(
            MetricKindMask::COUNTER | MetricKindMask::HISTOGRAM,
            Some(Duration::from_secs(10)),
        )
        .install()
        .expect("failed to install Prometheus recorder");
}

#[cfg(test)]

mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::eigenmetrics::EigenPerformanceMetrics;
    use alloy_primitives::Address;
    use alloy_primitives::FixedBytes;
    use eigen_client_avsregistry::reader::AvsRegistryChainReader;
    use eigen_client_elcontracts::reader::ELChainReader;
    use eigen_metrics_collectors_economic::fake_collector::FakeCollector;
    use eigen_metrics_collectors_rpc_calls::RpcCallsMetrics;
    use eigen_testing_utils::anvil_constants;
    use num_bigint::BigInt;
    use tokio::time::sleep;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_prometheus_server() {
        use eigen_logging::get_test_logger;
        let socket: SocketAddr = "127.0.0.1:9091".parse().unwrap();
        init_registry(socket);

        let operator_addr = Address::ZERO;
        let operator_id = FixedBytes::<32>::default();
        let http_anvil = "http://localhost:8545";
        let el_reader = ELChainReader::new(
            get_test_logger(),
            Address::ZERO,
            anvil_constants::get_delegation_manager_address().await,
            anvil_constants::get_avs_directory_address().await,
            http_anvil.to_string(),
        );
        let avs_registry_reader = AvsRegistryChainReader::new(
            get_test_logger(),
            anvil_constants::get_registry_coordinator_address().await,
            anvil_constants::get_operator_state_retriever_address().await,
            http_anvil.to_string(),
        )
        .await
        .unwrap();

        let mut quorums_names = HashMap::new();
        quorums_names.insert(1, "rust".to_string());
        let avs_name = "eigensdk-rs";
        let mut collector = FakeCollector::new(
            get_test_logger(),
            operator_addr,
            operator_id,
            el_reader,
            avs_registry_reader,
            quorums_names,
            avs_name,
        );

        collector.set_stake("1", "first", avs_name, 2.0);
        sleep(Duration::from_secs(1)).await;
        let client = reqwest::Client::new();
        let mut body = get_metrics_body(&client, "http://127.0.0.1:9091/metrics").await;

        assert!(body.contains("eigen_registered_stakes"));
        assert!(body.contains("quorum_number___1"));
        assert!(body.contains("quorum_name___first"));
        assert!(body.contains("avs_name___eigensdk_rs"));
        assert!(body.contains("eigen_registered_stakes___quorum_number___1__quorum_name___first__avs_name___eigensdk_rs__ 2"));

        let is_operator_frozen: bool = true;
        let mut quorum_stake_map = HashMap::new();
        quorum_stake_map.insert(1, BigInt::from(23));
        let _ = collector
            .collect(is_operator_frozen, quorum_stake_map)
            .await;

        body = get_metrics_body(&client, "http://127.0.0.1:9091/metrics").await;

        assert!(body.contains("eigen_registered_stakes___quorum_number___1__quorum_name___rust__avs_name___eigensdk_rs__ 23"));

        // Initialize all the metrics
        let metrics = EigenPerformanceMetrics::new(get_test_logger());
        let rpc_calls = RpcCallsMetrics::new(get_test_logger());

        sleep(Duration::from_secs(1)).await;

        async fn get_metrics_body(client: &reqwest::Client, url: &str) -> String {
            let resp = client.get(url).send().await.unwrap();
            resp.text().await.unwrap()
        }
        let client = reqwest::Client::new();
        let resp = client
            .get("http://127.0.0.1:9091/metrics")
            .send()
            .await
            .unwrap();

        let mut body = resp.text().await.unwrap();
        assert!(body.contains("eigen_performance_score 100"));

        metrics.set_performance_score(80.0);
        rpc_calls.set_rpc_request_duration_seconds("eth_getBlockByNumber", "rethv1.0.3", 100.0);
        rpc_calls.set_rpc_request_total("eth_getBlockByNumber", "rethv1.0.3", 10);

        sleep(Duration::from_secs(1)).await;

        body = get_metrics_body(&client, "http://127.0.0.1:9091/metrics").await;
        assert!(body.contains("eigen_performance_score 80"));
        assert!(body.contains("eigen_rpc_request_duration_seconds___method____eth_getBlockByNumber__client_version___rethv1_0_3__{quantile=\"1\"} 100"));
        assert!(body.contains("eigen_rpc_request_total___method___eth_getBlockByNumber__client_version___rethv1_0_3__ 10"));
    }
}
