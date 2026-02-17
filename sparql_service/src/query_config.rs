use serde::{Deserialize, Serialize};
use rdf::rdf_core::RdfDataConfig;
use std::io::Read;
use std::{io, path::Path};
use thiserror::Error;

/// This struct can be used to define configuration of RDF data readers
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct QueryConfig {
    /// Default base to resolve relative IRIs, if it is `None` relative IRIs will be marked as errors`
    pub data_config: Option<RdfDataConfig>,
}

impl QueryConfig {
    pub fn new() -> QueryConfig {
        Self {
            data_config: Some(RdfDataConfig::default()),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<QueryConfig, QueryConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| QueryConfigError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| QueryConfigError::ReadingConfigError {
                path_name: path_name.clone(),
                error: e,
            })?;
        let config: QueryConfig = toml::from_str(s.as_str()).map_err(|e| QueryConfigError::TomlError {
            path_name: path_name.to_string(),
            error: e,
        })?;
        Ok(config)
    }
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum QueryConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading TOML from {path_name:?}. Error: {error:?}")]
    TomlError { path_name: String, error: toml::de::Error },
}
