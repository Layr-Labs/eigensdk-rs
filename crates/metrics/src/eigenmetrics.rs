use eigen_logging::logger::SharedLogger;
use metrics::{describe_gauge, gauge};
use std::fmt::Debug;

// TODO(supernova) : feeearnedtotal is not turned on yet,so not implemented yet
// https://github.com/Layr-Labs/eigensdk-go/blob/67787e959b727b115628a34e796df3a9ef42f646/metrics/eigenmetrics.go#L23
#[derive(Debug)]
pub struct EigenPerformanceMetrics {
    logger: SharedLogger,
}

impl EigenPerformanceMetrics {
    /// TODO(supernova): fee_earned_total is not yet implemented . As its not yet turned on
    /// The performance metric is a score between 0 and 100 and each developer can define their own way of calculating the score.
    /// The score is calculated based on the performance of the AVS Node and the performance of the backing services.
    pub fn new(logger: SharedLogger) -> Self {
        describe_gauge!("eigen_performance_score", "A gauge with performance score");
        gauge!("eigen_performance_score").set(100_f64);
        Self { logger }
    }

    pub fn set_performance_score(&self, score: f64) {
        self.logger.debug(
            &format!("set performance score , new score {}", score),
            "eigen-metrics.set_performance_score",
        );
        gauge!("eigen_performance_score").set(score);
    }
}
