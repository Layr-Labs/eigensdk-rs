use std::{fs, path::Path};

use eyre::Result;
use serde::de::DeserializeOwned;

/// Loads a config from a file
///
/// # Arguments
///
/// * `path` - The path to the config file
///
/// # Returns
///
/// * `eyre::Result<T>` - The config struct
pub fn load_config<P, T>(path: P) -> Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let s =
        fs::read_to_string(&path).map_err(|e| eyre::eyre!("Could not read config file: {}", e))?;
    toml::from_str(&s).map_err(|e| eyre::eyre!("Could not parse config file: {}", e))
}
