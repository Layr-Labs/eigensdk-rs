use std::sync::Arc;

use alloy::{
    consensus::Receipt,
    primitives::{map::HashMap, BlockNumber, TxHash},
    rpc::types::{Block, Header, Transaction, TransactionReceipt},
    transports::{RpcError, TransportErrorKind},
};
use tokio::sync::Mutex;

#[async_trait::async_trait]
pub trait EthBackend {
    /// Get the latest block number.
    ///
    /// # Returns
    ///
    /// The latest block number.
    async fn block_number(&self) -> Result<BlockNumber, RpcError<TransportErrorKind>>;

    // when not implemented, will txmanager will fall on gasTipCap
    async fn get_max_priority_fee_per_gas(&self) -> Result<u128, RpcError<TransportErrorKind>>;

    async fn estimate_gas(&self) -> Result<u64, RpcError<TransportErrorKind>>;

    async fn get_block_by_number(
        &self,
        number: BlockNumber,
        hydrate: bool,
    ) -> Result<Block, RpcError<TransportErrorKind>>;
}

pub struct FakeEthBackend {
    congested_blocks: u64,
    base_fee_per_gas: u64,
    mining_params: Arc<Mutex<MiningParams>>,
}

pub struct MiningParams {
    block_number: u64,
    // we use a single nonce for now because the txmgr only supports a single sender
    nonce: u64,
    mempool: HashMap<u64, Transaction>,
    mined_txs: HashMap<TxHash, Receipt>,
    gas_tip_cap: u128,
}

impl FakeEthBackend {
    pub async fn start_mining(self) {
        tokio::spawn(async move {
            loop {
                let mut mining_params = self.mining_params.lock().await;
                if let Some(tx) = mining_params.mempool.get(&mining_params.nonce) {
                    if tx.max_priority_fee_per_gas.unwrap() < mining_params.gas_tip_cap {
                        let receipt = TransactionReceipt {
                            block_number: Some(mining_params.block_number),
                            transaction_hash: tx.hash,
                        };
                        mining_params.mined_txs.insert(tx.hash, receipt);
                        mining_params.nonce += 1;
                        mining_params.block_number += 1;
                        dbg!("mined tx");
                    } else {
                        dbg!("tx not mined");
                    }
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                    continue;
                }
                drop(mining_params);
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });
    }
}

#[async_trait::async_trait]
impl EthBackend for FakeEthBackend {
    async fn block_number(&self) -> Result<BlockNumber, RpcError<TransportErrorKind>> {
        Ok(self.mining_params.lock().await.block_number)
    }

    async fn get_max_priority_fee_per_gas(&self) -> Result<u128, RpcError<TransportErrorKind>> {
        let prev_gas_tip_cap = self.mining_params.lock().await.gas_tip_cap;
        if self.congested_blocks > 0 {
            self.mining_params.lock().await.gas_tip_cap += 1;
        }
        Ok(prev_gas_tip_cap)
    }

    async fn estimate_gas(&self) -> Result<u64, RpcError<TransportErrorKind>> {
        Ok(0)
    }

    async fn get_block_by_number(
        &self,
        _number: BlockNumber,
        _hydrate: bool,
    ) -> Result<Block, RpcError<TransportErrorKind>> {
        let header = Header {
            base_fee_per_gas: Some(self.base_fee_per_gas),
            ..Default::default()
        };
        Ok(Block {
            header,
            ..Default::default()
        })
    }
}
