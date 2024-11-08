use alloy::{
    consensus::TxEnvelope,
    eips::BlockNumberOrTag,
    primitives::{BlockNumber, TxHash},
    rpc::types::{Block, TransactionReceipt, TransactionRequest},
    transports::{RpcError, TransportErrorKind},
};

#[async_trait::async_trait]
pub trait EthBackend {
    /// Get the latest block number.
    ///
    /// # Returns
    ///
    /// The latest block number.
    async fn get_block_number(&self) -> Result<BlockNumber, RpcError<TransportErrorKind>>;

    // when not implemented, will txmanager will fall on gasTipCap
    async fn get_max_priority_fee_per_gas(&self) -> Result<u128, RpcError<TransportErrorKind>>;

    async fn estimate_gas<'a>(
        &self,
        tx: &'a TransactionRequest,
    ) -> Result<u64, RpcError<TransportErrorKind>>;

    async fn get_block_by_number(
        &self,
        number: BlockNumberOrTag,
        hydrate: bool,
    ) -> Result<Option<Block>, RpcError<TransportErrorKind>>;

    async fn send_tx_envelope(
        &self,
        tx: TxEnvelope,
    ) -> Result<TxHash, RpcError<TransportErrorKind>>;

    async fn get_transaction_receipt(&self, hash: TxHash) -> Option<TransactionReceipt>;
}
