use rudof_iri::IriS;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Deserializer, Serialize};
use shex_ast::ShExFormat;
use std::io::Read;
use std::path::Path;
use thiserror::Error;

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Debug, PartialEq, Clone)]
pub struct ShExConfig {
    /// Show information about extended shapes
    pub(crate) show_extends: bool,

    /// Show information about schema imports
    pub(crate) show_imports: bool,

    /// Show information about shapes
    pub(crate) show_shapes: bool,

    /// Show dependencies
    pub(crate) show_dependencies: bool,

    /// Show ShEx Schema Internal Representation
    pub(crate) show_ir: bool,

    /// Default ShEx format
    pub(crate) shex_format: ShExFormat,

    /// Check if schema is well formed
    pub(crate) check_well_formed: bool,

    /// Information about RDF data config which is used for Schemas represented in RDF
    pub(crate) rdf_config_shex: RdfDataConfig,
    rdf_config_needs_fixup: bool,

    /// Default IRI to resolve relative IRIs
    pub(crate) base: Option<IriS>,
    base_needs_fixup: bool,
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ShExConfig {
    #[inline] fn default_show_extends() -> bool { true }
    #[inline] fn default_show_imports() -> bool { true }
    #[inline] fn default_show_shapes() -> bool { true }
    #[inline] fn default_show_dependencies() -> bool { true }
    #[inline] fn default_show_ir() -> bool { true }
    #[inline] fn default_shex_format() -> ShExFormat { ShExFormat::ShExC }
    #[inline] fn default_check_well_formed() -> bool { true }
    #[inline] fn default_rdf_config_shex() -> RdfDataConfig { RdfDataConfig::new() }
    #[inline] fn default_base() -> Option<IriS> { None }

    pub fn fixup(&mut self, rdf_data_config: RdfDataConfig, base: Option<IriS>) {
        if self.rdf_config_needs_fixup {
            self.rdf_config_needs_fixup = false;
            self.rdf_config_shex = rdf_data_config;
        }

        if self.base_needs_fixup {
            self.base_needs_fixup = false;
            self.base = base;
        }
    }
}

impl<'de> Deserialize<'de> for ShExConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct Raw {
            #[serde(rename = "show_extends", default = "ShExConfig::default_show_extends")]
            show_extends: bool,
            #[serde(rename = "show_imports", default = "ShExConfig::default_show_imports")]
            show_imports: bool,
            #[serde(rename = "show_shapes", default = "ShExConfig::default_show_shapes")]
            show_shapes: bool,
            #[serde(rename = "show_dependencies", default = "ShExConfig::default_show_dependencies")]
            show_dependencies: bool,
            #[serde(rename = "show_ir", default = "ShExConfig::default_show_ir")]
            show_ir: bool,
            #[serde(rename = "shex_format", default = "ShExConfig::default_shex_format")]
            shex_format: ShExFormat,
            #[serde(rename = "check_well_formed", default = "ShExConfig::default_check_well_formed")]
            check_well_formed: bool,
            #[serde(rename = "rdf", default)]
            rdf_config_shex: Option<RdfDataConfig>,
            #[serde(rename = "base_iri", default)]
            base: Option<IriS>,
        }

        let raw = Raw::deserialize(deserializer)?;

        Ok(Self {
            show_extends: raw.show_extends,
            show_imports: raw.show_imports,
            show_shapes: raw.show_shapes,
            show_dependencies: raw.show_dependencies,
            show_ir: raw.show_ir,
            shex_format: raw.shex_format,
            check_well_formed: raw.check_well_formed,
            rdf_config_needs_fixup: raw.rdf_config_shex.is_none(),
            rdf_config_shex: raw.rdf_config_shex.unwrap_or(Self::default_rdf_config_shex()),
            base_needs_fixup: raw.base.is_none(),
            base: raw.base,
        })
    }
}

impl ShExConfig {
    pub fn new() -> Self {
        Self {
            show_extends: Self::default_show_extends(),
            show_imports: Self::default_show_imports(),
            show_shapes: Self::default_show_shapes(),
            show_dependencies: Self::default_show_dependencies(),
            show_ir: Self::default_show_ir(),
            check_well_formed: Self::default_check_well_formed(),
            rdf_config_shex: Self::default_rdf_config_shex(),
            rdf_config_needs_fixup: false,
            shex_format: Self::default_shex_format(),
            base: Self::default_base(),
            base_needs_fixup: false,
        }
    }

    #[cfg(not(target_family = "wasm"))]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, ShExConfigError> {
        let path_name = path.as_ref().display().to_string();
        let mut f = std::fs::File::open(path).map_err(|e| ShExConfigError::FromPathError {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        let mut s = String::new();
        f.read_to_string(&mut s).map_err(|e| ShExConfigError::FromPathError {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        let config: ShExConfig = toml::from_str(s.as_str()).map_err(|e| ShExConfigError::TomlError {
            path: path_name.clone(),
            error: e.to_string(),
        })?;
        Ok(config)
    }

    pub fn with_show_extends(mut self, flag: bool) -> Self {
        self.show_extends = flag;
        self
    }
    pub fn with_show_imports(mut self, flag: bool) -> Self {
        self.show_imports = flag;
        self
    }
    pub fn with_show_shapes(mut self, flag: bool) -> Self {
        self.show_shapes = flag;
        self
    }
    pub fn with_show_dependencies(mut self, flag: bool) -> Self {
        self.show_dependencies = flag;
        self
    }
    pub fn with_show_ir(mut self, flag: bool) -> Self {
        self.show_ir = flag;
        self
    }
    pub fn with_shex_format(mut self, format: ShExFormat) -> Self {
        self.shex_format = format;
        self
    }
    pub fn with_check_well_formed(mut self, flag: bool) -> Self {
        self.check_well_formed = flag;
        self
    }
    pub fn with_rdf_config_shex(mut self, cfg: RdfDataConfig) -> Self {
        self.rdf_config_shex = cfg;
        self
    }
    pub fn with_base(mut self, iri: Option<IriS>) -> Self {
        self.base = iri;
        self
    }

    pub fn without_showing_stats(&mut self) {
        self.show_extends = false;
        self.show_imports = false;
        self.show_shapes = false;
        self.show_dependencies = false;
    }
}

impl ShExConfig {
    pub fn show_extends(&self) -> bool {
        self.show_extends
    }

    pub fn show_imports(&self) -> bool {
        self.show_imports
    }

    pub fn show_shapes(&self) -> bool {
        self.show_shapes
    }

    pub fn show_dependencies(&self) -> bool {
        self.show_dependencies
    }

    pub fn show_ir(&self) -> bool {
        self.show_ir
    }

    pub fn shex_format(&self) -> &ShExFormat {
        &self.shex_format
    }

    pub fn check_well_formed(&self) -> bool {
        self.check_well_formed
    }

    pub fn rdf_config_shex(&self) -> &RdfDataConfig {
        &self.rdf_config_shex
    }

    pub fn base(&self) -> Option<&IriS> {
        self.base.as_ref()
    }
}

impl Default for ShExConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug, Clone)]
pub enum ShExConfigError {
    #[error("Error reading config file from path {path}: {error}")]
    FromPathError { path: String, error: String },

    #[error("Error reading config file from path {path}: {error}")]
    TomlError { path: String, error: String },
}
