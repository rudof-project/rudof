use std::{fs, io};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ShEx2SparqlConfig {
    pub this_variable_name: String,
}

impl Default for ShEx2SparqlConfig {
    fn default() -> Self {
        Self {
            this_variable_name: "this".to_string(),
        }
    }
}

impl ShEx2SparqlConfig {
    pub fn from_file(file_name: &str) -> Result<ShEx2SparqlConfig, ShEx2SparqlConfigError> {
        let config_str = fs::read_to_string(&file_name).map_err(|e| {
            ShEx2SparqlConfigError::ReadingConfigError {
                path_name: file_name.to_string(),
                error: e,
            }
        })?;
        serde_yaml::from_str::<ShEx2SparqlConfig>(&config_str).map_err(|e| {
            ShEx2SparqlConfigError::YamlError {
                path_name: file_name.to_string(),
                error: e,
            }
        })
    }
}

#[derive(Error, Debug)]
pub enum ShEx2SparqlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yaml::Error,
    },
}
