use std::io::Read;
use std::{io, path::Path};

use thiserror::Error;

use iri_s::IriS;
use serde::{Deserialize, Serialize};

/// This struct can be used to define configuration of RDF data readers
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ServiceConfig {
    /// Default base to resolve relative IRIs, if it is `None` relative IRIs will be marked as errors`
    pub base: Option<IriS>,
}

impl ServiceConfig {
    pub fn new() -> ServiceConfig {
        Self {
            base: Some(IriS::new_unchecked("base://")),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ServiceConfig, ServiceConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f =
            std::fs::File::open(path).map_err(|e| ServiceConfigError::ReadingConfigError {
                path_name: path_name.clone(),
                error: e,
            })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ServiceConfigError::ReadingConfigError {
                path_name: path_name.clone(),
                error: e,
            })?;

        let config: ServiceConfig =
            toml::from_str(s.as_str()).map_err(|e| ServiceConfigError::TomlError {
                path_name: path_name.to_string(),
                error: e,
            })?;
        Ok(config)
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ServiceConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: io::Error },

    #[error("Reading TOML from {path_name:?}. Error: {error:?}")]
    TomlError {
        path_name: String,
        error: toml::de::Error,
    },
}
