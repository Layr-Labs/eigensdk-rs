use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    about = "Eigenlayer batch keys manager",
    long_about = "Used to manage batch keys for testing"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Generate keys for testing purpose.
This command creates ecdsa or bls key pair for testing purposes. It generates
all the relevant files for reading and keys and decrypted it and also gets 
you the private keys in plaintext.

It creates the following artifacts based on arguments
- passwords.txt - contains all passwords to decrypt keys
- private_key_hex.txt - will create plaintext private keys
- keys/* - create all the encrypted json files in this folder",
        alias = "g"
    )]
    Generate {
        #[arg(long, help = "key type to create (ecdsa or bls)")]
        #[clap(value_enum)]
        key_type: KeyType,

        #[arg(long, help = "number of keys to create", default_value = "1")]
        num_keys: u32,

        #[arg(long, help = "folder to store keys")]
        output_dir: Option<String>,
    },

    #[command(
        about = "Given a private key, output its associated operatorId (hash of bn254 G1 pubkey).",
        alias = "d"
    )]
    DeriveOperatorId {
        #[arg(
            long,
            help = "(bn254) private key from which to derive operatorId from"
        )]
        private_key: String,
    },

    #[command(
        about = "Stores an ecdsa key to a file, in web3 secret storage format.",
        alias = "c"
    )]
    Convert {
        #[arg(long, help = "private key to store (in hex)")]
        private_key: String,

        #[arg(long, help = "file to store key")]
        output_file: Option<String>,

        #[arg(long, help = "password to encrypt key")]
        password: Option<String>,
    },
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum KeyType {
    Ecdsa,
    Bls,
}
