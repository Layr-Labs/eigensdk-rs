#[derive(Debug)]
pub struct Client {}

use alloy::eips::BlockNumberOrTag;
use alloy::primitives::BlockNumber;
use alloy::rpc::types::eth::Block;

#[async_trait::async_trait]
pub trait BackendClient {
    type Error;

    /// Get the latest block number.
    ///
    /// # Returns
    ///
    /// The latest block number.
    async fn block_number(&self) -> Result<BlockNumber, Self::Error>;

    /// Get the block hash given its block number.
    ///
    /// # Arguments
    ///
    /// * `number` - The block number.
    ///
    /// # Returns
    ///
    /// The block having that number.
    async fn block_by_number(&self, number: BlockNumberOrTag)
        -> Result<Option<Block>, Self::Error>;
}
