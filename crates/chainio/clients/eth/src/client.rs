#[derive(Debug)]
pub struct Client {}

use async_trait::async_trait;
use ethers::providers::Middleware;
use ethers::types::{Block, BlockNumber, U64};

pub enum BackendType {
    HttpBAckend,
    WasmBackend,
}

#[async_trait::async_trait]
pub trait BackendClient {
    type Error;

    async fn block_number(&self, backend_type: &BackendType) -> Result<U64, Self::Error>;
    async fn block_by_number(
        &self,
        backend_type: &BackendType,
        number: Option<BlockNumber>,
    ) -> Result<Option<Block<ethers::types::TxHash>>, Self::Error>;
}
