use alloy_primitives::{FixedBytes, TxHash};
use alloy_provider::{PendingTransactionBuilder, Provider, ProviderBuilder};
use alloy_transport::{TransportErrorKind, TransportResult};
use eigen_utils::get_provider;
use tokio::time::{sleep, Duration};
use url::Url;

/// Retrieves the status of a transaction from its hash.
///
/// # Arguments
///
/// `rpc_url` - The RPC URL.
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

/// Wait for a transaction to finish.
///
/// # Arguments
///
/// `rpc_url` - The RPC URL.
/// `tx_hash` - The hash of the transaction.
///
/// # Returns
///
/// A [`TransportResult`] containing the transaction hash.
pub async fn wait_transaction(rpc_url: String, tx_hash: FixedBytes<32>) -> TransportResult<TxHash> {
    let url =
        Url::parse(&rpc_url).map_err(|_| TransportErrorKind::custom_str("Invalid RPC URL"))?;
    let root_provider = ProviderBuilder::new().on_http(url);
    let pending_tx = PendingTransactionBuilder::new(&root_provider, tx_hash);
    pending_tx.watch().await
}
