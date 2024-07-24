use crate::client::AssetID;
use crate::status::Status;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum TransactionOperation {
    contract_call,
    transfer,
    mint,
    burn,
    typed_message,
    raw,
}

impl TransactionOperation {
    pub fn as_str(&self) -> &'static str {
        match self {
            TransactionOperation::contract_call => "CONTRACT_CALL",
            TransactionOperation::transfer => "TRANSFER",
            TransactionOperation::mint => "MINT",
            TransactionOperation::burn => "BURN",
            TransactionOperation::typed_message => "TYPED_MESSAGE",
            TransactionOperation::raw => "RAW",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    #[serde(rename = "type")]
    type_field: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExtraParams {
    #[serde(rename = "ContractCallData")]
    calldata: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractCallRequest {
    operation: TransactionOperation,
    #[serde(rename = "externalTxId")]
    external_tx_id: String,
    #[serde(rename = "assetId")]
    asset_id: AssetID,
    source: Account,
    destination: Account,
    amount: String,
    #[serde(rename = "extraParameters")]
    extra_parameters: ExtraParams,
    #[serde(rename = "replaceTxByHash")]
    replace_tx_by_hash: String,
}

#[allow(unused)]
pub struct ContractCallResponse {
    id: String,
    status: Status,
}

// impl ContractCallRequest{

//     pub fn new(external_tx_id:String,asset_id : AssetID,source_account_id : String, destination_account_id: String,amount:String,calldata:String,replace_tx_by_hash:String )-> Self{

//         ContractCallRequest{

//             operation: ContractCallResponse

//         }

//     }

//     pub fn  contract_call(&self,req: ContractCallRequest) -> ContractCallResponse{

//     }

// }
