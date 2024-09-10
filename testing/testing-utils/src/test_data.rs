use serde::Deserialize;
use std::fmt::Debug;

/// Test data for generic loading of JSON data files.
#[derive(Deserialize, Debug)]
pub struct TestData<T> {
    /// Input data read from JSON file.
    pub input: T,
    // output: BlsAggregationServiceResponse,
}
