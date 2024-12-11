use alloy_provider::{
    fillers::{
        BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
    },
    network::{Ethereum, EthereumWallet},
    Identity, ProviderBuilder, RootProvider, WsConnect,
};
use alloy_pubsub::PubSubFrontend;
use alloy_signer_local::PrivateKeySigner;
use alloy_transport::{RpcError, TransportErrorKind};
use alloy_transport_http::{Client, Http};
use std::str::FromStr;
use url::Url;

#[allow(clippy::type_complexity)]
pub fn get_signer(
    key: &str,
    rpc_url: &str,
) -> alloy_provider::fillers::FillProvider<
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
> {
    let signer = PrivateKeySigner::from_str(key).expect("wrong key ");
    let wallet = EthereumWallet::from(signer);
    let url = Url::parse(rpc_url).expect("Wrong rpc url");
    ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(wallet.clone())
        .on_http(url)
}

#[allow(clippy::type_complexity)]
pub fn get_provider(
    rpc_url: &str,
) -> FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
> {
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
