use crate::error::FireBlockError;
use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use mime::APPLICATION_JSON;
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

const X_API_KEY: &str = "X-API-KEY";

/// AssetID represents the asset identifier as supported by fireblocks
/// TODO : Add more assetid identifiers
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum AssetID {
    ETH,
    #[serde(rename = "ETH_TEST5")]
    EthTest5,
    #[serde(rename = "BTC_TEST")]
    BtcTest,
    #[serde(rename = "BASECHAIN_ETH_TEST5")]
    BaseChainEthTest5,
    #[serde(rename = "ETH_TEST6")]
    EthTest6,
}

impl std::fmt::Display for AssetID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssetID::ETH => "ETH",
                AssetID::EthTest5 => "ETH_TEST5",
                AssetID::BtcTest => "BTC_TEST",
                AssetID::BaseChainEthTest5 => "BASECHAIN_ETH_TEST5",
                AssetID::EthTest6 => "ETH_TEST6",
            }
        )
    }
}

// Initialize AssetIDByChain as a HashMap
pub static ASSET_ID_BY_CHAIN: Lazy<HashMap<u64, AssetID>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(1, AssetID::ETH);
    m.insert(2, AssetID::EthTest5);
    m.insert(17000, AssetID::EthTest6);
    m
});

pub const JWT_EXPIRATION_SECONDS: i64 = 30;

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub code: i32,
}

/// Payload for JWT
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Fireblocks api uri
    uri: String,
    /// Unique identifier. Each request needs to have a unique identifier.
    nonce: String,
    /// The time at which the JWT was issued, in seconds since Epoch.
    iat: i64,
    /// Expiration time of jwt
    exp: i64,
    /// Api key
    sub: String,
    #[serde(rename = "bodyHash")]
    /// Hex-encoded SHA-256 hash of the raw HTTP request body.
    body_hash: String,
}

/// Fireblock Client
#[derive(Debug)]
pub struct Client {
    /// Api key
    api_key: String,
    /// Fireblocks generated secret key based on RS256 (RSASSA-PKCS1-v1_5 using SHA-256 hash) algorithm
    private_key: String,
    ///  Aandbox:  https://sandbox-api.fireblocks.io/v1 , Mainnet: https://api.fireblocks.io/v1
    api_url: String,
}

impl Client {
    pub fn new(api_key: String, private_key: String, api_url: String) -> Self {
        Self {
            api_key,
            private_key,
            api_url,
        }
    }

    ///  Sign the payload
    pub fn sign_jwt(&self, path: &str, body: Option<&str>) -> Result<String, FireBlockError> {
        let now = Utc::now().timestamp();
        let nonce = Uuid::new_v4().to_string();
        let body_hash = match body {
            Some(b) => hex::encode(Sha256::digest(b.as_bytes())),
            None => hex::encode(Sha256::digest("".as_bytes())),
        };

        let claims = Claims {
            uri: path.to_owned(),
            nonce,
            iat: now,
            exp: now + JWT_EXPIRATION_SECONDS, // Adjusted to ensure it's within the required timeframe
            sub: self.api_key.clone(),
            body_hash,
        };

        let encoding_key = EncodingKey::from_rsa_pem(self.private_key.as_bytes())
            .map_err(FireBlockError::JsonWebTokenError)?;

        encode(&Header::new(Algorithm::RS256), &claims, &encoding_key)
            .map_err(FireBlockError::JsonWebTokenError)
    }

    /// GET : Request to the fireblocks endpoint using the given path.
    pub async fn get_request(&self, path: &str) -> Result<String, FireBlockError> {
        let token = self.sign_jwt(path, None)?;

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
        headers.insert(X_API_KEY, HeaderValue::from_str(&self.api_key)?);

        // Make the GET request
        let response = client
            .get(self.api_url.to_owned() + path)
            .headers(headers)
            .send()
            .await?;

        // Check response status and return result
        if response.status().is_success() {
            let response_text = response.text().await?;
            Ok(response_text)
        } else {
            Err(FireBlockError::from(format!(
                "GET Request failed with status: {}",
                response.status()
            )))
        }
    }

    /// POST: Post a request using the fireblocks path api and the appropriate body parameters
    pub async fn post_request(
        &self,
        path: &str,
        body: Option<&str>,
    ) -> Result<String, FireBlockError> {
        let token = self.sign_jwt(path, body)?;

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
        headers.insert(X_API_KEY, HeaderValue::from_str(&self.api_key)?);

        // Make the POST request
        let response = client
            .post(self.api_url.to_owned() + path) // Use api_url here
            .headers(headers)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref()) // Set Content-Type header
            .body(body.unwrap_or("").to_string())
            .send()
            .await?;

        // Check response status and return result
        if response.status().is_success() {
            let response_text = response.text().await?;
            Ok(response_text)
        } else {
            Err(FireBlockError::from(format!(
                "POST Request failed with status: {}",
                response.status()
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AssetID, ASSET_ID_BY_CHAIN};

    #[test]
    fn test_asset_id_by_chain() {
        assert_eq!(AssetID::ETH, *ASSET_ID_BY_CHAIN.get(&1).unwrap());
    }
}
