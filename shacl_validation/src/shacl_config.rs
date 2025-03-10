use std::{io, path::Path};

use srdf::RdfDataConfig;
use thiserror::Error;

use serde::{Deserialize, Serialize};

/// This struct can be used to define configuration of SHACL
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ShaclConfig {
    pub data: Option<RdfDataConfig>,
}

impl ShaclConfig {
    pub fn new() -> ShaclConfig {
        Self {
            data: Some(RdfDataConfig::default()),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ShaclConfig, ShaclConfigError> {
        let path_name = path.as_ref().display().to_string();
        let f = std::fs::File::open(path).map_err(|e| ShaclConfigError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;

        let config: ShaclConfig =
            serde_yaml_ng::from_reader(f).map_err(|e| ShaclConfigError::YamlError {
                path_name: path_name.to_string(),
                error: e,
            })?;
        Ok(config)
    }
}

impl Default for ShaclConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ShaclConfigError {
    #[error("Reading SHACL Config path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading SHACL config YAML from {path_name:?}. Error: {error:?}")]
    YamlError {
        path_name: String,
        error: serde_yaml_ng::Error,
    },
}
