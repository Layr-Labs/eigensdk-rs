use posthog_rs::{client, Client, Error, Event};

pub struct TelemetryConfig {
    pub key: String,
    pub user_id: String,
    client: Client,
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

    pub fn capture_event(&self, event: &str) -> Result<(), Error> {
        let event = Event::new(event, &self.user_id);
        self.client.capture(event)
    }
}
