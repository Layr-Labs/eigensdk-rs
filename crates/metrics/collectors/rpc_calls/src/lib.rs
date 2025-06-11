#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
use metrics::{describe_counter, describe_histogram, Key, Label};
use tracing::{debug, instrument};

/// RpcCallsMetrics
#[derive(Debug)]
pub struct RpcCallsMetrics {
    // Avoid allowing instantiation of this struct
    _private: (),
}

impl Default for RpcCallsMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl RpcCallsMetrics {
    pub fn new() -> Self {
        describe_histogram!(
            "eigen_rpc_request_duration_seconds",
            "Duration of json-rpc <method> in seconds from Ethereum Execution client <client>"
        );
        describe_counter!(
            "eigen_rpc_request_total",
            "Total of json-rpc <method> requests from Ethereum Execution client <client>"
        );

        Self { _private: () }
    }

    /// set_rpc_request_duration_seconds
    #[instrument(skip_all)]
    pub fn set_rpc_request_duration_seconds(
        &self,
        method: &str,
        client_version: &str,
        duration: f64,
    ) {
        let key = Key::from_parts(
            "eigen_rpc_request_duration_seconds",
            vec![
                Label::new("method ", method.to_string()),
                Label::new("client_version", client_version.to_string()),
            ],
        );

        metrics::histogram!(key.to_string()).record(duration);
        debug!("set rpc requet duration seconds");
    }

    /// set_rpc_request_total
    #[instrument(skip_all)]
    pub fn set_rpc_request_total(
        &self,
        method: &str,
        client_version: &str,
        rpc_request_total: u64,
    ) {
        let key = Key::from_parts(
            "eigen_rpc_request_total",
            vec![
                Label::new("method", method.to_string()),
                Label::new("client_version", client_version.to_string()),
            ],
        );

        metrics::counter!(key.to_string()).absolute(rpc_request_total);
        debug!("set rpc request total");
    }
}
