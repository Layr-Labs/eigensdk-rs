use alloy::primitives::Address;
use clap::{ArgGroup, Parser, Subcommand};
use rust_bls_bn254::{
    CHINESE_SIMPLIFIED_WORD_LIST, CHINESE_TRADITIONAL_WORD_LIST, CZECH_WORD_LIST,
    ENGLISH_WORD_LIST, ITALIAN_WORD_LIST, KOREAN_WORD_LIST, PORTUGUESE_WORD_LIST,
    SPANISH_WORD_LIST,
};
#[derive(Parser, Debug)]
#[command(
    about = "Eigenlayer CLI tools",
    long_about = "Tools used for AVS development purpose"
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Given an initial contract address, which can be either a registry coordinator or service manager address,
outputs addresses for all relevant Eigenlayer and AVS contracts within the network",
        name = "egnaddrs",
        group(
            ArgGroup::new("manager_or_coordinator")
                .required(true)
                .args(&["service_manager", "registry_coordinator"]),
        )
    )]
    EigenAddress {
        #[arg(
            long,
            help = "ServiceManager contract address",
            group = "manager_or_coordinator"
        )]
        service_manager: Option<Address>,

        #[arg(
            long,
            help = "BLSRegistryCoordinatorWithIndices contract address",
            group = "manager_or_coordinator"
        )]
        registry_coordinator: Option<Address>,

        #[arg(long, help = "rpc url", default_value = "http://localhost:8545")]
        rpc_url: String,
    },

    #[command(about = "EigenLayer CLI Key tools", name = "egnkey")]
    EigenKey {
        #[command(subcommand)]
        subcommand: EigenKeyCommand,
    },
}

#[derive(Subcommand, Debug)]
pub enum EigenKeyCommand {
    #[command(
        about = "Generate keys for testing purpose.
This command creates ecdsa or bls key pair for testing purposes. It generates
all the relevant files for reading the key and decrypts it and also gets
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
    ConvertECDSA {
        #[arg(long, help = "private key to store (in hex)")]
        private_key: String,

        #[arg(long, help = "file to store key")]
        output_file: Option<String>,

        #[arg(long, help = "password to encrypt key")]
        password: Option<String>,
    },

    #[command(
        about = "Using Pbkfd2 / scrypt encryption to secure the bls key.",
        alias = "b"
    )]
    BlsConvert {
        #[arg(long, help = "Bls keystore type (pbkdf2 or scrypt)")]
        #[clap(value_enum)]
        key_type: BlsKeystoreType,

        #[arg(long, help = "bls key to encrypt in hex")]
        secret_key: String,

        #[arg(long, help = "file path to store key")]
        output_path: String,

        #[arg(long, help = "password to encrypt key(default is empty string)")]
        password: Option<String>,
    },
    #[command(about = "Create a new mnemonic from default word lists", alias = "md")]
    CreateNewMnemonicFromDefaultWordList {
        #[arg(long, help = "Mnemonic language select")]
        #[clap(value_enum)]
        language: MnemonicLanguage,
    },
    #[command(
        about = "Create a new mnemonic from given word list at path",
        alias = "mp"
    )]
    CreateNewMnemonicFromPath {
        #[arg(long, help = "Mnemonic language select")]
        #[clap(value_enum)]
        language: MnemonicLanguage,
        #[arg(long, help = "Path to a the directory where lists are stored)")]
        path: String,
    },
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum KeyType {
    Ecdsa,
    Bls,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum BlsKeystoreType {
    Pbkdf2,
    Scrypt,
}

#[derive(clap::ValueEnum, Debug, Clone)]
pub enum MnemonicLanguage {
    English,
    ChineseSimplified,
    ChineseTraditional,
    Italian,
    Czech,
    Korean,
    Portuguese,
    Spanish,
}

impl MnemonicLanguage {
    pub fn try_from(&self) -> (&str, &str) {
        match self {
            MnemonicLanguage::English => ("english", ENGLISH_WORD_LIST),
            MnemonicLanguage::ChineseSimplified => {
                ("chinese_simplified", CHINESE_SIMPLIFIED_WORD_LIST)
            }
            MnemonicLanguage::ChineseTraditional => {
                ("chinese_traditional", CHINESE_TRADITIONAL_WORD_LIST)
            }
            MnemonicLanguage::Italian => ("italian", ITALIAN_WORD_LIST),
            MnemonicLanguage::Czech => ("czech", CZECH_WORD_LIST),
            MnemonicLanguage::Korean => ("korean", KOREAN_WORD_LIST),
            MnemonicLanguage::Portuguese => ("portuguese", PORTUGUESE_WORD_LIST),
            MnemonicLanguage::Spanish => ("spanish", SPANISH_WORD_LIST),
        }
    }
}
