use args::{Args, Commands};
use generate::generate;
pub mod args;
mod generate;

pub fn execute_egnkey(args: Args) {
    match args.command {
        Commands::Generate {
            key_type,
            num_keys,
            output_dir,
        } => generate(key_type, num_keys, output_dir),
        Commands::Convert {
            private_key,
            output_file,
            password,
        } => todo!(),
        Commands::DeriveOperatorId { private_key } => todo!(),
    };
}

#[cfg(test)]
pub mod test {
    use crate::{
        args::{Args, Commands, KeyType},
        execute_egnkey,
        generate::{DEFAULT_KEY_FOLDER, PASSWORD_FILE, PRIVATE_KEY_HEX_FILE},
    };
    use eth_keystore::decrypt_key;
    use k256::SecretKey;
    use rstest::rstest;
    use std::fs;
    use tempfile::tempdir;

    #[rstest]
    #[case(KeyType::Ecdsa)]
    #[case(KeyType::Bls)]
    fn generate_key(#[case] key_type: KeyType) {
        let output_dir = tempdir().unwrap();
        let output_path = output_dir.path();
        let command = Commands::Generate {
            key_type: key_type.clone(),
            num_keys: 1,
            output_dir: output_path.to_str().map(String::from),
        };
        let args = Args { command };

        execute_egnkey(args);

        let private_key_hex = fs::read_to_string(output_path.join(PRIVATE_KEY_HEX_FILE)).unwrap();
        let password = fs::read_to_string(output_path.join(PASSWORD_FILE)).unwrap();
        let key_name = match key_type {
            KeyType::Ecdsa => "ecdsa",
            KeyType::Bls => "bls",
        };
        let key_path = output_path
            .join(DEFAULT_KEY_FOLDER)
            .join(format!("1.{}.key.json", key_name));

        let decrypted_bytes = decrypt_key(key_path, password).unwrap();
        let decrypted_private_key = SecretKey::from_slice(&decrypted_bytes).unwrap().to_bytes();

        let private_key = hex::decode(private_key_hex).unwrap();

        assert_eq!(private_key, decrypted_private_key.as_slice());
    }
}
