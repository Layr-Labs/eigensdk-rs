use serde::{de::DeserializeOwned, Deserialize};
use std::fmt::Debug;
use std::fs::File;
use std::{env, io::BufReader};

/// Test data for generic loading of JSON data files.
#[derive(Deserialize, Debug)]
pub struct TestData<T> {
    /// Input data read from JSON file.
    pub input: T,
    // output: BlsAggregationServiceResponse,
}

impl<T: DeserializeOwned> TestData<T> {
    /// Create a new instance of `TestData` with the given input data.
    /// If the `TEST_DATA_PATH` environment variable is set, it reads the JSON file from the path,
    /// otherwise (or if it fails) it uses the default input data.
    ///
    /// # Arguments
    ///
    /// * `default_input` - The default input data to use if the JSON file is not found or fails to read.
    ///
    /// # Returns
    ///
    /// A new instance of `TestData` with the input data read from the JSON file or the default input data.
    pub fn new(default_input: T) -> Self {
        if let Ok(path) = env::var("TEST_DATA_PATH") {
            let Ok(file) = File::open(path) else {
                return TestData {
                    input: default_input,
                };
            };
            let reader = BufReader::new(file);
            let Ok(ret) = serde_json::from_reader(reader) else {
                return TestData {
                    input: default_input,
                };
            };
            ret
        } else {
            // use default values
            TestData {
                input: default_input,
            }
        }
    }
}
