use alloy::eips::BlockNumberOrTag;
use alloy::network::{EthereumWallet, TransactionBuilder};
use alloy::primitives::TxHash;
use alloy::providers::{Provider, RootProvider};
use alloy::rpc::types::eth::{TransactionReceipt, TransactionRequest};
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::RpcError;
use eigen_logging::logger::SharedLogger;
use std::cmp;
use std::time::Duration;
use thiserror::Error;
use tokio::time::{timeout, Instant};

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

pub struct TxRequest {
    pub tx: TransactionRequest,
    attempts: Vec<(Instant, TxHash)>, // (timestamp, tx_hash)
}

impl TxRequest {
    fn new(tx: TransactionRequest) -> Self {
        TxRequest {
            tx,
            attempts: Vec::new(),
        }
    }

    fn add_attempt(&mut self, tx_hash: TxHash, requested_at: Instant) {
        self.attempts.push((requested_at, tx_hash));
    }
}

#[derive(Debug)]
pub struct GeometricTxManagerParams {
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
        self.logger.info("new transaction", &format!("{:?}", tx));

        for _ in 0..self.params.max_send_transaction_retry {
            let estimated_gas_tip_cap = self.estimate_gas_tip_cap().await;
            self.update_gas_tip_cap(tx, estimated_gas_tip_cap).await?;

            let signed_tx = tx
                .clone()
                .build(&self.wallet)
                .await
                .inspect_err(|err| {
                    self.logger
                        .error("Failed to build and sign transaction", &err.to_string())
                })
                .map_err(|_| TxManagerError::SendTxError)?;

            let send_result = self.provider.send_tx_envelope(signed_tx.clone()).await;
            match send_result {
                Ok(tx) => {
                    self.logger.info(
                        "Transaction sent. Pending transaction: ",
                        &tx.tx_hash().to_string(),
                    );
                    let mut tx_request = TxRequest::new(signed_tx.into());
                    tx_request.add_attempt(*tx.tx_hash(), Instant::now());
                    return self.monitor_tx(tx_request).await;
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
        self.logger.error("Failed to send transaction", "");
        Err(TxManagerError::SendTxError)
    }

    /// waits until the transaction is confirmed (or failed) and resends it with a higher gas price if it
    /// is not mined within a timeout.
    /// It returns the receipt once the transaction has been confirmed.
    /// It returns an error if the transaction fails to be sent.
    pub async fn monitor_tx(
        &self,
        mut tx: TxRequest,
    ) -> Result<TransactionReceipt, TxManagerError> {
        let mut interval = tokio::time::interval(self.params.get_tx_receipt_ticker_duration);
        let mut retry_from_failures = 0;
        loop {
            interval.tick().await;

            // check if any tx got mined
            let confirmed_txs = timeout(
                self.params.txn_confirmation_timeout,
                self.ensure_any_tx_confirmed(&tx.attempts),
            )
            .await
            .unwrap()?;
            if let Some(receipt) = confirmed_txs {
                return Ok(receipt);
            }

            // send new tx with higher gas price
            self.speedup_tx(&mut tx.tx).await?;
            self.logger.info(
                "transaction not mined within timeout, resending with higher gas price",
                "",
            );
            let send_result = self.provider.send_transaction(tx.tx.clone()).await;
            match send_result {
                Ok(new_tx) => {
                    self.logger
                        .info("successfully sent txn: ", &new_tx.tx_hash().to_string());
                    tx.add_attempt(*new_tx.tx_hash(), Instant::now());
                }
                Err(e) => {
                    if retry_from_failures >= self.params.max_send_transaction_retry {
                        self.logger
                            .error("Failed to send transaction after retries", &e.to_string());
                        return Err(TxManagerError::SendTxError);
                    }
                    self.logger
                        .error("Failed to send transaction", &e.to_string());
                    retry_from_failures += 1;
                    continue;
                }
            }
        }
    }

    /// waits until at least one of the transactions is confirmed (mined + confirmationBlocks
    /// blocks). It returns the receipt of the first transaction that is confirmed (only one tx can ever be mined given they
    /// all have the same nonce).
    pub async fn ensure_any_tx_confirmed(
        &self,
        attempts: &Vec<(Instant, TxHash)>,
    ) -> Result<Option<TransactionReceipt>, TxManagerError> {
        let mut interval = tokio::time::interval(self.params.get_tx_receipt_ticker_duration);
        for (_requested_at, tx_hash) in attempts {
            interval.tick().await;
            let receipt = self
                .provider
                .get_transaction_receipt(*tx_hash)
                .await
                .map_err(|e| {
                    self.logger.error("Failed to get receipt", &e.to_string());
                    TxManagerError::WaitForReceiptError
                })?;

            if let Some(r) = receipt {
                let block_number = self.provider.get_block_number().await.unwrap_or(0);
                if r.block_number.unwrap_or(0) + self.params.confirmation_blocks > block_number {
                    self.logger.info(
                        "Transaction mined but not enough confirmations at current chain tip",
                        "",
                    );
                    break;
                }
                return Ok(Some(r));
            }
        }
        Ok(None)
    }

    // increases the gas price of the existing transaction by specified percentage.
    // It makes sure the new gas price is not lower than the current gas price.
    pub async fn speedup_tx(&self, tx: &mut TransactionRequest) -> Result<(), TxManagerError> {
        let new_gas_tip_cap = {
            let estimated_gas_tip_cap = self.estimate_gas_tip_cap().await;
            let bumped_gas_tip_cap = self
                .add_tip_cap_buffer(tx.max_priority_fee_per_gas.unwrap_or(0))
                .await;
            cmp::max(estimated_gas_tip_cap, bumped_gas_tip_cap)
        };

        self.update_gas_tip_cap(tx, new_gas_tip_cap).await?;
        self.logger.info(
            &format!(
                "increasing gas price, max_fee_per_gas={:?}, max_priority_fee_per_gas={:?}",
                tx.max_fee_per_gas, tx.max_priority_fee_per_gas
            ),
            "",
        );
        Ok(())
    }

    pub async fn update_gas_tip_cap(
        &self,
        tx: &mut TransactionRequest,
        new_gas_tip_cap: u128,
    ) -> Result<(), TxManagerError> {
        let gas_fee_cap = self.estimate_gas_fee_cap(new_gas_tip_cap).await?;
        tx.set_max_priority_fee_per_gas(new_gas_tip_cap);
        tx.set_max_fee_per_gas(gas_fee_cap);

        // we reestimate the gas limit because the state of the chain may have changed,
        // which could cause the previous gas limit to be insufficient
        let gas_limit = self
            .provider
            .estimate_gas(tx)
            .await
            .map_err(|_| TxManagerError::SendTxError)?;
        let gas_limit_buffered = (self.params.gas_multiplier * gas_limit as f64) as u64; // add gas buffer
        tx.set_gas_limit(gas_limit_buffered);
        Ok(())
    }

    // returns the gas fee cap for a transaction, calculated as:
    // gasFeeCap = 2 * baseFee + gasTipCap
    // Rationale: https://www.blocknative.com/blog/eip-1559-fees
    pub async fn estimate_gas_fee_cap(&self, gas_tip_cap: u128) -> Result<u128, TxManagerError> {
        let header = self
            .provider
            .get_block_by_number(BlockNumberOrTag::Latest, false)
            .await
            .ok()
            .flatten()
            .map(|block| block.header)
            .ok_or(TxManagerError::SendTxError)
            .inspect_err(|_| self.logger.error("Failed to get latest block header", ""))?;
        let base_fee = header.base_fee_per_gas.ok_or(TxManagerError::SendTxError)?;
        Ok(base_fee as u128 * 2 + gas_tip_cap)
    }

    pub async fn estimate_gas_tip_cap(&self) -> u128 {
        self.provider.get_max_priority_fee_per_gas().await
        .inspect_err(|err|
            self.logger.info("eth_maxPriorityFeePerGas is unsupported by current backend, using fallback gasTipCap",
            &err.to_string()))
        .unwrap_or(self.params.fallback_gas_tip_cap as _)
    }

    pub async fn add_tip_cap_buffer(&self, gas_tip_cap: u128) -> u128 {
        gas_tip_cap * (self.params.gas_tip_multiplier * 100.0) as u128 / 100
    }
}

#[cfg(test)]
mod tests {
    use super::{GeometricTxManager, GeometricTxManagerParams};
    use alloy::network::TransactionBuilder;
    use alloy::primitives::{address, U256};
    use alloy::providers::ProviderBuilder;
    use alloy::rpc::types::eth::TransactionRequest;
    use eigen_logging::log_level::LogLevel;
    use eigen_logging::{get_logger, init_logger};
    use eigen_signer::signer::Config;
    use eigen_testing_utils::anvil::start_anvil_container;
    use std::time::Duration;

    const TEST_PRIVATE_KEY: &str =
        "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";

    #[tokio::test]
    async fn test_send_single_transaction() {
        let (_container, rpc_url, _ws_endpoint) = start_anvil_container().await;
        init_logger(LogLevel::Info);
        let logger = get_logger();
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
        init_logger(LogLevel::Info);
        let logger = get_logger();
        let config = Config::PrivateKey(TEST_PRIVATE_KEY.to_string());
        let signer = Config::signer_from_config(config).unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url.parse().unwrap());
        let params = GeometricTxManagerParams {
            txn_confirmation_timeout: Duration::from_secs(5),
            ..Default::default()
        };

        let geometric_tx_manager =
            GeometricTxManager::new(logger, signer, provider, params).unwrap();
        // TODO: simulate congested network increasing gas base fee and gas tip needed. Mock the provider

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
