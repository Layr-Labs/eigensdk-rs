use super::telemetry_config::TelemetryConfig;
use posthog_rs::{Error, Event};

pub struct Telemetry {}

impl Telemetry {
    pub fn set_config(key: &str, user_id: &str) -> Result<(), TelemetryConfig> {
        let config = TelemetryConfig::new(key, user_id);
        crate::TELEMETRY_CELL.set(config)?;
        Ok(())
    }

    pub fn capture_event(event: &str) -> Result<(), Error> {
        let cell = crate::TELEMETRY_CELL.get().unwrap();
        let event = Event::new(event, &cell.user_id);
        cell.client.capture(event)
    }
}
