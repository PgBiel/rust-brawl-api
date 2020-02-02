#![cfg(test)]

use std::fs::File;
use std::path::Path;

use serde::{
    self, Deserialize
};
use serde_json;
use brawl_api::Client;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct TestConfig {
    pub key: String,

    pub tags: TCTags,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Deserialize)]
pub struct TCTags {
    pub player: String,

    pub club: String,
}

/// Opens test configuration.
///
/// # Panics
///
/// Panics if opening was not possible, or parsing it failed.
pub fn open_test_config_panic() -> TestConfig {
    let path = Path::new("tests/test_config.json");

    let file = match File::open(path) {
        Err(why) => panic!(
            "Failed to open test_config.json (see tests/README.md): {}",
            why
        ),

        Ok(f) => f,
    };

    let config: TestConfig = serde_json::from_reader(file).unwrap_or_else(|e| {
        panic!("Failed to parse test_config.json (see tests/READMe.md): {}", e)
    });

    config
}

/// Creates a client with the test_config data.
pub fn create_test_client() -> Client {
    let config: TestConfig = open_test_config_panic();
    Client::new(&config.key)
}