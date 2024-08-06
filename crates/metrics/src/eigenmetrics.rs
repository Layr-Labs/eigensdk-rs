use eigen_logging::{logger::Logger, tracing_logger::TracingLogger};
use eigen_metrics_derive::Metrics;
use metrics::Gauge;

// Performance Metrics
#[derive(Clone, Metrics)]
#[metrics(scope = "eigenmetrics.performancemetrics")]
pub struct EigenPerformance {
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
pub struct EigenPerformanceMetrics {
    /// The performance metric is a score between 0 and 100 and each developer can define their own way of calculating the score.
    /// The score is calculated based on the performance of the AVS Node and the performance of the backing services.
    eigen_metrics: EigenPerformance,

    logger: TracingLogger,
}

impl EigenPerformanceMetrics {
    pub fn new(logger: TracingLogger) -> Self {
        let eigen_metrics_metrics = Self {
            eigen_metrics: EigenPerformance {
                performance_score: metrics::register_gauge!("eigen_performance_score"),
            },
            logger,
        };
        EigenPerformance::describe();
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
        self.logger.debug(
            &format!("set performance score , new score {}", score),
            &["eigen-metrics.set_performance_score"],
        );
        self.eigen_metrics.performance_score.set(score)
    }
}
