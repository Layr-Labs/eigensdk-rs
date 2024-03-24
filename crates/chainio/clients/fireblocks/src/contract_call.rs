use crate::client::AssetID;
use crate::status::Status;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
