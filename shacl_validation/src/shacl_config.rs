use rdf::rdf_core::RdfDataConfig;
use std::io::Read;
use std::{io, path::Path};
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
        let mut f =
            std::fs::File::open(path).map_err(|e| ShaclConfigError::ReadingConfigError {
                path_name: path_name.clone(),
                error: e,
            })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ShaclConfigError::ReadingConfigError {
                path_name: path_name.clone(),
                error: e,
            })?;
        let config: ShaclConfig =
            toml::from_str(s.as_str()).map_err(|e| ShaclConfigError::TomlError {
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

    #[error("Reading SHACL config TOML from {path_name:?}. Error: {error:?}")]
    TomlError {
        path_name: String,
        error: toml::de::Error,
    },
}
