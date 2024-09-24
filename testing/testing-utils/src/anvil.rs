use std::path::{Path, PathBuf};
use testcontainers::{
    core::{ExecCommand, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

const ANVIL_IMAGE: &str = "ghcr.io/foundry-rs/foundry";
const ANVIL_TAG: &str = "nightly-5b7e4cb3c882b28f3c32ba580de27ce7381f415a";
const ANVIL_STATE_PATH: &str = "./crates/contracts/anvil/contracts_deployed_anvil_state.json"; // relative path from the project root

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

/// Start an Anvil container for testing with contract state loaded.
pub async fn start_anvil_container() -> (ContainerAsync<GenericImage>, String, String) {
    let relative_path = PathBuf::from(ANVIL_STATE_PATH);
    let absolute_path = workspace_dir().join(relative_path);
    let absolute_path_str = absolute_path.to_str().unwrap();

    let container = GenericImage::new(ANVIL_IMAGE, ANVIL_TAG)
        .with_wait_for(WaitFor::message_on_stdout("Listening on"))
        .with_exposed_port(8545.tcp())
        .with_entrypoint("anvil")
        .with_mount(testcontainers::core::Mount::bind_mount(
            absolute_path_str,
            "/contracts_deployed_anvil_state.json",
        ))
        .with_cmd([
            "--host",
            "0.0.0.0",
            "--load-state",
            "/contracts_deployed_anvil_state.json",
            "--base-fee",
            "0",
            "--gas-price",
            "0",
        ])
        .start()
        .await
        .expect("Error starting anvil container");

    let port = container
        .ports()
        .await
        .unwrap()
        .map_to_host_port_ipv4(8545)
        .unwrap();

    let http_endpoint = format!("http://localhost:{}", port);
    let ws_endpoint = format!("ws://localhost:{}", port);

    mine_anvil_blocks(&container, 200).await;

    (container, http_endpoint, ws_endpoint)
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
