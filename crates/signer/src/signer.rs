use alloy_signer_local::PrivateKeySigner;
use eth_keystore::decrypt_key;
use std::path::Path;
use thiserror::Error;

#[derive(Debug)]
/// Represents the input params to create a signer
pub enum Config {
    /// Hexadecimal private key
    PrivateKey(String),
    /// Keystore path and password
    Keystore(String, String),
    /// Web3 endpoint and address
    Web3(String, String),
}

#[derive(Error, Debug)]
#[allow(missing_docs)]
pub enum SignerError {
    #[error("invalid private key")]
    InvalidPrivateKey,
    #[error("invalid keystore password")]
    InvalidPassword,
}

impl Config {
    /// Creates a signer from given config.
    pub fn signer_from_config(c: Config) -> Result<PrivateKeySigner, SignerError> {
        // TODO: check chain id to select signer
        match c {
            Config::PrivateKey(key) => key
                .parse::<PrivateKeySigner>()
                .map_err(|_| SignerError::InvalidPrivateKey),
            Config::Keystore(path, password) => {
                let keypath = Path::new(&path);
                let private_key =
                    decrypt_key(&keypath, password).map_err(|_| SignerError::InvalidPassword)?;
                PrivateKeySigner::from_slice(&private_key)
                    .map_err(|_| SignerError::InvalidPrivateKey)
            }
            Config::Web3(_endpoint, _address) => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use alloy_consensus::TxLegacy;
    use alloy_network::TxSignerSync;
    use alloy_primitives::{address, bytes, U256};
    use alloy_signer::Signature;
    use alloy_signer_local::PrivateKeySigner;
    use hex_literal::hex;
    use std::str::FromStr;

    #[test]
    fn sign_transaction_with_private_key() {
        let private_key =
            "dcf2cbdd171a21c480aa7f53d77f31bb102282b3ff099c78e3118b37348c72f7".to_owned();
        let config = Config::PrivateKey(private_key);
        let mut tx = TxLegacy {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        let signer = Config::signer_from_config(config);

        let signature = signer.unwrap().sign_transaction_sync(&mut tx).unwrap();
        let expected_signature = Signature::from_rs_and_parity(
            U256::from_str(
                "99963972037857174861280476053118856715670512199525969754644366601434507134123",
            )
            .unwrap(),
            U256::from_str(
                "54587766196536123534774489028213321677166972433316011091332824361042811624091",
            )
            .unwrap(),
            37,
        )
        .unwrap();
        assert_eq!(signature, expected_signature);
    }

    #[test]
    fn sign_transaction_with_keystore() {
        let path = "mockdata/dummy.key.json".to_owned();
        let password = "testpassword".to_owned();
        let config = Config::Keystore(path, password);
        let mut tx = TxLegacy {
            to: address!("d8dA6BF26964aF9D7eEd9e03E53415D37aA96045").into(),
            value: U256::from(1_000_000_000),
            gas_limit: 2_000_000,
            nonce: 0,
            gas_price: 21_000_000_000,
            input: bytes!(),
            chain_id: Some(1),
        };

        let private_key = hex!("7a28b5ba57c53603b0b07b56bba752f7784bf506fa95edc395f5cf6c7514fe9d");
        let expected_signer = PrivateKeySigner::from_slice(&private_key).unwrap();
        let expected_signature = expected_signer.sign_transaction_sync(&mut tx).unwrap();

        let signer = Config::signer_from_config(config);
        let signature = signer.unwrap().sign_transaction_sync(&mut tx).unwrap();

        assert_eq!(signature, expected_signature);
    }
}
