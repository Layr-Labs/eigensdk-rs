use chrono::Utc;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum AssetID {
    ETH,
    ETH_TEST5,
}

impl std::fmt::Display for AssetID {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AssetID::ETH => "ETH",
                AssetID::ETH_TEST5 => "ETH_TEST5",
            }
        )
    }
}

// Initialize AssetIDByChain as a HashMap
#[allow(unused)]
fn asset_id_by_chain() -> HashMap<u64, AssetID> {
    let mut map = HashMap::new();
    map.insert(1, AssetID::ETH);
    map.insert(2, AssetID::ETH_TEST5);
    map
}

#[allow(unused)]
#[allow(non_camel_case_types)]
struct client {}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub message: String,
    pub code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    uri: String,
    nonce: String,
    iat: i64,
    exp: i64,
    sub: String,
    bodyHash: String,
}

/// Fireblock Client
#[derive(Debug)]
pub struct Client {
    api_key: String,
    private_key: String,
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

    pub fn sign_jwt(
        &self,
        path: &str,
        body: Option<&str>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
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
            exp: now + 30, // Adjusted to ensure it's within the required timeframe
            sub: self.api_key.clone(),
            bodyHash: body_hash,
        };

        let token = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(self.private_key.as_bytes())?,
        )?;
        Ok(token)
    }
    pub async fn get_request(&self, path: &str) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.sign_jwt(path, None)?;

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
        headers.insert("X-API-Key", HeaderValue::from_str(&self.api_key)?);

        // Make the GET request
        let response = client
            .get(self.api_url.to_owned() + path) // Use api_url here
            .headers(headers)
            .send()
            .await?;

        // Check response status and return result
        if response.status().is_success() {
            let response_text = response.text().await?;
            Ok(response_text)
        } else {
            Err(format!(
                "GET Request failed with status: {}",
                response.status()
            ))?
        }
    }

    pub async fn post_request(
        &self,
        path: &str,
        body: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let token = self.sign_jwt(path, Some(body))?;

        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token))?,
        );
        headers.insert("X-API-Key", HeaderValue::from_str(&self.api_key)?);

        // Make the POST request
        let response = client
            .post(self.api_url.to_owned() + path) // Use api_url here
            .headers(headers)
            .header(CONTENT_TYPE, "application/json") // Set Content-Type header
            .body(body.to_string())
            .send()
            .await?;

        // Check response status and return result
        if response.status().is_success() {
            let response_text = response.text().await?;
            Ok(response_text)
        } else {
            Err(format!(
                "POST Request failed with status: {}",
                response.status()
            ))?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{asset_id_by_chain, AssetID};

    #[test]
    fn test_asset_id_by_chain() {
        let asset_id_by_chain = asset_id_by_chain();
        assert_eq!(AssetID::ETH, *asset_id_by_chain.get(&1).unwrap());
    }
}
