pub mod config;

use config::Config;

fn main() {
    println!("Hello, world!");

    let config = Config::new();
    println!("{:?}", config);
}
