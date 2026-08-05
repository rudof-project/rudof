use rudof_config::TomlConfig;
use rudof_iri::IriS;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};
use shex_ast::ShExFormat;

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShExConfig {
    /// Show information about extended shapes
    #[serde(rename = "show_extends")]
    pub(crate) show_extends: bool,

    /// Show information about schema imports
    #[serde(rename = "show_imports")]
    pub(crate) show_imports: bool,

    /// Show information about shapes
    #[serde(rename = "show_shapes")]
    pub(crate) show_shapes: bool,

    /// Show dependencies
    #[serde(rename = "show_dependencies")]
    pub(crate) show_dependencies: bool,

    /// Show ShEx Schema Internal Representation
    #[serde(rename = "show_ir")]
    pub(crate) show_ir: bool,

    /// Default ShEx format
    #[serde(rename = "shex_format")]
    pub(crate) shex_format: ShExFormat,

    /// Check if schema is well formed
    #[serde(rename = "check_well_formed")]
    pub(crate) check_well_formed: bool,

    /// Information about RDF data config which is used for Schemas represented in RDF
    #[serde(rename = "rdf", skip_serializing)]
    pub(crate) rdf_config_shex: RdfDataConfig,

    /// Default IRI to resolve relative IRIs
    #[serde(rename = "base_iri", skip_serializing_if = "Option::is_none")]
    pub(crate) base: Option<IriS>,
}

/// Serde stuff
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
impl ShExConfig {
    #[inline] fn default_show_extends() -> bool { true }
    #[inline] fn default_show_imports() -> bool { true }
    #[inline] fn default_show_shapes() -> bool { true }
    #[inline] fn default_show_dependencies() -> bool { false }
    #[inline] fn default_show_ir() -> bool { false }
    #[inline] fn default_shex_format() -> ShExFormat { ShExFormat::ShExC }
    #[inline] fn default_check_well_formed() -> bool { true }
    #[inline] fn default_rdf_config_shex() -> RdfDataConfig { RdfDataConfig::new() }
    #[inline] fn default_base() -> Option<IriS> { None }
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
            shex_format: Self::default_shex_format(),
            base: Self::default_base(),
        }
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

impl TomlConfig for ShExConfig {}


