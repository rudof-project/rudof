use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize};
#[cfg(not(target_family = "wasm"))]
use std::io::Read;
#[cfg(not(target_family = "wasm"))]
use std::path::Path;
use thiserror::Error;

/// This struct can be used to define configuration of RDF data readers
#[derive(Deserialize, PartialEq, Debug, Clone)]
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
}

impl Default for QueryConfig {
    fn default() -> Self {
        Self::new()
    }
}


