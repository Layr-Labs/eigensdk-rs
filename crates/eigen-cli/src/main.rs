use clap::Parser;
use eigen_cli::{
    args::{Args, Commands},
    eigen_address::ContractAddresses,
};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Commands::GetAddresses {
            service_manager,
            registry_coordinator,
            rpc_url,
        } => {
            let addresses =
                ContractAddresses::get_addresses(service_manager, registry_coordinator, rpc_url)
                    .await
                    .unwrap();
            println!("{}", serde_json::to_string_pretty(&addresses).unwrap());
        }
    }
}
