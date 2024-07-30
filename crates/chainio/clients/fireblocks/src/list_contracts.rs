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

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[allow(unused)]
/// Get List Contracts trait for "/v1/contracts" requests
pub trait ListContracts {
    async fn list_contracts(&self) -> Result<WhitelistedContractResponse, FireBlockError>;
}

impl ListContracts for Client {
    async fn list_contracts(&self) -> Result<WhitelistedContractResponse, FireBlockError> {
        let list_contracts_result = self.get_request("/v1/contracts").await;
        match list_contracts_result {
            Ok(list_contracts_object) => {
                if list_contracts_object.trim() == "[]" {
                    return Ok(WhitelistedContractResponse {
                        contracts: Vec::new(),
                    });
                }
                let serialized_tx: Result<Vec<WhitelistedContract>, _> =
                    serde_json::from_str(&list_contracts_object);
                match serialized_tx {
                    Ok(contracts) => Ok(WhitelistedContractResponse { contracts }),
                    Err(e) => Err(FireBlockError::SerdeError(e)),
                }
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    #[cfg(feature = "fireblock-tests")]
    async fn test_list_contracts() {
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

        let _ = client.list_contracts().await.unwrap();
    }
}
