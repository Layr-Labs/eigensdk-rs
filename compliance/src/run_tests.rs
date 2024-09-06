use std::io::Error;
use std::process::{Command, Output};

pub(crate) fn run_rust_test(
    rust_repo_path: &str,
    package_name: &str,
    test_name: &str,
) -> Result<Output, Error> {
    Command::new("cargo")
        .current_dir(rust_repo_path)
        .arg("test")
        .arg("-p")
        .arg(package_name)
        .arg(test_name)
        .arg("--")
        .arg("--nocapture")
        .output()
}

// go test ./... -run TestAvsRegistryServiceChainCaller_GetOperatorsAvsState -v -args -data="./xzy.json"
pub(crate) fn run_go_test(rust_repo_path: &str, test_name: &str) -> Result<Output, Error> {
    Command::new("go")
        .current_dir(rust_repo_path)
        .arg("test")
        .arg("./...")
        .arg("-run")
        .arg(test_name)
        .output()
}
