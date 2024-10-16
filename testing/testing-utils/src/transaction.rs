use alloy::providers::Provider;
use alloy_primitives::FixedBytes;
use eigen_utils::get_provider;
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
pub async fn get_transaction_status(rpc_url: String, tx_hash: FixedBytes<32>) -> bool {
    // this sleep is needed so that we wait for the tx to be processed
    sleep(Duration::from_millis(500)).await;
    get_provider(&rpc_url)
        .clone()
        .get_transaction_receipt(tx_hash)
        .await
        .unwrap_or(None)
        .map(|receipt| receipt.status())
        .unwrap_or(false)
}
