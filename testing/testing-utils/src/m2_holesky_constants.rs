use alloy::primitives::{address, Address};

/// Holesky rpc provider
pub const HOLESKY_RPC_PROVIDER: &str = "https://ethereum-holesky-rpc.publicnode.com";

/// https://holesky.etherscan.io/address/0xA44151489861Fe9e3055d95adC98FbD462B948e7
pub const DELEGATION_MANAGER_ADDRESS: Address =
    address!("A44151489861Fe9e3055d95adC98FbD462B948e7");
/// https://holesky.etherscan.io/address/0xdfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6
pub const STRATEGY_MANAGER_ADDRESS: Address = address!("dfB5f6CE42aAA7830E94ECFCcAd411beF4d4D5b6");
/// https://holesky.etherscan.io/address/0x30770d7E3e71112d7A6b7259542D1f680a70e315
pub const EIGENPOD_MANAGER_ADDRESS: Address = address!("30770d7E3e71112d7A6b7259542D1f680a70e315");
/// https://holesky.etherscan.io/address/0x055733000064333CaDDbC92763c58BF0192fFeBf
pub const AVS_DIRECTORY_ADDRESS: Address = address!("055733000064333CaDDbC92763c58BF0192fFeBf");
/// https://holesky.etherscan.io/address/0xcAe751b75833ef09627549868A04E32679386e7C
pub const SLASHER_ADDRESS: Address = address!("cAe751b75833ef09627549868A04E32679386e7C	");
/// https://holesky.etherscan.io/address/0xAcc1fb458a1317E886dB376Fc8141540537E68fE
pub const REWARDS_COORDINATOR: Address = address!("Acc1fb458a1317E886dB376Fc8141540537E68fE");
/// https://holesky.etherscan.io/address/0x7D704507b76571a51d9caE8AdDAbBFd0ba0e63d3
#[allow(non_upper_case_globals)]
pub const StrategyBase_stETH: Address = address!("7D704507b76571a51d9caE8AdDAbBFd0ba0e63d3");
/// https://holesky.etherscan.io/address/0x3A8fBdf9e77DFc25d09741f51d3E181b25d0c4E0
#[allow(non_upper_case_globals)]
pub const StrategyBase_rETH: Address = address!("3A8fBdf9e77DFc25d09741f51d3E181b25d0c4E0");
/// https://holesky.etherscan.io/address/0x80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9
#[allow(non_upper_case_globals)]
pub const StrategyBase_WETH: Address = address!("80528D6e9A2BAbFc766965E0E26d5aB08D9CFaF9");
/// https://holesky.etherscan.io/address/0x31B6F59e1627cEfC9fA174aD03859fC337666af7
#[allow(non_upper_case_globals)]
pub const StrategyBase_ETHx: Address = address!("31B6F59e1627cEfC9fA174aD03859fC337666af7");
/// https://holesky.etherscan.io/address/0x7673a47463F80c6a3553Db9E54c8cDcd5313d0ac
#[allow(non_upper_case_globals)]
pub const StrategyBase_ankrETH: Address = address!("7673a47463F80c6a3553Db9E54c8cDcd5313d0ac");
/// https://holesky.etherscan.io/address/0x70EB4D3c164a6B4A5f908D4FBb5a9cAfFb66bAB6
#[allow(non_upper_case_globals)]
pub const StrategyBase_cbETH: Address = address!("70EB4D3c164a6B4A5f908D4FBb5a9cAfFb66bAB6");
/// https://holesky.etherscan.io/address/0x46281E3B7fDcACdBa44CADf069a94a588Fd4C6Ef
#[allow(non_upper_case_globals)]
pub const StrategyBase_osETH: Address = address!("46281E3B7fDcACdBa44CADf069a94a588Fd4C6Ef");
/// https://holesky.etherscan.io/address/0x7CA911E83dabf90C90dD3De5411a10F1A6112184
#[allow(non_upper_case_globals)]
pub const StrategyBase_wBETH: Address = address!("7CA911E83dabf90C90dD3De5411a10F1A6112184");
/// https://holesky.etherscan.io/address/0x9281ff96637710Cd9A5CAcce9c6FAD8C9F54631c
#[allow(non_upper_case_globals)]
pub const StrategyBase_sfrxETH: Address = address!("9281ff96637710Cd9A5CAcce9c6FAD8C9F54631c");
/// https://holesky.etherscan.io/address/0x05037A81BD7B4C9E0F7B430f1F2A22c31a2FD943
#[allow(non_upper_case_globals)]
pub const StrategyBase_lsETH: Address = address!("05037A81BD7B4C9E0F7B430f1F2A22c31a2FD943");
/// https://holesky.etherscan.io/address/0xaccc5A86732BE85b5012e8614AF237801636F8e5
#[allow(non_upper_case_globals)]
pub const StrategyBase_mETH: Address = address!("accc5A86732BE85b5012e8614AF237801636F8e5");
/// https://holesky.etherscan.io/address/0xbeaC0eeEeeeeEEeEeEEEEeeEEeEeeeEeeEEBEaC0
pub const BEACON_CHAIN_ETH: Address = address!("beaC0eeEeeeeEEeEeEEEEeeEEeEeeeEeeEEBEaC0");
/// https://holesky.etherscan.io/address/0x7261C2bd75a7ACE1762f6d7FAe8F63215581832D
pub const EIGEN_POD_BEACON: Address = address!("7261C2bd75a7ACE1762f6d7FAe8F63215581832D");
/// https://holesky.etherscan.io/address/0x642c646053eaf2254f088e9019ACD73d9AE0FA32
pub const DELAYED_WITHDRAWAL_ROUTER: Address = address!("642c646053eaf2254f088e9019ACD73d9AE0FA32");
/// https://holesky.etherscan.io/address/0x4C116BB629bff7A8373c2378bBd919f8349B8f25
pub const EIGEN_LAYER_BEACON_ORACLE: Address = address!("4C116BB629bff7A8373c2378bBd919f8349B8f25");
/// https://holesky.etherscan.io/address/0x3B78576F7D6837500bA3De27A60c7f594934027E
pub const EIGEN_TOKEN: Address = address!("3B78576F7D6837500bA3De27A60c7f594934027E");
/// https://holesky.etherscan.io/address/0x275cCf9Be51f4a6C94aBa6114cdf2a4c45B9cb27
pub const BEIGEN: Address = address!("275cCf9Be51f4a6C94aBa6114cdf2a4c45B9cb27");
/// https://holesky.etherscan.io/address/0x43252609bff8a13dFe5e057097f2f45A24387a84
pub const EIGEN_STRATEGY: Address = address!("43252609bff8a13dFe5e057097f2f45A24387a84");

// Middlware contracts

/// https://holesky.etherscan.io/address/0x53012C69A189cfA2D9d29eb6F19B32e0A2EA3490
pub const REGISTRY_COORDINATOR: Address = address!("53012C69A189cfA2D9d29eb6F19B32e0A2EA3490");
/// https://holesky.etherscan.io/address/0x066cF95c1bf0927124DFB8B02B401bc23A79730D
pub const BLS_APK_REGISTRY: Address = address!("066cF95c1bf0927124DFB8B02B401bc23A79730D");
/// https://holesky.etherscan.io/address/0xB4baAfee917fb4449f5ec64804217bccE9f46C67
pub const OPERATOR_STATE_RETRIEVER: Address = address!("B4baAfee917fb4449f5ec64804217bccE9f46C67");
/// https://holesky.etherscan.io/address/0xBDACD5998989Eec814ac7A0f0f6596088AA2a270
pub const STAKE_REGISTRY: Address = address!("BDACD5998989Eec814ac7A0f0f6596088AA2a270");
/// https://holesky.etherscan.io/address/0x870679E138bCdf293b7Ff14dD44b70FC97e12fc0
pub const SERVICE_MANAGER_ADDRESS: Address = address!("870679E138bCdf293b7Ff14dD44b70FC97e12fc0");
