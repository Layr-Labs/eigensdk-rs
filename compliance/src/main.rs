//! Runs tests on both implementations of the SDK (Rust and Go).

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]

mod run_tests;
mod utils;

use convert_case::{Case, Casing};
use run_tests::{run_go_test, run_rust_test};
use std::io::Error;
use utils::read_lines;

// TODO! test:
// go: TestAvsRegistryServiceChainCaller_GetOperatorsAvsState
// rust: test_get_operator_avs_state

fn main() {
    let _ = dotenvy::dotenv();
    let rust_sdk_path = std::env::var("RUST_SDK_PATH").unwrap_or("../".to_string());
    let go_sdk_path = std::env::var("GO_SDK_PATH").unwrap_or("../".to_string());

    let Ok(lines) = read_lines("./function_list.txt") else {
        return;
    };

    for line in lines.map_while(Result::ok) {
        let _ = process_line(&line, rust_sdk_path.as_str(), go_sdk_path.as_str());
    }
}

fn process_line(line: &str, rust_sdk_path: &str, go_sdk_path: &str) -> Result<u8, Error> {
    println!("Run tests: {line}");
    let ret_rust = run_rust_test(rust_sdk_path, "eigen-services-blsaggregation", line);

    println!("status rust: {}", ret_rust.unwrap().status);

    let ret_go = run_go_test(
        go_sdk_path,
        "TestAvsRegistryServiceChainCaller_GetOperatorsAvsState",
    )?;

    println!("status go: {}", ret_go.status);

    //cargo test -p eigen-services-blsaggregation "$line" -- --nocapture
    Ok(0)
}
