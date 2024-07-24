use crate::{
    client::{AssetID, Client},
    contract_call::{Account, TransactionOperation},
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
pub struct Transaction {
    id: String,
    status: Status,
    #[serde(rename = "subStatus")]
    sub_status: String,
    #[serde(rename = "txHash")]
    tx_hash: String,
    operation: TransactionOperation,
    #[serde(rename = "createdAt")]
    created_at: i64,
    #[serde(rename = "lastUpdated")]
    last_updated: i64,
    #[serde(rename = "assetId")]
    asset_id: AssetID,
    source: Account,
    #[serde(rename = "sourceAddress")]
    source_address: String,
    destination: Account, // 1
    #[serde(rename = "destinationAddress")]
    destination_address: String,
    #[serde(rename = "destinationAddressDescription")]
    destination_address_description: String,
    #[serde(rename = "destinationTag")]
    destination_tag: String,
    #[serde(rename = "amountInfo")]
    amount_info: AmountInfo,
    #[serde(rename = "feeInfo")]
    fee_info: FeeInfo,
    #[serde(rename = "feeCurrency")]
    fee_currency: String,
    #[serde(rename = "extraParameters")]
    extra_parameters: Option<ExtraParameters>,
    #[serde(rename = "numOfConfirmations")]
    num_of_confirmations: i64,
    #[serde(rename = "blockInfo")]
    block_info: BlockInfo,
}

#[allow(unused)]
/// Get Transaction trait for "/v1/transactions/tx_id" get requests
pub trait GetTransaction {
    async fn get_transaction(&self, tx_id: String) -> Result<Transaction, String>;
}

impl GetTransaction for Client {
    /// Get transaction api
    async fn get_transaction(&self, tx_id: String) -> Result<Transaction, String> {
        let transaction_object_result = self
            .get_request(&format!("/v1/transactions/{}", tx_id))
            .await;

        match transaction_object_result {
            Ok(transaction) => {
                println!("Transaction: {:?}", transaction);
                let serialized_tx: Transaction = serde_json::from_str(&transaction).unwrap();
                Ok(serialized_tx)
            }
            Err(e) => Err(e.to_string()),
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
        println!(
            "Current working directory: {:?}",
            env::current_dir().unwrap()
        );
        println!("private key path :{:?}", private_key_path);
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
