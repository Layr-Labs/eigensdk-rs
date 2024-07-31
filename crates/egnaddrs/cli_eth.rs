use alloy_primitives::Address;
use clap::Parser;
use eigen_utils::{binding::RegistryCoordinator, get_provider};
use ethers::providers::{Http, Middleware, Provider};
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

#[tokio::main]
async fn main() {
    let args = Args::parse();
    print_addrs(args).await;
}

async fn print_addrs(args: Args) {
    // The default RPC URL is the local anvil address
    let client = Provider::<Http>::try_from(args.rpc_url).expect("Invalid RPC URL"); // check get_provide utl
    let chain_id = client.get_chainid().await.expect("Invalid RPC URL");
    println!("{}", chain_id);

    // TODO: get service_manager_addr and registry_coordinator_addr
    // use service_manager to get eigen layer contracts addresses (slasher, delegation-manager, strategy-manager)
    // use registry_coordinator to get avs contracts addresses (bls-apk-registry, index-registry, stake-registry)
}
