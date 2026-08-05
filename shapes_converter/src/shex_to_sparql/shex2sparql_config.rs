use std::fs::File;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Deserializer};
use shex_validation::ShExConfig;
use thiserror::Error;

#[derive(PartialEq, Debug, Clone)]
pub struct ShEx2SparqlConfig {
    pub(crate) this_variable_name: String,
    pub(crate) shex: ShExConfig,
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ShEx2SparqlConfig {
    #[inline] fn default_this_variable_name() -> String { "this".to_string() }
    #[inline] fn default_shex() -> ShExConfig { ShExConfig::default() }
}

impl ShEx2SparqlConfig {
    pub fn new() -> Self {
        Self {
            this_variable_name: Self::default_this_variable_name(),
            shex: Self::default_shex(),
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ShEx2SparqlConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = File::open(path).map_err(|e| ShEx2SparqlConfigError::ReadingConfigError {
            error: e.to_string(),
            path_name: path_name.clone(),
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s).map_err(|e| ShEx2SparqlConfigError::ReadingConfigError {
            error: e.to_string(),
            path_name: path_name.clone(),
        })?;
        let config: ShEx2SparqlConfig = toml::from_str(&s.as_str()).map_err(|e| ShEx2SparqlConfigError::TomlError {
            error: e.to_string(),
            path_name: path_name.clone(),
        })?;
        Ok(config)
    }

    pub fn with_this_variable_name(mut self, name: String) -> Self {
        self.this_variable_name = name;
        self
    }

    pub fn with_shex(mut self, cfg: ShExConfig) -> Self {
        self.shex = cfg;
        self
    }
}

impl ShEx2SparqlConfig {
    pub fn this_variable_name(&self) -> &String {
        &self.this_variable_name
    }

    pub fn shex(&self) -> &ShExConfig {
        &self.shex
    }
}

impl Default for ShEx2SparqlConfig {
    fn default() -> Self {
        Self::new()
    }
}


