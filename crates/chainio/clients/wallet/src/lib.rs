pub mod error;
pub mod privatekey_wallet;
use error::PrivateKeyWalletError;
use ethers_core::types::{transaction::eip2718::TypedTransaction, TransactionReceipt, TxHash};

pub type TxId = String;
pub trait WalletTrait {
    async fn send_transaction(&self, tx: TypedTransaction)
        -> Result<TxHash, PrivateKeyWalletError>;

    async fn get_transaction_receipt(
        &self,
        tx_id: TxId,
    ) -> Result<Option<TransactionReceipt>, PrivateKeyWalletError>;
}
