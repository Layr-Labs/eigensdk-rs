//use alloy::{
//    network::TransactionBuilder,
//    node_bindings::Anvil,
//    primitives::U256,
//    providers::{Provider, ProviderBuilder},
//    rpc::types::TransactionRequest,
//};

use alloy_consensus::{SignableTransaction, TxEip1559};
use alloy_contract::SolCallBuilder;
use alloy_network::{Ethereum, EthereumWallet};
use alloy_primitives::{address, fixed_bytes, Address, Bytes, FixedBytes, B256, I256, U256, U64};
use alloy_provider::{Provider, ProviderBuilder};
use alloy_rpc_client::PollerBuilder;
use alloy_signer::Signer;
// use futures_util::StreamExt;
//use alloy_transactions::TransactionRequest;
//use alloy_sol_types::sol;
//use alloy_provider::Provider;
//use alloy_rpc_types::Filter;

// TODO!!!

//use alloy_signer_local::PrivateKeySigner;
//use eth_keystore::decrypt_key;
//use std::path::Path;
//

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

/*
async fn test() -> Result<(), Box<dyn std::error::Error>> {
use alloy_contract::SolCallBuilder;
use alloy_network::Ethereum;
use alloy_primitives::{Address, U256};
use alloy_provider::ProviderBuilder;
use alloy_sol_types::sol;

sol! {
    #[sol(rpc)] // <-- Important! Generates the necessary `MyContract` struct and function methods.
    #[sol(bytecode = "0x1234")] // <-- Generates the `BYTECODE` static and the `deploy` method.
    contract MyContract {
        constructor(address) {} // The `deploy` method will also include any constructor arguments.

        #[derive(Debug)]
        function doStuff(uint a, bool b) public payable returns(address c, bytes32 d);
    }
}

// Build a provider.
let provider = ProviderBuilder::new().with_recommended_fillers().on_builtin("http://localhost:8545").await?;

// If `#[sol(bytecode = "0x...")]` is provided, the contract can be deployed with `MyContract::deploy`,
// and a new instance will be created.
let constructor_arg = Address::ZERO;
let contract = MyContract::deploy(&provider, constructor_arg).await?;

// Otherwise, or if already deployed, a new contract instance can be created with `MyContract::new`.
let address = Address::ZERO;
let contract = MyContract::new(address, &provider);

// Build a call to the `doStuff` function and configure it.
let a = U256::from(123);
let b = true;
let call_builder = contract.doStuff(a, b).value(U256::from(50e18 as u64));

// Send the call. Note that this is not broadcasted as a transaction.
let call_return = call_builder.call().await?;
println!("{call_return:?}"); // doStuffReturn { c: 0x..., d: 0x... }

// Use `send` to broadcast the call as a transaction.
let _pending_tx = call_builder.send().await?;
Ok(())
}
*/

// TODO!
// continue with this:
// https://github.com/alloy-rs/examples/blob/main/examples/transactions/examples/encode_decode_eip1559.rs
#[cfg(test)]
mod tests {
    use alloy_consensus::{SignableTransaction, TxEip1559};
    use alloy_eips::eip2930::AccessList;
    use alloy_primitives::{address, hex, TxKind, U256};

    #[test]
    fn test_tx() {
        let tx = TxEip1559 {
            chain_id: 1,
            nonce: 0x42,
            gas_limit: 44386,
            to: TxKind::Call( address!("6069a6c32cf691f5982febae4faf8a6f3ab2f0f6")),
            value: U256::from(0_u64),
            input: hex!("a22cb4650000000000000000000000005eee75727d804a2b13038928d36f8b188945a57a0000000000000000000000000000000000000000000000000000000000000000").into(),
            max_fee_per_gas: 0x4a817c800,
            max_priority_fee_per_gas: 0x3b9aca00,
            access_list: AccessList::default(),
        };
    }
}
