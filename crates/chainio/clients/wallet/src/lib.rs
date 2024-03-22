pub mod fireblocks_wallet;
pub mod privatekey_wallet;
use ethers_core::types::{
    transaction::eip2718::TypedTransaction, Address, Transaction, TransactionRequest, TxHash, H256,
};

pub type TxId = String;
pub trait WalletTrait {
    async fn send_transaction(&self, tx: TypedTransaction) -> Result<TxHash, String>;

    // fn get_transaction_receipt(&self,tx_id : TxId) ->
}
