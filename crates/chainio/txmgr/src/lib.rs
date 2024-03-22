use eigensdk_logging::logger::Logger;
use eigensdk_signerv2::SignerV2;
use ethers::{
    core::types::Address,
    providers::{Http, Middleware, Provider},
    types::{
        transaction::{eip1559::Eip1559TransactionRequest, eip2718::TypedTransaction},
        Transaction, TransactionReceipt,
    },
};

use eigensdk_client_wallet::{privatekey_wallet::PrivateKeyWallet, WalletTrait};
use std::sync::Arc;

pub struct TxManager;

pub struct SimpleTxManager {
    wallet: PrivateKeyWallet,
    client: Provider<Http>,
    // signer_fn: Box<SignerV2>,
    log: Logger,
    sender: Address,
}

impl SimpleTxManager {
    pub fn new(
        wallet: PrivateKeyWallet,
        client: Provider<Http>,
        log: Logger,
        // signer_fn: Box<SignerV2>,
        sender: Address,
    ) -> Self {
        SimpleTxManager {
            wallet,
            client,
            log,
            // signer_fn,
            sender,
        }
    }

    pub async fn send(&self, tx: Eip1559TransactionRequest) -> Result<TransactionReceipt, String> {
        let tx_id = self
            .wallet
            .send_transaction(TypedTransaction::Eip1559(tx))
            .await
            .unwrap();
        let provider = Arc::new(self.client.clone());
        let receipt = provider.get_transaction_receipt(tx_id).await.unwrap();

        if let Some(rece) = receipt {
            Ok(rece)
        } else {
            return Err("receipt not found ".to_string());
        }
    }
}
