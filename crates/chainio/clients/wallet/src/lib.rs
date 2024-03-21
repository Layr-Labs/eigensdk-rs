mod fireblocks_wallet;
mod privatekey_wallet;
use ethers_core::types::{
    transaction::eip2718::TypedTransaction, Address, Transaction, TransactionRequest,
};

type TxId = String;
// pub trait Wallet {

//     fn send_transaction(&self, tx : TypedTransaction) -> Result<TxId,String>;

//     // fn get_transaction_receipt(&self,tx_id : TxId) ->

// }

pub struct Wallet {}
