use std::{fs, io};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ShEx2UmlConfig {}

impl Default for ShEx2UmlConfig {
    fn default() -> Self {
        Self {}
    }
}

impl ShEx2UmlConfig {
    pub fn from_file(file_name: &str) -> Result<ShEx2UmlConfig, ShEx2UmlConfigError> {
        let config_str =
            fs::read_to_string(file_name).map_err(|e| ShEx2UmlConfigError::ReadingConfigError {
                path_name: file_name.to_string(),
                error: e,
            })?;
        serde_yaml::from_str::<ShEx2UmlConfig>(&config_str).map_err(|e| {
            ShEx2UmlConfigError::YamlError {
                path_name: file_name.to_string(),
                error: e,
            }
        })
    }
}

#[derive(Error, Debug)]
pub enum ShEx2UmlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yaml::Error,
    },
}
