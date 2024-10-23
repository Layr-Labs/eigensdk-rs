# Eigen-telemetry

To enable and use telemetry in your avs/code, you need to include `eigen-telemetry` crate and the rest of the SDK crates with the feature telemetry enabled. To do so, in your `Cargo.toml` file put:

``` toml
eigen-telemetry = "*"
eigen-client-avsregistry = { version = "*", features = "telemetry" }
```

In the main function the code must set the config of the telemetry with the given parameters:

``` rust
fn main() {
    let _ = eigen_telemetry::telemetry::Telemetry::set_config("YOUR_TELEMETRY_KEY", "YOUR_USER_ID");

    // ... the rest of the code ...
}
```
