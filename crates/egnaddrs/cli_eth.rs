use alloy_primitives::Address;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long)]
    service_manager: Option<Address>,

    #[arg(long)]
    registry_coordinator: Option<Address>,

    #[arg(long)]
    rpc_url: String,
}

fn main() {
    let args = Args::parse();
    print_addrs(args.rpc_url);
}

fn print_addrs(param: String) {
    println!("print_addrs: {}", param);
}
