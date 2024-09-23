use serde::{Deserialize, Serialize};

use crate::{client::Client, error::FireBlockError, list_contracts::Assets};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct WhitelistedAccount {
    id: String,

    name: String,

    pub assets: Vec<Assets>,
}
impl WhitelistedAccount {
    pub fn id(&self) -> String {
        self.id.clone()
    }
}

#[allow(async_fn_in_trait)]
/// Get List External Accounts trait for "/v1/external_wallets" requests
pub trait ListExternalAccounts {
    async fn list_external_accounts(&self) -> Result<Vec<WhitelistedAccount>, FireBlockError>;
}

impl ListExternalAccounts for Client {
    async fn list_external_accounts(&self) -> Result<Vec<WhitelistedAccount>, FireBlockError> {
        let list_external_accounts = self.get_request("/v1/external_wallets").await?;

        if list_external_accounts.trim() == "[]" {
            let default_accounts = vec![WhitelistedAccount::default()];
            return Ok(default_accounts);
        }
        serde_json::from_str(&list_external_accounts).map_err(FireBlockError::SerdeError)
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "fireblock-tests")]
    use super::*;
    #[cfg(feature = "fireblock-tests")]
    use std::env;

    #[tokio::test]
    #[cfg(feature = "fireblock-tests")]
    async fn test_list_external_accounts() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key =
            std::fs::read_to_string(private_key_path).expect("Failed to read private key file");
        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );

        let _ = client.list_external_accounts().await.unwrap();
    }
}
