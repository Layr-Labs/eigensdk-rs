use crate::args::Args;
use alloy_contract::Error as ContractError;
use alloy_primitives::Address;
use alloy_provider::Provider;
use eigen_utils::{
    binding::{DelegationManager, IBLSSignatureChecker, RegistryCoordinator},
    get_provider,
};
use serde::{Deserialize, Serialize};
pub mod args;

pub const ANVIL_RPC_URL: &str = "http://localhost:8545";

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct EigenAddressesResponse {
    avs: AvsAddresses,
    eigenlayer: EigenAddresses,
    network: NetworkInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all(serialize = "kebab-case"))]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct NetworkInfo {
    chain_id: String,
    rpc_url: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all(serialize = "kebab-case"))]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct EigenAddresses {
    delegation_manager: Address,
    slasher: Address,
    strategy_manager: Address,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all(serialize = "kebab-case"))]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct AvsAddresses {
    bls_apk_registry: Address,
    index_registry: Address,
    registry_coordinator: Address,
    service_manager: Address,
    stake_registry: Address,
}

async fn get_registry_coord_and_service_manager_addr<T, P, N>(
    args: Args,
    client: P,
) -> Result<(Address, Address), ContractError>
where
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N>,
    N: alloy_contract::private::Network,
{
    match (args.registry_coordinator, args.service_manager) {
        (Some(registry_coord_addr), _) => {
            let registry_coordinator = RegistryCoordinator::new(registry_coord_addr, &client);
            let service_manager_addr = registry_coordinator.serviceManager().call().await?._0;
            Ok((registry_coord_addr, service_manager_addr))
        }
        (_, Some(service_manager_addr)) => {
            let service_manager = IBLSSignatureChecker::new(service_manager_addr, client);
            let registry_coord_addr = service_manager.registryCoordinator().call().await?._0;
            Ok((registry_coord_addr, service_manager_addr))
        }
        _ => {
            // TODO: fix this adding a new Error type
            panic!("must provide either --registry-coordinator or --service-manager flag")
        }
    }
}

async fn get_eigenlayer_contract_addresses<T, P, N>(
    service_manager_addr: Address,
    client: P,
) -> Result<EigenAddresses, ContractError>
where
    P: alloy_contract::private::Provider<T, N>,
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    N: alloy_contract::private::Network,
{
    let service_manager = IBLSSignatureChecker::new(service_manager_addr, &client);
    let delegation_manager = service_manager.delegation().call().await?._0;
    let delegation_manager_client = DelegationManager::new(delegation_manager, &client);
    let slasher = delegation_manager_client.slasher().call().await?._0;
    let strategy_manager = delegation_manager_client.strategyManager().call().await?._0;

    Ok(EigenAddresses {
        slasher,
        delegation_manager,
        strategy_manager,
    })
}

async fn get_avs_contract_addresses<T, P, N>(
    registry_coordinator: Address,
    client: P,
) -> Result<AvsAddresses, ContractError>
where
    P: alloy_contract::private::Provider<T, N>,
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    N: alloy_contract::private::Network,
{
    let registry_coordinator_instance = RegistryCoordinator::new(registry_coordinator, &client);
    let service_manager = registry_coordinator_instance
        .serviceManager()
        .call()
        .await?
        ._0;
    let bls_apk_registry = registry_coordinator_instance
        .blsApkRegistry()
        .call()
        .await?
        ._0;
    let index_registry = registry_coordinator_instance
        .indexRegistry()
        .call()
        .await?
        ._0;
    let stake_registry = registry_coordinator_instance
        .stakeRegistry()
        .call()
        .await?
        ._0;

    Ok(AvsAddresses {
        service_manager,
        registry_coordinator,
        bls_apk_registry,
        index_registry,
        stake_registry,
    })
}

pub async fn get_addresses(args: Args) -> Result<EigenAddressesResponse, ContractError> {
    let rpc_url = args.rpc_url.clone();
    let client = get_provider(&rpc_url);
    let chain_id = client.get_chain_id().await?.to_string();
    let (registry_coord_addr, service_manager_addr) =
        get_registry_coord_and_service_manager_addr(args, client.clone()).await?;
    let avs = get_avs_contract_addresses(registry_coord_addr, client.clone()).await?;
    let eigenlayer = get_eigenlayer_contract_addresses(service_manager_addr, client).await?;

    let network = NetworkInfo { rpc_url, chain_id };
    Ok(EigenAddressesResponse {
        network,
        eigenlayer,
        avs,
    })
}

#[cfg(test)]
mod test {
    use super::{get_addresses, Args, EigenAddressesResponse, ANVIL_RPC_URL};
    use eigen_testing_utils::anvil_constants::{
        get_registry_coordinator_address, get_service_manager_address,
    };
    use tokio;

    // TODO: start anvil with the right deployed state
    #[tokio::test]
    async fn egnaddrs_with_service_manager_flag() {
        let service_manager_address = get_service_manager_address().await;

        let args = Args {
            registry_coordinator: None,
            service_manager: Some(service_manager_address),
            rpc_url: ANVIL_RPC_URL.into(),
        };
        let expected_addresses: EigenAddressesResponse = serde_json::from_str(
            r#"{
            "avs": {
              "bls-apk-registry": "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
              "index-registry": "0x851356ae760d987E095750cCeb3bC6014560891C",
              "registry-coordinator": "0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9",
              "service-manager": "0x84eA74d481Ee0A5332c457a4d796187F6Ba67fEB",
              "stake-registry": "0xf5059a5D33d5853360D16C683c16e67980206f36"
            },
            "eigenlayer": {
              "delegation-manager": "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9",
              "slasher": "0xa513E6E4b8f2a923D98304ec87F64353C4D5C853",
              "strategy-manager": "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707"
            },
            "network": {
              "chain-id": "31337",
              "rpc-url": "http://localhost:8545"
            }
          }"#,
        )
        .unwrap();
        let addresses = get_addresses(args).await.unwrap();

        assert_eq!(expected_addresses, addresses);
    }

    #[tokio::test]
    async fn egnaddrs_with_registry_coordinator_flag() {
        let registry_coordinator_address = get_registry_coordinator_address().await;

        let args = Args {
            registry_coordinator: Some(registry_coordinator_address),
            service_manager: None,
            rpc_url: ANVIL_RPC_URL.into(),
        };
        let expected_addresses: EigenAddressesResponse = serde_json::from_str(
            r#"{
            "avs": {
              "bls-apk-registry": "0x1613beB3B2C4f22Ee086B2b38C1476A3cE7f78E8",
              "index-registry": "0x851356ae760d987E095750cCeb3bC6014560891C",
              "registry-coordinator": "0xa82fF9aFd8f496c3d6ac40E2a0F282E47488CFc9",
              "service-manager": "0x84eA74d481Ee0A5332c457a4d796187F6Ba67fEB",
              "stake-registry": "0xf5059a5D33d5853360D16C683c16e67980206f36"
            },
            "eigenlayer": {
              "delegation-manager": "0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9",
              "slasher": "0xa513E6E4b8f2a923D98304ec87F64353C4D5C853",
              "strategy-manager": "0x5FC8d32690cc91D4c39d9d3abcBD16989F875707"
            },
            "network": {
              "chain-id": "31337",
              "rpc-url": "http://localhost:8545"
            }
          }"#,
        )
        .unwrap();

        let addresses = get_addresses(args).await.unwrap();

        assert_eq!(expected_addresses, addresses);
    }
}
