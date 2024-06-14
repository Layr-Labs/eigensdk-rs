use eigen_metrics_derive::Metrics;
use metrics::{Counter, Gauge};
/// TODO implement other metrics also . For now only perfomrmance is supported.
#[derive(Clone, Metrics)]
#[metrics(scope = "eigenmetrics.performancemetrics")]
pub struct EigenMetrics {
    /// performance score
    #[metric(
        rename = "eigen_performance_score",
        describe = " A gauge with performance score "
    )]
    performance_score: Gauge,
}

impl EigenMetrics {
    pub fn new() -> Self {
        let gauge = Self {
            performance_score: metrics::register_gauge!("eigen_performance_score"),
        };
        EigenMetrics::describe();
        gauge.performance_score.set(100 as f64);
        gauge
    }

    pub fn performance_score(&self) -> Gauge {
        self.performance_score.clone()
    }
}
