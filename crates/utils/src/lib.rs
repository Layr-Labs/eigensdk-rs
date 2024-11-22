#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]
#![cfg(not(doctest))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![allow(unused_imports, clippy::all, rustdoc::all)]
//! This module contains the sol! generated bindings for solidity contracts.
//! This is autogenerated code.
//! Do not manually edit these files.
//! These files may be overwritten by the codegen system at any time.
pub mod accesscontrol;
pub mod accesscontrolenumerable;
pub mod address;
pub mod addressupgradeable;
pub mod allocationmanager;
pub mod allocationmanagerstorage;
pub mod avsdirectory;
pub mod avsdirectorystorage;
pub mod backingeigen;
pub mod beaconchainproofs;
pub mod beaconproxy;
pub mod beigen_and_eigen_upgrade;
pub mod beigen_timelock_reduction;
pub mod beigen_upgrade;
pub mod blsapkregistry;
pub mod byteslib;
pub mod context;
pub mod contextupgradeable;
pub mod contractsregistry;
pub mod countersupgradeable;
pub mod create2;
pub mod delegationmanager;
pub mod delegationmanagerstorage;
pub mod deploy_preprod_rewardscoordinator;
pub mod deployfromscratch;
pub mod deploystrategies;
pub mod doubleendedqueue;
pub mod ecdsaupgradeable;
pub mod eigen;
pub mod eigen_strategy_deploy;
pub mod eigen_timelock_reduction;
pub mod eigen_token_deploy;
pub mod eigen_upgrade;
pub mod eigenpod;
pub mod eigenpod_minor_upgrade_deploy;
pub mod eigenpodmanager;
pub mod eigenpodmanagerstorage;
pub mod eigenpodpausingconstants;
pub mod eigenpodstorage;
pub mod eigenstrategy;
pub mod eip712upgradeable;
pub mod emptycontract;
pub mod endian;
pub mod enumerableset;
pub mod erc165;
pub mod erc1967proxy;
pub mod erc1967upgrade;
pub mod erc20;
pub mod erc20burnable;
pub mod erc20permitupgradeable;
pub mod erc20presetfixedsupply;
pub mod erc20presetminterpauser;
pub mod erc20upgradeable;
pub mod erc20votesupgradeable;
pub mod ethposdepositmock;
pub mod existingdeploymentparser;
pub mod iaccesscontrol;
pub mod iaccesscontrolenumerable;
pub mod iallocationmanager;
pub mod iallocationmanagererrors;
pub mod iallocationmanagerevents;
pub mod iallocationmanagertypes;
pub mod iavsdirectory;
pub mod iavsdirectoryerrors;
pub mod iavsdirectoryevents;
pub mod iavsdirectorytypes;
pub mod ibackingeigen;
pub mod ibeacon;
pub mod iblssignaturechecker;
pub mod idelegationfaucet;
pub mod idelegationmanager;
pub mod idelegationmanagererrors;
pub mod idelegationmanagerevents;
pub mod idelegationmanagertypes;
pub mod ieigen;
pub mod ieigenpod;
pub mod ieigenpoderrors;
pub mod ieigenpodevents;
pub mod ieigenpodmanager;
pub mod ieigenpodmanagererrors;
pub mod ieigenpodmanagerevents;
pub mod ieigenpodtypes;
pub mod ierc1155receiver;
pub mod ierc1271upgradeable;
pub mod ierc165;
pub mod ierc1822proxiable;
pub mod ierc1967;
pub mod ierc20;
pub mod ierc20metadata;
pub mod ierc20metadataupgradeable;
pub mod ierc20permit;
pub mod ierc20permitupgradeable;
pub mod ierc20upgradeable;
pub mod ierc5267upgradeable;
pub mod ierc5805upgradeable;
pub mod ierc6372upgradeable;
pub mod ierc721receiver;
pub mod iethposdeposit;
pub mod initializable;
pub mod ipausable;
pub mod ipauserregistry;
pub mod irewardscoordinator;
pub mod irewardscoordinatorerrors;
pub mod irewardscoordinatorevents;
pub mod irewardscoordinatortypes;
pub mod isafe;
pub mod isharemanager;
pub mod isignatureutils;
pub mod islasher;
pub mod isocketupdater;
pub mod istrategy;
pub mod istrategyerrors;
pub mod istrategyevents;
pub mod istrategyfactory;
pub mod istrategymanager;
pub mod istrategymanagererrors;
pub mod istrategymanagerevents;
pub mod itimelock;
pub mod itransparentupgradeableproxy;
pub mod ivotesupgradeable;
pub mod iwhitelister;
pub mod longtail_upgrade_preprod;
pub mod m2_deploy_holesky_from_scratch;
pub mod m2_deploy_holesky_preprod;
pub mod mainnet_unpause_deposits;
pub mod mainnetpepedeploy;
pub mod mainnetrewardscoordinatordeploy;
pub mod mainnetstrategyfactorydeploy;
pub mod math;
pub mod mathupgradeable;
pub mod merkle;
pub mod mockavsservicemanager;
pub mod multisendcallonly;
pub mod operatorstateretriever;
pub mod ownable;
pub mod ownableupgradeable;
pub mod pausable;
pub mod pauserregistry;
pub mod pepe_deploy_preprod;
pub mod preprod_upgrade_beigen_and_eigen;
pub mod proxy;
pub mod proxyadmin;
pub mod reentrancyguardupgradeable;
pub mod registrycoordinator;
pub mod rewardscoordinator;
pub mod rewardscoordinatorstorage;
pub mod safecast;
pub mod safecastupgradeable;
pub mod safeerc20;
pub mod servicemanagerbase;
pub mod signaturecheckerupgradeable;
pub mod signatureutils;
pub mod signedmath;
pub mod signedmathupgradeable;
pub mod slashinglib;
pub mod snapshots;
pub mod staker;
pub mod stakeregistry;
pub mod storageslot;
pub mod strategybase;
pub mod strategybasetvllimits;
pub mod strategydeployer;
pub mod strategyfactory;
pub mod strategyfactorystorage;
pub mod strategymanager;
pub mod strategymanagerstorage;
pub mod strings;
pub mod stringsupgradeable;
pub mod timelockcontroller;
pub mod timelockencoding;
pub mod transparentupgradeableproxy;
pub mod upgrade;
pub mod upgrade_mainnet_rewardscoordinator;
pub mod upgrade_preprod_rewardscoordinator;
pub mod upgradeablebeacon;

use alloy::network::{Ethereum, EthereumWallet};
use alloy::providers::{
    fillers::{
        BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller,
    },
    Identity, ProviderBuilder, RootProvider, WsConnect,
};
use alloy::pubsub::PubSubFrontend;
use alloy::signers::local::PrivateKeySigner;
use alloy::transports::http::{Client, Http};
use alloy::transports::RpcError;
use alloy::transports::TransportErrorKind;
use reqwest::Url;
use std::str::FromStr;

#[allow(clippy::type_complexity)]
pub fn get_signer(
    key: &str,
    rpc_url: &str,
) -> alloy::providers::fillers::FillProvider<
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
