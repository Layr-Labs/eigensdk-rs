use posthog_rs::{client, Client};

pub struct TelemetryConfig {
    pub key: String,
    pub user_id: String,
    pub(crate) client: Client,
}

impl TelemetryConfig {
    pub(crate) fn new(key: &str, user_id: &str) -> Self {
        let client = client(key);

        Self {
            key: key.to_owned(),
            user_id: user_id.to_owned(),
            client,
        }
    }
}
