//! Runs tests on both implementations of the SDK (Rust and Go).

#![doc(
    html_logo_url = "https://github.com/Layr-Labs/eigensdk-rs/assets/91280922/bd13caec-3c00-4afc-839a-b83d2890beb5",
    issue_tracker_base_url = "https://github.com/Layr-Labs/eigensdk-rs/issues/"
)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::Command;


/// Reads a file line by line and returns an iterator over the lines.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // Statements here are executed when the compiled binary is called.

    let Ok(lines) = read_lines("./function_list.txt") else {
        return;
    };

    for line in lines.flatten() {
        process_line(&line);
    }
}

fn process_line(line: &str) {
    println!("Run tests: {line}");
    let ret_rust = Command::new("cargo")
            .current_dir("./Eigen/eigen-rs-lambda")
            .arg("test")
            .arg("-p")
            .arg("eigen-services-blsaggregation")
            .arg(line)
            .arg("--")
            .arg("--nocapture")
            .output()
            .expect("failed to execute process");

    println!("status rust: {}", ret_rust.status);


    // go test ./... -run TestAvsRegistryServiceChainCaller_GetOperatorsAvsState -v -args -data="./xzy.json"
    let ret_go = Command::new("go")
            .current_dir("./Eigen/eigensdk-go_lambda")
            .arg("test")
            .arg("./...")
            .arg("-run")
            .arg("TestAvsRegistryServiceChainCaller_GetOperatorsAvsState")
            .output()
            .expect("failed to execute process");

    println!("status go: {}", ret_go.status);
    println!("stdout go: {}", String::from_utf8_lossy(&ret_go.stdout));
    println!("stderr go: {}", String::from_utf8_lossy(&ret_go.stderr));

    //cargo test -p eigen-services-blsaggregation "$line" -- --nocapture
}
