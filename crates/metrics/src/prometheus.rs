use metrics_exporter_prometheus::PrometheusBuilder;
use metrics_util::MetricKindMask;
use std::{net::SocketAddr, time::Duration};

#[allow(unused)]
fn init_registry(socket_addr: SocketAddr) {
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
    use super::*;
    use crate::eigenmetrics::EigenPerformanceMetrics;
    use eigen_metrics_collectors_economic::RegisteredStakesMetrics;
    use eigen_metrics_collectors_rpc_calls::RpcCallsMetrics;
    use tokio::time::sleep;
    use tokio::time::Duration;

    #[tokio::test]
    async fn test_prometheus_server() {
        use eigen_logging::get_test_logger;
        let socket: SocketAddr = "127.0.0.1:9091".parse().unwrap();
        init_registry(socket);

        // Initialize all the metrics
        let metrics = EigenPerformanceMetrics::new(get_test_logger());
        let registered_metrics = RegisteredStakesMetrics::new(get_test_logger());
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
        registered_metrics.set_stake("4th", "hello Eigen", 8.0);

        sleep(Duration::from_secs(1)).await;

        body = get_metrics_body(&client, "http://127.0.0.1:9091/metrics").await;
        assert!(body.contains("eigen_performance_score 80"));
        assert!(body.contains(
            "Key_eigen_registered_stakes___quorum_number___4th__quorum_name___hello_Eigen__ 8"
        ));
        assert!(body.contains("eigen_rpc_request_duration_seconds___method____eth_getBlockByNumber__client_version___rethv1_0_3__{quantile=\"1\"} 100"));
        assert!(body.contains("eigen_rpc_request_total___method___eth_getBlockByNumber__client_version___rethv1_0_3__ 10"));
    }
}
