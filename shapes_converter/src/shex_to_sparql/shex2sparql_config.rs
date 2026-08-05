use rudof_config::TomlConfig;
use serde::{Deserialize, Serialize};
use shex_validation::ShExConfig;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShEx2SparqlConfig {
    #[serde(rename = "this_variable_name")]
    pub(crate) this_variable_name: String,
    #[serde(rename = "shex", skip_serializing)]
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

impl TomlConfig for ShEx2SparqlConfig {}


