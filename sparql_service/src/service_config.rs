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
    /// This will be false if no base option is found in the [`ServiceConfig`] section,
    /// If it's true, it can be overriden by [`rudof_lib`](https://crates.io/rudof_lib).
    base_needs_fixup: bool,
}

impl ServiceConfig {
    pub fn new() -> ServiceConfig {
        Self {
            base: Self::default_iri(),
            base_needs_fixup: false
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<ServiceConfig, ServiceConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ServiceConfigError::ReadingConfigError {
            path_name: path_name.clone(),
            error: e,
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s)
            .map_err(|e| ServiceConfigError::ReadingConfigError {
                path_name: path_name.clone(),
                error: e,
            })?;

        let config: ServiceConfig = toml::from_str(s.as_str()).map_err(|e| ServiceConfigError::TomlError {
            path_name: path_name.to_string(),
            error: e,
        })?;
        Ok(config)
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

    pub fn fixup(&mut self, base_iri: Option<IriS>) {
        if self.base_needs_fixup {
            self.base_needs_fixup = false;
            self.base = base_iri;
        }
    }
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self::new()
    }
}




#[derive(Error, Debug)]
pub enum ServiceConfigError {
    #[cfg(not(target_family = "wasm"))]
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: std::io::Error },

    #[error("Reading TOML from {path_name:?}. Error: {error:?}")]
    TomlError { path_name: String, error: toml::de::Error },
}
