use posthog_rs::{client, Client, Error, Event};
use std::sync::OnceLock;

static TELEMETRY_CELL: OnceLock<TelemetryConfig> = OnceLock::new();

pub struct Telemetry {}

pub struct TelemetryConfig {
    pub key: String,
    pub user_id: String,
    client: Client,
}

impl Telemetry {
    pub fn new(key: &str, user_id: &str) -> Result<(), TelemetryConfig> {
        let config = TelemetryConfig::new(key, user_id);
        TELEMETRY_CELL.set(config)?;
        Ok(())
    }

    pub fn capture_event(event: &str) -> Result<(), Error> {
        let cell = TELEMETRY_CELL.get().unwrap();
        let event = Event::new(event, &cell.user_id);
        cell.client.capture(event)
    }
}

impl TelemetryConfig {
    pub fn new(key: &str, user_id: &str) -> Self {
        let client = client(key);

        Self {
            key: key.to_owned(),
            user_id: user_id.to_owned(),
            client,
        }
    }
}
