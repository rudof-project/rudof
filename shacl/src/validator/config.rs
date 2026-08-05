use crate::error::ShaclConfigError;
use rudof_rdf::rdf_core::RdfDataConfig;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Deserializer};

/// This struct can be used to define the configuration of SHACL
#[derive(PartialEq, Debug, Clone)]
pub struct ShaclConfig {
    pub(crate) data: RdfDataConfig,
}

impl ShaclConfig {
    pub fn new() -> Self {
        Self {
            data: Self::default_data_config(),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ShaclConfigError> {
        let mut f = File::open(path)?;

        let mut s = String::new();
        f.read_to_string(&mut s)?;

        toml::from_str(s.as_str()).map_err(|e| ShaclConfigError::UnmarshallError(e.into()))
    }

    pub fn with_rdf_data(mut self, data: RdfDataConfig) -> Self {
        self.data = data;
        self
    }
}

impl ShaclConfig {
    pub fn rdf_data(&self) -> &RdfDataConfig {
        &self.data
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ShaclConfig {
    #[inline] fn default_data_config() -> RdfDataConfig { RdfDataConfig::default() }
}

impl Default for ShaclConfig {
    fn default() -> Self {
        Self::new()
    }
}
