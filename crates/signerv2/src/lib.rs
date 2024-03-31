use crate::error::SignerV2Error;
use eth_keystore::decrypt_key;
use ethers::{core::types::Signature, signers::Wallet};
use ethers_core::types::transaction::eip2718::TypedTransaction;
use ethers_signers::Signer;

pub mod error;

pub trait SignerV2 {
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, SignerV2Error>;
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
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, SignerV2Error> {
        let wallet_result = Wallet::from_bytes(&self.private_key);

        match wallet_result {
            Ok(wallet) => {
                let signer_tx_result = wallet.sign_transaction(tx).await;

                match signer_tx_result {
                    Ok(signer_tx) => {
                        return Ok(signer_tx);
                    }
                    Err(_) => return Err(SignerV2Error::SignTransaction),
                }
            }
            Err(_) => return Err(SignerV2Error::BuildWallet),
        }
    }
}

impl SignerV2 for KeyStoreSigner {
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, SignerV2Error> {
        let private_key = decrypt_key(&self.path, &self.password).unwrap();

        let wallet_result = Wallet::from_bytes(&private_key);

        match wallet_result {
            Ok(wallet) => {
                let signer_tx_result = wallet.sign_transaction(tx).await;

                match signer_tx_result {
                    Ok(signer_tx) => {
                        return Ok(signer_tx);
                    }
                    Err(_) => return Err(SignerV2Error::SignTransaction),
                }
            }
            Err(_) => return Err(SignerV2Error::BuildWallet),
        }
    }
}

pub struct Config {
    private_key: Vec<u8>,
    keystore_path: String,
    password: String,
}

impl Config {
    fn is_private_key_signer(&self) -> bool {
        !self.private_key.is_empty()
    }

    fn is_local_key_store_signer(&self) -> bool {
        !self.keystore_path.is_empty()
    }
}

pub struct SignerFromCOonfig {
    c: Config,
}

impl SignerV2 for SignerFromCOonfig {
    async fn sign_transaction(&self, tx: &TypedTransaction) -> Result<Signature, SignerV2Error> {
        if self.c.is_local_key_store_signer() {
            let private_key_result = decrypt_key(&self.c.keystore_path, &self.c.password);

            match private_key_result {
                Ok(private_key) => {
                    let wallet_result = Wallet::from_bytes(&private_key);

                    match wallet_result {
                        Ok(wallet) => {
                            let signer_tx_result = wallet.sign_transaction(tx).await;

                            match signer_tx_result {
                                Ok(signer_tx) => {
                                    return Ok(signer_tx);
                                }
                                Err(_) => return Err(SignerV2Error::SignTransaction),
                            }
                        }
                        Err(_) => return Err(SignerV2Error::BuildWallet),
                    }
                }
                Err(_) => return Err(SignerV2Error::Decryptkey),
            }
        } else if self.c.is_private_key_signer() {
            let private_key_result = decrypt_key(&self.c.keystore_path, &self.c.password);

            match private_key_result {
                Ok(private_key) => {
                    let wallet_result = Wallet::from_bytes(&private_key);

                    match wallet_result {
                        Ok(wallet) => {
                            let signer_tx_result = wallet.sign_transaction(tx).await;

                            match signer_tx_result {
                                Ok(signer_tx) => {
                                    return Ok(signer_tx);
                                }
                                Err(_) => return Err(SignerV2Error::SignTransaction),
                            }
                        }
                        Err(_) => return Err(SignerV2Error::BuildWallet),
                    }
                }
                Err(_) => return Err(SignerV2Error::BuildWallet),
            }
        } else {
            return Err(SignerV2Error::ConfigNotFound);
        }
    }
}
