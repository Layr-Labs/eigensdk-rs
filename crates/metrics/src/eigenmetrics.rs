use eigen_metrics_derive::Metrics;
use metrics::Gauge;

/// TODO(supernova): fee_earned_total is not yet implemented . As its not yet turned on 
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

impl EigenMetrics {
    pub fn new() -> Self {
        let gauge = Self {
            performance_score: metrics::register_gauge!("eigen_performance_score"),
        };
        EigenMetrics::describe();
        // Default value is 100 . It will go down if node doesn't perform well
        gauge.performance_score.set(100_f64);
        gauge
    }

    /// Get the performance score
    /// To set the performance score , use set() function available in [`Gauge`]
    pub fn performance_score(&self) -> Gauge {
        self.performance_score.clone()
    }
}
