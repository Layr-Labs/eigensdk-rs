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
    pub fn new(default_input: T) -> Self {
        if let Ok(path) = env::var("TEST_DATA_PATH") {
            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);
            serde_json::from_reader(reader).unwrap()
        } else {
            // use default values
            TestData {
                input: default_input,
            }
        }
    }
}
