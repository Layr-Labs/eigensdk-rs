use crate::client::{AssetID, Client};
use alloy_primitives::Address;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AssetAddress {
    asset_id: AssetID,
    address: Address,
    tag: String,
    description: String,
    #[serde(rename = "type")]
    type_field: String,
    legacy_address: Address,
    enterprise_address: Address,
    user_defined: Address,
}

pub trait GetAssetAddresses {
    async fn get_asset_addresses(&self, vault_id: String, asset_id: AssetID) -> AssetAddress;
}

impl GetAssetAddresses for Client {
    async fn get_asset_addresses(&self, vault_id: String, asset_id: AssetID) -> AssetAddress {
        let res = self
            .get_request(&format!(
                "/v1/vault/accounts/{}/{}/addresses",
                vault_id, asset_id
            ))
            .await
            .unwrap();

        let asset_address: AssetAddress = serde_json::from_str(&res).unwrap();
        asset_address
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_asset_addresses() {
        let api_key = "4aff0abf-9e81-4cf4-8bae-c9fc0c8a13af".to_string();
        let private_key = "0x29962ee54b8107328d033006e33627ecdbc9f5adc8b3e8ab4064e34a1193110c";
        let api_url = "https://api.fireblocks.io".to_string();
        let vault_id = "";
        let asset_id: AssetID = AssetID::ETH;
        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );

        let s = client
            .get_asset_addresses(vault_id.to_string(), asset_id)
            .await;
    }
}
