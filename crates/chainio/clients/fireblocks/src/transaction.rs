use serde::{Deserialize, Serialize};

use crate::{
    client::AssetID,
    contract_call::{Account, ExtraParams, TransactionOperation},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionRequest {
    operation: String,
    external_tx_id: String,
    asset_id: AssetID,
    source: Account,
    destination: Account,
    amount: String,
    extra_parameters: ExtraParams,
    replace_tx_by_hash: String,
    gas_price: String,
    gas_limit: String,
    max_fee: String,
    priority_fee: String,
    fee_level: String,
}

impl TransactionRequest {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        operation: String,
        external_tx_id: String,
        asset_id: AssetID,
        source: Account,
        destination: Account,
        amount: String,
        extra_parameters: ExtraParams,
        replace_tx_by_hash: String,
        gas_price: String,
        gas_limit: String,
        max_fee: String,
        priority_fee: String,
        fee_level: String,
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
            max_fee,
            priority_fee,
            fee_level,
        }
    }

    pub fn get_contract_call(&mut self) -> Self {
        self.operation = TransactionOperation::contract_call.as_str().to_string();
        self.clone()
    }
}

impl std::fmt::Display for TransactionRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "TransactionRequest {{
    operation: {},
    external_tx_id: {},
    asset_id: {},
    source: {},
    destination: {},
    amount: {},
    extra_parameters: {},
    replace_tx_by_hash: {},
    gas_price: {},
    gas_limit: {},
    max_fee: {},
    priority_fee: {},
    fee_level: {}
}}",
            self.operation,
            self.external_tx_id,
            self.asset_id,
            self.source,
            self.destination,
            self.amount,
            self.extra_parameters,
            self.replace_tx_by_hash,
            self.gas_price,
            self.gas_limit,
            self.max_fee,
            self.priority_fee,
            self.fee_level
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    id: String,
    status: String,
}
