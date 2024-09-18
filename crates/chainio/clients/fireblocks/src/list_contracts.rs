use crate::{
    client::{AssetID, Client},
    error::FireBlockError,
    status::Status,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Assets {
    pub id: Option<AssetID>,
    balance: Option<String>,
    pub status: Option<Status>,
    pub address: Option<String>,
    tag: Option<String>,
    locked_amount: Option<String>,
    activation_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WhitelistedContract {
    id: String,
    name: String,
    #[serde(default)]
    assets: Vec<Assets>,
}

impl WhitelistedContract {
    pub fn assets(&self) -> Vec<Assets> {
        self.assets.clone()
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhitelistedContractResponse {
    contracts: Vec<WhitelistedContract>,
}

impl WhitelistedContractResponse {
    pub fn contracts(&self) -> Vec<WhitelistedContract> {
        self.contracts.clone()
    }
}

/// Get List Contracts trait for "/v1/contracts" requests
pub trait ListContracts {
    async fn list_contracts(&self) -> Result<WhitelistedContractResponse, FireBlockError>;
}

impl ListContracts for Client {
    async fn list_contracts(&self) -> Result<WhitelistedContractResponse, FireBlockError> {
        let list_contracts_object = self.get_request("/v1/contracts").await?;

        if list_contracts_object.trim() == "[]" {
            return Ok(WhitelistedContractResponse {
                contracts: Vec::new(),
            });
        }

        let contracts: Vec<WhitelistedContract> =
            serde_json::from_str(&list_contracts_object).map_err(FireBlockError::SerdeError)?;

        Ok(WhitelistedContractResponse { contracts })
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
    async fn test_list_contracts() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key = std::fs::read_to_string(private_key_path.clone())
            .expect("Failed to read private key file");
        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );

        let _ = client.list_contracts().await.unwrap();
    }
}
