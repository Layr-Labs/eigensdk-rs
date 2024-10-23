use alloy::network::{Ethereum, EthereumWallet, TransactionBuilder, TxSigner};
use alloy::providers::{PendingTransactionBuilder, Provider, RootProvider};
use alloy::rpc::types::eth::{TransactionReceipt, TransactionRequest};
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::RpcError;
use eigen_logging::logger::SharedLogger;
use std::time::Duration;
use thiserror::Error;

pub type Transport = alloy::transports::http::Http<reqwest::Client>;

/// Possible errors raised in Tx Manager
#[derive(Error, Debug)]
pub enum TxManagerError {
    #[error("signer error")]
    SignerError,
    #[error("send error")]
    SendTxError,
    #[error("wait_for_receipt error")]
    WaitForReceiptError,
    #[error("address error")]
    AddressError,
    #[error("invalid url error")]
    InvalidUrlError,
}

/// A simple transaction manager that encapsulates operations to send transactions to an Ethereum node.
pub struct GeometricTxManager {
    logger: SharedLogger,
    wallet: EthereumWallet,
    provider: RootProvider<Transport>,
    params: GeometricTxManagerParams,
}

#[derive(Debug)]
pub struct GeometricTxManagerParams {
    // TODO: make fields optional and use default values
    pub confirmation_blocks: u64,
    pub txn_broadcast_timeout: Duration,
    pub txn_confirmation_timeout: Duration,
    pub max_send_transaction_retry: i32,
    pub get_tx_receipt_ticker_duration: Duration,
    pub fallback_gas_tip_cap: u64,
    pub gas_multiplier: f64,
    pub gas_tip_multiplier: f64,
}

impl Default for GeometricTxManagerParams {
    fn default() -> Self {
        GeometricTxManagerParams {
            confirmation_blocks: 0,
            txn_broadcast_timeout: Duration::from_secs(2 * 60), // 2 minutes
            txn_confirmation_timeout: Duration::from_secs(5 * 12), // 5 blocks
            max_send_transaction_retry: 3,
            get_tx_receipt_ticker_duration: Duration::from_secs(3),
            fallback_gas_tip_cap: 5_000_000_000, // 5 gwei
            gas_multiplier: 1.20,
            gas_tip_multiplier: 1.25,
        }
    }
}

impl GeometricTxManager {
    /// Creates a new SimpleTxManager.
    ///
    /// # Arguments
    ///
    /// * `logger`: The logger to be used.
    /// * `gas_limit_multiplier`: The gas limit multiplier.
    /// * `private_key`: The private key of the wallet.
    /// * `rpc_url`: The RPC URL. It could be an anvil node or any other node.
    /// * `params`: The parameters for the GeometricTxManager.
    ///
    /// # Returns
    ///
    /// * The SimpleTxManager created.
    ///
    /// # Errors
    ///
    /// * If the URL is invalid.
    pub fn new(
        logger: SharedLogger,
        signer: PrivateKeySigner,
        provider: RootProvider<Transport>,
        params: GeometricTxManagerParams,
    ) -> Result<GeometricTxManager, TxManagerError> {
        let wallet = EthereumWallet::from(signer);
        Ok(GeometricTxManager {
            logger,
            wallet,
            provider,
            params,
        })
    }

    /// Send is used to send a transaction to the Ethereum node. It takes an unsigned/signed transaction,
    /// sends it to the Ethereum node and waits for the receipt.
    /// If you pass in a signed transaction it will ignore the signature
    /// and re-sign the transaction after adding the nonce and gas limit.
    ///
    /// # Arguments
    ///
    /// * `tx`: The transaction to be sent.
    ///
    /// # Returns
    ///
    /// * `TransactionReceipt` The transaction receipt.
    ///
    /// # Errors
    ///
    /// * `TxManagerError` - If the transaction cannot be sent, or there is an error
    ///   signing the transaction or estimating gas and nonce.
    pub async fn send_tx(
        &self,
        tx: &mut TransactionRequest,
    ) -> Result<TransactionReceipt, TxManagerError> {
        self.logger.debug("new transaction", &format!("{:?}", tx));
        let _from = self.wallet.default_signer().address();
        let signed_tx = tx
            .clone()
            .build(&self.wallet)
            .await
            .inspect_err(|err| {
                self.logger
                    .error("Failed to build and sign transaction", &err.to_string())
            })
            .map_err(|_| TxManagerError::SendTxError)?;

        let mut pending_tx = None;
        for _ in 0..self.params.max_send_transaction_retry {
            // TODO: estimate gas tip cap

            // TODO: update gas tip cap

            // send transaction
            let send_result = self
                .provider
                .send_transaction(signed_tx.clone().into())
                .await;

            match send_result {
                Ok(tx) => {
                    self.logger.debug(
                        "Transaction sent. Pending transaction: ",
                        &tx.tx_hash().to_string(),
                    );
                    pending_tx = Some(tx);
                    break;
                }
                Err(RpcError::Transport(
                    // TODO: check if this is the timeout error
                    alloy::transports::TransportErrorKind::MissingBatchResponse(id),
                )) => {
                    self.logger.warn(
                        "Failed to send transaction due to timeout",
                        &format!("{:?}", id.as_string()),
                    );
                    continue;
                }
                Err(e) => {
                    self.logger
                        .error("Failed to send transaction", &e.to_string());
                    return Err(TxManagerError::SendTxError);
                }
            };
        }

        let Some(sent_tx) = pending_tx else {
            self.logger.error(
                "Failed to send transaction",
                &signed_tx.tx_hash().to_string(),
            );
            return Err(TxManagerError::SendTxError);
        };

        GeometricTxManager::wait_for_receipt(self, sent_tx).await
    }

    /// Waits for the transaction receipt.
    ///
    /// This is a wrapper around `PendingTransactionBuilder::get_receipt`.
    ///
    /// # Arguments
    ///
    /// * `pending_tx`: The pending transaction builder we want to wait for.
    ///
    /// # Returns
    ///
    /// * The block number in which the transaction was included.
    /// * `None` if the transaction was not included in a block or an error ocurred.
    ///
    /// # Errors
    ///
    /// * `TxManagerError` - If the transaction receipt cannot be retrieved.
    pub async fn wait_for_receipt(
        &self,
        pending_tx: PendingTransactionBuilder<'_, Transport, Ethereum>,
    ) -> Result<TransactionReceipt, TxManagerError> {
        pending_tx
            .get_receipt()
            .await
            .inspect_err(|err| self.logger.error("Failed to get receipt", &err.to_string()))
            .map_err(|_| TxManagerError::WaitForReceiptError)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::{GeometricTxManager, GeometricTxManagerParams};
    use alloy::consensus::TxLegacy;
    use alloy::network::TransactionBuilder;
    use alloy::primitives::{address, bytes, TxKind::Call, U256};
    use alloy::providers::ProviderBuilder;
    use alloy::rpc::types::eth::TransactionRequest;
    use eigen_logging::get_test_logger;
    use eigen_signer::signer::Config;
    use eigen_testing_utils::anvil::start_anvil_container;

    const TEST_PRIVATE_KEY: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    #[tokio::test]
    async fn test_send_transaction_from_legacy() {
        let (_container, rpc_url, _ws_endpoint) = start_anvil_container().await;
        let logger = get_test_logger();
        let config = Config::PrivateKey(TEST_PRIVATE_KEY.to_string());
        let signer = Config::signer_from_config(config).unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());

        let geometric_tx_manager =
            GeometricTxManager::new(logger, signer, provider, Default::default()).unwrap();

        let to = address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720");
        let account_nonce = 0x69;
        let tx = TxLegacy {
            to: Call(to),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: account_nonce,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(31337),
        };

        let mut tx_request: TransactionRequest = tx.into();
        // send transaction and wait for receipt
        let receipt = geometric_tx_manager.send_tx(&mut tx_request).await.unwrap();
        let block_number = receipt.block_number.unwrap();
        println!("Transaction mined in block: {}", block_number);
        assert!(block_number > 0);
        assert_eq!(receipt.to, Some(to));
    }

    #[tokio::test]
    async fn test_send_transaction_from_eip1559() {
        let (_container, rpc_url, _ws_endpoint) = start_anvil_container().await;
        let logger = get_test_logger();
        let config = Config::PrivateKey(TEST_PRIVATE_KEY.to_string());
        let signer = Config::signer_from_config(config).unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());

        let geometric_tx_manager =
            GeometricTxManager::new(logger, signer, provider, Default::default()).unwrap();

        let to = address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720");
        let account_nonce = 0x69;
        let mut tx = TransactionRequest::default()
            .with_to(to)
            .with_nonce(account_nonce)
            .with_chain_id(31337)
            .with_value(U256::from(100))
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000)
            .with_gas_price(21_000_000_000);

        // send transaction and wait for receipt
        let receipt = geometric_tx_manager.send_tx(&mut tx).await.unwrap();
        let block_number = receipt.block_number.unwrap();
        println!("Transaction mined in block: {}", block_number);
        assert!(block_number > 0);
        assert_eq!(receipt.to, Some(to));
    }

    #[tokio::test]
    async fn test_send_transaction_to_congested_network() {
        let (_container, rpc_url, _ws_endpoint) = start_anvil_container().await;
        let logger = get_test_logger();
        let config = Config::PrivateKey(TEST_PRIVATE_KEY.to_string());
        let signer = Config::signer_from_config(config).unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());
        let mut params = GeometricTxManagerParams::default();
        params.txn_confirmation_timeout = Duration::from_secs(5);

        let geometric_tx_manager =
            GeometricTxManager::new(logger, signer, provider, Default::default()).unwrap();

        let to = address!("a0Ee7A142d267C1f36714E4a8F75612F20a79720");
        let account_nonce = 0x69;
        let mut tx = TransactionRequest::default()
            .with_to(to)
            .with_nonce(account_nonce)
            .with_chain_id(31337)
            .with_value(U256::from(100))
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000)
            .with_gas_price(21_000_000_000);

        // send transaction and wait for receipt
        let receipt = geometric_tx_manager.send_tx(&mut tx).await.unwrap();
        let block_number = receipt.block_number.unwrap();
        println!("Transaction mined in block: {}", block_number);
        assert!(block_number > 0);
        assert_eq!(receipt.to, Some(to));
    }
}
