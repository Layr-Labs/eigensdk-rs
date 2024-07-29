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

#[allow(unused)]
/// Get List External Accounts trait for "/v1/external_wallets" requests
pub trait ListExternalAccounts {
    async fn list_external_accounts(&self) -> Result<Vec<WhitelistedAccount>, FireBlockError>;
}

impl ListExternalAccounts for Client {
    async fn list_external_accounts(&self) -> Result<Vec<WhitelistedAccount>, FireBlockError> {
        let list_external_accounts_result = self.get_request("/v1/external_wallets").await;

        match list_external_accounts_result {
            Ok(list_external_accounts) => {
                if list_external_accounts.trim() == "[]" {
                    let mut default_accounts = Vec::new();
                    default_accounts.push(WhitelistedAccount::default());
                    return Ok(default_accounts);
                }
                let serialized_tx: Result<Vec<WhitelistedAccount>, _> =
                    serde_json::from_str(&list_external_accounts);
                match serialized_tx {
                    Ok(whitelisted_accounts) => Ok(whitelisted_accounts),
                    Err(e) => Err(FireBlockError::SerdeError(e)),
                }
            }
            Err(e) => Err(e),
        }
    }
}
