use crate::ANVIL_RPC_URL;
use alloy_primitives::Address;
use clap::{ArgGroup, Parser, Subcommand};

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
        alias = "a",
        group(
            ArgGroup::new("manager_or_coordinator")
                .required(true)
                .args(&["service_manager", "registry_coordinator"]),
        )
    )]
    GetAddresses {
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

        #[arg(long, help = "rpc url", default_value = ANVIL_RPC_URL)]
        rpc_url: String,
    },
}
