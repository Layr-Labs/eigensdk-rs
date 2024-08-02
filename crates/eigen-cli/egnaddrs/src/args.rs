use crate::ANVIL_RPC_URL;
use alloy_primitives::Address;
use clap::{ArgGroup, Parser};

#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("manager_or_coordinator")
        .required(true)
        .args(&["service_manager", "registry_coordinator"]),
))]
#[command(
    version,
    about = "Used to help debug and test deployments and contract setups.",
    long_about = "This utility facilitates the debugging and testing of Eigenlayer and AVS contract deployments by retrieving and displaying a comprehensive list of contract addresses. Starting from an initial contract address provided, it recursively identifies and prints addresses for all relevant Eigenlayer and AVS contracts within the network. This includes service managers, registry coordinators, and various registries, thus providing a view of the deployment's structure within the network."
)]
pub struct Args {
    #[arg(long, help = "ServiceManager contract address")]
    pub service_manager: Option<Address>,

    #[arg(long, help = "BLSRegistryCoordinatorWithIndices contract address")]
    pub registry_coordinator: Option<Address>,

    #[arg(long, help = "rpc url", default_value = ANVIL_RPC_URL)]
    pub rpc_url: String,
}
