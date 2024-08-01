use eigen_logging::tracing_logger::TracingLogger;
use eigen_metrics_derive::Metrics;
use metrics::Gauge;

// Performance Metrics
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

// TODO(supernova) : feeearnedtotal is not turned on yet,so not implemented yet
// https://github.com/Layr-Labs/eigensdk-go/blob/67787e959b727b115628a34e796df3a9ef42f646/metrics/eigenmetrics.go#L23

#[derive(Debug)]
pub struct EigenMetricsMetrics {
    eigen_metrics: EigenMetrics,

    logger: TracingLogger,
}

impl EigenMetricsMetrics {
    pub fn new(logger: TracingLogger) -> Self {
        let eigen_metrics_metrics = Self {
            eigen_metrics: EigenMetrics {
                performance_score: metrics::register_gauge!("eigen_performance_score"),
            },
            logger,
        };
        EigenMetrics::describe();
        eigen_metrics_metrics
            .eigen_metrics
            .performance_score
            .set(100_f64);
        eigen_metrics_metrics
    }

    pub fn performance_score(&self) -> Gauge {
        self.eigen_metrics.performance_score.clone()
    }

    pub fn set_performance_score(&self, score: f64) {
        self.eigen_metrics.performance_score.set(score)
    }
}
