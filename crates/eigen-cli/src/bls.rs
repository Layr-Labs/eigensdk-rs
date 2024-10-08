use crate::args::BlsKeystoreType;
use crate::EigenBlsKeyStoreError;
use rust_bls_bn254::keystores::{pbkdf2_keystore::Pbkdf2Keystore, scrypt_keystore::ScryptKeystore};

/// BlsKeystore   
pub enum BlsKeystore {
    Pbkdf2,
    Scrypt,
}

impl BlsKeystore {
    /// Create a new [`BlsKeystore`] instance.
    /// [`BlsKeystore::Pbkdft`] or [`BlsKeystore::Scrypt`]
    pub fn new_keystore(
        self,
        secret_key: String,
        output_path: String,
        password: Option<&str>,
    ) -> Result<(), EigenBlsKeyStoreError> {
        match self {
            BlsKeystore::Pbkdf2 => {
                let secret_in_bytes = secret_key.into_bytes();
                let bls_key = secret_in_bytes.as_slice();
                let mut keystore = Pbkdf2Keystore::new();
                keystore.encrypt(
                    bls_key,
                    password.unwrap_or_default(),
                    &output_path.to_string(),
                    None,
                    None,
                )?;
                keystore.to_keystore().save(&output_path.to_string())?;
                Ok(())
            }
            BlsKeystore::Scrypt => {
                let secret_in_bytes = secret_key.into_bytes();
                let bls_key = secret_in_bytes.as_slice();
                let mut keystore = ScryptKeystore::new();
                keystore.encrypt(
                    bls_key,
                    password.unwrap_or_default(),
                    &output_path.to_string(),
                    None,
                    None,
                )?;
                keystore.to_keystore().save(&output_path.to_string())?;
                Ok(())
            }
        }
    }
}

impl From<BlsKeystoreType> for BlsKeystore {
    fn from(value: BlsKeystoreType) -> Self {
        match value {
            BlsKeystoreType::Pbkdf2 => BlsKeystore::Pbkdf2,
            BlsKeystoreType::Scrypt => BlsKeystore::Scrypt,
        }
    }
}
