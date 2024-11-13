use alloy::{
    consensus::TxEnvelope,
    eips::BlockNumberOrTag,
    primitives::{BlockNumber, TxHash},
    providers::{Provider, RootProvider},
    rpc::types::{Block, TransactionReceipt, TransactionRequest},
    transports::{http::Http, RpcError, TransportErrorKind, TransportResult},
};
use reqwest::Client;

use crate::eth_backend_trait::EthBackend;

pub struct AlloyBackend {
    pub provider: RootProvider<Http<Client>>,
}

#[async_trait::async_trait]
impl EthBackend for AlloyBackend {
    async fn send_tx_envelope(&self, tx: TxEnvelope) -> TransportResult<TxHash> {
        Ok(*self.provider.send_tx_envelope(tx).await?.tx_hash())
    }

    async fn get_block_number(&self) -> Result<BlockNumber, RpcError<TransportErrorKind>> {
        Ok(self.provider.get_block_number().await?)
    }

    async fn get_max_priority_fee_per_gas(&self) -> Result<u128, RpcError<TransportErrorKind>> {
        Ok(self.provider.get_max_priority_fee_per_gas().await?)
    }

    async fn estimate_gas<'a>(
        &self,
        tx: &'a TransactionRequest,
    ) -> Result<u64, RpcError<TransportErrorKind>> {
        Ok(self.provider.estimate_gas(tx).await?)
    }

    async fn get_block_by_number(
        &self,
        number: BlockNumberOrTag,
        hydrate: bool,
    ) -> Result<Option<Block>, RpcError<TransportErrorKind>> {
        Ok(self.provider.get_block_by_number(number, hydrate).await?)
    }

    async fn get_transaction_receipt(&self, hash: TxHash) -> Option<TransactionReceipt> {
        self.provider
            .get_transaction_receipt(hash)
            .await
            .ok()
            .flatten()
    }
}
