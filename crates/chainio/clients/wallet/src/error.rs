use thiserror::Error;

#[derive(Debug, Error)]
pub enum PrivateKeyWalletError {
    /// convert txid(string) to TxHash
    #[error("Failed to convert TxId to TxHash")]
    TxIdtoTxHash,

    /// Get transaction receipt
    #[error("Failed to get transaction receipt")]
    GetTransactionReceipt,

    /// Get Block Number
    #[error("Failed to get block number")]
    GetBlockNumber,

    /// Get Block
    #[error("Get Block from Number")]
    GetBlock,

    /// Estimate Gas and nonce
    #[error("Faile to estimate gas and nonce")]
    EstimateGasAndNonce,

    /// Failed to send transaction
    #[error("Failed to send transaction")]
    SendTransaction,

    /// pvtkey to ethers wallet
    #[error("Failed to build Wallet from pvt key string")]
    BuildWallet,

    /// estimate eip 1559 fees
    #[error("failed to estimate eip1559 fees")]
    EstimateEip1559Fees,

    /// estimate gas
    #[error("Failed to estimate gas")]
    EstimateGas,

    /// Transaction receipt
    #[error("Failed to get transaction receipt")]
    TransactionReceipt,
}
