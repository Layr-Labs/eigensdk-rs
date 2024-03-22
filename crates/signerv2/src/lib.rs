use ethers::{core::types::Signature, signers::Wallet};
use ethers_core::types::{
    transaction::eip2718::TypedTransaction, Address, Transaction, TransactionRequest,
};
use ethers_signers::Signer;
use std::str::FromStr;

pub trait SignerV2 {
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

impl PrivateKeySigner {
    async fn set_private_key(&mut self, pvt_key: String) {
        self.private_key = pvt_key;
    }
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

// type SignerFn = dyn Fn(Address) -> Box<dyn SignerV2>;

// fn get_signer_function
