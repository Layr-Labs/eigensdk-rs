#[derive(Debug)]
pub struct Client {}

use alloy_primitives::TxHash;

pub enum BackendType {
    HttpBackend,
    WasmBackend,
}

#[async_trait::async_trait]
pub trait BackendClient {
    type Error;

    /// Get the latest block number.
    ///
    /// # Arguments
    ///
    /// * `backend_type` - The type of the backend: HttpBackend or WasmBackend.
    ///
    /// # Returns
    ///
    /// The latest block number.
    async fn block_number(&self, backend_type: &BackendType) -> Result<u64, Self::Error>;

    /// Get the block hash given its block number.
    ///
    /// # Arguments
    ///
    /// * `backend_type` - The type of the backend: HttpBackend or WasmBackend.
    /// * `number` - The block number.
    ///
    /// # Returns
    ///
    /// The block hash of the block having that number.
    async fn block_by_number(
        &self,
        backend_type: &BackendType,
        number: u64,
    ) -> Result<Option<TxHash>, Self::Error>;
}
