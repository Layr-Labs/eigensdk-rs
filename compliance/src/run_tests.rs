use std::io::Error;
use std::process::{Command, Output};
use std::str::Utf8Error;

pub(crate) fn run_rust_test(rust_repo_path: &str, test_name: &str) -> Result<Output, Error> {
    // TODO: send TEST_DATA_PATH env var
    Command::new("cargo")
        .current_dir(rust_repo_path)
        .env("TEST_DATA_PATH", "/bin")
        .arg("test")
        .arg(test_name)
        .arg("--")
        .arg("--nocapture")
        .output()
}

// go test ./... -run TestAvsRegistryServiceChainCaller_GetOperatorsAvsState -v -args -data="./xzy.json"
pub(crate) fn run_go_test(go_repo_path: &str, test_name: &str) -> Result<Output, Error> {
    // TODO: send TEST_DATA_PATH env var
    Command::new("go")
        .current_dir(go_repo_path)
        .arg("test")
        .arg("./...")
        .arg("-run")
        .arg(test_name)
        .output()
}

/// Parses the output of a Go test command and returns a vector with the test names that were
/// effectively run.
pub(crate) fn parse_go_output(stdout: &[u8]) -> Result<Vec<String>, Utf8Error> {
    Ok(std::str::from_utf8(stdout)?
        .lines()
        .filter(|line| !line.contains("[no test") && line.len() > 2)
        .map(|line| line.to_string())
        .collect::<Vec<String>>())
}

pub(crate) fn parse_rust_output(stdout: &[u8]) -> Result<Vec<String>, Utf8Error> {
    Ok(std::str::from_utf8(stdout)?
        .lines()
        .filter(|line| line.contains("... ok"))
        .map(|line| line.to_string())
        .collect::<Vec<String>>())
}
