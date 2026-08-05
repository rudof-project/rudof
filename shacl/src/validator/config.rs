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
