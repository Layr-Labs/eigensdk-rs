use ethers::{core::types::Signature, signers::Wallet};
use ethers_core::types::{
    transaction::eip2718::TypedTransaction, Address, Transaction, TransactionRequest,
};
use ethers_signers::Signer;
use std::str::FromStr;

trait SignerV2 {
    async fn sign_transaction(
        &self,
        address: Address,
        tx: &TypedTransaction,
    ) -> Result<Signature, String>;
}

pub struct PrivateKeySigner {
    private_key: String,
    chain_id: u64,
}

impl SignerV2 for PrivateKeySigner {
    async fn sign_transaction(
        &self,
        address: Address,
        tx: &TypedTransaction,
    ) -> Result<Signature, String> {
        let wallet = Wallet::from_str(&self.private_key).unwrap();
        let public_address = wallet.address();

        let signer_tx = wallet.sign_transaction(tx).await.unwrap();

        Ok(signer_tx)
    }
}
