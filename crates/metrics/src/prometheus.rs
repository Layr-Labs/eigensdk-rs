use crate::eigenmetrics::EigenMetrics;
use eyre::Result;
use hyper::{
    body::Body,
    service::{make_service_fn, service_fn},
    Request, Response, Server,
};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::{convert::Infallible, net::SocketAddr};
use tokio::runtime::Runtime;
use tokio::time::sleep;
use tokio::time::Duration;

fn init_registry() -> PrometheusHandle {
    let recorder = PrometheusBuilder::new().build_recorder();
    let handle = recorder.handle();
    let boxed_recorder = Box::new(recorder);
    let static_recorder: &'static dyn metrics::Recorder = Box::leak(boxed_recorder);
    metrics::set_recorder(static_recorder).expect("failed to set metrics recorder");
    handle
}

async fn serve_metrics(addr: SocketAddr, handle: PrometheusHandle) -> eyre::Result<()> {
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

#[tokio::test]
async fn test_prometheus_server() {
    let socket: SocketAddr = "127.0.0.1:9091".parse().unwrap();
    let handle = init_registry();

    // Initialize EigenMetrics
    let metrics = EigenMetrics::new();

    // Run the metrics server in a background task
    let server_handle = tokio::spawn(async move {
        serve_metrics(socket, handle).await.unwrap();
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

    sleep(Duration::from_secs(1)).await;

    body = get_metrics_body(&client, "http://127.0.0.1:9091/metrics").await;
    assert!(body.contains("eigen_performance_score 80"));
    // Shutdown the server
    server_handle.abort();
}
