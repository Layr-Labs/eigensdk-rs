#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
use eigen_logging::logger::SharedLogger;
use metrics::{describe_gauge, gauge, Key, Label};

/// RegisteredStakes Metrics with logger
#[derive(Debug)]
pub struct RegisteredStakesMetrics {
    logger: SharedLogger,
}

impl RegisteredStakesMetrics {
    /// Operator stakes in AVS registry contract.
    /// Most commonly represents a weighted combination of delegated shares in the DelegationManager EigenLayer contract.
    pub fn new(logger: SharedLogger) -> Self {
        describe_gauge!(
            "eigen_registered_stakes",
            "A gauge with weighted delegation of delegated shares in delegation manager contract"
        );

        Self { logger }
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
        gauge!(key.to_string()).set(value);
        self.logger.debug(
            &format!(
                "set registered stakes , quorum_name: {} , quorum_number: {} , value: {}  ",
                quorum_name, quorum_number, value
            ),
            "eigen-metrics-collectors-economic.set_stake",
        );
    }
}
