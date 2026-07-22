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
    shex_needs_fixup: bool,
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ShEx2SparqlConfig {
    #[inline] fn default_this_variable_name() -> String { "this".to_string() }
    #[inline] fn default_shex() -> ShExConfig { ShExConfig::default() }

    pub fn fixup(&mut self, shex: ShExConfig) {
        if self.shex_needs_fixup {
            self.shex_needs_fixup = false;
            self.shex = shex;
        }
    }
}

impl ShEx2SparqlConfig {
    pub fn new() -> Self {
        Self {
            this_variable_name: Self::default_this_variable_name(),
            shex: Self::default_shex(),
            shex_needs_fixup: false,
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

impl<'de> Deserialize<'de> for ShEx2SparqlConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "this_variable_name", default = "ShEx2SparqlConfig::default_this_variable_name")]
            this_variable_name: String,
            #[serde(rename = "shex", default)]
            shex: Option<ShExConfig>,
        }

        let raw = Raw::deserialize(deserializer)?;

        Ok(Self {
            this_variable_name: raw.this_variable_name,
            shex_needs_fixup: raw.shex.is_none(),
            shex: raw.shex.unwrap_or(Self::default_shex())
        })
    }
}

impl Default for ShEx2SparqlConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug, Clone)]
pub enum ShEx2SparqlConfigError {
    #[error("Reading path {path_name:?} error: {error:?}")]
    ReadingConfigError { path_name: String, error: String },

    #[error("Reading TOML from {path_name:?}. Error: {error:?}")]
    TomlError { path_name: String, error: String },
}
