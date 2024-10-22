# Eigen-telemetry

To enable and use telemetry in your avs/code, you need to include the SDK crates with the feature telemetry enabled.

In the main function the code must set the config of the telemetry with the given parameters:

``` rust
fn main() {
    let _ = eigen_telemetry::telemetry::Telemetry::set_config("YOUR_TELEMETRY_KEY", "YOUR_USER_ID");
    // if this sentence returns an error, that's a unrecoverable error
    // ...
}
```
