use clap::Parser;
use egnkey::{args::Args, execute_egnkey};

#[tokio::main]
async fn main() {
    let args = Args::parse();
    execute_egnkey(args).unwrap();
}
