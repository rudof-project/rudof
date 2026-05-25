use crate::error::ShaclConfigError;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// This struct can be used to define the configuration of SHACLco
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ShaclConfig {
    data: Option<RdfDataConfig>,
}

impl ShaclConfig {
    pub fn new() -> Self {
        Self {
            data: Some(RdfDataConfig::default()),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ShaclConfigError> {
        let mut f = File::open(path)?;

        let mut s = String::new();
        f.read_to_string(&mut s)?;

        toml::from_str(s.as_str()).map_err(|e| ShaclConfigError::UnmarshallError(e.into()))
    }
}

impl Default for ShaclConfig {
    fn default() -> Self {
        Self::new()
    }
}
