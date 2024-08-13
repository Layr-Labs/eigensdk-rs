use eigen_logging::{logger::Logger, EigenLogger};
use hyper::{
    body::Body,
    service::{make_service_fn, service_fn},
    Request, Response, Server,
};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::{convert::Infallible, net::SocketAddr};

#[allow(unused)]
fn init_registry() -> PrometheusHandle {
    let recorder = PrometheusBuilder::new().build_recorder();
    let handle = recorder.handle();
    let boxed_recorder = Box::new(recorder);
    let static_recorder: &'static dyn metrics::Recorder = Box::leak(boxed_recorder);
    metrics::set_recorder(static_recorder).expect("failed to set metrics recorder");
    handle
}

#[allow(unused)]
async fn serve_metrics(
    addr: SocketAddr,
    handle: PrometheusHandle,
    logger: EigenLogger,
) -> eyre::Result<()> {
    match (&logger.tracing_logger, &logger.noop_logger) {
        (Some(tracing_logger), _) => tracing_logger.info(
            &format!("Starting metrics server at port {}", addr),
            &["eigen-metrics.serve_metrics"],
        ),
        (_, Some(noop_logger)) => noop_logger.info(
            &format!("Starting metrics server at port {}", addr),
            &["eigen-metrics.serve_metrics"],
        ),
        _ => println!("Both TracingLogger and NoopLogger are None"),
    }
    let make_svc = make_service_fn(move |_| {
        let handle = handle.clone();

        async move {
            Ok::<_, Infallible>(service_fn(move |_: Request<Body>| {
                let metrics = handle.render();
                async move { Ok::<_, Infallible>(Response::new(Body::from(metrics))) }
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    server.await?;
    Ok(())
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
        let handle = init_registry();

        // Initialize EigenMetrics
        let metrics = EigenPerformanceMetrics::new(get_test_logger());
        let registered_metrics = RegisteredStakesMetrics::new(get_test_logger());
        let rpc_calls = RpcCallsMetrics::new(get_test_logger());

        // Run the metrics server in a background task
        let server_handle = tokio::spawn(async move {
            serve_metrics(socket, handle, get_test_logger())
                .await
                .unwrap();
        });

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

        metrics.performance_score().set(80.0);
        rpc_calls.set_rpc_request_duration_seconds("eth_getBlockByNumber", "rethv1.0.3", 100.0);
        rpc_calls.set_rpc_request_total("eth_getBlockByNumber", "rethv1.0.3", 10);
        registered_metrics.set_stake("4th", "hello Eigen", 8.0);

        sleep(Duration::from_secs(1)).await;

        body = get_metrics_body(&client, "http://127.0.0.1:9091/metrics").await;
        assert!(body.contains("eigen_performance_score 80"));
        assert!(body.contains(
            "Key_eigen_registered_stakes___quorum_number___4th__quorum_name___hello_Eigen__ 8"
        ));
        assert!(body.contains("eigen_rpc_request_duration_seconds___method___eth_getBlockByNumber__client_version___rethv1_0_3__{quantile=\"1\"} 100"));
        assert!(body.contains("eigen_rpc_request_total___method___eth_getBlockByNumber__client_version___rethv1_0_3__ 10"));
        // Shutdown the server
        server_handle.abort();
    }
}
