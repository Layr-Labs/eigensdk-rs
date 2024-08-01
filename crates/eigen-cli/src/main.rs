use clap::Parser;
use eigen_cli::{args::Args, eigen_address::EigenAddresses};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addresses = EigenAddresses::get_addresses(args).await.unwrap();

    println!("{}", serde_json::to_string_pretty(&addresses).unwrap());
}
