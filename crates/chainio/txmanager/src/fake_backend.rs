use alloy::{
    consensus::{ReceiptEnvelope, Transaction, TxEnvelope},
    eips::BlockNumberOrTag,
    primitives::{map::HashMap, Address, BlockNumber, TxHash},
    rpc::types::{Block, Header, TransactionReceipt, TransactionRequest},
    transports::{RpcError, TransportErrorKind, TransportResult},
};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::eth_backend_trait::EthBackend;

pub struct FakeEthBackend {
    pub base_fee_per_gas: u64,
    pub mining_params: Arc<Mutex<MiningParams>>,
}

pub struct MiningParams {
    pub block_number: u64,
    pub congested_blocks: u64,
    // we use a single nonce for now because the txmgr only supports a single sender
    pub nonce: u64,
    pub mempool: HashMap<u64, TxEnvelope>,
    pub mined_txs: HashMap<TxHash, u64>,
    pub max_priority_fee: u128,
}

impl FakeEthBackend {
    #[allow(dead_code)] // only used for testing
    pub async fn start_mining(&self) {
        let params = self.mining_params.clone();
        tokio::spawn(async move {
            loop {
                let mut params = params.lock().await;
                if let Some(tx) = params.mempool.get(&params.nonce).cloned() {
                    if tx.max_priority_fee_per_gas().unwrap() >= params.max_priority_fee {
                        let block_number = params.block_number;
                        params.mined_txs.insert(*tx.tx_hash(), block_number);
                        params.nonce += 1;
                        params.block_number += 1;
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
        let prev_max_priority_fee = {
            let mut params = self.mining_params.lock().await;
            if params.congested_blocks > 0 {
                params.max_priority_fee += 1;
            }
            params.max_priority_fee
        };
        Ok(prev_max_priority_fee)
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
        let tx_hash = *tx.tx_hash();
        let nonce = tx.nonce();
        {
            let mut params = self.mining_params.lock().await;
            if nonce < params.nonce {
                return Err(TransportErrorKind::custom_str("tx.nonce < current nonce"));
            }
            params.mempool.insert(nonce, tx);
        }
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
                authorization_list: None,
            })
    }
}
