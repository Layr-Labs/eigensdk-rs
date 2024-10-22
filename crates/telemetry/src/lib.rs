pub mod telemetry;
pub mod telemetry_config;

use std::sync::OnceLock;
use telemetry_config::TelemetryConfig;

static TELEMETRY_CELL: OnceLock<TelemetryConfig> = OnceLock::new();
