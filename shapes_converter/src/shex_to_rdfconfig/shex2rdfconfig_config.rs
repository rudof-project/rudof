use std::{fs, io};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct ShEx2RdfConfigConfig {}

impl ShEx2RdfConfigConfig {
    pub fn from_file(file_name: &str) -> Result<ShEx2SparqlConfig, ShEx2SparqlConfigError> {
        let config_str = fs::read_to_string(file_name).map_err(|e| {
            ShEx2RdfConfigConfigError::ReadingConfigError {
                path_name: file_name.to_string(),
                error: e,
            }
        })?;
        serde_yaml_ng::from_str::<ShEx2RdfConfigConfig>(&config_str).map_err(|e| {
            ShEx2SparqlConfigError::YamlError {
                path_name: file_name.to_string(),
                error: e,
            }
        })
    }
}

#[derive(Error, Debug)]
pub enum ShEx2RdfConfigConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yaml_ng::Error,
    },
}
