use serde::{Deserialize, Serialize};

use crate::contract_call::{
    DestinationTransferPeerPath, ExtraParameters, TransactionOperation, TransferPeerPath,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub operation: TransactionOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_tx_id: Option<String>,
    pub source: TransferPeerPath,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<DestinationTransferPeerPath>,
    pub amount: String,
    pub extra_parameters: Option<ExtraParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_limit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    replace_tx_by_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "maxFee")]
    pub max_fee: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "priorityFee")]
    priority_fee: Option<String>,
}

impl TransactionRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation: TransactionOperation,
        external_tx_id: Option<String>,
        asset_id: String,
        source: TransferPeerPath,
        destination: Option<DestinationTransferPeerPath>,
        amount: String,
        extra_parameters: Option<ExtraParameters>,
        replace_tx_by_hash: Option<String>,
        gas_price: Option<String>,
        gas_limit: Option<String>,
        note: Option<String>,
        max_fee: Option<String>,
        priority_fee: Option<String>,
    ) -> Self {
        Self {
            operation,
            external_tx_id,
            asset_id,
            source,
            destination,
            amount,
            extra_parameters,
            replace_tx_by_hash,
            gas_price,
            gas_limit,
            note,
            max_fee,
            priority_fee,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    id: String,
    status: String,
}
