use crate::client::AssetID;
use crate::client::Client;
use crate::status::Status;
use crate::transaction::TransactionRequest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExtraParams {
    calldata: String,
}

impl std::fmt::Display for ExtraParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ExtraParams {{ contractCallData: {} }}", self.calldata)
    }
}

/// Contract Call Request
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractCallRequest {
    /// Type of Operation
    operation: TransactionOperation,
    /// Tx id
    external_tx_id: String,
    /// [`AssetID`]
    asset_id: AssetID,
    /// Source [`Account`]
    source: Account,
    /// Destination [`Account`]
    destination: Account,
    /// Amount
    amount: String,
    /// Any extra parameters
    extra_parameters: ExtraParams,
    /// Replacement tx hash
    replace_tx_by_hash: String,
}

#[allow(unused)]
pub struct ContractCallResponse {
    /// Response id
    id: String,
    /// Response [`Status`]
    status: Status,
}

#[allow(async_fn_in_trait)]
pub trait ContractCall {
    async fn contract_call(
        &self,
        transaction_request: TransactionRequest,
    ) -> Result<ContractCallRequest, String>;
}

impl ContractCall for Client {
    async fn contract_call(
        &self,
        mut transaction_request: TransactionRequest,
    ) -> Result<ContractCallRequest, String> {
        let contract_call_request = transaction_request.get_contract_call();

        let contract_call_result = self
            .post_request(&format!("/v1/transactions/{}", contract_call_request), None)
            .await;
        println!("contract call rquest: {:?}", contract_call_result);
        match contract_call_result {
            Ok(contract_call) => {
                let contract_call_req: ContractCallRequest =
                    serde_json::from_str(&contract_call).unwrap();
                Ok(contract_call_req)
            }
            Err(e) => Err(e.to_string()),
        }
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
    async fn test_contract_call() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key =
            std::fs::read_to_string(private_key_path).expect("Failed to read private key file");
        // let tx_id = "10d377ac-0655-45c3-9d05-4fe0887787f3";

        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );
        let external_tx_id = "";
        let _account = Account {
            type_field: "".to_string(),
            id: "".to_string(),
        };
        let source = Account {
            type_field: "".to_string(),
            id: "".to_string(),
        };
        let destination = Account {
            type_field: "".to_string(),
            id: "".to_string(),
        };
        let amount = "";
        let extra_parameters = ExtraParams {
            calldata: "".to_string(),
        };
        let replace_tx_by_hash = "".to_string();
        let gas_price = "".to_string();
        let gas_limit = "".to_string();
        let max_fee = "".to_string();
        let priority_fee = "".to_string();
        let fee_level = "".to_string();
        let tx_request = TransactionRequest::new(
            TransactionOperation::ContractCall.as_str().to_string(),
            external_tx_id.to_string(),
            AssetID::EthTest5,
            source,
            destination,
            amount.to_string(),
            extra_parameters,
            replace_tx_by_hash,
            gas_price,
            gas_limit,
            max_fee,
            priority_fee,
            fee_level,
        );

        let _ = client.contract_call(tx_request).await.unwrap();
    }
}
