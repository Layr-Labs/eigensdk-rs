use alloy_consensus::TxEip1559;
use alloy_network::EthereumWallet;
use alloy_primitives::Address;

pub struct SimpleTxManager {
    wallet: EthereumWallet,
    //    client: eth::Client,
    //log: logging::Logger,
    sender: Address,
    gas_limit_multiplier: f64,
}

/*
Go interface:

// ========
type TxManager interface {
    // Send is used to send a transaction
    // It takes an unsigned transaction and then signs it before sending
    // It also takes care of nonce management and gas estimation
    Send(ctx context.Context, tx *types.Transaction) (*types.Receipt, error)

    // GetNoSendTxOpts This generates a noSend TransactOpts so that we can use
    // this to generate the transaction without actually sending it
    GetNoSendTxOpts() (*bind.TransactOpts, error)
}
// =======

// Send is used to send a transaction to the Ethereum node. It takes an unsigned/signed transaction
// and then sends it to the Ethereum node.
// It also takes care of gas estimation and adds a buffer to the gas limit
// If you pass in a signed transaction it will ignore the signature
// and resign the transaction after adding the nonce and gas limit.
// To check out the whole flow on how this works, check out the README.md in this folder
func (m *SimpleTxManager) Send(ctx context.Context, tx *types.Transaction) (*types.Receipt, error) {
    // Estimate gas and nonce
    // can't print tx hash in logs because the tx changes below when we complete and sign it
    // so the txHash is meaningless at this point
    // ...
}

func NoopSigner(addr common.Address, tx *types.Transaction) (*types.Transaction, error) {
    return tx, nil
}

// GetNoSendTxOpts This generates a noSend TransactOpts so that we can use
// this to generate the transaction without actually sending it
func (m *SimpleTxManager) GetNoSendTxOpts() (*bind.TransactOpts, error) {
}

func (m *SimpleTxManager) waitForReceipt(ctx context.Context, txID wallet.TxID) (*types.Receipt, error) {
}

func (m *SimpleTxManager) queryReceipt(ctx context.Context, txID wallet.TxID) *types.Receipt {
}

// estimateGasAndNonce we are explicitly implementing this because
// * We want to support legacy transactions (i.e. not dynamic fee)
// * We want to support gas management, i.e. add buffer to gas limit
func (m *SimpleTxManager) estimateGasAndNonce(ctx context.Context, tx *types.Transaction) (*types.Transaction, error) {
}

*/

impl SimpleTxManager {
    /*
    pub fn new(
        wallet: wallet::Wallet,
        client: eth::Client,
        log: logging::Logger,
        sender: common::Address,
        gas_limit_multiplier: f64,
    ) -> SimpleTxManager {
        SimpleTxManager {
            wallet,
            client,
            log,
            sender,
            gas_limit_multiplier,
        }
    }
    */

    pub fn with_gas_limit_multiplier(&mut self, multiplier: f64) {
        self.gas_limit_multiplier = multiplier;
    }

    pub fn send(tx: &TxEip1559) {
        /*
            // Spin up a local Anvil node.
            // Ensure `anvil` is available in $PATH.
            let anvil = Anvil::new().try_spawn()?;

            // Create a provider.
            let rpc_url = anvil.endpoint().parse()?;
            let provider = ProviderBuilder::new().on_http(rpc_url);

            // Create two users, Alice and Bob.
            let alice = anvil.addresses()[0];
            let bob = anvil.addresses()[1];
        */

        // #####################

        /*
        // Set up the provider
        let provider = Provider::<Http>::try_from("https://mainnet.infura.io/v3/YOUR-PROJECT-ID")?;

        // Set up the signer (you'll need to replace this with your actual private key)
        let signer = Signer::from_private_key("YOUR_PRIVATE_KEY_HERE".parse()?);

        // Create the transaction request
        let tx = TransactionRequest::new()
            .to(Address::from_str(
                "0x742d35Cc6634C0532925a3b844Bc454e4438f44e",
            )?)
            .value(U256::from(1_000_000_000_000_000_000u128)) // 1 ETH in wei
            .gas_limit(21000)
            .gas_price(provider.get_gas_price().await?)
            .nonce(
                provider
                    .get_transaction_count(signer.address(), None)
                    .await?,
            );

        // Sign and send the transaction
        let signed_tx = signer.sign_transaction(&tx).await?;
        let pending_tx = provider.send_raw_transaction(signed_tx.rlp()).await?;

        // Wait for the transaction to be mined
        let receipt = pending_tx.await?;
        println!("Transaction mined in block: {:?}", receipt.block_number);

        // #####################
        let tx = TransactionRequest::default()
            .with_to(bob)
            .with_nonce(0)
            .with_chain_id(anvil.chain_id())
            .with_value(U256::from(100))
            .with_gas_limit(21_000)
            .with_max_priority_fee_per_gas(1_000_000_000)
            .with_max_fee_per_gas(20_000_000_000);
        */

        /*
        // Send the transaction and wait for the broadcast.
        let pending_tx = provider.send_transaction(tx).await?;

        println!("Pending transaction... {}", pending_tx.tx_hash());

        // Wait for the transaction to be included and get the receipt.
        let receipt = pending_tx.get_receipt().await?;

        println!(
            "Transaction included in block {}",
            receipt.block_number.expect("Failed to get block number")
        );

        assert_eq!(receipt.from, alice);
        assert_eq!(receipt.to, Some(bob));

        Ok(())
        */
    }

    /// Waits for the transaction receipt.
    /// The waiting is done by polling the node for the transaction receipt, using `PollerBuilder`
    /// in `alloy_rpc_client`
    pub fn wait_for_receipt() {
        // # async fn example<T: alloy_transport::Transport + Clone>(client: alloy_rpc_client::RpcClient<T>) -> Result<(), Box<dyn std::error::Error>> {
        //
        // let poller: PollerBuilder<_, (), U64> = client
        //     .prepare_static_poller("eth_blockNumber", ())
        //     .with_poll_interval(std::time::Duration::from_secs(5));
        // let mut stream = poller.into_stream();
        // while let Some(block_number) = stream.next().await {
        //    println!("polled block number: {block_number}");
        // }
        // # Ok(())
        // # }
    }
}

#[cfg(test)]
mod tests {
    use alloy_consensus::TxLegacy;
    use alloy_network::TxSigner;
    use alloy_node_bindings::Anvil;
    use alloy_primitives::{bytes, TxKind::Call, U256};
    use alloy_provider::{Provider, ProviderBuilder};
    use eigen_signer::signer::Config;
    use tokio;

    const PRIVATE_KEY: &str = "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7";

    #[tokio::test]
    async fn test_send_signed_transaction() {
        // Spin up a local Anvil node.
        // Ensure `anvil` is available in $PATH.
        let anvil = Anvil::new().try_spawn().unwrap();

        // Create a provider.
        let rpc_url = anvil.endpoint().parse().unwrap();
        let provider = ProviderBuilder::new().on_http(rpc_url);

        // Create two users, Alice and Bob.
        let _alice = anvil.addresses()[0];
        let bob = anvil.addresses()[1];

        let config = Config::PrivateKey(PRIVATE_KEY.into());
        let signer = Config::signer_from_config(config).unwrap();

        let mut tx = TxLegacy {
            to: Call(bob),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(31337),
        };
        let _signed_tx = signer.sign_transaction(&mut tx).await.unwrap();

        // send transaction and get receipt
        let receipt = provider
            .send_transaction(tx.into())
            .await
            .unwrap()
            .get_receipt()
            .await
            .unwrap();
        // do something with the receipt
        let block_number = receipt.block_number.unwrap();
        println!("Transaction mined in block: {:?}", block_number);
        assert!(block_number > 0);
    }
}
