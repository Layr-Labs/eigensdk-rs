use alloy::providers::{
    fillers::{
        BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
    },
    network::{Ethereum, EthereumWallet},
    Identity, ProviderBuilder, RootProvider, WsConnect,
};
use alloy::pubsub::PubSubFrontend;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::{Client, Http};
use alloy::transports::{RpcError, TransportErrorKind};
use std::str::FromStr;
use url::Url;

pub type SdkProvider = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

pub type SdkSigner = FillProvider<
    JoinFill<
        JoinFill<
            Identity,
            JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
        >,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;

pub fn get_signer(key: &str, rpc_url: &str) -> SdkSigner {
    let signer = PrivateKeySigner::from_str(key).expect("wrong key ");
    let wallet = EthereumWallet::from(signer);
    let url = Url::parse(rpc_url).expect("Wrong rpc url");
    ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(url)
}

pub fn get_provider(rpc_url: &str) -> SdkProvider {
    let url = Url::parse(rpc_url).expect("Wrong rpc url");
    ProviderBuilder::new()
        .with_recommended_fillers()
        .on_http(url)
}

#[allow(clippy::type_complexity)]
pub async fn get_ws_provider(
    rpc_url: &str,
) -> Result<RootProvider<PubSubFrontend>, RpcError<TransportErrorKind>> {
    let ws = WsConnect::new(rpc_url);
    ProviderBuilder::new().on_ws(ws).await
}

/// Emitted when a new pubkey is registered
pub const NEW_PUBKEY_REGISTRATION_EVENT: &str =
    "NewPubkeyRegistration(address,(uint256,uint256),(uint256[2],uint256[2]))";

pub const OPERATOR_SOCKET_UPDATE: &str = "OperatorSocketUpdate(bytes32,string)";
