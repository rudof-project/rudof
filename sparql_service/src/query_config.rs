use rudof_config::TomlConfig;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};

/// This struct can be used to define configuration of RDF data readers
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default)]
pub struct QueryConfig {
    /// Default base to resolve relative IRIs, if it is `None` relative IRIs will be marked as errors`
    #[serde(rename = "rdf", skip_serializing)]
    pub(crate) data_config: RdfDataConfig,
}

impl QueryConfig {
    pub fn new() -> Self {
        Self {
            data_config: Self::default_data_config(),
        }
    }

    pub fn with_data_config(mut self, cfg: RdfDataConfig) -> Self {
        self.data_config = cfg;
        self
    }
}

impl QueryConfig {
    pub fn data_config(&self) -> &RdfDataConfig {
        &self.data_config
    }
}

#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl QueryConfig {
    #[inline] fn default_data_config() -> RdfDataConfig { RdfDataConfig::default() }
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlConfig for QueryConfig {}

