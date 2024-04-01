use thiserror::Error;



#[derive(Debug,Error)]
pub enum TxMgrError{
    
    #[error("Failed to send transaction in tx mgr ")]
    SendTransaction,


    /// Failed to get tx receipt
    #[error("Failed to get tx receipt")]
    GetTransactionReceipt,


    /// Empty receipt
    #[error("Empty Receipt")]
    EmptyReceipt



}