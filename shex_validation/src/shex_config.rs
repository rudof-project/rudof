use rudof_iri::IriS;
use rudof_rdf::rdf_core::RdfDataConfig;
use serde::{Deserialize, Serialize};
use shex_ast::ShExFormat;
use std::io::Read;
use std::path::Path;
use thiserror::Error;

/// This struct can be used to customize the behavour of ShEx validators
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ShExConfig {
    /// Show information about extended shapes
    pub show_extends: Option<bool>,

    /// Show information about schema imports
    pub show_imports: Option<bool>,

    /// Show information about shapes
    pub show_shapes: Option<bool>,

    /// Show dependencies
    pub show_dependencies: Option<bool>,

    /// Show ShEx Schema Internal Representation
    pub show_ir: Option<bool>,

    /// Default ShEx format
    pub shex_format: Option<ShExFormat>,

    /// Check if schema is well formed
    pub check_well_formed: Option<bool>,

    /// Information about RDF data config which is used for Schemas represented in RDF
    pub rdf_config_shex: Option<RdfDataConfig>,

    /// Default IRI to resolve relative IRIs
    pub base: Option<IriS>,
}

impl Default for ShExConfig {
    fn default() -> Self {
        Self {
            show_extends: Some(true),
            show_imports: Some(true),
            show_shapes: Some(true),
            show_dependencies: Some(true),
            show_ir: Some(true),
            check_well_formed: Some(true),
            rdf_config_shex: Some(RdfDataConfig::default()),
            shex_format: Some(ShExFormat::ShExC),
            base: None,
        }
    }
}

impl ShExConfig {
    pub fn rdf_config(&self) -> RdfDataConfig {
        match &self.rdf_config_shex {
            None => RdfDataConfig::default(),
            Some(c) => c.clone(),
        }
    }

    pub fn check_well_formed(&self) -> bool {
        self.check_well_formed.unwrap_or(true)
    }

    pub fn with_show_extends(mut self, flag: bool) -> Self {
        self.show_extends = Some(flag);
        self
    }

    pub fn set_show_extends(mut self, flag: bool) {
        self.show_extends = Some(flag);
    }

    pub fn with_show_imports(mut self, flag: bool) -> Self {
        self.show_imports = Some(flag);
        self
    }

    pub fn with_show_shapes(mut self, flag: bool) -> Self {
        self.show_shapes = Some(flag);
        self
    }

    pub fn with_show_dependencies(mut self, flag: bool) -> Self {
        self.show_dependencies = Some(flag);
        self
    }

    pub fn without_showing_stats(&mut self) {
        self.show_extends = Some(false);
        self.show_imports = Some(false);
        self.show_shapes = Some(false);
        self.show_dependencies = Some(false);
    }
}

#[derive(Error, Debug, Clone)]
pub enum ShExConfigError {
    #[error("Error reading config file from path {path}: {error}")]
    FromPathError { path: String, error: String },

    #[error("Error reading config file from path {path}: {error}")]
    TomlError { path: String, error: String },
}
