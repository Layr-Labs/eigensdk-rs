use eigen_metrics_derive::Metrics;
use metrics::{Gauge, Key, Label};

#[derive(Clone, Metrics)]
#[metrics(scope = "eigen.registeredstakes")]
pub struct RegisteredStakes {
    #[metric(
        rename = "eigen_registered_stakes",
        describe = " A gauge with weighted delegation of delegated shares in delegation manager contract "
    )]
    registered_stake: Gauge,
}

impl RegisteredStakes {
    pub fn new() -> Self {
        let gauge = Self {
            registered_stake: metrics::register_gauge!("eigen_registered_stakes"),
        };
        RegisteredStakes::describe();

        gauge
    }

    pub fn set_stake(&self, quorum_number: &str, quorum_name: &str, value: f64) {
        // Create the metric key with dynamic labels
        let key = Key::from_parts(
            "eigen_registered_stakes",
            vec![
                Label::new("quorum_number", quorum_number.to_string()),
                Label::new("quorum_name", quorum_name.to_string()),
            ],
        );

        // Register or retrieve the gauge with the specified key and set the value
        metrics::gauge!(key.to_string(), value);
    }

    pub fn registered_stakes(&self) -> Gauge {
        self.registered_stake.clone()
    }
}
