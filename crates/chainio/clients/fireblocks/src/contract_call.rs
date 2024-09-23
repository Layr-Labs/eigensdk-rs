use crate::client::Client;
use crate::error::FireBlockError;
use crate::transaction::TransactionRequest;
use crate::transaction::TransactionResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionOperation {
    ContractCall,
    Transfer,
    Mint,
    Burn,
    TypedMessage,
    Raw,
}

impl TransactionOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionOperation::ContractCall => "CONTRACT_CALL",
            TransactionOperation::Transfer => "TRANSFER",
            TransactionOperation::Mint => "MINT",
            TransactionOperation::Burn => "BURN",
            TransactionOperation::TypedMessage => "TYPED_MESSAGE",
            TransactionOperation::Raw => "RAW",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    #[serde(rename = "type")]
    type_field: String,
    id: String,
}

impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Account {{ id: {}, type: {} }}",
            self.id, self.type_field
        )
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ExtraParameters {
    ContractCallData(String),
    RawMessageData(RawMessageData),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RawMessageData {
    pub messages: Vec<UnsignedMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UnsignedMessage {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferPeerPath {
    #[serde(rename = "type")]
    pub peer_type: Option<PeerType>,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DestinationTransferPeerPath {
    #[serde(rename = "type")]
    pub peer_type: PeerType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_address: Option<OneTimeAddress>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OneTimeAddress {
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum PeerType {
    VAULT_ACCOUNT,
    EXCHANGE_ACCOUNT,
    INTERNAL_WALLET,
    EXTERNAL_WALLET,
    ONE_TIME_ADDRESS,
    NETWORK_CONNECTION,
    FIAT_ACCOUNT,
    COMPOUND,
    END_USER_WALLET,
    CONTRACT,
}

#[allow(async_fn_in_trait)]
pub trait ContractCall {
    async fn contract_call(
        &self,
        transaction_request: TransactionRequest,
    ) -> Result<TransactionResponse, FireBlockError>;
}

impl ContractCall for Client {
    async fn contract_call(
        &self,
        transaction_request: TransactionRequest,
    ) -> Result<TransactionResponse, FireBlockError> {
        let transaction_json =
            serde_json::to_string(&transaction_request).map_err(FireBlockError::SerdeError)?;

        // Call POST request
        let contract_call = self
            .post_request("/v1/transactions/", Some(&transaction_json.to_string()))
            .await?;
        serde_json::from_str(&contract_call).map_err(FireBlockError::SerdeError)
    }
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "fireblock-tests")]
    use super::*;
    #[cfg(feature = "fireblock-tests")]
    use crate::client::AssetID;
    #[cfg(feature = "fireblock-tests")]
    use std::env;

    #[tokio::test]
    #[cfg(feature = "fireblock-tests")]
    async fn test_contract_call() {
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
        let source = TransferPeerPath {
            peer_type: Some(PeerType::VAULT_ACCOUNT),
            id: Some("13".to_string()),
        };
        let destination = DestinationTransferPeerPath {
            peer_type: PeerType::ONE_TIME_ADDRESS,
            one_time_address: Some(OneTimeAddress {
                address: "0x43506849D7C04F9138D1A2050bbF3A0c054402dd".to_string(),
                tag: None,
            }),
            id: None,
        };
        let amount = "0";
        let extra_params = ExtraParameters::ContractCallData("0x095ea7b3000000000000000000000000d09971d8ed6c6a5e57581e90d593ee5b94e348d4ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff".to_string());
        let tx_request = TransactionRequest::new(
            TransactionOperation::ContractCall,
            None,
            AssetID::EthTest5.to_string(),
            source,
            Some(destination),
            amount.to_string(),
            Some(extra_params),
            None,
            None,
            None,
            None,
            None,
            None,
        );

        let _ = client.contract_call(tx_request).await.unwrap();
    }
}
