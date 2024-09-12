use super::anvil_constants::ANVIL_RPC_URL;
use alloy_primitives::FixedBytes;
use alloy_provider::Provider;
use tokio::time::{sleep, Duration};

/// Retrieves the status of a transaction from its hash.
///
/// # Arguments
///
/// `tx_hash` - The hash of the transaction.
///
/// # Returns
///
/// A bool indicating wether the transaction was successful or not.
pub async fn get_transaction_status(tx_hash: FixedBytes<32>) -> bool {
    // this sleep is needed so that we wait for the tx to be processed
    sleep(Duration::from_millis(500)).await;
    ANVIL_RPC_URL
        .clone()
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap()
        .unwrap()
        .status()
}
