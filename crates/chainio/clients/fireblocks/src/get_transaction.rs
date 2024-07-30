use crate::{
    client::{AssetID, Client},
    contract_call::{Account, TransactionOperation},
    error::FireBlockError,
    status::Status,
};
use serde::{Deserialize, Serialize};

/// Amount Info
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct AmountInfo {
    amount: String,
    #[serde(rename = "requestedAmount")]
    requested_amount: String,
    #[serde(rename = "netAmount")]
    net_amount: String,
    #[serde(rename = "amountUSD")]
    amount_usdc: String,
}

/// Fee Info
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FeeInfo {
    network_fee: String,
    // service_fee: String,
    gas_price: String,
}

/// Extra Parameters
#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct ExtraParameters {
    #[serde(rename = "contractCallData")]
    contract_calldata: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
struct BlockInfo {
    #[serde(rename = "blockHeight")]
    block_height: String,
    #[serde(rename = "blockHash")]
    block_hash: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    id: String,
    status: Status,
    sub_status: String,
    tx_hash: String,
    operation: TransactionOperation,
    created_at: i64,
    last_updated: i64,
    asset_id: AssetID,
    source: Account,
    source_address: String,
    destination: Account,
    destination_address: String,
    destination_address_description: String,
    destination_tag: String,
    amount_info: AmountInfo,
    fee_info: FeeInfo,
    fee_currency: String,
    extra_parameters: Option<ExtraParameters>,
    num_of_confirmations: i64,
    block_info: BlockInfo,
}

#[allow(unused)]
/// Get Transaction trait for "/v1/transactions/tx_id" get requests
pub trait GetTransaction {
    async fn get_transaction(&self, tx_id: String) -> Result<Transaction, FireBlockError>;
}

impl GetTransaction for Client {
    async fn get_transaction(&self, tx_id: String) -> Result<Transaction, FireBlockError> {
        let transaction_object_result = self
            .get_request(&format!("/v1/transactions/{}", tx_id))
            .await;

        match transaction_object_result {
            Ok(transaction) => {
                let serialized_tx_result: Result<Transaction, _> =
                    serde_json::from_str(&transaction);

                match serialized_tx_result {
                    Ok(serialized_tx) => Ok(serialized_tx),
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
    async fn test_get_transaction() {
        let api_key = env::var("FIREBLOCKS_API_KEY").expect("FIREBLOCKS_API_KEY not set");
        let private_key_path =
            env::var("FIREBLOCKS_PRIVATE_KEY_PATH").expect("FIREBLOCKS_PRIVATE_KEY_PATH not set");
        let api_url = env::var("FIREBLOCKS_API_URL").expect("FIREBLOCKS_API_URL not set");
        let private_key =
            std::fs::read_to_string(private_key_path).expect("Failed to read private key file");
        let tx_id = "10d377ac-0655-45c3-9d05-4fe0887787f3";

        let client = Client::new(
            api_key.to_string(),
            private_key.to_string(),
            api_url.clone(),
        );

        let _ = client.get_transaction(tx_id.to_string()).await.unwrap();
    }
}
