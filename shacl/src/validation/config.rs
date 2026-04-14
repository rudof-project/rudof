use std::fs::File;
use std::io::Read;
use std::path::Path;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};
use crate::validation::error::ShaclConfigError;

/// This struct can be used to define the configuration of SHACLco
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub(crate) struct ShaclConfig {
    data: Option<RdfDataConfig>
}

impl ShaclConfig {
    pub fn new() -> Self {
        Self { data: Some(RdfDataConfig::default()) }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ShaclConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = File::open(path).map_err(|e| ShaclConfigError::ReadingConfig {
            error: e,
            path_name: path_name.clone(),
        })?;

        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ShaclConfigError::ReadingConfig {
                path_name: path_name.clone(),
                error: e,
            })?;
        
        toml::from_str(s.as_str()).map_err(|e| ShaclConfigError::Toml {
            path_name: path_name.clone(),
            error: e,
        })
    }
}

impl Default for ShaclConfig {
    fn default() -> Self {
        Self::new()
    }
}