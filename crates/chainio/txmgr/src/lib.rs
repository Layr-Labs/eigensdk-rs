use crate::error::TxMgrError;
use ethers::{
    core::types::Address,
    providers::{Http, Middleware, Provider},
    types::{
        transaction::{eip1559::Eip1559TransactionRequest, eip2718::TypedTransaction},
        TransactionReceipt,
    },
};

use eigensdk_client_wallet::{privatekey_wallet::PrivateKeyWallet, WalletTrait};
use std::sync::Arc;

pub struct TxManager;

pub struct SimpleTxManager {
    pub wallet: PrivateKeyWallet,
    client: Provider<Http>,
    sender: Address,
}

impl SimpleTxManager {
    pub fn new(wallet: PrivateKeyWallet, client: Provider<Http>, sender: Address) -> Self {
        SimpleTxManager {
            wallet,
            client,
            sender,
        }
    }

    pub async fn send(
        &self,
        tx: Eip1559TransactionRequest,
    ) -> Result<TransactionReceipt, TxMgrError> {
        let tx_id_result = self
            .wallet
            .send_transaction(TypedTransaction::Eip1559(tx))
            .await;

        match tx_id_result {
            Ok(tx_id) => {
                let provider = Arc::new(self.client.clone());
                let receipt_result = provider.get_transaction_receipt(tx_id).await;

                match receipt_result {
                    Ok(receipt) => {
                        if let Some(rece) = receipt {
                            Ok(rece)
                        } else {
                            return Err(TxMgrError::EmptyReceipt);
                        }
                    }
                    Err(_) => return Err(TxMgrError::GetTransactionReceipt),
                }
            }
            Err(_) => return Err(TxMgrError::SendTransaction),
        }
    }
}
