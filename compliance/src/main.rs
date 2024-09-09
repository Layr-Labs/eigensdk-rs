//! Runs tests on both implementations of the SDK (Rust and Go).

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]

mod run_tests;
mod utils;

use convert_case::{Case, Casing};
use run_tests::{parse_go_output, parse_rust_output, run_go_test, run_rust_test};
use std::io::Error;
use utils::read_lines;

// TODO! test:
// go: TestAvsRegistryServiceChainCaller_GetOperatorsAvsState
// rust: test_get_operator_avs_state
//
//
// TestAvsRegistryServiceChainCaller_GetOperatorAvsState
// TestAvsRegistryServiceChainCaller_GetOperatorsAvsState

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
    let rust_function_test = format!("test_{}", line.to_case(Case::Snake));
    let go_function_test = line.to_case(Case::Pascal).to_string();
    println!("{}", rust_function_test);
    println!("{}", go_function_test);

    let ret_rust = run_rust_test(rust_sdk_path, &rust_function_test)?;
    let ret_rust_parsed = parse_rust_output(&ret_rust.stdout).unwrap();

    let ret_go = run_go_test(go_sdk_path, &go_function_test)?;
    let ret_go_parsed = parse_go_output(&ret_go.stdout).unwrap();

    println!("status rust: {}", ret_rust.status);
    println!("output rust: {}", ret_rust_parsed[0]);
    println!("count tests run: {}", ret_rust_parsed.len());

    println!("==============");

    println!("status go: {}", ret_go.status);
    println!("output go: {}", ret_go_parsed[0]);
    println!("count tests run: {}", ret_go_parsed.len());

    // TODO! check if the test was run in the go/rust output and if the status is 0
    Ok(0)
}
