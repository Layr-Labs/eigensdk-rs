//! Runs tests on both implementations of the SDK (Rust and Go).

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]

mod run_tests;
mod utils;

use run_tests::{run_go_test, run_rust_test};
use utils::read_lines;

fn main() {
    let Ok(lines) = read_lines("./function_list.txt") else {
        return;
    };

    for line in lines.flatten() {
        process_line(&line);
    }
}

fn process_line(line: &str) {
    println!("Run tests: {line}");
    let ret_rust = run_rust_test(
        "./Eigen/eigen-rs-lambda",
        "eigen-services-blsaggregation",
        line,
    );

    println!("status rust: {}", ret_rust.status);

    let ret_go = run_go_test(
        "./Eigen/eigensdk-go_lambda",
        "TestAvsRegistryServiceChainCaller_GetOperatorsAvsState",
    );

    println!("status go: {}", ret_go.status);
    println!("stdout go: {}", String::from_utf8_lossy(&ret_go.stdout));
    println!("stderr go: {}", String::from_utf8_lossy(&ret_go.stderr));

    //cargo test -p eigen-services-blsaggregation "$line" -- --nocapture
}
