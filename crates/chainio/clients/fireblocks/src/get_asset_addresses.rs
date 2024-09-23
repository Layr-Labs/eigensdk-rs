use crate::{
    client::{AssetID, Client},
    error::FireBlockError,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetAddress {
    asset_id: AssetID,
    address: String,
    tag: String,
    description: String,
    #[serde(rename = "type")]
    type_field: String,
    legacy_address: String,
    enterprise_address: String,
    user_defined: bool,
    bip_44_address_index: u32,
}

/// Addresses Response struct that contains an array of [`AssetAddress`]
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressesResponse {
    addresses: Vec<AssetAddress>,
}

/// Get Asset Addresses trait for "/v1/vault/accounts/{vault_id}/{asset_id}/addresses_paginated" requests
#[allow(async_fn_in_trait)]
pub trait GetAssetAddresses {
    async fn get_asset_addresses(
        &self,
        vault_id: String,
        asset_id: AssetID,
    ) -> Result<AddressesResponse, FireBlockError>;
}

impl GetAssetAddresses for Client {
    async fn get_asset_addresses(
        &self,
        vault_id: String,
        asset_id: AssetID,
    ) -> Result<AddressesResponse, FireBlockError> {
        let asset_addresses = self
            .get_request(&format!(
                "/v1/vault/accounts/{}/{}/addresses_paginated",
                vault_id, asset_id
            ))
            .await?;

        serde_json::from_str(&asset_addresses).map_err(FireBlockError::SerdeError)
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
    async fn test_asset_addresses() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key =
            std::fs::read_to_string(private_key_path).expect("Failed to read private key file");
        let vault_id = "13";
        let asset_id: AssetID = AssetID::EthTest5;
        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );

        let _ = client
            .get_asset_addresses(vault_id.to_string(), asset_id)
            .await
            .unwrap();
    }
}
