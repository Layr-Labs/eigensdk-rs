use std::sync::Arc;

use alloy::{
    consensus::{ReceiptEnvelope, Transaction, TxEnvelope},
    eips::BlockNumberOrTag,
    primitives::{map::HashMap, Address, BlockNumber, TxHash},
    providers::{Provider, RootProvider},
    rpc::types::{Block, Header, TransactionReceipt, TransactionRequest},
    transports::{http::Http, RpcError, TransportErrorKind, TransportResult},
};
use reqwest::Client;
use tokio::sync::Mutex;

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

pub struct FakeEthBackend {
    pub base_fee_per_gas: u64,
    pub mining_params: Arc<Mutex<MiningParams>>, // TODO: use one lock for each param?
}

pub struct MiningParams {
    pub block_number: u64,
    pub congested_blocks: u64,
    // we use a single nonce for now because the txmgr only supports a single sender
    pub nonce: u64,
    pub mempool: HashMap<u64, TxEnvelope>,
    pub mined_txs: HashMap<TxHash, u64>,
    pub gas_tip_cap: u128,
}

impl FakeEthBackend {
    pub async fn start_mining(&self) {
        let params = self.mining_params.clone();
        tokio::spawn(async move {
            dbg!("mining...");
            loop {
                let mut params = params.lock().await;
                if let Some(tx) = params.mempool.get(&params.nonce).cloned() {
                    if tx.max_priority_fee_per_gas().unwrap() >= params.gas_tip_cap {
                        let block_number = params.block_number;
                        params.mined_txs.insert(*tx.tx_hash(), block_number);
                        params.nonce += 1;
                        params.block_number += 1;
                        dbg!("mined tx");
                    } else {
                        dbg!("tx not mined");
                    }
                }
                params.congested_blocks = params.congested_blocks.saturating_sub(1);
                drop(params);
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });
    }
}

#[async_trait::async_trait]
impl EthBackend for FakeEthBackend {
    async fn get_block_number(&self) -> Result<BlockNumber, RpcError<TransportErrorKind>> {
        Ok(self.mining_params.lock().await.block_number)
    }

    async fn get_max_priority_fee_per_gas(&self) -> Result<u128, RpcError<TransportErrorKind>> {
        let mut params = self.mining_params.lock().await;
        let prev_gas_tip_cap = params.gas_tip_cap;
        if params.congested_blocks > 0 {
            params.gas_tip_cap += 1;
        }
        Ok(prev_gas_tip_cap)
    }

    async fn estimate_gas<'a>(
        &self,
        _tx: &'a TransactionRequest,
    ) -> Result<u64, RpcError<TransportErrorKind>> {
        Ok(0)
    }

    async fn get_block_by_number(
        &self,
        _number: BlockNumberOrTag,
        _hydrate: bool,
    ) -> Result<Option<Block>, RpcError<TransportErrorKind>> {
        let header = Header {
            base_fee_per_gas: Some(self.base_fee_per_gas),
            ..Default::default()
        };
        Ok(Some(Block {
            header,
            ..Default::default()
        }))
    }

    async fn send_tx_envelope(&self, tx: TxEnvelope) -> TransportResult<TxHash> {
        if tx.max_fee_per_gas() < self.base_fee_per_gas as u128 {
            return Err(TransportErrorKind::custom_str(
                "tx.max_fee_per_gas < base_fee_per_gas",
            ));
        }
        let mut params = self.mining_params.lock().await;
        let tx_hash = *tx.tx_hash();
        let nonce = tx.nonce();
        if nonce < params.nonce {
            return Err(TransportErrorKind::custom_str("tx.nonce < current nonce"));
        }
        params.mempool.insert(nonce, tx);
        Ok(tx_hash)
    }

    async fn get_transaction_receipt(&self, hash: TxHash) -> Option<TransactionReceipt> {
        let params = self.mining_params.lock().await;
        params
            .mined_txs
            .get(&hash)
            .map(|block_number| TransactionReceipt {
                block_number: Some(*block_number),
                transaction_hash: hash,
                gas_used: 0,
                inner: ReceiptEnvelope::Legacy(Default::default()),
                transaction_index: None,
                block_hash: None,
                effective_gas_price: 0,
                blob_gas_used: None,
                blob_gas_price: None,
                from: Address::default(),
                to: None,
                contract_address: None,
                state_root: None,
                authorization_list: None,
            })
    }
}

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
