use alloy_network::{Ethereum, EthereumWallet};
use alloy_primitives::{address, fixed_bytes, Address, Bytes, FixedBytes, B256, I256, U256};
//use alloy_provider::Provider;
//use alloy_rpc_types::Filter;

// TODO!!!

//use alloy_signer_local::PrivateKeySigner;
//use eth_keystore::decrypt_key;
//use std::path::Path;
//
//
pub struct SimpleTxManager {
    wallet: EthereumWallet,
    //    client: eth::Client,
    //    log: logging::Logger,
    sender: Address,
    gas_limit_multiplier: f64,
}

impl SimpleTxManager {
    /*
    pub fn new(
        wallet: wallet::Wallet,
        client: eth::Client,
        log: logging::Logger,
        sender: common::Address,
        gas_limit_multiplier: f64,
    ) -> SimpleTxManager {
        SimpleTxManager {
            wallet,
            client,
            log,
            sender,
            gas_limit_multiplier,
        }
    }
    */

    pub fn with_gas_limit_multiplier(&mut self, multiplier: f64) {
        self.gas_limit_multiplier = multiplier;
    }
}
