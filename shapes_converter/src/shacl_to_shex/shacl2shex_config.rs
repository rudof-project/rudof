use rudof_config::TomlConfig;
use serde::{Deserialize, Serialize};
#[cfg(not(target_family = "wasm"))]
use shacl::validator::ShaclConfig;

/// Defines the configuration of the converter
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default)]
pub struct Shacl2ShExConfig {
    /// Starting shapes mode. Default: NonBNodes
    #[serde(rename = "starting_shapes_mode")]
    pub(crate) starting_shapes_mode: StartShapeMode,

    /// If true, embed blank nodes in the ShEx schema
    #[serde(rename = "embed_bnodes")]
    pub(crate) embed_bnodes: bool,

    /// SHACL configuration
    #[cfg(not(target_family = "wasm"))]
    #[serde(rename = "shacl", skip_serializing)]
    pub(crate) shacl: ShaclConfig,

    /// Add an `rdf:type` constraint for `sh:targetClass` declarations
    #[serde(rename = "add_target_class")]
    pub(crate) add_target_class: bool,
}

impl Shacl2ShExConfig {
    pub fn new() -> Self {
        Self {
            starting_shapes_mode: Self::default_starting_shapes_mode(),
            embed_bnodes: Self::default_embed_bnodes(),
            #[cfg(not(target_family = "wasm"))]
            shacl: Self::default_shacl(),
            add_target_class: Self::default_add_target_class(),
        }
    }

    pub fn with_starting_shapes_mode(mut self, mode: StartShapeMode) -> Self {
        self.starting_shapes_mode = mode;
        self
    }

    pub fn with_embed_bnodes(mut self, flag: bool) -> Self {
        self.embed_bnodes = flag;
        self
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn with_shacl(mut self, cfg: ShaclConfig) -> Self {
        self.shacl = cfg;
        self
    }

    pub fn with_add_target_class(mut self, flag: bool) -> Self {
        self.add_target_class = flag;
        self
    }
}

impl Shacl2ShExConfig {
    pub fn starting_shapes_mode(&self) -> &StartShapeMode {
        &self.starting_shapes_mode
    }

    pub fn embed_bnodes(&self) -> bool {
        self.embed_bnodes
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn shacl(&self) -> &ShaclConfig {
        &self.shacl
    }

    pub fn add_target_class(&self) -> bool {
        self.add_target_class
    }
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl Shacl2ShExConfig {
    #[inline] fn default_starting_shapes_mode() -> StartShapeMode { StartShapeMode::default() }
    #[inline] fn default_embed_bnodes() -> bool { false }
    #[cfg(not(target_family = "wasm"))]
    #[inline] fn default_shacl() -> ShaclConfig { ShaclConfig::default() }
    #[inline] fn default_add_target_class() -> bool { false }
}

impl Default for Shacl2ShExConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl TomlConfig for Shacl2ShExConfig {}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum StartShapeMode {
    /// Process shapes which are not blank nodes
    #[default]
    #[serde(rename = "non-bnodes")]
    NonBNodes,
}
