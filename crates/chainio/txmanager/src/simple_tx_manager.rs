use alloy_consensus::{Transaction, TxEip1559, TxLegacy};
use alloy_eips::BlockNumberOrTag;
use alloy_network::{Ethereum, TransactionBuilder, TxSigner};
use alloy_primitives::{Address, TxKind};
use alloy_provider::{PendingTransactionBuilder, Provider, ProviderBuilder, RootProvider};
use alloy_rpc_types_eth::{TransactionInput, TransactionReceipt, TransactionRequest};
use alloy_signer_local::PrivateKeySigner;
use eigen_logging::{logger::Logger, tracing_logger::TracingLogger};
use eigen_signer::signer::Config;
use k256::ecdsa::SigningKey;
use reqwest::Url;
use thiserror::Error;

static FALLBACK_GAS_TIP_CAP: u128 = 5_000_000_000;

pub type Transport = alloy_transport_http::Http<reqwest::Client>;

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

pub struct SimpleTxManager<'log> {
    logger: &'log TracingLogger,
    gas_limit_multiplier: f64,
    private_key: String,
    provider: RootProvider<Transport>,
}

impl<'log> SimpleTxManager<'log> {
    /// Creates a new SimpleTxManager.
    ///
    /// # Arguments
    ///
    /// - `logger`: The logger to be used.
    /// - `gas_limit_multiplier`: The gas limit multiplier.
    /// - `private_key`: The private key of the wallet.
    /// - `rpc_url`: The RPC URL. It could be an anvil node or any other node.
    ///
    /// # Returns
    ///
    /// - The SimpleTxManager created.
    ///
    /// # Errors
    ///
    /// - If the URL is invalid.
    pub fn new(
        logger: &'log TracingLogger,
        gas_limit_multiplier: f64,
        private_key: &str,
        rpc_url: &str,
    ) -> Result<SimpleTxManager<'log>, TxManagerError> {
        let url = Url::parse(rpc_url).map_err(|_| TxManagerError::InvalidUrlError)?;
        let provider = ProviderBuilder::new().on_http(url);
        Ok(SimpleTxManager {
            logger,
            gas_limit_multiplier,
            private_key: private_key.to_string(),
            provider,
        })
    }

    /// Returns the address of the wallet, beloing to the given private key.
    ///
    /// # Returns
    ///
    /// - The address of the wallet.
    ///
    /// # Errors
    ///
    /// - If the private key is invalid.
    pub fn get_address(&self) -> Result<Address, TxManagerError> {
        let private_key_signing_key = SigningKey::from_slice(self.private_key.as_bytes())
            .map_err(|_| TxManagerError::AddressError)?;
        Ok(Address::from_private_key(&private_key_signing_key))
    }

    pub fn with_gas_limit_multiplier(&mut self, multiplier: f64) {
        self.gas_limit_multiplier = multiplier;
    }

    fn create_local_signer(&self) -> Result<PrivateKeySigner, TxManagerError> {
        let config = Config::PrivateKey(self.private_key.clone());
        Config::signer_from_config(config).map_err(|_| TxManagerError::SignerError)
    }

    /// Sends a EIP1559 transaction.
    ///
    /// # Arguments
    ///
    /// - `tx`: The transaction to be sent.
    ///
    /// # Returns
    ///
    /// - The transaction receipt.
    ///
    /// # Errors
    ///
    /// - If the transaction could not be signed.
    /// - If the transaction could not be sent.
    /// - If the transaction could not be mined.
    pub async fn send_eip1559_tx(
        &self,
        tx: &mut TxEip1559,
    ) -> Result<TransactionReceipt, TxManagerError> {
        let signer = self.create_local_signer()?;
        let _ = signer
            .sign_transaction(tx)
            .await
            .map_err(|_| TxManagerError::SignerError)?;

        self.logger.debug("Transaction signed", &[&tx]);

        // send transaction and get receipt
        let pending_tx = self
            .provider
            .send_transaction(tx.clone().into())
            .await
            .map_err(|_| TxManagerError::SendTxError)?;

        self.logger
            .debug("Transaction sent. Pending transaction: ", &[&pending_tx]);

        // wait for the transaction to be mined
        SimpleTxManager::wait_for_receipt(pending_tx).await
    }

    /// Send is used to send a transaction to the Ethereum node. It takes an unsigned/signed transaction
    /// and then sends it to the Ethereum node.
    /// If you pass in a signed transaction it will ignore the signature
    /// and resign the transaction after adding the nonce and gas limit.
    ///
    /// # Arguments
    ///
    /// - `tx`: The transaction to be sent.
    ///
    /// # Returns
    ///
    /// - The transaction receipt.
    pub async fn send_legacy_tx(
        &self,
        tx: &mut TxLegacy,
    ) -> Result<TransactionReceipt, TxManagerError> {
        // TODO: It also takes care of gas estimation and adds a buffer to the gas limit
        // TODO: Estimating gas and nonce
        //m.log.Debug("Estimating gas and nonce")
        //tx, err := m.estimateGasAndNonce(ctx, tx)
        self.estimate_gas_and_nonce(tx);
        let signer = self.create_local_signer()?;
        let _signed_tx = signer
            .sign_transaction(tx)
            .await
            .map_err(|_| TxManagerError::SignerError)?;

        self.logger.debug("Transaction signed", &[&tx]);

        // send transaction and get receipt
        let pending_tx = self
            .provider
            .send_transaction(tx.clone().into())
            .await
            .map_err(|_| TxManagerError::SendTxError)?;

        self.logger
            .debug("Transaction sent. Pending transaction: ", &[&pending_tx]);

        // wait for the transaction to be mined
        SimpleTxManager::wait_for_receipt(pending_tx).await
    }

    async fn estimate_gas_and_nonce(&self, tx: &mut TxLegacy) -> Result<(), TxManagerError> {
        let gas_tip_cap = match self.provider.get_max_priority_fee_per_gas().await {
            Ok(gas_tip) => gas_tip,
            Err(err) => {
                self.logger.info("eth_maxPriorityFeePerGas is unsupported by current backend, using fallback gasTipCap", &[err]);
                FALLBACK_GAS_TIP_CAP
            }
        };

        let header = match self.provider.get_block_by_number(BlockNumberOrTag::Latest, false).await.unwrap() {
            Some(block) => block.header,
            None => {
                self.logger.error("Failed to get latest block header", &[()]);
                return Err(TxManagerError::SendTxError);
            }
        };

        // 2*baseFee + gasTipCap makes sure that the tx remains includeable for 6 consecutive 100% full blocks.
	    // see https://www.blocknative.com/blog/eip-1559-fees
        let gas_fee_cap = header.base_fee_per_gas.unwrap() * 2 + gas_tip_cap; // TODO: Check unwrap
        // TODO: check if gas_fee_cap is always 1.

        let gas_limit = tx.gas_limit();

        // we only estimate if gas_limit is not already set
        if gas_limit == 0 {
            let from = self.get_address()?;
            let to = match tx.to() {
                TxKind::Call(c) => c,
                TxKind::Create => return Err(TxManagerError::SendTxError),
            };
            let mut tx_request = TransactionRequest::default().to(to).from(from).value(tx.value()).input(TransactionInput::new(tx.input));
            tx_request.set_max_priority_fee_per_gas(gas_tip_cap);
            tx_request.set_max_fee_per_gas(gas_fee_cap);

            let gas_limit = self
                .provider
                .estimate_gas(&tx_request)
                .await
                .map_err(|_| TxManagerError::SendTxError)?;

        }

        todo!() // TODO: build tx with gas limit
    }

    /// Waits for the transaction receipt.
    ///
    /// This is a wrapper around `PendingTransactionBuilder::get_receipt`.
    ///
    /// # Arguments
    ///
    /// - `pending_tx`: The pending transaction builder we want to wait for.
    ///
    /// # Returns
    ///
    /// - The block number in which the transaction was included.
    /// - `None` if the transaction was not included in a block or an error ocurred.
    pub async fn wait_for_receipt(
        pending_tx: PendingTransactionBuilder<'_, Transport, Ethereum>,
    ) -> Result<TransactionReceipt, TxManagerError> {
        pending_tx
            .get_receipt()
            .await
            .map_err(|_| TxManagerError::WaitForReceiptError)
    }

    /*
    // GetNoSendTxOpts This generates a noSend TransactOpts so that we can use
    // this to generate the transaction without actually sending it
    func (m *SimpleTxManager) GetNoSendTxOpts() (*bind.TransactOpts, error) {
    }

    func (m *SimpleTxManager) queryReceipt(ctx context.Context, txID wallet.TxID) *types.Receipt {
    }

    // estimateGasAndNonce we are explicitly implementing this because
    // * We want to support legacy transactions (i.e. not dynamic fee)
    // * We want to support gas management, i.e. add buffer to gas limit
    func (m *SimpleTxManager) estimateGasAndNonce(ctx context.Context, tx *types.Transaction) (*types.Transaction, error) {
    }
    */
}

#[cfg(test)]
mod tests {
    use super::SimpleTxManager;
    use alloy_consensus::TxLegacy;
    use alloy_node_bindings::Anvil;
    use alloy_primitives::{bytes, TxKind::Call, U256};
    use eigen_logging::{log_level::LogLevel, logger::Logger, tracing_logger::TracingLogger};
    use once_cell::sync::OnceCell;
    use tokio;

    static TEST_LOGGER: OnceCell<TracingLogger> = OnceCell::new();
    const PRIVATE_KEY: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";

    #[tokio::test]
    async fn test_send_signed_transaction() {
        TEST_LOGGER.get_or_init(|| {
            TracingLogger::new_text_logger(false, String::from(""), LogLevel::Debug, false)
        });

        // Spin up a local Anvil node.
        // Ensure `anvil` is available in $PATH.
        let anvil = Anvil::new().try_spawn().unwrap();
        let rpc_url: String = anvil.endpoint().parse().unwrap();

        // Create a provider.
        let logger = TEST_LOGGER.get().unwrap();
        let simple_tx_manager =
            SimpleTxManager::new(logger, 1.0, PRIVATE_KEY, rpc_url.as_str()).unwrap();

        // Create two users, Alice and Bob.
        let _alice = anvil.addresses()[0];
        let bob = anvil.addresses()[1];

        let mut tx = TxLegacy {
            to: Call(bob),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(31337),
        };

        //// send transaction and wait for receipt
        let receipt = simple_tx_manager.send_legacy_tx(&mut tx).await.unwrap();
        let block_number = receipt.block_number.unwrap();
        println!("Transaction mined in block: {}", block_number);
        assert!(block_number > 0);
        assert_eq!(receipt.to, Some(bob));
    }
}
