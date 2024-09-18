use testcontainers::{
    core::{ExecCommand, IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    ContainerAsync, GenericImage, ImageExt,
};

/// Start an Anvil container for testing with contract state loaded.
pub async fn start_anvil_container() -> (ContainerAsync<GenericImage>, String, String) {
    let container = GenericImage::new(
        "ghcr.io/foundry-rs/foundry",
        "nightly-5b7e4cb3c882b28f3c32ba580de27ce7381f415a",
    )
    .with_wait_for(WaitFor::message_on_stdout("Listening on"))
    .with_exposed_port(8545.tcp())
    .with_entrypoint("anvil")
    .with_mount(testcontainers::core::Mount::bind_mount(
        "/Users/tomas/Lambda/eigen-rs/crates/contracts/anvil/contracts_deployed_anvil_state.json", // replace with dump path
        "/contracts_deployed_anvil_state.json",
    ))
    .with_cmd([
        "--host",
        "0.0.0.0",
        "--load-state",
        "/contracts_deployed_anvil_state.json",
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
    container
        .exec(ExecCommand::new([
            "cast",
            "rpc",
            "anvil_mine",
            n.to_string().as_str(),
        ]))
        .await
        .expect("Failed to mine anvil blocks");
}
