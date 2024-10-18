use posthog_rs::{Client, Event};

pub struct TelemetryConfig {
    pub key: String,
    pub user_id: String,
    client: Client,
}

impl TelemetryConfig {
    pub fn new(key: String, user_id: String) -> Self {
        let client = posthog_rs::client(key);

        Self { key, user_id, client }
    }

    pub fn capture_event(&self) {
        let event = posthog_rs::Event::new("app_started", "1234");
        self.client.capture(event).unwrap();
    }
    let event = posthog_rs::Event::new("app_started", "1234");
    client.capture(event).unwrap();

}
