use rand::{distributions::Alphanumeric, Rng};
use std::{
    fs::{self, File},
    path::Path,
};
use uuid::Uuid;

use crate::args::KeyType;

const PASSWORD_LENGTH: usize = 20;
const DEFAULT_KEY_FOLDER: &str = "keys";
const PASSWORD_FILE: &str = "password.txt";
const PRIVATE_KEY_HEX_FILE: &str = "private_key_hex.txt";

enum KeyGenerator {
    ECDSAKeyGenerator,
    BLSKeyGenerator,
}

impl KeyGenerator {
    pub fn generate_keys(self, num_keys: u32, dir_path: String, password: String) {
        match self {
            KeyGenerator::ECDSAKeyGenerator => {
                Self::generate_ecdsa_key(num_keys, dir_path, password)
            }
            KeyGenerator::BLSKeyGenerator => Self::generate_bls_key(num_keys, dir_path, password),
        }
    }

    fn generate_ecdsa_key(num_keys: u32, path: String, password_file: String) {
        todo!()
        // for i := 0; i < numKeys; i++ {
        //     key, err := crypto.GenerateKey()
        //     privateKeyHex := hex.EncodeToString(key.D.Bytes())

        //     // Check if the length of privateKeyHex is 32 bytes (64 characters)
        //     lenPrivateKey := len(privateKeyHex)
        //     if lenPrivateKey != 64 {
        //         fmt.Printf("Private key Ignore: %s %d\n", privateKeyHex, lenPrivateKey)
        //         // Reset count
        //         i--
        //         continue
        //     }

        //     if err != nil {
        //         return err
        //     }

        //     password := generateRandomPassword()
        //     if err != nil {
        //         return err
        //     }

        //     fileName := fmt.Sprintf("%d.ecdsa.key.json", i+1)
        //     err = ecdsa.WriteKey(filepath.Clean(path+"/"+DefaultKeyFolder+"/"+fileName), key, password)
        //     if err != nil {
        //         return err
        //     }

        //     _, err = passwordFile.WriteString(password + "\n")
        //     if err != nil {
        //         return err
        //     }

        //     _, err = privateKeyFile.WriteString("0x" + privateKeyHex + "\n")
        //     if err != nil {
        //         return err
        //     }

        //     if (i+1)%50 == 0 {
        //         fmt.Printf("Generated %d keys\n", i+1)
        //     }
        // }
        // return nil
    }

    fn generate_bls_key(num_keys: u32, path: String, password_file: String) {
        todo!()
    }
}

impl From<KeyType> for KeyGenerator {
    fn from(value: KeyType) -> Self {
        match value {
            KeyType::Ecsda => KeyGenerator::ECDSAKeyGenerator,
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
                KeyType::Ecsda => "ecsda",
                KeyType::Bls => "bls",
            };
            format!("{}-{}", key_name, id.to_string())
        }
        Some(dir) => dir,
    };
    fs::create_dir_all(&dir_name).unwrap();

    // generate keys
    let password = generate_random_password();
    KeyGenerator::from(key_type).generate_keys(num_keys, dir_name, password)
}
