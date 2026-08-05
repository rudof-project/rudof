#[cfg(not(target_family = "wasm"))]
use std::{io::Read, path::Path};

use thiserror::Error;

use rudof_iri::IriS;
use serde::{Deserialize, Deserializer};

/// This struct can be used to define configuration of RDF data readers
#[derive(PartialEq, Debug, Clone)]
pub struct ServiceConfig {
    /// Default base to resolve relative IRIs, if it is `None` relative IRIs will be marked as errors`
    pub(crate) base: Option<IriS>,
}

impl ServiceConfig {
    pub fn new() -> ServiceConfig {
        Self {
            base: Self::default_iri(),
        }
    }

    pub fn with_base(mut self, iri: Option<IriS>) -> Self {
        self.base = iri;
        self
    }
}

impl ServiceConfig {
    pub fn base(&self) -> Option<&IriS> {
        self.base.as_ref()
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ServiceConfig {
    #[inline] fn default_iri() -> Option<IriS> { None }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self::new()
    }
}





