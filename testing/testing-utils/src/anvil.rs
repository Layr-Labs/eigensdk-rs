use std::path::{Path, PathBuf};
use testcontainers::{
    core::{ExecCommand, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};
const ANVIL_IMAGE: &str = "ghcr.io/foundry-rs/foundry";
const ANVIL_TAG: &str = "latest";
const M2_ANVIL_STATE_PATH: &str =
    "./crates/m2_contracts/anvil/m2_contracts_deployed_anvil_state/state.json"; // relative path from the project root
const OPERATOR_SET_ANVIL_STATE_PATH: &str =
    "./crates/operator_sets_contracts/anvil/operatorset_contracts_deployed_anvil_state/state.json";
fn workspace_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .unwrap()
        .stdout;
    let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
    cargo_path.parent().unwrap().to_path_buf()
}

/// Mine Anvil blocks.
pub async fn mine_anvil_blocks(container: &ContainerAsync<GenericImage>, n: u32) {
    let mut output = container
        .exec(ExecCommand::new([
            "cast",
            "rpc",
            "anvil_mine",
            n.to_string().as_str(),
        ]))
        .await
        .expect("Failed to mine anvil blocks");

    // blocking operation until the mining execution finishes
    output.stdout_to_vec().await.unwrap();
    assert_eq!(output.exit_code().await.unwrap().unwrap(), 0);
}

/// Start an anvil container for testing, using the dump state file `ANVIL_STATE_PATH`
async fn start_anvil_with_state(
    state_path: &str,
) -> (ContainerAsync<GenericImage>, String, String) {
    let relative_path = PathBuf::from(state_path);
    let absolute_path = workspace_dir().join(relative_path);
    let absolute_path_str = absolute_path.to_str().unwrap();

    let container = GenericImage::new(ANVIL_IMAGE, ANVIL_TAG)
        .with_entrypoint("anvil")
        .with_wait_for(WaitFor::message_on_stdout("Listening on"))
        .with_exposed_port(8545.tcp())
        .with_mount(testcontainers::core::Mount::bind_mount(
            absolute_path_str,
            "/state.json",
        ))
        .with_cmd([
            "--host",
            "0.0.0.0",
            "--load-state",
            "/state.json",
            "--base-fee",
            "0",
            "--gas-price",
            "0",
            "--port",
            "8545",
        ])
        .start()
        .await
        .unwrap();

    let port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(8545.tcp())
        .unwrap();

    let http_endpoint = format!("http://localhost:{port}");
    let ws_endpoint = format!("ws://localhost:{port}");

    (container, http_endpoint, ws_endpoint)
}

/// Start an anvil container for testing, using the dump state file for M2 contracts
pub async fn start_m2_anvil_container() -> (ContainerAsync<GenericImage>, String, String) {
    start_anvil_with_state(M2_ANVIL_STATE_PATH).await
}

/// Start an anvil container for testing, using the dump state file for operator sets
pub async fn start_anvil_container() -> (ContainerAsync<GenericImage>, String, String) {
    start_anvil_with_state(OPERATOR_SET_ANVIL_STATE_PATH).await
}

/// Deposit 1 eth to the account in anvil
pub async fn set_account_balance(container: &ContainerAsync<GenericImage>, address: &str) {
    let mut output = container
        .exec(ExecCommand::new([
            "cast",
            "rpc",
            "anvil_setBalance",
            address,
            "0xDE0B6B3A7640000", // 1 ETH in WEI
        ]))
        .await
        .expect("Failed to set balance to account");

    // blocking operation until the execution finishes
    output.stdout_to_vec().await.unwrap();
}
