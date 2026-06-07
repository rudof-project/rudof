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
    data_needs_fixup: bool,
}

impl ShaclConfig {
    pub fn new() -> Self {
        Self {
            data: Self::default_data_config(),
            data_needs_fixup: false
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
impl ShaclConfig {
    #[inline] fn default_data_config() -> RdfDataConfig { RdfDataConfig::default() }

    pub fn fixup(&mut self, rdf_data: RdfDataConfig) {
        if self.data_needs_fixup {
            self.data = rdf_data;
        }
    }
}

impl<'de> Deserialize<'de> for ShaclConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "rdf", default)]
            data: Option<RdfDataConfig>
        }

        let raw = Raw::deserialize(deserializer)?;

        Ok(Self {
            data_needs_fixup: raw.data.is_none(),
            data: raw.data.unwrap_or(Self::default_data_config())
        })
    }
}

impl Default for ShaclConfig {
    fn default() -> Self {
        Self::new()
    }
}
