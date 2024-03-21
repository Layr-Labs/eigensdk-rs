use eigensdk_client_wallet::Wallet;
use eigensdk_logging::logger::Logger;
use eigensdk_signerv2::SignerV2;
use ethers::{
    core::k256::ecdsa::signature::Signer,
    providers::{Http, Provider},
    types::Sign,
};

pub struct TxManager;

pub struct SimpleTxManager {
    wallet: Wallet,
    client: Provider<Http>,
    signer_fn: Box<SignerV2>,
    log: Logger,
    sender: Address,
}

impl SimpleTxManager {
    pub fn new(
        wallet: Wallet,
        client: Provider<Http>,
        log: Logger,
        signer_fn: Box<SignerV2>,
        sender: Address,
    ) -> Self {
        SimpleTxManager {
            wallet,
            client,
            log,
            signer_fn,
            sender,
        }
    }
}
