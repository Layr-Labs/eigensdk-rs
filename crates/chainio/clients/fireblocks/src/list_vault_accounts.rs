use crate::{
    client::{AssetID, Client},
    error::FireBlockError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Asset {
    id: AssetID,
    total: String,
    balance: String,
    available: String,
    locked_amount: Option<String>,
    pending: Option<String>,
    frozen: Option<String>,
    staked: Option<String>,
    block_height: Option<String>,
    #[serde(default)]
    block_hash: Option<String>,
}

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultAccount {
    id: String,
    name: String,
    #[serde(rename = "hiddenOnUI")]
    hidden_on_ui: bool,
    #[serde(rename = "autoFuel")]
    auto_fuel: bool,
    assets: Vec<Asset>,
}

impl VaultAccount {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VaultAccountResponse {
    #[serde(rename = "accounts")]
    vault_accounts: Vec<VaultAccount>,
    paging: Paging,
}

impl VaultAccountResponse {
    pub fn vault_accounts(&self) -> Vec<VaultAccount> {
        self.vault_accounts.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Paging {}

/// Get List Vault Accounts trait for "/v1/vault/accounts_paged" get requests
pub trait ListVaultAccounts {
    async fn list_vault_accounts(&self) -> Result<VaultAccountResponse, FireBlockError>;
}

impl ListVaultAccounts for Client {
    async fn list_vault_accounts(&self) -> Result<VaultAccountResponse, FireBlockError> {
        let list_vault_accounts = self.get_request("/v1/vault/accounts_paged").await?;
        serde_json::from_str(&list_vault_accounts).map_err(FireBlockError::SerdeError)
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
    async fn test_list_vault_accounts() {
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

        let _ = client.list_vault_accounts().await.unwrap();
    }
}
