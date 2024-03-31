use eth_keystore::decrypt_key;
use ethers::{core::types::Signature, signers::Wallet};
use ethers_core::types::{transaction::eip2718::TypedTransaction, Address};
use ethers_signers::Signer;
use std::fs;
use std::str::FromStr;

pub trait SignerV2 {
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, String>;
}

#[derive(Clone, Debug)]
pub struct KeyStoreSigner {
    path: String,
    password: String,
}

#[derive(Debug, Clone)]
pub struct PrivateKeySigner {
    private_key: Vec<u8>,
}

impl PrivateKeySigner {
    async fn set_private_key(&mut self, pvt_key: Vec<u8>) {
        self.private_key = pvt_key;
    }
}

impl SignerV2 for PrivateKeySigner {
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, String> {
        let wallet = Wallet::from_bytes(&self.private_key).unwrap();

        let signer_tx = wallet.sign_transaction(tx).await.unwrap();

        Ok(signer_tx)
    }
}

impl SignerV2 for KeyStoreSigner {
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, String> {
        let private_key = decrypt_key(&self.path, &self.password).unwrap();

        let wallet = Wallet::from_bytes(&private_key).unwrap();

        let signer_tx = wallet.sign_transaction(tx).await.unwrap();

        Ok(signer_tx)
    }
}

pub struct Config {
    private_key: Vec<u8>,
    keystore_path: String,
    password: String,
    client: String,
    address: String,
}

impl Config {
    fn is_private_key_signer(&self) -> bool {
        !self.private_key.is_empty()
    }

    fn is_local_key_store_signer(&self) -> bool {
        !self.keystore_path.is_empty()
    }

    fn is_remote_signer(&self) -> bool {
        if self.client.is_empty() && self.address.is_empty() {
            return true;
        }
        false
    }
}

pub struct SignerFromCOonfig {
    c: Config,
}

impl SignerV2 for SignerFromCOonfig {
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, String> {
        if self.c.is_local_key_store_signer() {
            let private_key = decrypt_key(&self.c.keystore_path, &self.c.password).unwrap();

            let wallet = Wallet::from_bytes(&private_key).unwrap();

            let signer_tx = wallet.sign_transaction(tx).await.unwrap();

            Ok(signer_tx)
        } else if self.c.is_private_key_signer() {
            let wallet = Wallet::from_bytes(&self.c.private_key).unwrap();

            let signer_tx = wallet.sign_transaction(tx).await.unwrap();

            Ok(signer_tx)
        } else {
            return Err("fljsdlf".to_string());
        }
    }
}
