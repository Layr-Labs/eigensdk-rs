use alloy_contract;
use alloy_primitives::Address;
use alloy_provider::Provider;
use clap::Parser;
use eigen_utils::{
    binding::{ContractsRegistry, DelegationManager, IBLSSignatureChecker},
    get_provider,
};
use serde::Serialize;
use serde_json;
use tokio;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Used to help debug and test deployments and contract setups.",
    long_about = "This utility facilitates the debugging and testing of Eigenlayer and AVS contract deployments by retrieving and displaying a comprehensive list of contract addresses. Starting from an initial contract address provided, it recursively identifies and prints addresses for all relevant Eigenlayer and AVS contracts within the network. This includes service managers, registry coordinators, and various registries, thus providing a view of the deployment's structure within the network."
)]
struct Args {
    #[arg(long, help = "ServiceManager contract address")]
    service_manager: Option<Address>,

    #[arg(long, help = "BLSRegistryCoordinatorWithIndices contract address")]
    registry_coordinator: Option<Address>,

    #[arg(long, help = "rpc url", default_value = "http://localhost:8545")]
    rpc_url: String,
}

#[derive(Serialize)]
pub struct EigenAddressesResponse {
    network: NetworkInfo,
    eigenlayer: EigenAddresses,
    avs: AvsAddresses,
}

#[derive(Serialize)]
pub struct NetworkInfo {
    rpc_url: String,
    chain_id: u64,
}

#[derive(Serialize)]
pub struct EigenAddresses {
    slasher: Address,
    delegation_manager: Address,
    strategy_manager: Address,
}

#[derive(Serialize)]
pub struct AvsAddresses {
    service_manager: Address,
    registry_coordinator: Address,
    bls_apk_registry: Address,
    index_registry: Address,
    stake_registry: Address,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addresses = get_addresses(args).await;

    println!("{}", serde_json::to_string_pretty(&addresses).unwrap());
}

async fn get_registry_coord_and_service_manager_addr<T, P, N>(
    args: Args,
    client: P,
) -> (Address, Address)
where
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    P: alloy_contract::private::Provider<T, N>,
    N: alloy_contract::private::Network,
{
    if let Some(registry_coord_addr) = args.registry_coordinator {
        let contracts_registry = ContractsRegistry::new(registry_coord_addr, client);

        let service_manager_addr = contracts_registry
            .contracts("mockAvsServiceManager".to_string())
            .call()
            .await
            .unwrap()
            ._0;

        (registry_coord_addr, service_manager_addr)
    } else if let Some(service_manager_addr) = args.service_manager {
        let service_manager = IBLSSignatureChecker::new(service_manager_addr, client);
        let registry_coord_addr = service_manager
            .registryCoordinator()
            .call()
            .await
            .unwrap()
            ._0;

        (registry_coord_addr, service_manager_addr)
    } else {
        // raise error
        panic!()
    }
}

async fn get_eigenlayer_contract_addresses<T, P, N>(
    service_manager_addr: Address,
    client: P,
) -> EigenAddresses
where
    P: alloy_contract::private::Provider<T, N>,
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    N: alloy_contract::private::Network,
{
    let service_manager = IBLSSignatureChecker::new(service_manager_addr, &client);
    let delegation_manager_addr = service_manager.delegation().call().await.unwrap()._0;
    let delegation_manager_client = DelegationManager::new(delegation_manager_addr, &client);
    let slasher_addr = delegation_manager_client.slasher().call().await.unwrap()._0;
    let strategy_manager_addr = delegation_manager_client
        .strategyManager()
        .call()
        .await
        .unwrap()
        ._0;

    EigenAddresses {
        slasher: slasher_addr,
        delegation_manager: delegation_manager_addr,
        strategy_manager: strategy_manager_addr,
    }
}

async fn get_avs_contract_addresses<T, P, N>(
    registry_coord_addr: Address,
    client: P,
) -> AvsAddresses
where
    P: alloy_contract::private::Provider<T, N>,
    T: alloy_contract::private::Transport + ::core::clone::Clone,
    N: alloy_contract::private::Network,
{
    let contracts_registry = ContractsRegistry::new(registry_coord_addr, &client);
    let service_manager_addr = contracts_registry
        .contracts("ServiceManagerBase".to_string())
        .call()
        .await
        .unwrap()
        ._0;
    let bls_pubkey_apk_addr = contracts_registry
        .contracts("blsApkRegistry".to_string())
        .call()
        .await
        .unwrap()
        ._0;
    let index_registry_addr = contracts_registry
        .contracts("indexRegistry".to_string())
        .call()
        .await
        .unwrap()
        ._0;
    let stake_registry_addr = contracts_registry
        .contracts("stakeRegistry".to_string())
        .call()
        .await
        .unwrap()
        ._0;

    AvsAddresses {
        service_manager: service_manager_addr,
        registry_coordinator: registry_coord_addr,
        bls_apk_registry: bls_pubkey_apk_addr,
        index_registry: index_registry_addr,
        stake_registry: stake_registry_addr,
    }
}

async fn get_addresses(args: Args) -> EigenAddressesResponse {
    let rpc_url = args.rpc_url.clone();
    let client = get_provider(&rpc_url);
    let chain_id = client.get_chain_id().await.unwrap();
    let (registry_coord_addr, service_manager_addr) =
        get_registry_coord_and_service_manager_addr(args, client.clone()).await;
    let avs = get_avs_contract_addresses(registry_coord_addr, client.clone()).await;

    let eigenlayer = get_eigenlayer_contract_addresses(service_manager_addr, client).await;

    let network = NetworkInfo { rpc_url, chain_id };
    EigenAddressesResponse {
        network,
        eigenlayer,
        avs,
    }
}

#[cfg(test)]
mod test {
    use crate::get_addresses;

    use super::Args;
    use eigen_testing_utils::anvil_constants::{
        get_registry_coordinator_address, get_service_manager_address,
    };
    use tokio;
    #[tokio::test]
    async fn egnaddrs_with_service_manager_flag() {
        let service_manager_address = get_service_manager_address().await;
        let args = Args {
            registry_coordinator: None,
            service_manager: Some(service_manager_address),
            rpc_url: "http://localhost:8545".into(),
        };
        let addresses = get_addresses(args).await;
        // TODO: add corresponding assert
        assert!(true);
    }

    #[tokio::test]
    async fn egnaddrs_with_registry_coordinator_flag() {
        let registry_coordinator_address = get_registry_coordinator_address().await;
        let args = Args {
            registry_coordinator: Some(registry_coordinator_address),
            service_manager: None,
            rpc_url: "http://localhost:8545".into(),
        };
        let addresses = get_addresses(args).await;
        // TODO: add corresponding assert
        assert!(true);
    }
}
