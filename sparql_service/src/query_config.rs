use rudof_rdf::rdf_core::RdfDataConfig;
#[cfg(not(target_family = "wasm"))]
use std::io::Read;
#[cfg(not(target_family = "wasm"))]
use std::path::Path;
use thiserror::Error;
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
    pub fn new() -> QueryConfig {
        Self {
            data_config: Some(RdfDataConfig::default()),
        }
    }
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self::new()
    }
}


