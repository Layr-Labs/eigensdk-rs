use crate::args::KeyType;
use ark_ff::UniformRand;
use ark_serialize::CanonicalSerialize;
use eigen_crypto_bls::PrivateKey;
use eth_keystore::encrypt_key;
use k256::{FieldBytes, SecretKey};
use rand::{distributions::Alphanumeric, Rng};
use rand_core::OsRng;
use std::io::Write;
use std::{
    fs::{self, File},
    path::Path,
};
use uuid::Uuid;

const PASSWORD_LENGTH: usize = 20;
pub const DEFAULT_KEY_FOLDER: &str = "keys";
pub const PASSWORD_FILE: &str = "password.txt";
pub const PRIVATE_KEY_HEX_FILE: &str = "private_key_hex.txt";

enum KeyGenerator {
    ECDSAKeyGenerator,
    BLSKeyGenerator,
}

impl KeyGenerator {
    pub fn generate_keys(self, num_keys: u32, path: &Path, password: String) {
        let key_path = path.join(DEFAULT_KEY_FOLDER);
        let private_key_path = path.join(PRIVATE_KEY_HEX_FILE);
        let password_path = path.join(PASSWORD_FILE);

        for i in 0..num_keys {
            let private_key = self.random_key();
            let private_key_hex = hex::encode(private_key.clone());

            // encrypt the private key into `path` directory
            let key_name = match self {
                KeyGenerator::ECDSAKeyGenerator => "ecdsa",
                KeyGenerator::BLSKeyGenerator => "bls",
            };
            let name = format!("{}.{}.key.json", i + 1, key_name);
            encrypt_key(
                key_path.clone(),
                &mut OsRng,
                private_key,
                password.clone(),
                Some(&name),
            )
            .unwrap();

            // write the private key into `private_key_file`
            let mut pk_file = File::create(private_key_path.clone()).unwrap();
            pk_file.write_all(private_key_hex.as_bytes()).unwrap();

            // write the password into `password_file`
            let mut password_file = File::create(password_path.clone()).unwrap();
            password_file.write_all(password.as_bytes()).unwrap();

            if (i + 1) % 50 == 0 {
                println!("Generated {} keys\n", i + 1);
            }
        }
    }

    fn random_key(&self) -> Vec<u8> {
        match self {
            KeyGenerator::ECDSAKeyGenerator => Self::random_ecdsa_key(),
            KeyGenerator::BLSKeyGenerator => Self::random_bls_key(),
        }
    }

    fn random_ecdsa_key() -> Vec<u8> {
        let private_key = SecretKey::random(&mut OsRng);
        private_key.to_bytes().as_slice().to_vec()
    }

    fn random_bls_key() -> Vec<u8> {
        let mut buffer = Vec::new();
        let private_key = PrivateKey::rand(&mut OsRng);
        private_key.serialize_uncompressed(&mut buffer).unwrap();
        buffer
    }
}

impl From<KeyType> for KeyGenerator {
    fn from(value: KeyType) -> Self {
        match value {
            KeyType::Ecdsa => KeyGenerator::ECDSAKeyGenerator,
            KeyType::Bls => KeyGenerator::BLSKeyGenerator,
        }
    }
}

fn generate_random_password() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(PASSWORD_LENGTH)
        .map(char::from)
        .collect()
}

pub fn generate(key_type: KeyType, num_keys: u32, output_dir: Option<String>) {
    // create dir
    let dir_name = match output_dir {
        None => {
            let id = Uuid::new_v4();
            let key_name = match key_type {
                KeyType::Ecdsa => "ecdsa",
                KeyType::Bls => "bls",
            };
            format!("{}-{}", key_name, id.to_string())
        }
        Some(dir) => dir,
    };
    let dir_path = Path::new(&dir_name);
    let key_path = dir_path.join(DEFAULT_KEY_FOLDER);
    fs::create_dir_all(&key_path).unwrap();

    // generate keys
    let password = generate_random_password();
    KeyGenerator::from(key_type).generate_keys(num_keys, dir_path, password)
}
